use crate::util::{
    assert_fails_if_not_initialized, initialize_correct_workspace, set_up_command,
    BacaDirectoryPaths,
};
use predicates::prelude::*;
use std::fs;

#[test]
fn tasks_not_initialized() -> Result<(), Box<dyn std::error::Error>> {
    assert_fails_if_not_initialized(&["last"])
}

#[test]
fn on_correct_repo_should_print_last_submit() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;

    cmd.arg("last");
    cmd.assert()
        .stdout(predicate::str::contains("[G] Funkcje sklejane"))
        .stdout(predicate::str::contains("C++"))
        .stdout(predicate::str::contains("2020-05-17 18:53:09"))
        .stdout(predicate::str::contains("4334"))
        .stdout(predicate::str::contains("100%"))
        .stdout(predicate::str::contains("4/4"))
        .stdout(predicate::str::contains("Ok"))
        .stdout(predicate::str::contains("test0/0"))
        .stdout(predicate::str::contains("test1/0"))
        .stdout(predicate::str::contains("test2/0"))
        .stdout(predicate::str::contains("test3/0"));
    dir.close()?;
    Ok(())
}

#[test]
fn on_corrupted_repo_should_report_error() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;
    fs::remove_file(dir.baca_config_file_path())?;

    cmd.arg("last");
    cmd.assert().stdout(predicate::str::contains("corrupted"));
    dir.close()?;
    Ok(())
}
