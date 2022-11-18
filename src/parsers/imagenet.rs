use crate::{annotation::*, bbox::BBox, imgsize::ImgSize, parsers::ParseErr};
use std::fs;
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
        let size = ImgSize { width: ann.size.width, height: ann.size.height };
        let boxes = ann.object.into_iter()
            .map(Into::into)
            .collect();

        Ann::new(ann.filename, Some(size), boxes)
    }
}

impl Ann {
    pub fn from_imagenet(path: &str) -> Result<Ann, ParseErr> {
        let content = fs::read_to_string(path)
            .map_err(|_| ParseErr {})?;
        let ann: InetAnn = from_str(&content)
            .map_err(|_| ParseErr {})?;
        
        Ok(ann.into())
    }

    pub fn from_pascal_voc(path: &str) -> Result<Ann, ParseErr> {
        Ann::from_imagenet(path)
    }
}