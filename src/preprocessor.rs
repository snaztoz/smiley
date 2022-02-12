use crate::error;
use indentation::{ErrorKind as IndentationErrorKind, Validator as IndentationValidator};
use itertools::Itertools;
use line::{builder::Builder as LineBuilder, Line};
use log::{debug, info};
use std::{fs, path::PathBuf, process};

pub mod builder;
pub mod indentation;
pub mod line;

#[derive(Default)]
pub struct Preprocessor {
    src: Option<PathBuf>,
    out: Option<PathBuf>,
    is_watch_mode: bool,

    indent_validator: IndentationValidator,
}

impl Preprocessor {
    pub fn run(&mut self) {
        assert!(self.src.is_some(), "src file is not setted properly");
        assert!(self.out.is_some(), "out file is not setted properly");

        info!("Running the preprocessor");

        let lines = self.read_src_file_lines();

        for (line, _) in lines.iter().tuple_windows::<(&Line, &Line)>() {
            self.validate_indentation(line);
        }
    }

    fn read_src_file_lines(&self) -> Vec<Line> {
        debug!("Reading src file content");

        let file_path = self.src.as_ref().unwrap();
        let file_content = fs::read_to_string(file_path).unwrap();

        let mut line_builder = LineBuilder::default();

        file_content
            .lines()
            .map(|raw_line| {
                line_builder
                    .build_line_from(raw_line)
                    .unwrap_or_else(|(row, col)| {
                        let src = self.src.as_deref().unwrap();
                        error::report_indentation_error(
                            src,
                            IndentationErrorKind::InconsistentIndentation,
                            row,
                            col,
                        );
                        process::exit(1);
                    })
            })
            .flatten()
            .chain([Line::eof()])
            .collect::<Vec<_>>()
    }

    fn validate_indentation(&mut self, line: &Line) {
        if let Err(err) = self.indent_validator.validate(line) {
            let src = self.src.as_deref().unwrap();
            let (kind, (row, col)) = err;

            error::report_indentation_error(src, kind, row, col);
            process::exit(1);
        }
    }
}
