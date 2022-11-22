use crate::{
    imgsize::ImgSize, 
    bbox::BBox, 
    annotation::Ann, 
    annotationset::AnnSet,
    parsers::{ParseError, folder::parse_folder},
};

use std::{
    fs,
    path::Path,
};

use serde::Deserialize;
use quick_xml::de::from_str;

#[derive(Deserialize)]
struct INetBndBox {
    xmin: f32, ymin: f32, xmax: f32, ymax: f32,
}

#[derive(Deserialize)]
struct InetSize {
    width: u32, height: u32,
}

#[derive(Deserialize)]
struct InetObj {
    name: String, bndbox: INetBndBox,
}

impl From<InetObj> for BBox {
    fn from(obj: InetObj) -> Self {
        BBox::new(
            obj.name,
            obj.bndbox.xmin,
            obj.bndbox.ymin,
            obj.bndbox.xmax,
            obj.bndbox.ymax,
            None,
        )
    }
}

#[derive(Deserialize)]
struct InetAnn {
    filename: String, size: InetSize, object: Vec<InetObj>,
}

impl From<InetAnn> for Ann {
    fn from(ann: InetAnn) -> Ann {
        let size = ImgSize::new(ann.size.width, ann.size.height);
        
        let boxes = ann.object.into_iter()
            .map(Into::into)
            .collect();

        Ann::new(ann.filename, Some(size), boxes)
    }
}

impl Ann {
    pub fn parse_imagenet<P: AsRef<Path>>(path: P) -> Result<Ann, ParseError> {
        let content = fs::read_to_string(path)
            .map_err(|_| ParseError {})?;
        
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