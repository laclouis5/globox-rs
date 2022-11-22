use crate::{
    bbox::BBox,
    annotation::Ann,
    annotationset::AnnSet,
    converters::ConvError,
    serde_records::imagenet::*,
};

use std::{path::Path, fs};

use quick_xml::se::to_string;

impl From<&BBox> for InetObj {
    fn from(bbox: &BBox) -> Self {
        let (xmin, ymin, xmax, ymax) = bbox.ltrb();
        let bndbox = INetBndBox { xmin, ymin, xmax, ymax };

        InetObj { name: bbox.label.clone(), bndbox }
    }
}

impl TryFrom<&Ann> for InetAnn {
    type Error = ConvError;

    fn try_from(ann: &Ann) -> Result<Self, Self::Error> {
        let filename = ann.img_id.clone();

        let size: InetSize = ann.img_size
            .ok_or(ConvError {})?
            .into();

        let objects = ann.bboxes.iter()
            .map(Into::<InetObj>::into)
            .collect::<Vec<_>>();

        let ann = InetAnn { filename, size, objects };
        
        Ok(ann)
    }
}

impl Ann {
    /// Serialize the annotation to a String of Imagenet annotation format.
    pub fn to_imagenet(&self) -> Result<String, ConvError> {
        let ann: InetAnn = self.try_into()?;

        // Faster to first write the whole content to a String
        // then write to a file. Plus those files are quite small. 
        to_string(&ann).map_err(|_| ConvError {})
    }

    /// Save the annotation to Imagenet annotation format.
    pub fn save_imagenet<P: AsRef<Path>>(&self, path: P) -> Result<(), ConvError> {
        let mut path = path.as_ref().to_path_buf();
        path.push(&self.img_id);
        path.set_extension("xml");

        let contents = self.to_imagenet()?;

        fs::write(path, contents).map_err(|_| ConvError {})
    }
}

impl Ann {
    /// Serialize the annotation to a String of Pascal VOC annotation format.
    pub fn to_pascal_voc(&self) -> Result<String, ConvError> {
        self.to_imagenet()
    }

    /// Save the annotation to Pascal VOC annotation format.
    pub fn save_pascal_voc<P: AsRef<Path>>(&self, path: P) -> Result<(), ConvError> {
        self.save_imagenet(path)
    }
}

impl AnnSet {
    /// Save the annotations to Imagenet annotation format.
    pub fn save_imagenet<P: AsRef<Path>>(&self, path: P) -> Result<(), ConvError> {
        self.save_all(|ann| ann.save_imagenet(&path))
    }

    /// Save the annotations to Pascal VOC annotation format.
    pub fn save_pascal_voc<P: AsRef<Path>>(&self, path: P) -> Result<(), ConvError> {
        self.save_imagenet(path)
    }
}