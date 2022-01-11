extern crate clap_verbosity_flag;

use clap::StructOpt;
use clap_verbosity_flag::Verbosity;
use env_logger::Builder as LoggerBuilder;
use log::LevelFilter;
use smiley::PreprocessorBuilder;
use std::path::PathBuf;

/// A (yet-another) simple CSS preprocessor
#[derive(Debug, StructOpt)]
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

    #[structopt(flatten)]
    verbose: Verbosity,
}

fn main() {
    let cli = Cli::parse();

    LoggerBuilder::new()
        .format_target(false)
        .format_timestamp(None)
        .filter_level(
            cli.verbose
                .log_level()
                .map_or(LevelFilter::Off, |level| level.to_level_filter()),
        )
        .init();

    PreprocessorBuilder::default()
        .with_src_file(&cli.src)
        .with_out_file(cli.out.as_deref())
        .in_watch_mode(cli.watch)
        .build()
        .run();
}
