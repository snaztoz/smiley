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
            "file does not exists: {}",
            filename
        )));
}
