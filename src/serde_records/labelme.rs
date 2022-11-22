use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct LMShape {
    pub(crate) label: String,
    pub(crate) points: Vec<Vec<f32>>,
    pub(crate) shape_type: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct LMAnn {
    #[serde(rename = "imagePath")]
    pub(crate) image_path: String,

    #[serde(rename = "imageWidth")]
    pub(crate) image_width: u32,

    #[serde(rename = "imageHeight")]
    pub(crate) image_height: u32,

    pub(crate) shapes: Vec<LMShape>,
}
