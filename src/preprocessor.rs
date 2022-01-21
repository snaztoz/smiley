use crate::{error, util};
use indentation::Indentation;
use itertools::Itertools;
use line::{Content as LineContent, Line};
use log::{debug, info};
use std::{path::PathBuf, process};

pub mod builder;
mod indentation;
mod line;

#[derive(Default)]
pub struct Preprocessor {
    src: Option<PathBuf>,
    out: Option<PathBuf>,
    is_watch_mode: bool,

    indent_type: Option<Indentation>,
    current_row: usize,
}

impl Preprocessor {
    pub fn run(&mut self) {
        assert!(self.src.is_some(), "src file is not setted properly");
        assert!(self.out.is_some(), "out file is not setted properly");

        info!("Running the preprocessor");

        let lines = self.read_src_file_lines();

        for (line, _) in lines.iter().tuple_windows::<(&Line, &Line)>() {
            self.current_row = line.row;

            if let Some((indent, _)) = line.indentation_mode {
                self.handle_indentation_type(indent);
            }
        }
    }

    fn read_src_file_lines(&self) -> Vec<Line> {
        debug!("Reading src file content");

        let file_content = util::read_file_with_empty_line_appended(self.src.as_ref().unwrap());

        let mut lines = file_content
            .lines()
            .enumerate()
            .filter(|(_, line)| !line.trim().is_empty())
            .map(|(i, line)| match Indentation::check_mode(line) {
                Ok(mode) => Line {
                    row: i + 1,
                    content: LineContent::Value(line.to_string()),
                    indentation_mode: mode,
                },

                Err(col) => {
                    error::report(
                        self.src.as_deref().unwrap(),
                        i + 1,
                        col,
                        "Inconsistent indentation: Smiley src files should only use either \
                        space\n\tor tab as indentation character, but not both",
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

    fn handle_indentation_type(&mut self, indent: Indentation) {
        match self.indent_type {
            Some(_) => self.validate_indentation_mode(indent),
            None => self.set_indentation_type(indent),
        }
    }

    fn validate_indentation_mode(&mut self, indent: Indentation) {
        if indent != *self.indent_type.as_ref().unwrap() {
            error::report(
                self.src.as_deref().unwrap(),
                self.current_row,
                0,
                "Inconsistent indentation: Smiley src files should only use either \
                space\n\tor tab as indentation character, but not both",
            );

            process::exit(1);
        }
    }

    fn set_indentation_type(&mut self, indent: Indentation) {
        if indent == Indentation::Space {
            debug!("Setting indentation mode to `space`");
        } else {
            debug!("Setting indentation mode to `tab`");
        }

        self.indent_type = Some(indent);
    }
}
