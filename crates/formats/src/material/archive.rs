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

    let read = read_source(&material, limits.max_total_bytes)?;
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

// Linked aliases and inline SBS assets must share the same decoded-byte budget.
fn read_source(material: &craft_mcmat::Material, budget: u64) -> Result<sbs::SbsRead> {
    let remaining = Cell::new(budget);
    sbs::SbsDocument::parse(&material.source)?.read_with_resource_budget(
        craft_mcmat::SOURCE_ENTRY,
        &|name| {
            let data = material.resources.get(name)?;
            remaining.set(remaining.get().checked_sub(data.len() as u64)?);
            Some(data.clone())
        },
        &remaining,
    )
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

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine as _;
    use std::io::Write;

    fn inline(bytes: &[u8]) -> String {
        let mut encoder = flate2::write::ZlibEncoder::new((bytes.len() as u32).to_be_bytes().to_vec(), flate2::Compression::default());
        encoder.write_all(bytes).unwrap();
        let encoded = base64::engine::general_purpose::STANDARD.encode(encoder.finish().unwrap());
        format!("<resource><identifier v=\"inline\"/><source><binembedded><strdata v=\"{encoded}\"/></binembedded></source></resource>")
    }

    fn linked(name: &str) -> String {
        format!("<resource><identifier v=\"{name}\"/><filepath v=\"resources/member.png\"/></resource>")
    }

    fn make_material(content: &str) -> craft_mcmat::Material {
        let source = format!("<package><content>{content}</content></package>");
        let bytes = craft_mcmat::write(source.as_bytes(), [("resources/member.png", b"abc".as_slice())]).unwrap();
        craft_mcmat::Material::open(&bytes, craft_mcmat::Limits::default()).unwrap()
    }

    #[test]
    fn inline_budget_failure_does_not_substitute_a_different_linked_asset() {
        let content = inline(b"four").replace("</resource>", "<filepath v=\"resources/member.png\"/></resource>");
        let material = make_material(&content);
        let read = read_source(&material, 3).unwrap();
        assert!(read.warnings.iter().any(|warning| warning.contains("resource budget")));
        assert!(read.pkg.resources.is_empty());
        let read = read_source(&material, 4).unwrap();
        assert!(read.warnings.is_empty());
        assert_eq!(read.pkg.resources[0].bytes.as_slice(), b"four");
        let damaged = content.replace("<strdata v=", "<unused v=");
        let read = read_source(&make_material(&damaged), 3).unwrap();
        assert!(read.warnings.is_empty());
        assert_eq!(read.pkg.resources[0].bytes.as_slice(), b"abc");
    }

    #[test]
    fn corrupt_inline_headers_on_either_side_of_the_budget_keep_linked_fallback() {
        for length in [3_u32, 4] {
            let mut raw = length.to_be_bytes().to_vec();
            raw.push(255);
            let encoded = base64::engine::general_purpose::STANDARD.encode(raw);
            let content = format!(
                "<resource><identifier v=\"inline\"/><filepath v=\"resources/member.png\"/><source><binembedded><strdata v=\"{encoded}\"/></binembedded></source></resource>"
            );
            let read = read_source(&make_material(&content), 3).unwrap();
            assert!(read.warnings.is_empty());
            assert_eq!(read.pkg.resources[0].bytes.as_slice(), b"abc");
        }
    }

    #[test]
    fn inline_and_member_assets_share_the_exact_budget_in_either_order() {
        for content in [format!("{}{}", inline(b"four"), linked("member")), format!("{}{}", linked("member"), inline(b"four"))] {
            let material = make_material(&content);
            let read = read_source(&material, 7).unwrap();
            assert!(read.warnings.is_empty(), "{:?}", read.warnings);
            assert_eq!(read.pkg.resources.iter().map(|resource| resource.bytes.len()).sum::<usize>(), 7);
            let read = read_source(&material, 6).unwrap();
            assert!(!read.warnings.is_empty(), "budget failure must reject the material");
            assert!(read.pkg.resources.iter().map(|resource| resource.bytes.len()).sum::<usize>() <= 6);
        }
    }

    #[test]
    fn multiple_inline_assets_and_member_aliases_cannot_bypass_the_budget() {
        for content in [format!("{}{}", inline(b"abc"), inline(b"def")), format!("{}{}", linked("a"), linked("b"))] {
            let material = make_material(&content);
            assert!(read_source(&material, 6).unwrap().warnings.is_empty());
            let read = read_source(&material, 5).unwrap();
            assert!(!read.warnings.is_empty());
            assert_eq!(read.pkg.resources.iter().map(|resource| resource.bytes.len()).sum::<usize>(), 3);
        }
    }

    #[test]
    fn standalone_sbs_keeps_inline_compatibility_and_zero_length_is_free() {
        let material = make_material(&format!("{}{}", inline(b""), linked("member")));
        assert!(read_source(&material, 3).unwrap().warnings.is_empty());
        let material = make_material(&format!("{}{}", inline(b"abc"), inline(b"def")));
        let read = sbs::read_sbs(&material.source, "source.sbs", &|_| None).unwrap();
        assert!(read.warnings.is_empty());
        assert_eq!(read.pkg.resources.iter().map(|resource| resource.bytes.len()).sum::<usize>(), 6);
    }
}
