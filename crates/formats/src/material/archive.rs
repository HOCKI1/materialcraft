use crate::{FormatError, Result, project::ProjectFiles, sbs};
use materialcraft_core::Package;
use std::cell::Cell;

pub use craft_mcmat::VERSION;
pub const MATERIAL_EXT: &str = "mcmat";

pub fn is_material(bytes: &[u8]) -> bool {
    craft_mcmat::is_material(bytes, craft_mcmat::Limits::default().max_entries)
}

pub fn write_material(package: &Package, entry: usize) -> Result<Vec<u8>> {
    let snapshot = super::snapshot(package, entry)?;
    let files = ProjectFiles::build(&snapshot, craft_mcmat::SOURCE_ENTRY, false)?;
    let resources = files.files.iter().filter(|(name, _)| **name != files.entry).map(|(name, bytes)| (name.as_str(), bytes.as_slice()));
    craft_mcmat::write(files.xml()?, resources).map_err(|err| FormatError::BadPackage(err.to_string()))
}

pub fn read_material(bytes: &[u8]) -> Result<(Package, usize)> {
    let limits = craft_mcmat::Limits::default();
    let material = craft_mcmat::Material::open(bytes, limits).map_err(|err| FormatError::BadPackage(err.to_string()))?;

    // Aliases of a compressed member share a single decoded-byte budget
    let remaining = Cell::new(limits.max_total_bytes);
    let read = sbs::read_sbs(&material.source, craft_mcmat::SOURCE_ENTRY, &|name| {
        let data = material.resources.get(name)?;
        remaining.set(remaining.get().checked_sub(data.len() as u64)?);
        Some(data.clone())
    })?;
    drop(material);

    if let Some(warning) = read.warnings.first() {
        return Err(FormatError::BadPackage(format!("source.sbs: {warning}")));
    }

    let pkg = read.pkg;
    let entry = super::entry_graph(&pkg)?;
    if pkg.graphs.iter().flat_map(|g| g.instances()).any(|instance| !instance.package.is_empty()) {
        return Err(FormatError::BadPackage("source.sbs has an external graph reference; republish with its dependencies".into()));
    }

    // Reuse the publisher’s dependency checks for third-party archives
    super::snapshot(&pkg, entry)?;
    Ok((pkg, entry))
}

/// Whether maps of `usage` hold colors (stored sRGB-encoded) rather than data (linear)
pub fn srgb_usage(usage: &str) -> bool {
    matches!(
        usage,
        "baseColor"
            | "diffuse"
            | "emissive"
            | "specular"
            | "specularEdgeColor"
            | "scatteringColor"
            | "absorptionColor"
            | "coatColor"
            | "sheenColor"
            | "reflection"
            | "ambient"
    )
}
