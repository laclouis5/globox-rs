pub mod cvat;
pub mod imagenet;
pub mod labelme;
pub mod openimage;
pub mod txt;
pub mod yolo;
pub mod coco;
pub mod folder;

use std::{
    path::Path,
//     fmt::Display,
//     error::Error,
};

#[derive(Debug)]
pub struct ParseErr { 
    // source: Box<dyn Error + 'static>
}

pub(crate) fn path_to_img_id<P: AsRef<Path>>(path: P, img_ext: &str) -> Result<String, ParseErr> {
    let img_id = path.as_ref()
            .with_extension(img_ext);
            
    let img_id = img_id
        .file_name()
        .ok_or(ParseErr {})?
        .to_str()
        .ok_or(ParseErr {})?;
        
    Ok(String::from(img_id))
}

// impl ParseErr {
//     pub fn new(source: Box<dyn Error + 'static>) -> Self {
//         ParseErr { source: source }
//     }
// }

// impl Display for ParseErr {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "parse error")
//     }
// }

// impl Error for ParseErr {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         Some(&*self.source)
//     }
// }