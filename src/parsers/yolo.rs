use crate::{
    imgsize::ImgSize,
    bbox::BBoxFmt,
    annotation::Ann,
    annotationset::AnnSet,
    parsers::ParseError,
};

use std::path::Path;

impl Ann {
    pub fn parse_yolo<P: AsRef<Path>>(
        path: P, 
        img_size: ImgSize,
        conf_last: bool,
        img_ext: &str,
    ) -> Result<Ann, ParseError> { 
        Ann::parse_txt_rel(path, BBoxFmt::XYWH, img_size, conf_last, img_ext)
    }
}

impl AnnSet {
    pub fn parse_yolo<P1: AsRef<Path>, P2: AsRef<Path>>(
        path: P1,
        imgs_path: P2,
        conf_last: bool,
        img_ext: &str,
    ) -> Result<AnnSet, ParseError> {
        AnnSet::parse_txt_rel(path, BBoxFmt::XYWH, imgs_path, conf_last, img_ext)
    }
}