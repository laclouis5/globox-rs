use crate::imgsize::ImgSize;
use crate::bbox::BBox;

#[derive(Debug, Clone)]
pub struct Ann {
    pub img_id: String,
    pub img_size: Option<ImgSize>,
    pub bboxes: Vec<BBox>,
}

impl Ann {
    pub fn new<S: Into<String>>(img_id: S, img_size: Option<ImgSize>, bboxes: Vec<BBox>) -> Self {
        Ann {img_id: img_id.into(), img_size, bboxes }
    }

    pub fn empty<S: Into<String>>(img_id: S) -> Self {
        Ann::new(img_id, None, vec![])
    }
}