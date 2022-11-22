use crate::{
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
        if let Some(img_size) = ann.img_size {
            let image_width = img_size.width;
            let image_height = img_size.height;

            let image_path = ann.img_id.clone();

            let shapes = ann.bboxes.iter()
                .map(Into::<LMShape>::into)
                .collect::<Vec<_>>();

            Ok(LMAnn { image_path, image_width, image_height, shapes })
        } else {
            Err(ConvError {})
        }
    }
}

impl Ann {
    /// Serialize the annotation to a String of LabelMe annotation format.
    pub fn to_labelme(&self) -> Result<String, ConvError> {
        let ann: LMAnn = self.try_into()?;
        
        to_string(&ann).map_err(|_| ConvError {})
    }

    /// Save the annotation in folder `path`.
    pub fn save_labelme<P: AsRef<Path>>(&self, path: P) -> Result<(), ConvError> {
        let mut path = path.as_ref().to_path_buf();
        path.push(&self.img_id);
        path.set_extension("json");

        let content = self.to_labelme()?;

        fs::write(path, content)
            .map_err(|_| ConvError {})
    }
}

impl AnnSet {
    /// Saves the annotations in folder `path`.
    pub fn save_labelme<P: AsRef<Path>>(&self, path: P) -> Result<(), ConvError> {
        for item in &self.items {
            let ann = item.1;
            ann.save_labelme(&path)?
        }

        Ok(())
    }
}