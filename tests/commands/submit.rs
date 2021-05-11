use crate::util::*;
use assert_fs::TempDir;
use predicates::prelude::*;
use std::fs;

fn assert_config_file_exists(temp: &TempDir) {
    let config_path = &*temp.path().join(".baca/instance");
    let pred = predicate::path::exists().eval(config_path);
    assert!(pred);
}

fn assert_task_config_file_exists(temp: &TempDir) {
    let config_path = &*temp.path().join(".baca/task");
    let pred = predicate::path::exists().eval(config_path);
    assert!(pred);
}

fn assert_task_config_file_does_not_exist(temp: &TempDir) {
    let config_path = &*temp.path().join(".baca/task");
    let pred = predicate::path::exists().not().eval(config_path);
    assert!(pred);
}

#[test]
fn not_initialized() -> Result<(), Box<dyn std::error::Error>> {
    assert_fails_if_not_initialized(&["submit", "-t", "1", "-l", "C++", "-f", "source.cpp"])
}

#[test]
fn inactive_task_should_report_error() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;
    let input_file = make_input_file_cpp(&dir)?;

    cmd.args(&[
        "submit",
        "-t",
        "1",
        "-l",
        "C++",
        "-f",
        input_file.path().to_str().unwrap(),
    ]);

    cmd.assert()
        .stdout(predicate::str::contains("source.cpp"))
        .stdout(predicate::str::contains("[A] Zera funkcji"))
        .stdout(predicate::str::contains("C++"))
        .stdout(predicate::str::contains("Error sending submit"));

    dir.close()?;
    Ok(())
}

#[test]
fn no_task_id_should_report_error() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;

    cmd.args(&["submit", "-l", "C++", "-f", "dummy.txt"]);

    cmd.assert()
        .stdout(predicate::str::contains("provide task_id"));

    dir.close()?;
    Ok(())
}

#[test]
fn no_file_should_report_error() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;

    cmd.args(&["submit", "-l", "C++", "-t", "2"]);

    cmd.assert()
        .stdout(predicate::str::contains("provide file"));

    dir.close()?;
    Ok(())
}

#[test]
fn no_language_should_report_error() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;

    cmd.args(&["submit", "-f", "dummy.txt", "-t", "2"]);

    cmd.assert()
        .stdout(predicate::str::contains("provide language"));

    dir.close()?;
    Ok(())
}

#[test]
fn invalid_filename_should_report_error() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;

    cmd.args(&["submit", "-f", "dummy.txt", "-t", "2", "-l", "C++"]);

    cmd.assert()
        .stdout(predicate::str::contains("Error reading source file"));

    dir.close()?;
    Ok(())
}

#[test]
fn invalid_language_should_report_error() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;

    cmd.args(&["submit", "-f", "dummy.txt", "-t", "2", "-l", "CPlusPlus"]);

    cmd.assert()
        .stdout(predicate::str::contains("cplusplus is not yet supported"));

    dir.close()?;
    Ok(())
}

#[test]
fn invalid_task_should_report_error() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;

    cmd.args(&["submit", "-f", "dummy.txt", "-t", "2123123", "-l", "C++"]);

    cmd.assert()
        .stdout(predicate::str::contains("Task no. 2123123 does not exist"));

    dir.close()?;
    Ok(())
}

#[test]
fn zip_should_zip() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;
    let input_file = make_input_file_cpp(&dir)?;

    cmd.args(&[
        "submit",
        "-t",
        "1",
        "-l",
        "C++",
        "-f",
        input_file.path().to_str().unwrap(),
        "--zip",
    ]);

    cmd.assert()
        .stdout(predicate::str::contains("Zipping source.cpp"));

    dir.close()?;
    Ok(())
}

#[test]
fn default_option_should_save_task() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;

    cmd.args(&[
        "submit",
        "-f",
        "dummy.txt",
        "-t",
        "2",
        "-l",
        "Java",
        "--default",
    ]);

    cmd.assert()
        .stdout(predicate::str::contains("Task config has been saved."))
        .stdout(predicate::str::contains("Submitting dummy.txt"))
        .stdout(predicate::str::contains("Java"))
        .stdout(predicate::str::contains("[B] Metoda Newtona"));

    let config_path = dir.baca_task_config_file_path();
    let does_exit_pred = predicate::path::exists().eval(&config_path);
    assert!(does_exit_pred);
    let saved_config = fs::read_to_string(&config_path)?;
    assert!(saved_config.contains("dummy.txt"));
    assert!(saved_config.contains("Java"));
    assert!(saved_config.contains("2"));

    dir.close()?;
    Ok(())
}

#[test]
fn saved_task_should_be_used() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;

    cmd.args(&[
        "submit",
        "-f",
        "dummy.txt",
        "-t",
        "2",
        "-l",
        "Java",
        "--default",
    ]);
    cmd.assert();

    let mut cmd = set_up_command(&dir)?;
    cmd.arg("submit");
    cmd.assert()
        .stdout(predicate::str::contains("Submitting dummy.txt"))
        .stdout(predicate::str::contains("Java"))
        .stdout(predicate::str::contains("[B] Metoda Newtona"));

    dir.close()?;
    Ok(())
}

#[test]
fn cmd_options_should_override_saved_task() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;

    cmd.args(&[
        "submit",
        "-f",
        "dummy.txt",
        "-t",
        "2",
        "-l",
        "Java",
        "--default",
    ]);
    cmd.assert();

    let mut cmd = set_up_command(&dir)?;
    cmd.args(&["submit", "-f", "hello.cpp", "-l", "C++"]);
    cmd.assert()
        .stdout(predicate::str::contains("Submitting hello.cpp"))
        .stdout(predicate::str::contains("C++"))
        .stdout(predicate::str::contains("[B] Metoda Newtona"));

    dir.close()?;
    Ok(())
}

#[test]
fn clear_should_remove_saved_task() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;

    cmd.args(&[
        "submit",
        "-f",
        "dummy.txt",
        "-t",
        "2",
        "-l",
        "Java",
        "--default",
    ]);
    cmd.assert();

    assert_task_config_file_exists(&dir);

    let mut cmd = set_up_command(&dir)?;
    cmd.arg("submit").arg("clear");
    cmd.assert();

    assert_task_config_file_does_not_exist(&dir);
    dir.close()?;
    Ok(())
}

#[test]
fn clear_on_already_clear_should_do_nothing() -> Result<(), Box<dyn std::error::Error>> {
    let dir = initialize_correct_workspace()?;
    let mut cmd = set_up_command(&dir)?;

    cmd.arg("submit").arg("clear");
    cmd.assert();

    assert_task_config_file_does_not_exist(&dir);
    assert_config_file_exists(&dir);
    dir.close()?;
    Ok(())
}
