use std::{
    path::Path,
    collections::hash_map::Entry,
};

use crate::{
    parsers::ParseError, 
    annotation::Ann,
    annotationset::AnnSet,
    bbox::{BBox, BBoxFmt}, 
    imgsize::ImgSize,
};

use csv;
use serde::Deserialize;

#[derive(Deserialize)]
struct OALine {
    #[serde(rename = "ImageID")]
    img_id: String,

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

impl AnnSet {
    pub fn parse_openimage<P1, P2>(
        path: P1, imgs_path: P2,
    ) -> Result<AnnSet, ParseError> 
    where 
        P1: AsRef<Path>,
        P2: AsRef<Path>,
    {
        let imgs_path = imgs_path.as_ref().to_path_buf();
        
        let mut annset = AnnSet::new();

        // Csv Reader is automatically buffered, no need to wrap it in BufReader.
        let mut reader = csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .from_path(path)
            .map_err(|_| ParseError {})?;

        let headers = reader.headers()
            .map_err(|_| ParseError {})?
            .clone();

        let mut raw_record = csv::StringRecord::new();

        while reader.read_record(&mut raw_record).map_err(|_| ParseError {})? {
            let line: OALine = raw_record.deserialize(Some(&headers))
                .map_err(|_| ParseError {})?;

            let img_id = line.img_id;
            let coords = (line.xmin, line.ymin, line.xmax, line.ymax);

            // TODO: Could avoid to String.clone() when key is present.
            match annset.items.entry(img_id.clone()) {
                Entry::Occupied(mut oe) => {
                    let ann = oe.get_mut();

                    let img_size = ann.img_size
                        .expect("Image size should have been populated during Ann init.");
                    
                    let bbox = BBox::create_rel(
                        line.label, 
                        coords,
                        BBoxFmt::LTRB, 
                        line.conf, 
                        img_size
                    );

                    ann.bboxes.push(bbox);
                },

                Entry::Vacant(ve) => {
                    let mut img_path = imgs_path.clone();
                    img_path.push(&img_id);
                    let img_size = ImgSize::from_file(&img_path)?;

                    let ann = Ann::new(img_id, Some(img_size), vec![]);

                    ve.insert(ann);
                },
            }
        }

        Ok(annset)
    }
}