use crate::preprocessor::line::{Col, Row};
use indoc::indoc;
use log::error;
use std::{fs, path::Path};

pub fn report(file: &Path, row: Row, col: Col, message: &str) {
    let content = fs::read_to_string(file).unwrap();
    let line = content.lines().nth(row - 1).unwrap();

    let location = format!("{}:{}:{}", file.display(), row, col);
    let err_report = format!(
        "\
    {}
    |
    |   `{}`
    |
    ",
        location,
        line.escape_default()
    );

    error!("{}\n\t{}", message, err_report);
}

pub fn report_indentation_error(file: &Path, row: Row, col: Col) {
    report(
        file,
        row,
        col,
        indoc! {"
            Inconsistent indentation: Smiley files should only use either
            \tspace or tab as indentation character, but never both
        "},
    );
}
