use crate::{annotationset::AnnSet, parsers::ParseErr, annotation::Ann, imgsize::ImgSize, bbox::BBox};

use std::{collections::HashMap, borrow::Cow};
use quick_xml::{events::{Event, attributes::Attributes}, reader::Reader, name::QName};

fn get_u32(attrs: &HashMap<QName, Result<Cow<str>, quick_xml::Error>>, name: &str) -> Result<u32, ParseErr> {
    attrs.get(&QName(name.as_bytes()))
        .ok_or(ParseErr {})?
        .as_ref()
        .map_err(|_| ParseErr {})?
        .parse::<u32>()
        .map_err(|_| ParseErr {})
}

fn get_f32(attrs: &HashMap<QName, Result<Cow<str>, quick_xml::Error>>, name: &str) -> Result<f32, ParseErr> {
    attrs.get(&QName(name.as_bytes()))
        .ok_or(ParseErr {})?
        .as_ref()
        .map_err(|_| ParseErr {})?
        .parse::<f32>()
        .map_err(|_| ParseErr{})
}

fn get_string(attrs: &HashMap<QName, Result<Cow<str>, quick_xml::Error>>, name: &str) -> Result<String, ParseErr> {
    let string = attrs.get(&QName(name.as_bytes()))
        .ok_or(ParseErr {})?
        .as_ref()
        .map_err(|_| ParseErr {})?
        .as_ref()
        .to_owned();

    Ok(string)
}

// TODO: Use HashMap.many_[mut] to obtain all requested values.
fn as_hash_map(attributes: Attributes) -> HashMap<QName, Result<Cow<str>, quick_xml::Error>> {
    attributes
        .filter_map(|a| {
            a.ok()
        })
        .map(|a| {
            (a.key, a.unescape_value())
        })
        .collect::<HashMap<_, _>>()
}

#[derive(Clone, Copy)]
enum SizeState {
    None, Started, Ended,
}

fn parse_cvat(path: &str) -> Result<AnnSet, ParseErr> {
    let mut reader = Reader::from_file(path)
        .map_err(|_| ParseErr {})?;

    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut anns = AnnSet::new();
    let mut size_state = SizeState::None;

    let mut ann: Option<Ann> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Err(_) => Err(ParseErr {})?,
            
            Ok(Event::Eof) => break,

            Ok(Event::Start(data)) => {
                match data.name().as_ref() {
                    b"size" => {
                        if let SizeState::None = size_state {
                            size_state = SizeState::Started;
                        }
                    },

                    b"image" => {
                        assert!(ann.is_none(), "Annotation should be empty.");

                        let img_attrs = as_hash_map(data.attributes());

                        let img_id = get_string(&img_attrs, "name")?;
                        let width = get_u32(&img_attrs, "width")?;
                        let height = get_u32(&img_attrs, "height")?;
                        let img_size = ImgSize { width, height };

                        ann = Some(Ann::new(img_id, Some(img_size), vec![]));
                    },

                    b"box" => {
                        let bbox_attrs = as_hash_map(data.attributes());

                        let label = get_string(&bbox_attrs, "label")?;
                        let xmin = get_f32(&bbox_attrs, "xtl")?;
                        let ymin = get_f32(&bbox_attrs, "ytl")?;
                        let xmax = get_f32(&bbox_attrs, "xbr")?;
                        let ymax = get_f32(&bbox_attrs, "ybr")?;

                        match &mut ann {
                            None => panic!("Ann should not be None at this point."),

                            Some(a) => {
                                let bbox = BBox::new(label, xmin, ymin, xmax, ymax, None);
                                a.boxes.push(bbox);
                            },
                        }
                    },

                    _ => (),
                }
            },

            Ok(Event::End(data)) => {
                if data.name().as_ref() == b"image" {
                    if let Some(a) = ann.take() {
                        anns.items.insert(a.img_id.clone(), a);
                    } else {
                        panic!("Ann should not be None at this point.")
                    }
                }
            },

            Ok(Event::Text(data)) => {
                if let SizeState::Started = size_state {
                    let size = data.unescape().ok()
                        .and_then(|s| s.parse::<usize>().ok());

                    if let Some(s) = size {
                        anns.items.reserve(s);
                    }

                    size_state = SizeState::Ended;
                }
            },

            _ => (),
        }

        buf.clear();
    }

    Ok(anns)
}

impl AnnSet {
    pub fn parse_cvat(path: &str) -> Result<AnnSet, ParseErr> {
        parse_cvat(path)
    }
}