use crate::{
    imgsize::ImgSize,
    bbox::{BBox, BBoxFmt},
    annotation::Ann,
    annotationset::AnnSet,
    parsers::ParseError,
    serde_records::coco::*,
};

use std::{
    fs, 
    path::Path, 
    collections::HashMap
};

use serde_json::from_str;

impl From<COCOImg> for Ann {
    fn from(img: COCOImg) -> Self {
        let img_size = ImgSize::new(img.width, img.height);

        Ann::new(img.img_id, Some(img_size), vec![])
    }
}

impl AnnSet {
    // Could optimize clones
    
    pub fn parse_coco<P: AsRef<Path>>(path: P) -> Result<AnnSet, ParseError> {
        let mut annset = AnnSet::new();

        let content = fs::read_to_string(path.as_ref())
            .map_err(|_| ParseError {})?;

        let coco = from_str::<COCOAnnSet>(&content)
            .map_err(|_| ParseError {})?;

        let to_label = coco.categories.iter()
            .map(|cat| {
                (cat.id, &cat.label)
            })
            .collect::<HashMap<_, _>>();

        let to_img = coco.images.iter()
            .map(|img| {
                (img.id, &img.img_id)
            })
            .collect::<HashMap<_, _>>();

        for img in &coco.images {
            let img_size = ImgSize::new(img.width, img.height);
            let ann = Ann::new(img.img_id.clone(), Some(img_size), vec![]);

            annset.insert(ann);
        }

        for coco_ann in coco.annotations {
            let img_id = to_img.get(&coco_ann.img_id)
                .ok_or(ParseError {})?;

            // Redundant. Would need a to_ann with &Ann

            let label = to_label.get(&coco_ann.cat_id)
                .ok_or(ParseError {})?;
            
            if let [l, t, w, h] = coco_ann.bbox[..] {
                let bbox = BBox::create(
                    String::from(*label), 
                    (l, t, w, h),
                    BBoxFmt::LTWH,
                    coco_ann.conf,
                );

                let ann = annset.get_mut(*img_id)
                    .expect("Image id must be present.");
                
                ann.bboxes.push(bbox);
            } else {
                Err(ParseError {})?
            }
        }

        Ok(annset)
    }
}