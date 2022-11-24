use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct COCOCat {
    pub(crate) id: usize,

    #[serde(rename = "name")]
    pub(crate) label: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct COCOImg {
    pub(crate) id: usize,
    
    pub(crate) width: u32,
    pub(crate) height: u32,

    #[serde(rename = "file_name")]
    pub(crate) img_id: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct COCOAnn {
    #[serde(rename = "category_id")]
    pub(crate) cat_id: usize,

    #[serde(rename = "image_id")]
    pub(crate) img_id: usize,

    pub(crate) bbox: Vec<f32>,

    #[serde(rename = "score")]
    pub(crate) conf: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct COCOAnnSet {
    pub(crate) categories: Vec<COCOCat>,
    pub(crate) images: Vec<COCOImg>,
    pub(crate) annotations: Vec<COCOAnn>,
}