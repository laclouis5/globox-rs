pub mod coco;
pub mod cvat;
pub mod folder;
pub mod imagenet;
pub mod labelme;
pub mod openimage;
pub mod txt;
pub mod yolo;

use std::{
    path::Path,
//     fmt::Display,
//     error::Error,
};

#[derive(Debug)]
pub struct ParseError { 
    // source: Box<dyn Error + 'static>
}

// pub type ParseResult<T> = Result<T, ParseError>;

pub(crate) fn path_to_img_id<P: AsRef<Path>>(path: P, img_ext: &str) -> Result<String, ParseError> {
    let img_id = path.as_ref()
            .with_extension(img_ext);
            
    let img_id = img_id
        .file_name()
        .ok_or(ParseError {})?
        .to_str()
        .ok_or(ParseError {})?;
        
    Ok(String::from(img_id))
}

// impl ParseError {
//     pub fn new(source: Box<dyn Error + 'static>) -> Self {
//         ParseError { source: source }
//     }
// }

// impl Display for ParseError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "parse error")
//     }
// }

// impl Error for ParseError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         Some(&*self.source)
//     }
// }