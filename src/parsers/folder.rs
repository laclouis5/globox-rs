use crate::{
    annotation::Ann,
    annotationset::AnnSet,
    parsers::ParseError,
};

use std::{
    path::{Path, PathBuf},
};

fn read_dir<P: AsRef<Path>>(path: P, file_ext: &str) -> Result<Vec<PathBuf>, ParseError> {
    path.as_ref()
        .read_dir()
        .map_err(|_| ParseError {})?
        .filter_map(|result| {
            match result {
                Err(_) => Some(Err(ParseError {})),

                Ok(entry) => {
                    let p = entry.path();
                    match p.extension() {
                        Some(e) if e == file_ext => {
                            Some(Ok(p))
                        },

                        _ => None
                    }
                }
            }
        })
        .collect::<Result<Vec<_>, _>>()
}

impl AnnSet {
    pub fn parse_folder<P, F>(
        path: P, 
        ext: &str,
        parser: F,
    ) -> Result<AnnSet, ParseError> where
        P: AsRef<Path>,
        F: Fn(&Path) -> Result<Ann, ParseError>,
    {
        // Eagerly read the directory so we can monitor parsing 
        // progress in the future.
        let files = read_dir(path, ext)?;

        let mut annset = AnnSet::with_capacity(files.len());
    
        for p in files {
            let ann = parser(p.as_ref())?;
            annset.insert(ann);
        }
    
        Ok(annset)
    }
}
