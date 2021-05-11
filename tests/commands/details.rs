use crate::util::*;
use predicates::prelude::predicate;
use std::fs;

#[test]
fn tasks_not_initialized() -> Result<(), Box<dyn std::error::Error>> {
    assert_fails_if_not_initialized(&["details", "123"])
}

#[test]
fn no_argument_should_report_error() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;

    cmd.arg("details");
    cmd.assert().stderr(predicate::str::contains(
        "required arguments were not provided",
    ));
    dir.close()?;
    Ok(())
}

#[test]
fn on_correct_argument_should_print_task() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;

    cmd.arg("details").arg("2796");
    cmd.assert()
        .stdout(predicate::str::contains("[D] Skalowany Gauss"))
        .stdout(predicate::str::contains("C++"))
        .stdout(predicate::str::contains("2020-04-20 15:39:42"))
        .stdout(predicate::str::contains("2796"))
        .stdout(predicate::str::contains("74"))
        .stdout(predicate::str::contains("2.95"))
        .stdout(predicate::str::contains("WrongAnswer"));
    dir.close()?;
    Ok(())
}

#[test]
fn on_corrupted_repo_should_report_error() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;
    fs::remove_file(dir.baca_config_file_path())?;

    cmd.arg("details").arg("123");
    cmd.assert().stdout(predicate::str::contains("corrupted"));
    dir.close()?;
    Ok(())
}
