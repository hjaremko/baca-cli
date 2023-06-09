use crate::util::get_baca_credentials;
use assert_cmd::Command;
use assert_fs::TempDir;
use predicates::prelude::*;
use std::fs::read_to_string;

fn baca_dir_exists(temp: &TempDir) -> bool {
    predicate::path::exists().eval(&*temp.path().join(".baca"))
}

fn config_exists(temp: &TempDir) -> bool {
    predicate::path::exists().eval(&*temp.path().join(".baca/connection"))
}

#[test]
#[ignore]
fn invalid_password() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new()?;

    let mut cmd = Command::cargo_bin("baca")?;

    cmd.current_dir(&temp);
    cmd.arg("-u");
    cmd.arg("init")
        .args(["--host", "mn2020", "--login", "jaremko", "-p", "invalid"]);
    cmd.assert()
        .stdout(predicate::str::contains("Invalid login or password"));

    assert!(!baca_dir_exists(&temp));
    temp.close()?;
    Ok(())
}

#[test]
#[ignore]
fn invalid_host() -> Result<(), Box<dyn std::error::Error>> {
    let (login, pass, _) = get_baca_credentials();
    let temp = assert_fs::TempDir::new()?;

    let mut cmd = Command::cargo_bin("baca")?;

    cmd.current_dir(&temp);
    cmd.arg("-u");
    cmd.arg("init")
        .args(["--host", "invalid", "--login", &login, "-p", &pass]);
    cmd.assert()
        .stdout(predicate::str::contains("Invalid host"));

    assert!(!baca_dir_exists(&temp));
    temp.close()?;
    Ok(())
}

#[test]
#[ignore]
fn success() -> Result<(), Box<dyn std::error::Error>> {
    let (login, pass, host) = get_baca_credentials();
    let temp = assert_fs::TempDir::new()?;

    let mut cmd = Command::cargo_bin("baca")?;

    cmd.current_dir(&temp);
    cmd.arg("-u");
    cmd.arg("init")
        .args(["--host", &host, "-p", &pass, "-l", &login]);
    cmd.assert().code(0);

    assert!(baca_dir_exists(&temp));
    assert!(config_exists(&temp));
    temp.close()?;
    Ok(())
}

#[test]
#[ignore]
fn should_save_version() -> Result<(), Box<dyn std::error::Error>> {
    let (login, pass, host) = get_baca_credentials();
    let temp = assert_fs::TempDir::new()?;

    let mut cmd = Command::cargo_bin("baca")?;

    cmd.current_dir(&temp);
    cmd.arg("-u");
    cmd.arg("init")
        .args(["--host", &host, "-p", &pass, "-l", &login]);
    cmd.assert().code(0);

    let version_path = temp.path().join(".baca/version");
    assert!(predicate::path::exists().eval(&version_path));
    let saved_version = read_to_string(version_path).unwrap();
    assert!(predicate::str::contains(env!("CARGO_PKG_VERSION")).eval(&saved_version));
    temp.close()?;
    Ok(())
}
