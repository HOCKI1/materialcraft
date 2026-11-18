//! Retain typed, including unconnected, material graph inputs when references become local.
use super::xml::{child_element, child_value, children_named, xml_escape};
use materialcraft_core::GraphPort;
use roxmltree::Node;
use std::{collections::BTreeSet, fmt::Write};

pub(super) fn read(element: Node<'_, '_>) -> Option<Result<Vec<GraphPort>, String>> {
    let interface = child_element(element, "materialcraftInterface")?;
    Some((|| {
        let mut inputs = Vec::new();
        let mut names = BTreeSet::new();
        for port in children_named(interface, "input") {
            if inputs.len() == 4096 {
                return Err("Graph instance exceeds the 4096-input limit".into());
            }
            let identifier = child_value(port, "identifier").unwrap_or("");
            if identifier.is_empty() || !names.insert(identifier) {
                return Err("Invalid or duplicate graph input identifier".into());
            }
            let type_code = child_value(port, "type").and_then(|value| value.parse().ok()).ok_or("Invalid graph input type")?;
            inputs.push(GraphPort { identifier: identifier.into(), type_code });
        }
        Ok(inputs)
    })())
}

pub(super) fn write(out: &mut String, inputs: &[GraphPort]) {
    out.push_str("<materialcraftInterface>");
    for port in inputs {
        let _ = write!(out, "<input><identifier v=\"{}\"/><type v=\"{}\"/></input>", xml_escape(&port.identifier), port.type_code);
    }
    out.push_str("</materialcraftInterface>");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interface_rejects_duplicate_malformed_and_excess_inputs() {
        let parse = |body: &str| {
            let xml = format!("<compInstance><materialcraftInterface>{body}</materialcraftInterface></compInstance>");
            let document = roxmltree::Document::parse(&xml).unwrap();
            read(document.root_element()).unwrap()
        };
        assert!(parse("<input><identifier v=\"a\"/><type v=\"bad\"/></input>").is_err());
        assert!(parse("<input><identifier v=\"a\"/><type v=\"1\"/></input>".repeat(2).as_str()).is_err());
        let mut body = String::new();
        for index in 0..4096 {
            write!(body, "<input><identifier v=\"p{index}\"/><type v=\"2\"/></input>").unwrap();
        }
        assert_eq!(parse(&body).unwrap().len(), 4096);
        body.push_str("<input><identifier v=\"extra\"/><type v=\"2\"/></input>");
        assert!(parse(&body).is_err());
    }
}
