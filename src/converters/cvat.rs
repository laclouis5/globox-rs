use crate::{
    imgsize::ImgSize,
    bbox::BBox,
    annotation::Ann,
    annotationset::AnnSet,
    converters::ConvError,
};

use std::{path::Path, fs, ffi::OsStr};

use quick_xml::{Writer, events::{Event, BytesDecl, BytesStart, BytesText}};

fn write_decl(writer: &mut Writer<fs::File>) -> Result<(), ConvError> {
    let event = Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None));
    writer.write_event(event).map_err(|_| ConvError {})
}

fn write_size(writer: &mut Writer<fs::File>, size: usize) -> Result<(), ConvError> {
    let meta_start = BytesStart::new("meta");
    writer.write_event(Event::Start(meta_start.borrow()))
        .map_err(|_| ConvError {})?;

        let task_start = BytesStart::new("task");
        writer.write_event(Event::Start(task_start.borrow()))
            .map_err(|_| ConvError {})?;

            let size_start = BytesStart::new("size");
            writer.write_event(Event::Start(size_start.borrow()))    
                .map_err(|_| ConvError {})?;
            writer.write_event(Event::Text(BytesText::new(size.to_string().as_str())))
                .map_err(|_| ConvError {})?;
            writer.write_event(Event::End(size_start.to_end()))
                .map_err(|_| ConvError {})?;

        writer.write_event(Event::End(task_start.to_end()))
            .map_err(|_| ConvError {})?;

    writer.write_event(Event::End(meta_start.to_end()))
        .map_err(|_| ConvError {})?;
    
    Ok(())
}

fn write_bbox(writer: &mut Writer<fs::File>, bbox: &BBox) -> Result<(), ConvError> {
    let label = &bbox.label;
    let (xmin, ymin, xmax, ymax) = bbox.ltrb();

    writer.create_element("box")
        .with_attribute(("label", label.as_str()))
        .with_attribute(("xtl", xmin.to_string().as_str()))
        .with_attribute(("ytl", ymin.to_string().as_str()))
        .with_attribute(("xbr", xmax.to_string().as_str()))
        .with_attribute(("ybr", ymax.to_string().as_str()))
        .write_empty()
        .map_err(|_| ConvError {})?;

    Ok(())
}

fn write_ann(writer: &mut Writer<fs::File>, ann: &Ann) -> Result<(), ConvError> {
    let name = &ann.img_id;

    let img_size = ann.img_size.ok_or(ConvError {})?;
    let ImgSize { width, height } = img_size;

    let mut ann_start = BytesStart::new("image");
    ann_start.push_attribute(("name", name.as_str()));
    ann_start.push_attribute(("width", width.to_string().as_str()));
    ann_start.push_attribute(("height", height.to_string().as_str()));

    writer.write_event(Event::Start(ann_start.borrow()))
        .map_err(|_| ConvError {})?;

    for bbox in &ann.bboxes {
        write_bbox(writer, bbox)?
    }

    writer.write_event(Event::End(ann_start.to_end()))
        .map_err(|_| ConvError {})?;

    Ok(())
}

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

        let mut writer = Writer::new_with_indent(file, b' ', 2);

        // Decl
        write_decl(&mut writer)
            .map_err(|_| ConvError {})?;

        // Annotation Start
        let ann_start = BytesStart::new("annotations");
        writer.write_event(Event::Start(ann_start.borrow()))
            .map_err(|_| ConvError {})?;

        // Write size
        write_size(&mut writer, self.len())
            .map_err(|_| ConvError {})?;

        // Image annotations
        for ann in self {
            write_ann(&mut writer, ann)?;
        }

        // Annotation End
        writer.write_event(Event::End(ann_start.to_end()))
            .map_err(|_| ConvError {})?;

        Ok(())
    }
}