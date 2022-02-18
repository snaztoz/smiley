use crate::preprocessor::line::position::Position;
use indoc::{formatdoc, indoc};
use log::error;
use std::{fs, path::Path};

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub pos: Position,
}

impl Error {
    pub fn report_file(&self, file: &Path) {
        let content = fs::read_to_string(file).unwrap();
        let message = self.kind.get_message();
        let escaped_line = content
            .lines()
            .nth(self.pos.row - 1)
            .unwrap()
            .escape_default();

        let location = format!("{}:{}:{}", file.display(), self.pos.row, self.pos.col);
        let err_report = formatdoc! {"
           --> {location}
            |
            |   `{escaped_line}`
            |
        "};

        error!("{message}\n{err_report}");
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    InconsistentIndentation,
    UnexpectedIndentation,
}

impl ErrorKind {
    fn get_message(&self) -> String {
        let msg = match self {
            ErrorKind::InconsistentIndentation => indoc! {"
                Inconsistent indentation

                Smiley files should only use either space or tab as
                indentation character, but never both
            "},
            ErrorKind::UnexpectedIndentation => indoc! {"
                Unexpected indentation
            "},
        };

        String::from(msg)
    }
}
