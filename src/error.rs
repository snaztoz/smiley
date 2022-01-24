use crate::preprocessor::{
    indentation::ErrorKind as IndentationErrorKind,
    line::{Col, Row},
};
use indoc::{formatdoc, indoc};
use log::error;
use std::{fs, path::Path};

pub fn report(file: &Path, row: Row, col: Col, message: &str) {
    let content = fs::read_to_string(file).unwrap();
    let line = content.lines().nth(row - 1).unwrap();

    let location = format!("{}:{}:{}", file.display(), row, col);
    let escaped_line = line.escape_default();

    let err_report = formatdoc! {"
        {location}
        |
        |   `{escaped_line}`
        |
    "};

    error!("{message}\n{err_report}");
}

pub fn report_indentation_error(file: &Path, kind: IndentationErrorKind, row: Row, col: Col) {
    let msg = match kind {
        IndentationErrorKind::InconsistentIndentation => indoc! {"
            Inconsistent indentation

            Smiley files should only use either space or tab as
            indentation character, but never both
        "},
        IndentationErrorKind::UnexpectedIndentation => indoc! {"
            Unexpected indentation
        "},
    };

    report(file, row, col, msg);
}
