use crate::util::*;
use predicates::prelude::{predicate, PredicateBooleanExt};
use std::fs;

#[test]
fn tasks_not_initialized() -> Result<(), Box<dyn std::error::Error>> {
    assert_fails_if_not_initialized(&["log"])
}

#[test]
fn no_argument_should_print_last_three() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;

    cmd.arg("log");
    cmd.assert()
        .stdout(predicate::str::contains("[G] Funkcje sklejane"))
        .stdout(predicate::str::contains("[A] Zera funkcji").not())
        .stdout(predicate::str::contains("[B] Metoda Newtona").not())
        .stdout(
            predicate::str::contains(r#"[C] FAD\x3Csup\x3E2\x3C/sup\x3E - Pochodne mieszane"#)
                .not(),
        )
        .stdout(predicate::str::contains("[D] Skalowany Gauss").not())
        .stdout(predicate::str::contains("[E] Metoda SOR").not())
        .stdout(predicate::str::contains("4334"))
        .stdout(predicate::str::contains("4328"))
        .stdout(predicate::str::contains("4326"))
        .stdout(predicate::str::contains("4325").not());
    dir.close()?;
    Ok(())
}

#[test]
fn with_given_1_should_print_last_1() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;

    cmd.arg("log").arg("1");
    cmd.assert()
        .stdout(predicate::str::contains("[G] Funkcje sklejane"))
        .stdout(predicate::str::contains("[A] Zera funkcji").not())
        .stdout(predicate::str::contains("[B] Metoda Newtona").not())
        .stdout(
            predicate::str::contains(r#"[C] FAD\x3Csup\x3E2\x3C/sup\x3E - Pochodne mieszane"#)
                .not(),
        )
        .stdout(predicate::str::contains("[D] Skalowany Gauss").not())
        .stdout(predicate::str::contains("[E] Metoda SOR").not())
        .stdout(predicate::str::contains("4334"))
        .stdout(predicate::str::contains("4328").not())
        .stdout(predicate::str::contains("4326").not())
        .stdout(predicate::str::contains("4325").not());
    dir.close()?;
    Ok(())
}

#[test]
fn with_given_more_than_available_should_print_all() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;

    cmd.arg("log").arg("1000000");
    cmd.assert()
        .stdout(predicate::str::contains("[G] Funkcje sklejane"))
        .stdout(predicate::str::contains("[A] Zera funkcji"))
        .stdout(predicate::str::contains("[B] Metoda Newtona"))
        .stdout(predicate::str::contains(
            r#"[C] FAD\x3Csup\x3E2\x3C/sup\x3E - Pochodne mieszane"#,
        ))
        .stdout(predicate::str::contains("[D] Skalowany Gauss"))
        .stdout(predicate::str::contains("[E] Metoda SOR"))
        .stdout(predicate::str::contains("4334"))
        .stdout(predicate::str::contains("4328"))
        .stdout(predicate::str::contains("4326"))
        .stdout(predicate::str::contains("532"));
    dir.close()?;
    Ok(())
}

#[test]
fn with_invalid_argument_should_report_error() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;

    cmd.arg("log").arg("nan");
    cmd.assert()
        .stdout(predicate::str::contains("Invalid argument"));
    dir.close()?;
    Ok(())
}

#[test]
fn on_corrupted_repo_should_report_error() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;
    fs::remove_file(dir.baca_config_file_path())?;

    cmd.arg("log");
    cmd.assert().stdout(predicate::str::contains("corrupted"));
    dir.close()?;
    Ok(())
}

#[test]
fn filter() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;

    cmd.arg("log").arg("100").arg("-t").arg("2");
    cmd.assert()
        .stdout(predicate::str::contains("[A] Zera funkcji").not())
        .stdout(predicate::str::contains("[B] Metoda Newtona"))
        .stdout(
            predicate::str::contains(r#"[C] FAD\x3Csup\x3E2\x3C/sup\x3E - Pochodne mieszane"#)
                .not(),
        )
        .stdout(predicate::str::contains("[D] Skalowany Gauss").not())
        .stdout(predicate::str::contains("[E] Metoda SOR").not())
        .stdout(predicate::str::contains("[G] Funkcje sklejane").not());
    dir.close()?;
    Ok(())
}
