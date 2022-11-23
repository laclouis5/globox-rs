use crate::{
    imgsize::ImgSize,
    bbox::BBox,
    annotation::Ann,
    annotationset::AnnSet,
    converters::ConvError,
};

use std::{path::Path, fs, ffi::OsStr};

use quick_xml::{Writer, events::BytesText};

impl AnnSet {
    pub fn save_cvat<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<(), ConvError> {
        if let Some(e) = path.as_ref().extension() {
            if e != OsStr::new("xml") {
                return Err(ConvError {})
            }
        }

        let file = fs::File::create(path)
            .map_err(|_| ConvError {})?;

        let mut writer = Writer::new(file);

        let root = writer.create_element("annotations");

        root.write_inner_content(|writer| {
            writer.create_element("meta")
                .write_inner_content(|writer| {
                    writer.create_element("task")
                        .write_inner_content(|writer| {
                            let elt = writer.create_element("size");
                            let size = self.len().to_string();
                            let content = BytesText::new(size.as_str());
                            elt.write_text_content(content)?;
                            Ok(())
                        })?;
                    Ok(())
                })?;
                
                // TODO: this sucks, let's use serde.
                // for ann in self {
                //     let name = &ann.img_id;
                //     let img_size = ann.img_size.ok_or(ConvError {})?;
                //     let ImgSize { width, height } = img_size;
                // }

            Ok(())
        })
        .map_err(|_| ConvError {})?;

        Ok(())
    }
}