use crate::{
    imgsize::ImgSize,
    bbox::{BBox, BBoxFmt},
    annotation::Ann,
    annotationset::AnnSet,
    parsers::ParseErr,
};


use std::{
    error::Error,
    fs, 
    path::Path, 
    collections::HashMap
};

use serde::Deserialize;
use serde_json::from_str;

#[derive(Debug, Deserialize)]
struct COCOCat {
    id: u32,

    #[serde(rename = "name")]
    label: String,
}

#[derive(Debug, Deserialize)]
struct COCOImg {
    id: u32,
    width: u32,
    height: u32,

    #[serde(rename = "filename")]
    img_id: String,
}

impl From<COCOImg> for Ann {
    fn from(img: COCOImg) -> Self {
        let img_size = ImgSize::new(img.width, img.height);

        Ann::new(img.img_id, Some(img_size), vec![])
    }
}

#[derive(Debug, Deserialize)]
struct COCOAnn {
    #[serde(rename = "category_id")]
    cat_id: u32,

    #[serde(rename = "image_id")]
    img_id: u32,

    bbox: Vec<f32>
}

#[derive(Debug, Deserialize)]
struct COCOAnnSet {
    categories: Vec<COCOCat>,
    images: Vec<COCOImg>,
    annotations: Vec<COCOAnn>,
}

impl AnnSet {
    // Could optimize clones
    pub fn parse_coco<P: AsRef<Path>>(path: P) -> Result<AnnSet, ParseErr> {
        let mut annset = AnnSet::new();

        let content = fs::read_to_string(path.as_ref())
            .map_err(|_| ParseErr {})?;

        let coco: COCOAnnSet = from_str(&content)
            .map_err(|_| ParseErr {})?;


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

            annset.items.insert(img.img_id.clone(), ann);
        }

        for coco_ann in coco.annotations {
            let img_id = to_img.get(&coco_ann.img_id)
                .ok_or(ParseErr {})?;

            // Redundant. Would need a to_ann with &Ann

            let label = to_label.get(&coco_ann.cat_id)
                .ok_or(ParseErr {})?;
            
            if let [l, t, w, h] = coco_ann.bbox[..] {
                let bbox = BBox::create(
                    String::from(*label), 
                    (l, t, w, h),
                    BBoxFmt::LTWH,
                    None
                );

                let ann = annset.items.get_mut(*img_id)
                    .expect("Image id must be present.");
                
                ann.boxes.push(bbox);
            } else {
                Err(ParseErr {})?
            }
        }

        Ok(annset)
    }
}