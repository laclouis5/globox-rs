pub mod cvat;
pub mod imagenet;
pub mod labelme;
pub mod openimage;
pub mod txt;
pub mod yolo;
pub mod coco;

// use std::{
//     fmt::Display,
//     error::Error,
// };

#[derive(Debug)]
pub struct ParseErr { 
    // source: Box<dyn Error + 'static>
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