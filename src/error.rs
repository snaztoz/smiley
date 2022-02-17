use crate::preprocessor::line::{
    error::{Error as LineError, ErrorKind as LineErrorKind},
    position::Position,
};
use indoc::{formatdoc, indoc};
use log::error;
use std::{fs, path::Path};

pub fn report(file: &Path, pos: Position, message: &str) {
    let content = fs::read_to_string(file).unwrap();
    let line = content.lines().nth(pos.row - 1).unwrap();

    let location = format!("{}:{}:{}", file.display(), pos.row, pos.col);
    let escaped_line = line.escape_default();

    let err_report = formatdoc! {"
        {location}
        |
        |   `{escaped_line}`
        |
    "};

    error!("{message}\n{err_report}");
}

pub fn report_line_building_error(file: &Path, err: LineError) {
    let msg = match err.kind {
        LineErrorKind::InconsistentIndentation => indoc! {"
            Inconsistent indentation

            Smiley files should only use either space or tab as
            indentation character, but never both
        "},
        LineErrorKind::UnexpectedIndentation => indoc! {"
            Unexpected indentation
        "},
    };

    report(file, err.pos, msg);
}
