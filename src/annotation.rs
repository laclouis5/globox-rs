use crate::imgsize::*;
use crate::bbox::*;

#[derive(Clone)]
pub struct Ann {
    img_id: String,
    img_size: Option<ImgSize>,
    boxes: Vec<BBox>,
}

impl Ann {
    pub fn new(img_id: &str, img_size: Option<ImgSize>, boxes: Vec<BBox>) -> Self {
        Ann {
            img_id: String::from(img_id),
            img_size,
            boxes,
        }
    }

    pub fn empty(img_id: &str) -> Self {
        Ann { img_id: String::from(img_id), img_size: None, boxes: vec![] }
    }
}

impl Ann {
    pub fn img_size(&self) -> Option<ImgSize> {
        self.img_size
    }

    pub fn boxes(&mut self) -> &mut Vec<BBox> {
        &mut self.boxes
    }

    pub fn set_img_id(&mut self, img_id: &str) {
        self.img_id = String::from(img_id)
    }
}