use log::error;
use std::{fs, path::Path};

pub fn report(file: &Path, row: usize, col: usize, message: &str) {
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
        location, line
    );

    error!("{}\n\n\t{}", message, err_report);
}
