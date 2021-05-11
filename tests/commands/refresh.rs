use crate::util::*;
use predicates::prelude::*;
use std::fs;

#[test]
fn on_not_initialized_should_report_error() -> Result<(), Box<dyn std::error::Error>> {
    assert_fails_if_not_initialized(&["refresh"])
}

#[test]
fn on_correct_repo_should_refresh_cookie() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;

    cmd.arg("refresh");
    cmd.assert()
        .stdout(predicate::str::contains("New session obtained"));
    dir.close()?;
    Ok(())
}

#[test]
fn on_corrupted_repo_should_report_error() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;
    fs::remove_file(dir.baca_config_file_path())?;

    cmd.arg("refresh");
    cmd.assert().stdout(predicate::str::contains("corrupted"));
    dir.close()?;
    Ok(())
}
