use crate::{
    imgsize::ImgSize,
    bbox::BBox, 
    annotation::Ann, 
    annotationset::AnnSet,
    parsers::ParseError, 
    serde_records::labelme::{LMShape, LMAnn},
};

use std::{
    fs,
    path::Path,
};

use serde_json::from_str;

impl TryFrom<LMShape> for BBox {
    type Error = ParseError;

    fn try_from(lm_bbox: LMShape) -> Result<BBox, ParseError> {
        if lm_bbox.shape_type != "rectangle" {
            return Err(ParseError {})
        }

        let (lt, rb) = match &lm_bbox.points[..] {
            [lt, rb] => Ok((lt, rb)),
            _ => Err(ParseError {})
        }?;

        let (xmin, ymin) = match lt[..] {
            [xmin, ymin] => Ok((xmin, ymin)),
            _ => Err(ParseError {})
        }?;

        let (xmax, ymax) = match rb[..] {
            [xmax, ymax] => Ok((xmax, ymax)),
            _ => Err(ParseError {})
        }?;

        Ok(BBox::new(lm_bbox.label, xmin, ymin, xmax, ymax, None))
    }
}

impl TryFrom<LMAnn> for Ann {
    type Error = ParseError;

    fn try_from(lm_ann: LMAnn) -> Result<Self, Self::Error> {
        let img_size = ImgSize::new(lm_ann.image_width, lm_ann.image_height);
        
        let boxes = lm_ann.shapes.into_iter()
            .filter(|b| b.shape_type == "rectangle")
            .map(|lm_bbox| lm_bbox.try_into())
            .collect::<Result<Vec<BBox>, ParseError>>()?;  // Change to try-collect` in the future

        Ok(Ann::new(lm_ann.image_path, Some(img_size), boxes))   
    }
}

impl Ann {
    pub fn parse_labelme<P: AsRef<Path>>(path: P) -> Result<Ann, ParseError> {
        // Faster to first read the whole file in memory then parse JSON, 
        // plus those files are quite small.
        let content = fs::read_to_string(path)
            .map_err(|_| ParseError {})?;

        let ann: LMAnn = from_str(&content)
            .map_err(|_| ParseError {})?;

        ann.try_into().map_err(|_| ParseError {})
    }
}

impl AnnSet {
    pub fn parse_labelme<P: AsRef<Path>>(path: P) -> Result<AnnSet, ParseError> {
        AnnSet::parse_folder(path, "json", |p| Ann::parse_labelme(p))
    }
}