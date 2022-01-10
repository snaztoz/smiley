use clap::Parser;
use smiley::Preprocessor;
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
    let mut preprocessor = Preprocessor::default();

    preprocessor.set_src_file(&cli.src);

    if cli.watch {
        preprocessor.set_to_watch_mode();
    }

    if let Some(out) = cli.out {
        preprocessor.set_out_file(&out);
    } else {
        preprocessor.set_out_file(&cli.src);
    }

    preprocessor.run();
}
