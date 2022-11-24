use crate::{annotationset::AnnSet, path::expand_user};

use super::SrcAnnFmt;

use std::time::Instant;

use clap::Args;

#[derive(Args)]
pub(super) struct Parse {
    #[arg(help = "The format of the annotations")]
    format: SrcAnnFmt,

    #[arg(help = "The file or directory path of the annotations")]
    path: String, 

    #[arg(long, help = "The image directory of the annotations")]
    imgs_path: Option<String>,

    #[arg(long, default_value = "jpg", help = "The image extension (YOLO only)")]
    img_ext: String,

    #[arg(long, help = "Whether or not the confidence is stored in last position (YOLO only)")]
    conf_last: bool,
}

impl Parse {
    pub(super) fn run(self) {
        let path = expand_user(self.path);

        let time = Instant::now();

        let annset = match self.format {
            SrcAnnFmt::Coco => {
                AnnSet::parse_coco(path)
                    .expect("failed to parse")
            }, 
            
            SrcAnnFmt::Cvat => {
                AnnSet::parse_cvat(path)
                    .expect("failed to parse")
            },

            SrcAnnFmt::OpenImage => {
                let imgs_path = self.imgs_path
                    .expect("parsing OpenImage requires `imgs_path`");
                let imgs_path = expand_user(imgs_path);

                AnnSet::parse_openimage(path, imgs_path)
                    .expect("failed to parse")
            },

            SrcAnnFmt::Labelme => {
                AnnSet::parse_labelme(path)
                    .expect("failed to parse")
            },

            SrcAnnFmt::PascalVoc => {
                AnnSet::parse_pascal_voc(path)
                    .expect("failed to parse")
            },

            SrcAnnFmt::Imagenet => {
                AnnSet::parse_imagenet(path)
                    .expect("failed to parse")
            }

            SrcAnnFmt::Yolo => {
                let imgs_path = self.imgs_path
                    .expect("parsing OpenImage requires `imgs_path`");
                let imgs_path = expand_user(imgs_path);

                AnnSet::parse_yolo(
                    path, 
                    imgs_path, 
                    self.conf_last, 
                    &self.img_ext,
                ).expect("failed to parse the annotations")
            }
        };

        println!("Parsed {} annotations in {:#?}.", annset.len(), time.elapsed());
    }
}