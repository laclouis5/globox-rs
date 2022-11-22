use crate::{
    imgsize::ImgSize,
    coords::abs_to_rel,
    bbox::{BBox, BBoxFmt},
    annotation::Ann,
    annotationset::AnnSet,
    converters::ConvError,
};

use std::{path::Path, fs};

use itertools::Itertools;

impl BBox {
    // TODO: Add error handling for " " in label.
    fn to_txt(&self, fmt: BBoxFmt, conf_last: bool) -> String {
        let label = &self.label;
        let (c1, c2, c3, c4) = self.coords(fmt);

        match (self.conf(), conf_last) {
            (None, _) => {
                format!("{label} {c1} {c2} {c3} {c4}")
            },
            (Some(conf), false) => {
                format!("{label} {conf} {c1} {c2} {c3} {c4}")
            },
            (Some(conf), true) => {
                format!("{label} {c1} {c2} {c3} {c4} {conf}")
            }
        }
    }

    // TODO: Add error handling for " " in label.
    fn to_txt_rel(&self, fmt: BBoxFmt, conf_last: bool, img_size: ImgSize) -> String {
        let label = &self.label;
        let coords = self.coords(fmt);
        let (c1, c2, c3, c4) = abs_to_rel(coords, img_size);

        match (self.conf(), conf_last) {
            (None, _) => {
                format!("{label} {c1} {c2} {c3} {c4}")
            },
            (Some(conf), false) => {
                format!("{label} {conf} {c1} {c2} {c3} {c4}")
            },
            (Some(conf), true) => {
                format!("{label} {c1} {c2} {c3} {c4} {conf}")
            }
        }
    }
}

impl Ann {
    pub fn to_txt(
        &self,
        fmt: BBoxFmt,
        conf_last: bool,
    ) -> String {
        self.bboxes.iter()
            .map(|bbox| bbox.to_txt(fmt, conf_last))
            .join("\n")
    }

    pub fn to_txt_rel(
        &self,
        fmt: BBoxFmt,
        conf_last: bool,
    ) -> Result<String, ConvError> {
        let img_size = self.img_size.ok_or(ConvError {})?;

        let content = self.bboxes.iter()
            .map(|bbox| bbox.to_txt_rel(fmt, conf_last, img_size))
            .join("\n");

        Ok(content)
    }
}

impl Ann {
    pub fn save_txt<P: AsRef<Path>>(
        &self,
        path: P,
        fmt: BBoxFmt,
        conf_last: bool,
    ) -> Result<(), ConvError> {
        let mut path = path.as_ref().to_path_buf();
        path.push(&self.img_id);
        path.set_extension("txt");

        let contents = self.to_txt(fmt, conf_last);

        fs::write(path, contents).map_err(|_| ConvError {})
    }

    pub fn save_txt_rel<P: AsRef<Path>>(
        &self,
        path: P,
        fmt: BBoxFmt,
        conf_last: bool,
    ) -> Result<(), ConvError> {
        let mut path = path.as_ref().to_path_buf();
        path.push(&self.img_id);
        path.set_extension("txt");

        let contents = self.to_txt_rel(fmt, conf_last)?;

        fs::write(path, contents).map_err(|_| ConvError {})
    }
}

impl AnnSet {
    pub fn save_txt<P: AsRef<Path>>(
        &self,
        path: P,
        fmt: BBoxFmt,
        conf_last: bool,
    ) -> Result<(), ConvError> {
        self.save_all(|ann| ann.save_txt(&path, fmt, conf_last))
    }

    pub fn save_txt_rel<P: AsRef<Path>>(
        &self,
        path: P,
        fmt: BBoxFmt,
        conf_last: bool,
    ) -> Result<(), ConvError> {
        self.save_all(|ann| ann.save_txt_rel(&path, fmt, conf_last))
    }
}