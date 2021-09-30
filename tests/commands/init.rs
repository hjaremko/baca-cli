use crate::util::get_baca_credentials;
use assert_cmd::Command;
use assert_fs::TempDir;
use predicates::prelude::*;

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
    let (login, pass, _) = get_baca_credentials();
    let temp = assert_fs::TempDir::new()?;

    let mut cmd = Command::cargo_bin("baca")?;

    cmd.current_dir(&temp);
    cmd.arg("-u");
    cmd.arg("init")
        .args(&["--host", "invalid", "--login", &login, "-p", &pass]);
    cmd.assert()
        .stdout(predicate::str::contains("Invalid host"));

    assert!(!baca_dir_exists(&temp));
    temp.close()?;
    Ok(())
}

#[test]
fn success() -> Result<(), Box<dyn std::error::Error>> {
    let (login, pass, host) = get_baca_credentials();
    let temp = assert_fs::TempDir::new()?;

    let mut cmd = Command::cargo_bin("baca")?;

    cmd.current_dir(&temp);
    cmd.arg("-u");
    cmd.arg("init")
        .args(&["--host", &host, "-p", &pass, "-l", &login]);
    cmd.assert().code(0);

    assert!(baca_dir_exists(&temp));
    assert!(config_exists(&temp));
    temp.close()?;
    Ok(())
}
