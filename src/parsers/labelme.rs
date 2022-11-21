use crate::{
    imgsize::ImgSize,
    bbox::BBox, 
    annotation::Ann, 
    annotationset::AnnSet,
    parsers::{ParseErr, folder::parse_folder}, 
};

use std::{
    fs,
    path::Path,
};

use serde::Deserialize;
use serde_json::from_str;

#[derive(Deserialize)]
struct LMShape {
    label: String,
    points: Vec<Vec<f32>>,
    shape_type: String,
}

impl TryFrom<LMShape> for BBox {
    type Error = ParseErr;

    fn try_from(lm_bbox: LMShape) -> Result<BBox, ParseErr> {
        if lm_bbox.shape_type != "rectangle" {
            return Err(ParseErr {})
        }

        let (tl, br) = match &lm_bbox.points[..] {
            [tl, br] => Ok((tl, br)),
            _ => Err(ParseErr {})
        }?;

        let (xmin, ymin) = match tl[..] {
            [xmin, ymin] => Ok((xmin, ymin)),
            _ => Err(ParseErr {})
        }?;

        let (xmax, ymax) = match br[..] {
            [xmax, ymax] => Ok((xmax, ymax)),
            _ => Err(ParseErr {})
        }?;

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

    shapes: Vec<LMShape>,
}

impl TryFrom<LMAnn> for Ann {
    type Error = ParseErr;

    fn try_from(lm_ann: LMAnn) -> Result<Self, Self::Error> {
        let img_size = ImgSize::new(lm_ann.image_width, lm_ann.image_height);
        
        let boxes = lm_ann.shapes.into_iter()
            .filter(|b| b.shape_type == "rectangle")
            .map(|lm_bbox| lm_bbox.try_into())
            .collect::<Result<Vec<BBox>, ParseErr>>()?;  // Change to try-collect` in the future

        Ok(Ann::new(lm_ann.image_path, Some(img_size), boxes))   
    }
}

impl Ann {
    pub fn parse_labelme<P: AsRef<Path>>(path: P) -> Result<Ann, ParseErr> {
        let content = fs::read_to_string(path)
            .map_err(|_| ParseErr {})?;

        let ann: LMAnn = from_str(&content)
            .map_err(|_| ParseErr {})?;

        ann.try_into().map_err(|_| ParseErr {})
    }
}

impl AnnSet {
    pub fn parse_labelme<P: AsRef<Path>>(path: P) -> Result<AnnSet, ParseErr> {
        parse_folder(path, "json", |p| Ann::parse_labelme(p))
    }
}