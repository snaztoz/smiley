extern crate clap_verbosity_flag;

use clap::StructOpt;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use env_logger::Builder as LoggerBuilder;
use indoc::formatdoc;
use log::{error, info};
use smiley::{error::Error, PreprocessorBuilder};
use std::{
    fs,
    path::{Path, PathBuf},
    process,
    time::Duration,
};

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
    verbose: Verbosity<InfoLevel>,
}

fn main() {
    let cli = Cli::parse();

    LoggerBuilder::new()
        .format_target(false)
        .format_timestamp(None)
        .filter_level(cli.verbose.log_level_filter())
        .init();

    let res = PreprocessorBuilder::default()
        .set_src_file(&cli.src)
        .set_out_file(cli.out.as_deref())
        .build()
        .run();

    match res {
        Ok(duration) => log_compilation_success(duration),

        Err(err) => {
            log_compilation_error(&cli.src, err);
            process::exit(1);
        }
    }
}

fn log_compilation_success(duration: Duration) {
    let duration = duration.as_secs_f32();
    info!("Successfully compiled in {duration}s");
}

fn log_compilation_error(file: &Path, error: Error) {
    let message = error.kind.get_message();
    let content = fs::read_to_string(file).unwrap();
    let line = content
        .lines()
        .nth(error.pos.row - 1)
        .unwrap()
        .escape_default();

    let location = format!("{}:{}:{}", file.display(), error.pos.row, error.pos.col);

    let err_report = formatdoc! {"
       --> {location}
        |
        |   `{line}`
        |
    "};

    error!("{message}\n{err_report}");
}
