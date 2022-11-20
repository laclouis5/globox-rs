use crate::parsers::ParseErr;

use std::path::Path;
use imagesize::{size, ImageSize};


#[derive(Debug, Clone, Copy)]
pub struct ImgSize {
    pub width: u32,
    pub height: u32,
}

impl From<ImageSize> for ImgSize {
    fn from(image_size: ImageSize) -> Self {
        ImgSize { width: image_size.width as u32, height: image_size.height as u32 }
    }
}

impl ImgSize {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<ImgSize, ParseErr> {
        size(path)
            .map_err(|_| ParseErr {})
            .map(Into::into)
    }
}