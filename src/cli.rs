use crate::annotationset::AnnSet;

use std::path::PathBuf;

use clap::{Parser, Args, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long)]
    #[arg(help = "Make the operation more talkative")]
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
    Coco, Cvat, Imagenet, Labelme, OpenImage, Txt, Yolo, PascalVoc
}

#[derive(Args)]
struct Parse {
    format: AnnFormat, 
    path: PathBuf, 

    #[arg(long)]
    imgs_path: Option<PathBuf>,
}

pub fn run() {
    let args = Cli::parse();

    match args.command {
        Commands::Parse(Parse{ path, format, imgs_path }) => {
            let annset = match format {
                AnnFormat::Coco => {
                    AnnSet::parse_coco(path.as_path())
                        .expect("invalid file")
                }, 
                
                AnnFormat::Cvat => {
                    AnnSet::parse_cvat(path.as_path())
                        .expect("invalid file")
                },

                AnnFormat::OpenImage => {
                    let imgs_path = imgs_path
                        .expect("openimage requires `img_path`");

                    AnnSet::parse_openimage(path.as_path(), imgs_path.as_path())
                        .expect("invalid file")
                }

                _ => {
                    panic!("Annotation format not yet supported")
                }
            };

            println!("Parsed {} annotations.", annset.items.len());
        }
    }
}