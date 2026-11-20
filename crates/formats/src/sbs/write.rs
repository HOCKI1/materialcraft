use super::{
    constant::{FIRST_UID, SELF_DEPENDENCY, UPDATER_VERSION},
    sbs_writer::SbsWriter,
    xml::xml_escape,
};
use crate::Result;
use materialcraft_core::Package;
use std::{collections::BTreeMap, fmt::Write as _};

/// Schema revision to declare when writing a package, including packages with older app-release headers.
pub fn output_format_version(imported: Option<&str>) -> &str {
    const SCHEMA: &str = "1.1.0.202401";
    let Some(version) = imported.filter(|v| !v.trim().is_empty()) else { return SCHEMA };
    // Earlier saves put the three-part app release in this field. Readers use it to select the XML grammar.
    let mut parts = version.split('.');
    let is_release = (0..3).all(|_| parts.next().is_some_and(|p| !p.is_empty() && p.bytes().all(|c| c.is_ascii_digit()))) && parts.next().is_none();
    if is_release { SCHEMA } else { version }
}

pub fn write_sbs(pkg: &Package) -> Result<Vec<u8>> {
    let format_version = output_format_version(pkg.format_version.as_deref());
    let mut writer = SbsWriter { last_uid: FIRST_UID, external_packages: BTreeMap::new() };
    let self_uid = writer.next_uid();

    let mut content = String::from("<content>");
    super::group_layout::write(&mut content, pkg, &mut writer, self_uid)?;
    content.push_str("</content>");

    let mut out = String::with_capacity(content.len() + 4096);
    let _ = write!(
        out,
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><package><identifier v=\"{}\"/><formatVersion v=\"{}\"/><updaterVersion v=\"{UPDATER_VERSION}\"/><fileUID v=\"0\"/><versionUID v=\"0\"/><dependencies>",
        xml_escape(&pkg.name),
        xml_escape(format_version)
    );
    let write_dependency = |out: &mut String, file: &str, uid: u64| {
        let _ = write!(
            out,
            "<dependency><filename v=\"{}\"/><uid v=\"{uid}\"/><type v=\"package\"/><fileUID v=\"0\"/><versionUID v=\"0\"/></dependency>",
            xml_escape(file)
        );
    };

    write_dependency(&mut out, SELF_DEPENDENCY, self_uid);
    for (file, uid) in &writer.external_packages {
        write_dependency(&mut out, file, *uid);
    }
    out.push_str("</dependencies>");
    out.push_str(&content);

    out.push_str("</package>\n");
    Ok(out.into_bytes())
}
