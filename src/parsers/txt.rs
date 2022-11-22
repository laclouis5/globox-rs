use crate::{
    imgsize::ImgSize,
    bbox::{BBox, BBoxFmt},
    annotationset::AnnSet,
    annotation::Ann,
    parsers::{ParseError, path_to_img_id},
    serde_records::txt::*,
};

use std::path::Path;
use csv;

impl Ann {
    fn parse_txt_raw<P: AsRef<Path>>(
        path: P, 
        fmt: BBoxFmt,
        rel: bool,
        img_size: Option<ImgSize>,
        img_id: String,
        conf_last: bool,
    ) -> Result<Ann, ParseError> {    
        let mut boxes = vec![];

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b' ')
            .from_path(path)
            .map_err(|_| ParseError {})?;

        let mut raw_record = csv::StringRecord::new();

        while reader.read_record(&mut raw_record).map_err(|_| ParseError {})? {
            let (label, coords, conf) = match raw_record.len() {
                5 => {
                    let rec: TxtLineGt = raw_record
                        .deserialize(None)
                        .map_err(|_| ParseError {})?;

                    let coords = (rec.1, rec.2, rec.3, rec.4);

                    (rec.0, coords, None)
                },

                6 => {
                    let rec: TxtLineDet = raw_record
                        .deserialize(None)
                        .map_err(|_| ParseError {})?;

                    if conf_last {
                        let coords = (rec.1, rec.2, rec.3, rec.4);
                        (rec.0, coords, Some(rec.5))
                    } else {
                        let coords = (rec.2, rec.3, rec.4, rec.5);
                        (rec.0, coords, Some(rec.1))
                    }
                },

                _ => { 
                    return Err(ParseError {})
                },
            };

            let bbox = if rel {
                let img_size = img_size
                    .expect("Parsing relative coordinates requires the image size to be provided.");
                
                BBox::create_rel(label, coords, fmt, conf, img_size)
            } else {
                BBox::create(label, coords, fmt, conf) 
            };

            boxes.push(bbox);
        }

        let ann = Ann::new(img_id, img_size, boxes);

        Ok(ann)
    }
}

impl Ann {
    pub fn parse_txt<P: AsRef<Path>>(
        path: P, 
        fmt: BBoxFmt,
        img_size: Option<ImgSize>,
        conf_last: bool,
        img_ext: &str,
    ) -> Result<Ann, ParseError> { 
        let img_id = path_to_img_id(path.as_ref(), img_ext)?;
        Ann::parse_txt_raw(path, fmt, false, img_size, img_id, conf_last)
    }

    pub fn parse_txt_rel<P: AsRef<Path>>(
        path: P, 
        fmt: BBoxFmt,
        img_size: ImgSize,
        conf_last: bool,
        img_ext: &str,
    ) -> Result<Ann, ParseError> { 
        let img_id = path_to_img_id(path.as_ref(), img_ext)?;
        Ann::parse_txt_raw(path, fmt, true, Some(img_size), img_id, conf_last)
    }
}

impl AnnSet {
    pub fn parse_txt<P1: AsRef<Path>, P2: AsRef<Path>>(
        path: P1,
        fmt: BBoxFmt,
        imgs_path: Option<P2>,
        conf_last: bool,
        img_ext: &str,
    ) -> Result<AnnSet, ParseError> { 
        let imgs_path = imgs_path
            .and_then(|p| Some(p.as_ref().to_path_buf()));

        AnnSet::parse_folder(path, "txt", |p| {
            let img_id = path_to_img_id(p, img_ext)?;
            
            let img_size = if let Some(imgs_path) = imgs_path.as_ref() {
                let mut img_path = imgs_path.clone();
                img_path.push(&img_id);

                Some(ImgSize::from_file(img_path)?)
            } else { None };

            Ann::parse_txt_raw(p, fmt, false, img_size, img_id, conf_last)
        })
    }

    pub fn parse_txt_rel<P1: AsRef<Path>, P2: AsRef<Path>>(
        path: P1,
        fmt: BBoxFmt,
        imgs_path: P2,
        conf_last: bool,
        img_ext: &str,
    ) -> Result<AnnSet, ParseError> { 
        let imgs_path = imgs_path.as_ref().to_path_buf();

        AnnSet::parse_folder(path, "txt", |p| {
            let img_id = path_to_img_id(p, img_ext)?;
        
            let mut img_path = imgs_path.clone();
            img_path.push(&img_id);

            let img_size = ImgSize::from_file(img_path)?;

            Ann::parse_txt_raw(p, fmt, true, Some(img_size), img_id, conf_last)
        })
    }
}