use assert_cmd::prelude::*;
use assert_fs::fixture::{FileWriteStr, NamedTempFile};
use predicates::prelude::*;
use std::process::Command;

#[test]
fn run_with_src_file() {
    let file = NamedTempFile::new("srcfile.smly").unwrap();
    file.write_str("").unwrap();

    let mut cmd = Command::cargo_bin("smiley").unwrap();

    cmd.arg(file.path());
    cmd.assert().success();
}

#[test]
fn run_without_src_file() {
    let mut cmd = Command::cargo_bin("smiley").unwrap();

    cmd.assert().failure();
}

#[test]
fn run_with_non_existing_src_file() {
    let filename = "a-non-existing-src-file";

    let mut cmd = Command::cargo_bin("smiley").unwrap();

    cmd.arg(filename);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(format!(
            "File does not exist: `{}`",
            filename
        )));
}

#[test]
fn run_with_invalid_src_file_extension() {
    let file = NamedTempFile::new("srcfile.txt").unwrap();
    file.write_str("").unwrap();

    let mut cmd = Command::cargo_bin("smiley").unwrap();

    cmd.arg(file.path());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid extension"));
}
