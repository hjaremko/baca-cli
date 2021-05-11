use crate::util::*;
use predicates::prelude::predicate;
use std::fs;

#[test]
fn tasks_not_initialized() -> Result<(), Box<dyn std::error::Error>> {
    assert_fails_if_not_initialized(&["tasks"])
}

#[test]
fn on_correct_repo_should_print_tasks() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;

    cmd.arg("tasks");
    cmd.assert()
        .stdout(predicate::str::contains("[A] Zera funkcji"))
        .stdout(predicate::str::contains("[B] Metoda Newtona"))
        .stdout(predicate::str::contains(
            r#"[C] FAD\x3Csup\x3E2\x3C/sup\x3E - Pochodne mieszane"#,
        ))
        .stdout(predicate::str::contains("[D] Skalowany Gauss"))
        .stdout(predicate::str::contains("[E] Metoda SOR"))
        .stdout(predicate::str::contains("[F] Interpolacja"))
        .stdout(predicate::str::contains("[G] Funkcje sklejane"));
    dir.close()?;
    Ok(())
}

#[test]
fn on_corrupted_repo_should_report_error() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;
    fs::remove_file(dir.baca_config_file_path())?;

    cmd.arg("tasks");
    cmd.assert().stdout(predicate::str::contains("corrupted"));
    dir.close()?;
    Ok(())
}
