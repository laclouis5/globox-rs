use crate::{
    coords::abs_to_rel,
    annotationset::AnnSet,
    converters::ConvError,
    serde_records::openimage::*,
};

use std::path::Path;

use csv;

impl AnnSet {
    pub fn save_openimage<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<(), ConvError> {
        let mut writer = csv::Writer::from_path(path)
            .map_err(|_| ConvError {})?;

        for ann in self {
            let img_id = &ann.img_id;
            let img_size = ann.img_size.ok_or(ConvError {})?;

            for bbox in &ann.bboxes {
                let label = &bbox.label;

                let coords = bbox.ltrb();
                let (xmin, ymin, xmax, ymax) = abs_to_rel(coords, img_size);

                let conf = bbox.conf();

                let line = OALine::new(
                    img_id.as_str(), label.as_str(), 
                    xmin, ymin, xmax, ymax,
                    conf
                );

                writer.serialize(line)  
                    .map_err(|_| ConvError {})?;
            }
        }

        writer.flush().map_err(|_| ConvError {})?;

        Ok(())
    }
}