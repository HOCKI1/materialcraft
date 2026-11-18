//! Copy reachable SBS definitions with local, collision-free identities.
use std::collections::{BTreeMap, BTreeSet};

use materialcraft_core::{Graph, GraphIndex, Package, Value};

use crate::{FormatError, Result};

fn bad(message: impl Into<String>) -> FormatError {
    FormatError::BadPackage(message.into())
}

/// The location of the entry in the source project is independent of its location in the publication.
pub fn snapshot(package: &Package, entry: usize) -> Result<Package> {
    let graph = package.graph(entry).map_err(|e| bad(e.to_string()))?;
    if graph.function.is_some() {
        return Err(bad(format!("select a material graph with connected outputs: `{}`", graph.sbs_path())));
    }

    let mut result = Package::new(&graph.name);
    result.format_version.clone_from(&package.format_version);
    let mut builder = SnapshotBuilder { root: package, result, paths: BTreeMap::new(), active: BTreeSet::new(), resources: BTreeMap::new(), nodes: 0 };
    builder.copy_graph("", entry)?;
    Ok(builder.result)
}

type Key = (String, usize);
struct SnapshotBuilder<'a> {
    root: &'a Package,
    result: Package,
    paths: BTreeMap<Key, (String, usize)>,
    active: BTreeSet<Key>,
    resources: BTreeMap<(String, String), String>,
    nodes: usize,
}

impl<'a> SnapshotBuilder<'a> {
    fn package(&self, owner: &str) -> Result<&'a Package> {
        if owner.is_empty() {
            return Ok(self.root);
        }

        self.root.dependencies.get(owner).map(AsRef::as_ref).ok_or_else(|| bad(format!("missing source package `{owner}`")))
    }

    fn reference(&mut self, owner: &str, name: &str, height: &mut usize) -> Result<String> {
        let (index, _) = self.package(owner)?.graph_by_name(name).ok_or_else(|| bad(format!("missing or ambiguous graph `{name}` in `{owner}`")))?;
        let (path, child_height) = self.copy_graph(owner, index)?;
        *height = (*height).max(child_height + 1);
        Ok(path)
    }

    fn copy_graph(&mut self, owner: &str, index: usize) -> Result<(String, usize)> {
        let key = (owner.to_owned(), index);
        let source = self.package(owner)?.graph(index).map_err(|e| bad(e.to_string()))?;
        if self.active.contains(&key) {
            return Err(bad(format!("cyclic graph reference at `{}`", source.sbs_path())));
        }

        // Bound the complete path, including a shared tail copied on an earlier branch.
        // Active entries are rejected above, so cached heights here are complete.
        const MAX_DEPENDENCY_DEPTH: usize = 32;
        if let Some((path, height)) = self.paths.get(&key) {
            if self.active.len() + height > MAX_DEPENDENCY_DEPTH {
                return Err(bad("published graph dependencies exceed the depth or graph count limit"));
            }
            return Ok((path.clone(), *height));
        }

        // Bound publishing independently of the cooker's per-function and per-instance limits
        if self.active.len() >= MAX_DEPENDENCY_DEPTH || self.paths.len() >= crate::sbs::MAX_GRAPHS {
            return Err(bad("published graph dependencies exceed the depth or graph count limit"));
        }

        let mut graph = source.clone();
        if graph.function.is_none() {
            let outputs: Vec<_> = graph.nodes.iter().filter(|node| node.def == "output").map(|node| node.id).collect();
            let index = GraphIndex::new(&graph);
            if self.paths.is_empty() && !outputs.iter().any(|id| !index.incoming(*id).is_empty()) {
                return Err(bad(format!("select a material graph with connected outputs: `{}`", graph.sbs_path())));
            }

            let needed = index.required_nodes(&outputs);
            graph.nodes.retain(|node| needed.contains(&node.id) || matches!(node.def.as_str(), "input_color" | "input_grayscale" | "input_value"));
            graph.connections.retain(|c| needed.contains(&c.from) && needed.contains(&c.to));
            if graph.topo_order().len() != graph.nodes.len() {
                return Err(bad(format!("cycle in graph `{}`", graph.sbs_path())));
            }
        }

        self.nodes = self.nodes.saturating_add(graph.nodes.len());
        if self.nodes > crate::sbs::MAX_NODES {
            return Err(bad("published graph exceeds the node count limit"));
        }

        graph.folder = if self.paths.is_empty() { String::new() } else { format!("Dependencies/g{}", self.paths.len()) };
        let path = graph.sbs_path();
        self.paths.insert(key.clone(), (path.clone(), 0));
        let mut height = 1;
        self.active.insert(key.clone());
        for instance in graph.instances_mut() {
            let package = if instance.package.is_empty() { owner } else { &instance.package };
            instance.graph = self.reference(package, &instance.graph, &mut height)?;
            instance.package.clear();
        }
        for node in &mut graph.nodes {
            if node.def == materialcraft_core::MISSING_NODE {
                return Err(bad(format!("unsupported node `{}` in `{path}`", node.label)));
            }

            if node.def == "graph_instance" {
                if let Some(instance) = &node.instance {
                    node.params.insert("graph".into(), Value::Text(instance.graph.clone()));
                } else if let Some(Value::Text(target)) = node.params.get_mut("graph") {
                    *target = self.reference(owner, target, &mut height)?;
                } else {
                    return Err(bad(format!("missing graph reference in `{path}`")));
                }
            }

            match node.def.as_str() {
                "bitmap" | "svg" => {
                    let Some(Value::Text(name)) = node.params.get_mut("resource").filter(|v| !matches!(v, Value::Text(s) if s.is_empty())) else {
                        return Err(bad(format!("missing resource for {} node {} in `{path}`", node.def, node.id.0)));
                    };
                    *name = self.resource(owner, name)?;
                }
                "text" => {
                    if let Some(Value::Text(name)) = node.params.get_mut("fontdata")
                        && !name.is_empty()
                    {
                        *name = self.resource(owner, name)?;
                    }
                }
                _ => {}
            }
        }
        for name in graph.input_images.values_mut() {
            *name = self.resource(owner, name)?;
        }

        self.active.remove(&key);
        self.paths.insert(key, (path.clone(), height));
        self.result.graphs.push(graph);
        Ok((path, height))
    }

    fn resource(&mut self, owner: &str, name: &str) -> Result<String> {
        let key = (owner.to_string(), name.to_string());
        if let Some(local) = self.resources.get(&key) {
            return Ok(local.clone());
        }

        let mut resource = self.package(owner)?.resource(name).cloned().ok_or_else(|| bad(format!("missing resource `{name}` in `{owner}`")))?;
        if resource.bytes.is_empty() {
            return Err(bad(format!("missing resource bytes for `{name}` ({})", resource.source)));
        }

        let local = format!("asset_{}", self.resources.len() + 1);
        resource.name.clone_from(&local);
        resource.folder = "Resources".into();
        resource.source.clear();
        self.resources.insert(key, local.clone());
        self.result.resources.push(resource);
        Ok(local)
    }
}

