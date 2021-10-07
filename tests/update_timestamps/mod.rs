use crate::util::get_baca_credentials;
use assert_cmd::Command;
use assert_fs::TempDir;
use predicates::prelude::*;

#[test]
fn update_check_timestamp_should_be_saved_if_no_update() -> Result<(), Box<dyn std::error::Error>> {
    let (login, pass, host) = get_baca_credentials();
    let temp = assert_fs::TempDir::new()?;

    let mut cmd = baca_verbose(&temp)?;
    cmd.arg("init")
        .args(&["--host", &host, "-p", &pass, "-l", &login]);
    cmd.assert()
        .stdout(predicate::str::contains("Checking for updates"))
        .success();

    let mut cmd = baca_verbose(&temp)?;
    cmd.assert()
        .stdout(predicate::str::contains("Checking for updates"))
        .success();

    let mut cmd = baca_verbose(&temp)?;
    cmd.assert()
        .stdout(predicate::str::contains("Checking for updates").not())
        .success();

    temp.close()?;
    Ok(())
}

#[test]
fn update_check_timestamp_should_not_be_saved_if_update() -> Result<(), Box<dyn std::error::Error>>
{
    let (login, pass, host) = get_baca_credentials();
    let temp = assert_fs::TempDir::new()?;

    let mut cmd = baca_verbose_dummy_repo(&temp)?;
    cmd.arg("init")
        .args(&["--host", &host, "-p", &pass, "-l", &login]);
    cmd.assert()
        .stdout(predicate::str::contains("New version"))
        .success();

    let mut cmd = baca_verbose_dummy_repo(&temp)?;
    cmd.assert()
        .stdout(predicate::str::contains("New version"))
        .success();

    let mut cmd = baca_verbose_dummy_repo(&temp)?;
    cmd.assert()
        .stdout(predicate::str::contains("New version"))
        .success();

    temp.close()?;
    Ok(())
}

#[test]
fn update_check_error_if_invalid_repo() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new()?;
    let mut cmd = baca_verbose(&temp)?;
    cmd.env("GITHUB_REPO", "does_not_exists");
    cmd.assert()
        .stdout(predicate::str::contains("Error checking for updates"))
        .success();

    temp.close()?;
    Ok(())
}

#[test]
fn forced_update_check_should_be_triggered_despite_timestamp(
) -> Result<(), Box<dyn std::error::Error>> {
    let (login, pass, host) = get_baca_credentials();
    let temp = assert_fs::TempDir::new()?;

    // Save timestamp
    let mut cmd = baca_verbose(&temp)?;
    cmd.arg("init")
        .args(&["--host", &host, "-p", &pass, "-l", &login]);
    cmd.assert()
        .stdout(predicate::str::contains("Checking for updates"))
        .success();

    let mut cmd = baca_verbose(&temp)?;
    cmd.assert()
        .stdout(predicate::str::contains("Checking for updates"))
        .success();

    let mut cmd = baca_verbose(&temp)?;
    cmd.assert()
        .stdout(predicate::str::contains("Checking for updates").not())
        .success();

    // Force update
    let mut cmd = baca_verbose(&temp)?;
    cmd.arg("-U");
    cmd.assert()
        .stdout(predicate::str::contains("Checking for updates"))
        .success();

    // Should not check again
    let mut cmd = baca_verbose(&temp)?;
    cmd.assert()
        .stdout(predicate::str::contains("Checking for updates").not())
        .success();

    temp.close()?;
    Ok(())
}

fn baca_verbose(temp: &TempDir) -> Result<Command, Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("baca")?;
    cmd.current_dir(&temp);
    cmd.arg("-v");
    Ok(cmd)
}

fn baca_verbose_dummy_repo(temp: &TempDir) -> Result<Command, Box<dyn std::error::Error>> {
    let mut cmd = baca_verbose(temp)?;
    cmd.env("GITHUB_REPO", "dummy");
    Ok(cmd)
}
