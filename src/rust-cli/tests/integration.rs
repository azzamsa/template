use std::process::Command;

use anyhow::Result;
use assert_cmd::{crate_name, prelude::*};
use predicates::prelude::*;

#[test]
fn help() -> Result<()> {
    let mut cmd = Command::cargo_bin(crate_name!()).unwrap();
    cmd.arg("-h");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Rust CLI Template"));

    Ok(())
}

#[test]
fn file_not_found() -> Result<()> {
    let mut cmd = Command::cargo_bin(crate_name!()).unwrap();
    cmd.arg("file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file"));

    Ok(())
}

#[test]
fn read_file() -> Result<()> {
    let mut cmd = Command::cargo_bin(crate_name!()).unwrap();
    cmd.arg("tests/fixtures/test.txt");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello!"));

    Ok(())
}
