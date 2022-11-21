use crate::annotationset::AnnSet;

use super::SrcAnnFmt;

use std::{path::PathBuf, time::Instant};

use clap::Args;

#[derive(Args)]
pub(super) struct Parse {
    #[arg(help = "The format of the annotations")]
    format: SrcAnnFmt,

    #[arg(help = "The file or directory path of the annotations")]
    path: PathBuf, 

    #[arg(long, help = "The image directory of the annotations")]
    imgs_path: Option<PathBuf>,

    #[arg(long, default_value = "jpg", help = "The image extension (YOLO only)")]
    img_ext: String,

    #[arg(long, help = "Whether or not the confidence is stored in last position (YOLO only)")]
    conf_last: bool,
}

impl Parse {
    pub(super) fn run(self) {
        let time = Instant::now();

        let annset = match self.format {
            SrcAnnFmt::Coco => {
                AnnSet::parse_coco(self.path.as_path())
                    .expect("failed to parse")
            }, 
            
            SrcAnnFmt::Cvat => {
                AnnSet::parse_cvat(self.path.as_path())
                    .expect("failed to parse")
            },

            SrcAnnFmt::OpenImage => {
                let imgs_path = self.imgs_path
                    .expect("parsing OpenImage requires `imgs_path`");

                AnnSet::parse_openimage(self.path.as_path(), imgs_path.as_path())
                    .expect("failed to parse")
            },

            SrcAnnFmt::Labelme => {
                AnnSet::parse_labelme(self.path.as_path())
                    .expect("failed to parse")
            },

            SrcAnnFmt::PascalVoc => {
                AnnSet::parse_pascal_voc(self.path.as_path())
                    .expect("failed to parse")
            },

            SrcAnnFmt::Imagenet => {
                AnnSet::parse_imagenet(self.path.as_path())
                    .expect("failed to parse")
            }

            SrcAnnFmt::Yolo => {
                let imgs_path = self.imgs_path
                    .expect("parsing YOLO requires `imgs_path`");

                AnnSet::parse_yolo(
                    self.path.as_path(), 
                    imgs_path.as_path(), 
                    self.conf_last, 
                    &self.img_ext,
                ).expect("failed to parse")
            }
        };

        println!("Parsed {} annotations in {:#?}.", annset.items.len(), time.elapsed());
    }
}