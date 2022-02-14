use crate::preprocessor::line::{error::ErrorKind as LineErrorKind, Col, Row};
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

pub fn report_line_building_error(file: &Path, kind: LineErrorKind, row: Row, col: Col) {
    let msg = match kind {
        LineErrorKind::InconsistentIndentation => indoc! {"
            Inconsistent indentation

            Smiley files should only use either space or tab as
            indentation character, but never both
        "},
        LineErrorKind::UnexpectedIndentation => indoc! {"
            Unexpected indentation
        "},

        _ => panic!("unexpected error kind"),
    };

    report(file, row, col, msg);
}