pub fn entry_graph(pkg: &Package) -> Result<usize> {
    let mut roots = pkg.graphs.iter().enumerate().filter(|(_, g)| g.folder.is_empty());
    match (roots.next(), roots.next()) {
        (Some((index, Graph { function: None, .. })), None) => Ok(index),
        _ => Err(bad("source.sbs must have exactly one root material graph; put helper graphs and functions in a dependency folder")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn graph(name: &str, children: &[String]) -> Graph {
        let mut graph = Graph::new(name);
        if children.is_empty() {
            let source = graph.add_node("uniform", [0.0; 2]);
            let output = graph.add_node("output", [0.0; 2]);
            graph.connect(source, "output", output, "input").unwrap();
        }
        for child in children {
            let source = graph.add_node("graph_instance", [0.0; 2]);
            graph.node_mut(source).unwrap().params.insert("graph".into(), Value::Text(child.clone()));
            let output = graph.add_node("output", [0.0; 2]);
            graph.connect(source, "output", output, "input").unwrap();
        }
        graph
    }

    fn shared_tail(chain: usize, tail: usize, cached_first: bool) -> Package {
        let mut package = Package::new("depth");
        let mut children = vec!["tail0".into(), "chain0".into()];
        if !cached_first {
            children.reverse();
        }
        package.graphs.push(graph("root", &children));
        for i in 0..chain {
            let next = if i + 1 == chain { "tail0".into() } else { format!("chain{}", i + 1) };
            package.graphs.push(graph(&format!("chain{i}"), &[next]));
        }
        for i in 0..tail {
            let children = if i + 1 == tail { vec![] } else { vec![format!("tail{}", i + 1)] };
            package.graphs.push(graph(&format!("tail{i}"), &children));
        }
        package
    }

    #[test]
    fn cached_graph_depth_is_independent_of_reference_order() {
        for cached_first in [true, false] {
            // Root + chain + shared tail: 32 is valid, 33 is not. The cached
            // tail can itself contain dependencies, not just a leaf.
            for tail in [1, 5] {
                assert!(snapshot(&shared_tail(31 - tail, tail, cached_first), 0).is_ok());
                assert!(snapshot(&shared_tail(32 - tail, tail, cached_first), 0).is_err());
            }
        }
    }
}
