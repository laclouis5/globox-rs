use crate::{
    imgsize::ImgSize,
    bbox::BBoxFmt,
    annotation::Ann,
    parsers::ParseErr,
};

use std::path::Path;

impl Ann {
    pub fn parse_yolo<P: AsRef<Path>>(
        path: P, 
        img_size: ImgSize,
        conf_last: bool,
        img_ext: &str,
    ) -> Result<Ann, ParseErr> { 
        Ann::parse_txt_rel(path, BBoxFmt::XYWH, img_size, conf_last, img_ext)
    }
}