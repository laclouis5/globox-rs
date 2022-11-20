use crate::{
    imgsize::ImgSize,
    bbox::BBox, 
    annotation::Ann, 
    parsers::ParseErr, 
};

use std::{
    fs,
    path::Path,
};

use serde::Deserialize;
use serde_json::from_slice;


// More a LMShape
#[derive(Deserialize)]
struct LMBBox {
    label: String,
    points: Vec<Vec<f32>>,
    shape_type: String,
}

impl TryFrom<LMBBox> for BBox {
    type Error = ParseErr;

    fn try_from(mut lm_bbox: LMBBox) -> Result<BBox, ParseErr> {
        if lm_bbox.shape_type != "rectangle" {
            return Err(ParseErr {})
        }

        let mut br = lm_bbox.points.pop()
            .ok_or(ParseErr {})?;
        let mut tl = lm_bbox.points.pop()
            .ok_or(ParseErr {})?;

        if !lm_bbox.points.is_empty() {
            return Err(ParseErr {})
        }

        let ymax = br.pop()
            .ok_or(ParseErr {})?;
        let xmax = br.pop()
            .ok_or(ParseErr {})?;

        if !br.is_empty() {
            return Err(ParseErr {})
        }

        let ymin = tl.pop()
            .ok_or(ParseErr {})?;
        let xmin = tl.pop()
            .ok_or(ParseErr {})?;

        if !tl.is_empty() {
            return Err(ParseErr {})
        }

        Ok(BBox::new(lm_bbox.label, xmin, ymin, xmax, ymax, None))
    }
}

#[derive(Deserialize)]
struct LMAnn {
    #[serde(rename = "imagePath")]
    image_path: String,

    #[serde(rename = "imageWidth")]
    image_width: u32,

    #[serde(rename = "imageHeight")]
    image_height: u32,

    shapes: Vec<LMBBox>,
}

impl TryFrom<LMAnn> for Ann {
    type Error = ParseErr;

    fn try_from(lm_ann: LMAnn) -> Result<Self, Self::Error> {
        let boxes = lm_ann.shapes.into_iter()
            .map(|lm_bbox| lm_bbox.try_into())
            .collect::<Result<Vec<BBox>, ParseErr>>()?;  // Change to try-collect` in the future
        
        let img_size = ImgSize { width: lm_ann.image_width, height: lm_ann.image_height };

        Ok(Ann::new(lm_ann.image_path, Some(img_size), boxes))   
    }
}

impl Ann {
    pub fn from_labelme<P: AsRef<Path>>(path: P) -> Result<Ann, ParseErr> {
        let content = fs::read_to_string(path)
            .map_err(|_| ParseErr {})?;

        let ann: LMAnn = from_slice(content.as_bytes())
            .map_err(|_| ParseErr {})?;

        ann.try_into().map_err(|_| ParseErr {})
    }
}