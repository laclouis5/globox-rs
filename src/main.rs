use globox::{
    imgsize::ImgSize,
    annotation::Ann,
    annotationset::AnnSet, 
};
use quick_xml::de::from_str;

use std::{error::Error, path::PathBuf};

use clap::{self, Parser};

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(clap::Subcommand)]
enum Commands {
    Parse { 
        format: String, 
        path: PathBuf, 

        #[arg(long)]
        imgs_path: Option<PathBuf> 
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Parse { path, format, imgs_path } => {
            let annset = match format.as_str() {
                "coco" => {
                    AnnSet::parse_coco(path.as_path())
                        .expect("invalid file")
                }, 
                
                "cvat" => {
                    AnnSet::parse_cvat(path.as_path())
                        .expect("invalid file")
                },

                "openimage" => {
                    let imgs_path = imgs_path
                        .expect("openimage requires `img_path`");

                    AnnSet::parse_openimage(path.as_path(), imgs_path.as_path())
                        .expect("invalid file")
                }

                _ => {
                    panic!("invalid parse format")
                }
            };

            println!("Parsed {} annotations.", annset.items.len());
        }
    }
}