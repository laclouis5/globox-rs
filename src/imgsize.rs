use crate::parsers::ParseError;

use std::path::Path;
use imagesize::{size, ImageSize};

/// An image size.
#[derive(Debug, Clone, Copy)]
pub struct ImgSize {
    /// The image width.
    pub width: u32,
    /// The image height.
    pub height: u32,
}

impl ImgSize {
    /// Creates a new image size.
    pub fn new(width: u32, height: u32) -> Self {
        ImgSize { width, height }
    }
}

impl From<ImageSize> for ImgSize {
    fn from(image_size: ImageSize) -> Self {
        ImgSize { width: image_size.width as u32, height: image_size.height as u32 }
    }
}

impl ImgSize {
    /// Returns the size of an image stored on disk file without loading it into memory.
    /// 
    /// # Arguments
    /// * `path`: The path of the image file.
    /// 
    /// # Errors
    /// The operation will fail if the file is unreadable or corrupted.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<ImgSize, ParseError> {
        size(path)
            .map_err(|_| ParseError {})
            .map(Into::into)
    }
}