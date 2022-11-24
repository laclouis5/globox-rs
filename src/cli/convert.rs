use crate::{annotationset::AnnSet, path::expand_user};

use super::{SrcAnnFmt, DstAnnFmt};

use clap::Args;

#[derive(Args)]
pub(super) struct Convert {
    #[arg(help = "The format of the input annotations")]
    src_fmt: SrcAnnFmt,

    #[arg(help = "The file or directory path of the input annotations")]
    src_path: String,

    #[arg(help = "The format of the output annotations")]
    dst_fmt: DstAnnFmt,

    #[arg(help = "The file or directory path of the output annotations")]
    dst_path: String,

    #[arg(long, help = "The image directory of the input annotations")]
    imgs_path: Option<String>,

    #[arg(long, default_value = "jpg", help = "The image extension of input annotation images (YOLO only)")]
    src_img_ext: String,

    #[arg(long, help = "Whether or not the confidence of input annotations is stored in last position (YOLO only)")]
    src_conf_last: bool,

    #[arg(long, help = "Whether or not the confidence of output annotations should be stored in last position (YOLO only)")]
    dst_conf_last: bool,
}

impl Convert {
    pub(super) fn run(self) {
        let path = expand_user(self.src_path);

        let anns = match self.src_fmt {
            SrcAnnFmt::Coco => AnnSet::parse_coco(path),
            SrcAnnFmt::Cvat => AnnSet::parse_cvat(path),
            SrcAnnFmt::Imagenet => AnnSet::parse_imagenet(path),
            SrcAnnFmt::Labelme => AnnSet::parse_labelme(path),
            SrcAnnFmt::OpenImage => {
                let imgs_path = self.imgs_path
                    .expect("parsing OpenImage requires `imgs_path`");
                let imgs_path = expand_user(imgs_path);

                AnnSet::parse_openimage(path, imgs_path)
            },
            SrcAnnFmt::Yolo => {
                let imgs_path = self.imgs_path
                    .expect("parsing OpenImage requires `imgs_path`");
                let imgs_path = expand_user(imgs_path);

                AnnSet::parse_yolo(path, imgs_path, self.src_conf_last, &self.src_img_ext)
            },
            SrcAnnFmt::PascalVoc => {
                AnnSet::parse_pascal_voc(path)
            },
        }
        .expect("failed to parse the annotations");

        let save_path = expand_user(self.dst_path);

        match self.dst_fmt {
            DstAnnFmt::Coco => anns.save_coco(save_path),
            DstAnnFmt::Cvat => anns.save_cvat(save_path),
            DstAnnFmt::Imagenet => anns.save_imagenet(save_path),
            DstAnnFmt::Labelme => anns.save_labelme(save_path),
            DstAnnFmt::OpenImage => anns.save_openimage(save_path),
            DstAnnFmt::Yolo => anns.save_yolo(save_path, self.dst_conf_last),
            DstAnnFmt::PascalVoc => anns.save_pascal_voc(save_path),
            DstAnnFmt::Vit => todo!("not yet implemented"),
        }
        .expect("failed to save the annotations");
    }
}