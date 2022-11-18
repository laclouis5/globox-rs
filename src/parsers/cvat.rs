use crate::{annotationset::AnnSet, parsers::ParseErr};

use std::collections::HashMap;
use quick_xml::{events::Event, reader::Reader};

fn parse_cvat(path: &str) -> Result<AnnSet, ParseErr> {
    let mut reader = Reader::from_file(path)
        .map_err(|_| ParseErr {})?;

    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut anns = HashMap::new();
    let mut in_size = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => return Err(ParseErr {}),
            
            Ok(Event::Eof) => break,

            Ok(Event::Start(data)) => {
                match data.name().as_ref() {
                    b"size" => {
                        in_size = true;
                    },

                    b"image" => {
                        let attrs = data.attributes()
                            .filter_map(|a| {
                                a.ok()
                            })
                            .map(|a| {
                                (a.key, a.value)
                            })
                            .collect::<HashMap<_, _>>();
                            // .collect::<Result<Vec<_>, _>>()
                            // .map_err(|_| ParseErr {})?;
                    },
                }
            },

            Ok(Event::Text(data)) => {
                if in_size {
                    let size = data.unescape()
                    .map_err(|_| ParseErr {})?
                    .parse::<usize>()
                    .map_err(|_| ParseErr {})?;                    

                    anns.reserve(size);

                    in_size = false
                }
            }
        }

        buf.clear();
    }
}