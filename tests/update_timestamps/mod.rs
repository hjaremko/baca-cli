use assert_cmd::Command;
use predicates::prelude::*;
use std::env;

#[test]
fn update_check_timestamp_should_be_saved() -> Result<(), Box<dyn std::error::Error>> {
    let pass = env::var("BACA_PASSWORD")?;
    let temp = assert_fs::TempDir::new()?;

    let mut cmd = Command::cargo_bin("baca")?;

    cmd.current_dir(&temp);
    cmd.arg("-v");
    cmd.arg("init")
        .args(&["--host", "mn2020", "-p", &pass, "-l", "jaremko"]);
    cmd.assert()
        .stdout(predicate::str::contains("Checking for updates"))
        .success();

    let mut cmd = Command::cargo_bin("baca")?;
    cmd.current_dir(&temp);
    cmd.arg("-v");
    cmd.arg("log");
    cmd.assert()
        .stdout(predicate::str::contains("Checking for updates"))
        .success();

    let mut cmd = Command::cargo_bin("baca")?;
    cmd.current_dir(&temp);
    cmd.arg("-v");
    cmd.arg("log");
    cmd.assert()
        .stdout(predicate::str::contains("Checking for updates").not())
        .success();

    temp.close()?;
    Ok(())
}
