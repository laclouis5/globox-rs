use crate::{
    imgsize::ImgSize,
    bbox::BBox,
    annotation::Ann,
    annotationset::AnnSet,
    converters::ConvError,
    serde_records::labelme::{LMShape, LMAnn},
};

use std::{
    path::Path,
    fs,
};

use serde_json::to_string;

impl From<&BBox> for LMShape {
    fn from(bbox: &BBox) -> Self {
        let (xmin, ymin, xmax, ymax) = bbox.ltrb();

        LMShape { 
            label: bbox.label.clone(), 
            points: vec![vec![xmin, ymin], vec![xmax, ymax]], 
            shape_type: String::from("rectangle"), 
        }
    }
}

impl TryFrom<&Ann> for LMAnn {
    type Error = ConvError;

    fn try_from(ann: &Ann) -> Result<Self, Self::Error> {
        let image_path = ann.img_id.clone();

        let img_size = ann.img_size.ok_or(ConvError {})?;
        let ImgSize { width: image_width, height: image_height } = img_size;

        let shapes = ann.bboxes.iter()
            .map(Into::<LMShape>::into)
            .collect::<Vec<_>>();

        Ok(LMAnn { image_path, image_width, image_height, shapes })
    }
}

impl Ann {
    /// Serialize the annotation to a String of Labelme annotation format.
    pub fn to_labelme(&self) -> Result<String, ConvError> {
        let ann: LMAnn = self.try_into()?;
        
        // Faster to first write the whole content to a String
        // then write to a file. Plus those files are quite small. 
        to_string(&ann).map_err(|_| ConvError {})
    }

    /// Save the annotation to Labelme annotation format.
    pub fn save_labelme<P: AsRef<Path>>(&self, path: P) -> Result<(), ConvError> {
        let mut path = path.as_ref().to_path_buf();
        path.push(&self.img_id);
        path.set_extension("json");

        let contents = self.to_labelme()?;

        fs::write(path, contents).map_err(|_| ConvError {})
    }
}

impl AnnSet {
    /// Save the annotations to Labelme annotation format.
    pub fn save_labelme<P: AsRef<Path>>(&self, path: P) -> Result<(), ConvError> {
        self.save_all(|ann| ann.save_labelme(&path))
    }
}