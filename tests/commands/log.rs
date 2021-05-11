use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn no_verbose_should_not_enable_logs() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("baca")?;

    cmd.arg("tasks");
    cmd.assert()
        .stdout(predicate::str::contains("INFO").not())
        .stdout(predicate::str::contains("DEBUG").not())
        .stdout(predicate::str::contains("TRACE").not())
        .stdout(predicate::str::contains("ERROR").not());

    Ok(())
}

#[test]
fn one_verbose_should_enable_info() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("baca")?;

    cmd.arg("-v").arg("tasks");
    cmd.assert()
        .stdout(predicate::str::contains("INFO"))
        .stdout(predicate::str::contains("DEBUG").not())
        .stdout(predicate::str::contains("TRACE").not());

    Ok(())
}

#[test]
fn two_verbose_should_enable_debug() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("baca")?;

    cmd.arg("-vv").arg("tasks");
    cmd.assert()
        .stdout(predicate::str::contains("INFO"))
        .stdout(predicate::str::contains("DEBUG"))
        .stdout(predicate::str::contains("TRACE").not());

    Ok(())
}

#[test]
fn three_verbose_should_enable_trace() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("baca")?;

    cmd.arg("-vvv").arg("tasks");
    cmd.assert()
        .stdout(predicate::str::contains("INFO"))
        .stdout(predicate::str::contains("DEBUG"))
        .stdout(predicate::str::contains("TRACE"));

    Ok(())
}
