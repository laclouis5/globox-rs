use super::{SrcAnnFmt, DstAnnFmt};

use std::path::PathBuf;

use clap::Args;

#[derive(Args)]
pub(super) struct Convert {
    #[arg(help = "The format of the input annotations")]
    src: SrcAnnFmt,

    #[arg(help = "The file or directory path of the input annotations")]
    src_path: PathBuf,

    #[arg(help = "The format of the output annotations")]
    dst: DstAnnFmt,

    #[arg(help = "The file or directory path of the output annotations")]
    dst_path: PathBuf,

    #[arg(long, help = "The image directory of the input annotations")]
    imgs_path: Option<PathBuf>,

    #[arg(long, default_value = "jpg", help = "The image extension of input annotation images (YOLO only)")]
    src_img_ext: String,

    #[arg(long, help = "Whether or not the confidence of input annotations is stored in last position (YOLO only)")]
    src_conf_last: bool,

    #[arg(long, default_value = "jpg", help = "The image extension of output annotation images (YOLO only)")]
    dst_img_ext: String,

    #[arg(long, help = "Whether or not the confidence of output annotations should be stored in last position (YOLO only)")]
    dst_conf_last: bool,
}

impl Convert {
    pub(super) fn run(self) {
        todo!("not implemented yet")
    }
}