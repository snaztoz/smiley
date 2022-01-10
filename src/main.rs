use clap::Parser;
use std::path::PathBuf;

/// A (yet-another) simple CSS preprocessor
#[derive(Parser, Debug)]
#[clap(about, version)]
struct Cli {
    /// Root source file
    src: PathBuf,

    /// Run smiley in `watch` mode
    #[clap(short, long)]
    watch: bool,

    /// Specify the output file path
    #[clap(short, long)]
    out: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    println!("{:?}", cli);
}
