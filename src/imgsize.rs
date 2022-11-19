use crate::parsers::ParseErr;

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
    fn from_file(path: &str) -> Result<ImgSize, ParseErr> {
        size(path)
            .map_err(|_| ParseErr {})
            .map(Into::into)
    }
}