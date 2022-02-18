use itertools::Itertools;
use line::{builder::Builder as LineBuilder, Line, NumberedLine};
use log::{debug, info};
use std::{fs, path::PathBuf, process};

pub mod builder;
pub mod line;

#[derive(Default)]
pub struct Preprocessor {
    src: Option<PathBuf>,
    out: Option<PathBuf>,
    is_watch_mode: bool,
}

impl Preprocessor {
    pub fn run(&mut self) {
        assert!(self.src.is_some(), "src file is not setted properly");
        assert!(self.out.is_some(), "out file is not setted properly");

        info!("Running the preprocessor");

        let lines = self.read_src_file_lines();

        for (line, next) in lines.iter().tuple_windows() {
            let (_, line) = line;
            let (_, next) = next;
            let _kind = line::determine_kind(line, next);
        }
    }

    fn read_src_file_lines(&self) -> Vec<NumberedLine> {
        debug!("Reading src file content");

        let file_path = self.src.as_ref().unwrap();
        let file_content = fs::read_to_string(file_path).unwrap();

        let mut line_builder = LineBuilder::default();

        file_content
            .lines()
            .map(|raw_line| {
                line_builder
                    .build_line_from(raw_line)
                    .unwrap_or_else(|err| {
                        let src = self.src.as_deref().unwrap();
                        err.report_file(src);
                        process::exit(1);
                    })
            })
            .flatten()
            .chain([(0, Line::eof())])
            .collect::<Vec<_>>()
    }
}
