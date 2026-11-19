use crate::{OutputFormat, ResourceBytes};
use serde::{Deserialize, Serialize};

/// An asset retained in memory until the package is saved: a bitmap or an SVG.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    /// Unique name, referenced by nodes (`resource` parameter)
    pub name: String,
    #[serde(default = "default_folder")]
    pub folder: String,
    /// File name inside the package's `resources/` folder
    pub file: String,
    /// External origin retained for linked library saves and repair when bytes are unavailable
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub source: String,
    /// Original bytes, saved beside native SBS or inside a MaterialCraft archive.
    #[serde(skip)]
    pub bytes: ResourceBytes,
}

impl Resource {
    /// A resource named `name`, stored as `file` in the package
    pub fn new(name: impl Into<String>, file: impl Into<String>, bytes: ResourceBytes) -> Resource {
        Resource { name: name.into(), folder: default_folder(), file: file.into(), source: String::new(), bytes }
    }

    pub fn sbs_path(&self) -> String {
        if self.folder.is_empty() { self.name.clone() } else { format!("{}/{}", self.folder, self.name) }
    }
}

fn default_folder() -> String {
    "Resources".into()
}

/// Original normalized channel precision of a grayscale or truecolor PNG resource.
/// The decoded float values alone cannot distinguish a byte from the equivalent repeated-byte word.
pub fn png_sample_format(bytes: &[u8]) -> Option<OutputFormat> {
    let header = bytes.strip_prefix(b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR")?;
    match header.get(8..10)? {
        [8, 0 | 2 | 4 | 6] => Some(OutputFormat::U8),
        [16, 0 | 2 | 4 | 6] => Some(OutputFormat::U16),
        _ => None,
    }
}
