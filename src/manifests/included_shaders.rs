use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "PascalCase"))]
pub struct IncludedShadersManifest {
    pub shaders: Vec<IncludedShader>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase", deserialize = "PascalCase"))]
pub struct IncludedShader {
    pub name: String,
    pub authors: Vec<String>,
    pub description: String,
    pub images: Option<Vec<String>>,
    pub shader_count: usize,
    pub texture_count: usize,
    pub preset_count: usize,
    pub addon_count: usize,
}
