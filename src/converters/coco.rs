use crate::{
    imgsize::ImgSize,
    annotationset::AnnSet,
    converters::ConvError,
    serde_records::coco::*,
};

use std::{
    collections::{BTreeSet, HashMap},
    path::Path, 
    fs,
    ffi::OsStr,
};

use serde_json::ser::to_string;

impl AnnSet {
    pub fn save_coco<P: AsRef<Path>>(&self, path: P) -> Result<(), ConvError> {
        if let Some(e) = path.as_ref().extension() {
            if e != OsStr::new("json") {
                return Err(ConvError {})
            }
        }

        let mut labels = BTreeSet::new();
        let mut img_ids = BTreeSet::new();
        let mut nb_bboxes = 0;

        for ann in self {
            img_ids.insert(ann.img_id.as_str());
            nb_bboxes += ann.bboxes.len();
            
            for bbox in &ann.bboxes {
                labels.insert(bbox.label.as_str());
            }
        }

        let to_cat_id = labels.iter()
            .enumerate()
            .map(|(k, &v)| (v, k))
            .collect::<HashMap<_, _>>();

        let to_img_id = img_ids.into_iter()
            .enumerate()
            .map(|(k, v)| (v, k))
            .collect::<HashMap<_, _>>();

        let mut cats: Vec<COCOCat> = Vec::with_capacity(to_cat_id.len());
        let mut imgs: Vec<COCOImg> = Vec::with_capacity(to_img_id.len());
        let mut anns: Vec<COCOAnn> = Vec::with_capacity(nb_bboxes);
        
        for ann in self {
            let img_id = to_img_id[ann.img_id.as_str()];
            let img_size = ann.img_size.ok_or(ConvError {})?;
            let ImgSize { width, height } = img_size;

            let img = COCOImg { 
                id: img_id,
                width, height,
                img_id: ann.img_id.clone(),
            };

            imgs.push(img);

            for bbox in &ann.bboxes {
                let (xmin, ymin, width, height) = bbox.ltwh();
                let coords = vec![xmin, ymin, width, height];

                let ann = COCOAnn {
                    cat_id: to_cat_id[bbox.label.as_str()],
                    img_id: img_id,
                    bbox: coords, 
                    conf: bbox.conf()
                };

                anns.push(ann);
            }
        }

        for label in labels {
            let cat = COCOCat {
                id: to_cat_id[label],
                label: label.into(),
            };

            cats.push(cat);
        }

        let annset = COCOAnnSet {
            categories: cats,
            images: imgs,
            annotations: anns,
        };

        let contents = to_string(&annset)
            .map_err(|_| ConvError {})?;

        fs::write(path, contents)
            .map_err(|_| ConvError {})?;

        Ok(())
    }
}