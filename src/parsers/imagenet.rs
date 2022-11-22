use crate::{
    imgsize::ImgSize, 
    bbox::BBox, 
    annotation::Ann, 
    annotationset::AnnSet,
    parsers::{ParseError, folder::parse_folder},
    serde_records::imagenet::*,
};

use std::{
    fs,
    path::Path,
};

use quick_xml::de::from_str;

impl From<InetObj> for BBox {
    fn from(obj: InetObj) -> Self {
        let bndbox = obj.bndbox;

        BBox::new(
            obj.name,
            bndbox.xmin,
            bndbox.ymin,
            bndbox.xmax,
            bndbox.ymax,
            None,
        )
    }
}

impl From<InetAnn> for Ann {
    fn from(ann: InetAnn) -> Ann {
        let size = ImgSize::new(ann.size.width, ann.size.height);
        
        let boxes = ann.objects.into_iter()
            .map(Into::into)
            .collect();

        Ann::new(ann.filename, Some(size), boxes)
    }
}

impl Ann {
    pub fn parse_imagenet<P: AsRef<Path>>(path: P) -> Result<Ann, ParseError> {
        let content = fs::read_to_string(path)
            .map_err(|_| ParseError {})?;
        
        // Annotation files are likely small. Parsing is likely faster
        // by first reading the entire file to memory before parsing it instead
        // of using a buffered reader (`from_reader`).
        let ann: InetAnn = from_str(&content)
            .map_err(|_| ParseError {})?;
        
        Ok(ann.into())
    }

    pub fn parse_pascal_voc<P: AsRef<Path>>(path: P) -> Result<Ann, ParseError> {
        Ann::parse_imagenet(path)
    }
}

impl AnnSet {
    pub fn parse_imagenet<P: AsRef<Path>>(path: P) -> Result<AnnSet, ParseError> {
        parse_folder(path, "xml", |p| Ann::parse_imagenet(p))
    }

    pub fn parse_pascal_voc<P: AsRef<Path>>(path: P) -> Result<AnnSet, ParseError> {
        parse_folder(path, "xml", |p| Ann::parse_pascal_voc(p))
    }
}