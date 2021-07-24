use assert_cmd::Command;
use assert_fs::TempDir;
use predicates::prelude::*;
use std::env;

fn baca_dir_exists(temp: &TempDir) -> bool {
    predicate::path::exists().eval(&*temp.path().join(".baca"))
}

fn config_exists(temp: &TempDir) -> bool {
    predicate::path::exists().eval(&*temp.path().join(".baca/instance"))
}

#[test]
fn invalid_password() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new()?;

    let mut cmd = Command::cargo_bin("baca")?;

    cmd.current_dir(&temp);
    cmd.arg("-u");
    cmd.arg("init")
        .args(&["--host", "mn2020", "--login", "jaremko", "-p", "invalid"]);
    cmd.assert()
        .stdout(predicate::str::contains("Invalid login or password"));

    assert!(!baca_dir_exists(&temp));
    temp.close()?;
    Ok(())
}

#[test]
fn invalid_host() -> Result<(), Box<dyn std::error::Error>> {
    let pass = env::var("BACA_PASSWORD")?;
    let temp = assert_fs::TempDir::new()?;

    let mut cmd = Command::cargo_bin("baca")?;

    cmd.current_dir(&temp);
    cmd.arg("-u");
    cmd.arg("init")
        .args(&["--host", "invalid", "--login", "jaremko", "-p", &pass]);
    cmd.assert()
        .stdout(predicate::str::contains("Invalid host"));

    assert!(!baca_dir_exists(&temp));
    temp.close()?;
    Ok(())
}

#[test]
fn host_not_provided() -> Result<(), Box<dyn std::error::Error>> {
    let pass = env::var("BACA_PASSWORD")?;
    let temp = assert_fs::TempDir::new()?;

    let mut cmd = Command::cargo_bin("baca")?;

    cmd.current_dir(&temp);
    cmd.arg("-u");
    cmd.arg("init").args(&["--login", "jaremko", "-p", &pass]);
    cmd.assert().stderr(predicate::str::contains("--host"));

    assert!(!baca_dir_exists(&temp));
    temp.close()?;
    Ok(())
}

#[test]
fn success() -> Result<(), Box<dyn std::error::Error>> {
    let pass = env::var("BACA_PASSWORD")?;
    let temp = assert_fs::TempDir::new()?;

    let mut cmd = Command::cargo_bin("baca")?;

    cmd.current_dir(&temp);
    cmd.arg("-u");
    cmd.arg("init")
        .args(&["--host", "mn2020", "-p", &pass, "-l", "jaremko"]);
    cmd.assert().code(0);

    assert!(baca_dir_exists(&temp));
    assert!(config_exists(&temp));
    temp.close()?;
    Ok(())
}
