use serde::{Deserialize, Serialize};

/// Destinations and bake settings for the selected source graph.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct MaterialExportSettings {
    /// Folder the MaterialX document and its resources go to
    pub folder: String,
    /// The .mcmat archive path
    pub archive: String,
    /// Pixels of the larger side; 0 keeps the graph's size
    pub size: u32,
    pub texture_depth: TextureDepth,
    #[serde(skip)]
    pub root: String,
    /// Height in scene units of a white displacement texel
    pub displacement_scale: f32,
}

impl Default for MaterialExportSettings {
    fn default() -> Self {
        Self { folder: String::new(), archive: String::new(), size: 0, texture_depth: TextureDepth::default(), root: String::new(), displacement_scale: 1.0 }
    }
}

/// PNG precision of the published textures; HDR and signed data always use lossless EXR
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextureDepth {
    /// 16 bits for data textures and 8 bits for color
    #[default]
    Automatic,
    Eight,
    Sixteen,
}

impl TextureDepth {
    pub const ALL: [TextureDepth; 3] = [TextureDepth::Automatic, TextureDepth::Eight, TextureDepth::Sixteen];

    pub fn label(self) -> &'static str {
        match self {
            TextureDepth::Automatic => "Automatic (16-bit data, 8-bit color)",
            TextureDepth::Eight => "8 bits",
            TextureDepth::Sixteen => "16 bits",
        }
    }
}
