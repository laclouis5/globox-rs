use super::SrcAnnFmt; 

use std::path::PathBuf;

use clap::Args;

#[derive(Args)]
pub(super) struct Eval {
    #[arg(help = "The format of the ground truth annotations")]
    gts_fmt: SrcAnnFmt,

    #[arg(help = "The file or directory path of the ground truth annotations")]
    gts_path: PathBuf,

    #[arg(help = "The format of the predicted annotations")]
    dets_fmt: SrcAnnFmt,

    #[arg(help = "The file or directory path of the predicted annotations")]
    dets_path: PathBuf,

    #[arg(long, help = "The image directory of the annotations")]
    imgs_path: Option<PathBuf>,

    #[arg(long, default_value = "jpg", help = "The image extension of ground truth annotation images (YOLO only)")]
    gts_img_ext: String,

    #[arg(long, help = "Whether or not the confidence of ground truth annotations is stored in last position (YOLO only)")]
    gts_conf_last: bool,

    #[arg(long, default_value = "jpg", help = "The image extension of predicted annotation images (YOLO only)")]
    dets_img_ext: String,

    #[arg(long, help = "Whether or not the confidence of predicted annotations is stored in last position (YOLO only)")]
    dets_conf_last: bool,
}

impl Eval {
    pub(super) fn run(self) {
        todo!("not implemented yet")
    }
}