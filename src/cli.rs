use crate::annotationset::AnnSet;

use std::path::PathBuf;

use clap::{Parser, Args, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long, help = "Make the operation more talkative")]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse and display basic statistics
    Parse(Parse),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum AnnFormat {
    Coco, Cvat, Imagenet, Labelme, OpenImage, Yolo, PascalVoc
}

#[derive(Args)]
struct Parse {
    #[arg(help = "the format of the annotations")]
    format: AnnFormat,

    #[arg(help = "the file or directory path of the annotations")]
    path: PathBuf, 

    #[arg(long, help = "the image directory for the annotations")]
    imgs_path: Option<PathBuf>,

    #[arg(long, default_value = "jpg", help = "the image extension (YOLO only)")]
    img_ext: String,

    #[arg(long, help = "whether the confidence is stored last or not (YOLO only)")]
    conf_last: bool,
}

pub fn run() {
    let args = Cli::parse();

    match args.command {
        Commands::Parse(parse) => {
            let Parse { 
                path, 
                format, 
                imgs_path, 
                img_ext, 
                conf_last,
            } = parse;

            let annset = match format {
                AnnFormat::Coco => {
                    AnnSet::parse_coco(path.as_path())
                        .expect("failed to parse")
                }, 
                
                AnnFormat::Cvat => {
                    AnnSet::parse_cvat(path.as_path())
                        .expect("failed to parse")
                },

                AnnFormat::OpenImage => {
                    let imgs_path = imgs_path
                        .expect("parsing OpenImage requires `imgs_path`");

                    AnnSet::parse_openimage(path.as_path(), imgs_path.as_path())
                        .expect("failed to parse")
                },

                AnnFormat::Labelme => {
                    AnnSet::parse_labelme(path.as_path())
                        .expect("failed to parse")
                },

                AnnFormat::PascalVoc => {
                    AnnSet::parse_pascal_voc(path.as_path())
                        .expect("failed to parse")
                },

                AnnFormat::Imagenet => {
                    AnnSet::parse_imagenet(path.as_path())
                        .expect("failed to parse")
                }

                AnnFormat::Yolo => {
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

            println!("Parsed {} annotations.", annset.items.len());
        }
    }
}