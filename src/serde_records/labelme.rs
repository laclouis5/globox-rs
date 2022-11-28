use serde::{Serialize, Deserialize};

use smol_str::SmolStr;

#[derive(Serialize, Deserialize)]
pub(crate) struct LMShape {
    pub(crate) label: SmolStr,
    pub(crate) points: Vec<Vec<f32>>,
    pub(crate) shape_type: SmolStr,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct LMAnn {
    #[serde(rename = "imagePath")]
    pub(crate) image_path: SmolStr,

    #[serde(rename = "imageWidth")]
    pub(crate) image_width: u32,

    #[serde(rename = "imageHeight")]
    pub(crate) image_height: u32,

    pub(crate) shapes: Vec<LMShape>,
}
