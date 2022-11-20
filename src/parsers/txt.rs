use crate::{
    imgsize::ImgSize,
    bbox::{BBox, BBoxFmt},
    annotation::Ann,
};

use std::path::Path;
use csv;

use super::ParseErr;

type TxtLineGt = (String, f32, f32, f32, f32);
type TxtLineDet = (String, f32, f32, f32, f32, f32);

impl Ann {
    fn parse_txt_raw<P: AsRef<Path>>(
        path: P, 
        fmt: BBoxFmt,
        rel: bool,
        img_size: Option<ImgSize>,
        conf_last: bool,
        img_ext: &str,
    ) -> Result<Ann, ParseErr> {    
        let img_id = path.as_ref()
            .with_extension(img_ext);
            
        let img_id = img_id
            .file_name()
            .ok_or(ParseErr {})?
            .to_str()
            .ok_or(ParseErr {})?;

        let mut boxes = vec![];

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b' ')
            .from_path(&path)
            .map_err(|_| ParseErr {})?;

        let mut raw_record = csv::StringRecord::new();

        while reader.read_record(&mut raw_record).map_err(|_| ParseErr {})? {
            let (label, coords, conf) = match raw_record.len() {
                5 => {
                    let rec: TxtLineGt = raw_record
                        .deserialize(None)
                        .map_err(|_| ParseErr {})?;

                    let coords = (rec.1, rec.2, rec.3, rec.4);

                    (rec.0, coords, None)
                },

                6 => {
                    let rec: TxtLineDet = raw_record
                        .deserialize(None)
                        .map_err(|_| ParseErr {})?;

                    if conf_last {
                        let coords = (rec.1, rec.2, rec.3, rec.4);
                        (rec.0, coords, Some(rec.5))
                    } else {
                        let coords = (rec.2, rec.3, rec.4, rec.5);
                        (rec.0, coords, Some(rec.1))
                    }
                },

                _ => { 
                    return Err(ParseErr {})
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

        let ann = Ann::new(String::from(img_id), img_size, boxes);

        Ok(ann)
    }

    pub fn parse_txt<P: AsRef<Path>>(
        path: P, 
        fmt: BBoxFmt,
        img_size: Option<ImgSize>,
        conf_last: bool,
        img_ext: &str,
    ) -> Result<Ann, ParseErr> { 
        Ann::parse_txt_raw(path, fmt, false, img_size, conf_last, img_ext)
    }

    pub fn parse_txt_rel<P: AsRef<Path>>(
        path: P, 
        fmt: BBoxFmt,
        img_size: ImgSize,
        conf_last: bool,
        img_ext: &str,
    ) -> Result<Ann, ParseErr> { 
        Ann::parse_txt_raw(path, fmt, true, Some(img_size), conf_last, img_ext)
    }
}