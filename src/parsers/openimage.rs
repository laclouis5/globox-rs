use std::collections::hash_map::Entry;

use crate::{parsers::ParseErr, annotation::Ann, annotationset::AnnSet, bbox::BBox};

use csv;
use serde::Deserialize;

#[derive(Deserialize)]
struct OALine {
    #[serde(rename = "ImageID")]
    imageid: String,

    #[serde(rename = "LabelName")]
    label: String,

    #[serde(rename = "Confidence")]
    conf: Option<f32>,

    #[serde(rename = "XMin")]
    xmin: f32,

    #[serde(rename = "YMin")]
    ymin: f32,

    #[serde(rename = "XMax")]
    xmax: f32,

    #[serde(rename = "YMax")]
    ymax: f32,
}

impl OALine {
    fn img_id_and_bbox(self) -> (String, BBox) {
        (
            self.imageid,
            BBox::new(self.label, self.xmin, self.ymin, self.xmax, self.ymax, self.conf)
        )
    }
}

// TODO: read the image sizes from img_path
impl AnnSet {
    pub fn parse_openimage(path: &str, img_path: &str) -> Result<AnnSet, ParseErr> {
        let mut annset = AnnSet::new();
        let reader = csv::Reader::from_path(path)
            .map_err(|_| ParseErr {})?;

        for line in reader.into_deserialize() {
            let line: OALine = line.map_err(|_| ParseErr {})?;
            let (image_id, bbox) = line.img_id_and_bbox();
            
            // Could avoid to always String.clone() even if key present.
            annset.items
                .entry(image_id.clone())
                .and_modify(|a| a.boxes.push(bbox))
                .or_insert(Ann::new(image_id, None, vec![]));
        }

        Ok(annset)
    }
}