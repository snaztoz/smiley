extern crate clap_verbosity_flag;

use clap::StructOpt;
use clap_verbosity_flag::Verbosity;
use env_logger::Builder as LoggerBuilder;
use log::LevelFilter;
use smiley::PreprocessorBuilder;
use std::{path::PathBuf, process};

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

    let res = PreprocessorBuilder::default()
        .set_src_file(&cli.src)
        .set_out_file(cli.out.as_deref())
        .set_watch_mode(cli.watch)
        .build()
        .run();

    if let Err(err) = res {
        err.report_file(&cli.src);
        process::exit(1);
    }
}
