use crate::error;
use indentation::{Checker as IndentationChecker, ErrorKind as IndentationErrorKind, Indentation};
use itertools::Itertools;
use line::{Content as LineContent, Line};
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

    indent_checker: IndentationChecker,
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

        let mut lines = file_content
            .lines()
            .enumerate()
            .filter(|(_, line)| !line.trim().is_empty())
            .map(|(i, line)| match Indentation::check_mode(line) {
                Ok(mode) => Line {
                    row: i + 1,
                    // remove indentations, and put the information
                    // inside indentation_mode instead
                    content: LineContent::Value(line.trim().to_string()),
                    indentation_mode: mode,
                },

                Err(col) => {
                    let src = self.src.as_deref().unwrap();
                    error::report_indentation_error(
                        src,
                        IndentationErrorKind::InconsistentIndentation,
                        i + 1,
                        col,
                    );
                    process::exit(1);
                }
            })
            .collect::<Vec<_>>();

        // Append EOF
        lines.push(Line {
            row: usize::MAX, // doesn't really matter
            content: LineContent::Eof,
            indentation_mode: None,
        });

        lines
    }

    fn validate_indentation(&mut self, line: &Line) {
        if let Err(err) = self.indent_checker.validate(line) {
            let src = self.src.as_deref().unwrap();
            let (kind, (row, col)) = err;

            error::report_indentation_error(src, kind, row, col);
            process::exit(1);
        }
    }
}
