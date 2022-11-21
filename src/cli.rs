mod parse;
mod convert;
mod eval;

use parse::Parse;
use convert::Convert;
use eval::Eval;

use clap::{Parser, Subcommand, ValueEnum};

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
enum DstAnnFmt {
    Coco, Cvat, Imagenet, Labelme, OpenImage, Yolo, PascalVoc, Vit
}

pub fn run() {
    let args = Cli::parse();

    match args.command {
        Commands::Parse(parse) => parse.run(),
        Commands::Convert(convert) => convert.run(),
        Commands::Eval(eval) => eval.run(),
    }
}