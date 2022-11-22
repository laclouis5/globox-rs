use crate::{
    bbox::BBoxFmt,
    annotation::Ann,
    annotationset::AnnSet,
    converters::ConvError,
};

use std::path::Path;

impl Ann {
    pub fn to_yolo(
        &self,
        conf_last: bool,
    ) -> Result<String, ConvError> {
        self.to_txt_rel(BBoxFmt::XYWH, conf_last)
    }

    pub fn save_yolo<P: AsRef<Path>>(
        &self,
        path: P,
        conf_last: bool,
    ) -> Result<(), ConvError> {
        self.save_txt_rel(path, BBoxFmt::XYWH, conf_last)
    }
}

impl AnnSet {
    pub fn save_yolo<P: AsRef<Path>>(
        &self,
        path: P,
        conf_last: bool,
    ) -> Result<(), ConvError> {
        self.save_txt_rel(path, BBoxFmt::XYWH, conf_last)
    }
}