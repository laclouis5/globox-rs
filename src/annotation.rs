use crate::imgsize::ImgSize;
use crate::bbox::BBox;

#[derive(Debug, Clone)]
pub struct Ann {
    pub img_id: String,
    pub img_size: Option<ImgSize>,
    pub boxes: Vec<BBox>,
}

impl Ann {
    pub fn new(img_id: String, img_size: Option<ImgSize>, boxes: Vec<BBox>) -> Self {
        Ann {img_id, img_size, boxes }
    }

    pub fn empty(img_id: String) -> Self {
        Ann { img_id, img_size: None, boxes: vec![] }
    }
}