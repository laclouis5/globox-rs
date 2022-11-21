use crate::annotationset::AnnSet;

use std::{
    path::{PathBuf},
    time::Instant,
};

use clap::{Parser, Args, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long, help = "Make the operations more talkative")]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse and display basic statistics
    Parse(Parse),

    /// Convert bounding box annotations between formats
    Convert(Convert),

    /// Evaluate bounding box annotations and predictions
    Eval(Eval),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum SrcAnnFmt {
    Coco, Cvat, Imagenet, Labelme, OpenImage, Yolo, PascalVoc
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum DestAnnFmt {
    Coco, Cvat, Imagenet, Labelme, OpenImage, Yolo, PascalVoc, Vit
}

#[derive(Args)]
struct Parse {
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

#[derive(Args)]
struct Convert {
    #[arg(help = "The format of the input annotations")]
    src: SrcAnnFmt,

    #[arg(help = "The file or directory path of the input annotations")]
    src_path: PathBuf,

    #[arg(help = "The format of the output annotations")]
    dst: DestAnnFmt,

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

#[derive(Args)]
struct Eval {
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

pub fn run() {
    let args = Cli::parse();

    match args.command {
        Commands::Parse(parse) => {
            let time = Instant::now();

            let Parse { 
                path, 
                format, 
                imgs_path, 
                img_ext, 
                conf_last,
            } = parse;

            let annset = match format {
                SrcAnnFmt::Coco => {
                    AnnSet::parse_coco(path.as_path())
                        .expect("failed to parse")
                }, 
                
                SrcAnnFmt::Cvat => {
                    AnnSet::parse_cvat(path.as_path())
                        .expect("failed to parse")
                },

                SrcAnnFmt::OpenImage => {
                    let imgs_path = imgs_path
                        .expect("parsing OpenImage requires `imgs_path`");

                    AnnSet::parse_openimage(path.as_path(), imgs_path.as_path())
                        .expect("failed to parse")
                },

                SrcAnnFmt::Labelme => {
                    AnnSet::parse_labelme(path.as_path())
                        .expect("failed to parse")
                },

                SrcAnnFmt::PascalVoc => {
                    AnnSet::parse_pascal_voc(path.as_path())
                        .expect("failed to parse")
                },

                SrcAnnFmt::Imagenet => {
                    AnnSet::parse_imagenet(path.as_path())
                        .expect("failed to parse")
                }

                SrcAnnFmt::Yolo => {
                    let imgs_path = imgs_path
                        .expect("parsing YOLO requires `imgs_path`");

                    AnnSet::parse_yolo(
                        path.as_path(), 
                        imgs_path.as_path(), 
                        conf_last, 
                        &img_ext,
                    )
                    .expect("failed to parse")
                }
            };

            println!("Parsed {} annotations in {:#?}.", annset.items.len(), time.elapsed());
        },

        Commands::Convert(_) => {
            todo!("not yet implemented");
        },

        Commands::Eval(_) => {
            todo!("not yet implemented");
        }
    }
}