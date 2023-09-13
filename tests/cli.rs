use anyhow::Result;
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn invalid_argument() -> Result<()> {
    let mut cmd = Command::cargo_bin("rust-dirname")?;

    cmd.arg("-a");
    cmd.assert().failure().stderr(predicate::str::contains(
        "error: unexpected argument '-a' found",
    ));
    Ok(())
}

#[test]
fn help_argument() -> Result<()> {
    let mut cmd = Command::cargo_bin("rust-dirname")?;

    cmd.arg("--help");
    let contains_predicate =
        predicate::str::contains("  -z, --zero     end each output line with NUL, not newline")
            .and(predicate::str::contains("  -h, --help     Print help"))
            .and(predicate::str::contains("  -V, --version  Print version"));
    cmd.assert().success().stdout(contains_predicate);
    Ok(())
}

#[test]
fn no_argument1() -> Result<()> {
    let mut cmd = Command::cargo_bin("rust-dirname")?;

    cmd.arg("a/b/c/d");
    let contains_predicate = predicate::str::contains("a/b/c");
    cmd.assert().success().stdout(contains_predicate);
    Ok(())
}

#[test]
fn no_argument2() -> Result<()> {
    let mut cmd = Command::cargo_bin("rust-dirname")?;

    cmd.arg("hoge");
    let contains_predicate = predicate::str::contains(".");
    cmd.assert().success().stdout(contains_predicate);
    Ok(())
}

#[test]
fn no_argument3() -> Result<()> {
    let mut cmd = Command::cargo_bin("rust-dirname")?;

    cmd.arg("hoge").arg("fizz/buzz");
    let contains_predicate = predicate::str::contains(".").and(predicate::str::contains("fizz"));
    cmd.assert().success().stdout(contains_predicate);
    Ok(())
}

#[test]
fn short_zero_argument1() -> Result<()> {
    let mut cmd = Command::cargo_bin("rust-dirname")?;

    cmd.arg("-z").arg("hoge").arg("fizz/buzz");
    let contains_predicate = predicate::str::contains(".fizz");
    cmd.assert().success().stdout(contains_predicate);
    Ok(())
}

#[test]
fn long_zero_argument1() -> Result<()> {
    let mut cmd = Command::cargo_bin("rust-dirname")?;

    cmd.arg("--zero").arg("hoge").arg("fizz/buzz");
    let contains_predicate = predicate::str::contains(".fizz");
    cmd.assert().success().stdout(contains_predicate);
    Ok(())
}
