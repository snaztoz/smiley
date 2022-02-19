use crate::error::Error;
use itertools::Itertools;
use line::{builder::Builder as LineBuilder, Line, NumberedLine};
use log::{debug, info};
use std::{fs, path::PathBuf};

pub mod builder;
pub mod line;

#[derive(Default)]
pub struct Preprocessor {
    src: Option<PathBuf>,
    out: Option<PathBuf>,
    is_watch_mode: bool,
}

impl Preprocessor {
    pub fn run(&mut self) -> Result<(), Error> {
        assert!(self.src.is_some(), "src file is not setted properly");
        assert!(self.out.is_some(), "out file is not setted properly");

        info!("Running the preprocessor");

        let lines = self.read_src_file_lines()?;

        for (line, next) in lines.iter().tuple_windows() {
            let (_, line) = line;
            let (_, next) = next;
            let _kind = line::determine_kind(line, next);
        }

        Ok(())
    }

    fn read_src_file_lines(&self) -> Result<Vec<NumberedLine>, Error> {
        debug!("Reading src file content");

        let file_path = self.src.as_ref().unwrap();
        let file_content = fs::read_to_string(file_path).unwrap();

        let mut line_builder = LineBuilder::default();
        let mut lines = vec![];

        for raw_line in file_content.lines() {
            if let Some(line) = line_builder.build_line_from(raw_line)? {
                lines.push(line);
            }
        }

        lines.push((0, Line::eof()));

        Ok(lines)
    }
}
