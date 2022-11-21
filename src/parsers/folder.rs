use crate::{
    annotation::Ann,
    annotationset::AnnSet,
    parsers::ParseErr,
};

use std::{
    path::{Path, PathBuf},
};

fn read_dir<P: AsRef<Path>>(path: P, ext: &str) -> Result<Vec<PathBuf>, ParseErr> {
    path.as_ref()
        .read_dir()
        .map_err(|_| ParseErr {})?
        .filter_map(|result| {
            match result {
                Err(_) => Some(Err(ParseErr {})),

                Ok(entry) => {
                    let p = entry.path();
                    match p.extension() {
                        Some(e) if e == ext => {
                            Some(Ok(p))
                        },

                        _ => None
                    }
                }
            }
        })
        .collect::<Result<Vec<_>, _>>()
}

pub fn parse_folder<P1: AsRef<Path>, F>(
    path: P1, 
    ext: &str,
    parser: F
) -> Result<AnnSet, ParseErr> where
    F: Fn(&Path) -> Result<Ann, ParseErr>,
{
    let files = read_dir(path, ext)?;
    let mut annset = AnnSet::with_capacity(files.len());

    for p in files {
        let ann = parser(p.as_ref())?;
        annset.items.insert(ann.img_id.clone(), ann);
    }

    Ok(annset)
}