use crate::imgsize::ImgSize;
use crate::bbox::BBox;

/// A collection of bounding box annotations for a single image.
#[derive(Debug, Clone)]
pub struct Ann {
    /// The image identifier.
    /// 
    /// This identifier must be unique and is generally the image name
    /// such as `the_image.jpg`.
    pub img_id: String,

    /// The image size.
    pub img_size: Option<ImgSize>,

    /// The bounding box annotations of the image.
    pub bboxes: Vec<BBox>,
}

impl Ann {
    /// Creates an annotation with the given bounding boxes.
    pub fn new<S: Into<String>>(img_id: S, img_size: Option<ImgSize>, bboxes: Vec<BBox>) -> Self {
        Ann {img_id: img_id.into(), img_size, bboxes }
    }

    /// Creates an empty annotation without bounding boxes.
    pub fn empty<S: Into<String>>(img_id: S) -> Self {
        Ann::new(img_id, None, vec![])
    }
}