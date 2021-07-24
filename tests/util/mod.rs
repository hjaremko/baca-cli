use assert_cmd::Command;
use assert_fs::fixture::ChildPath;
use assert_fs::prelude::{FileTouch, FileWriteStr, PathChild};
use assert_fs::TempDir;
use predicates::prelude::predicate;
use std::env;
use std::error::Error;
use std::path::Path;

pub trait BacaDirectoryPaths {
    fn baca_config_file_path(&self) -> Box<Path>;
    fn baca_task_config_file_path(&self) -> Box<Path>;
}

impl BacaDirectoryPaths for TempDir {
    fn baca_config_file_path(&self) -> Box<Path> {
        self.path().join(".baca/instance").into_boxed_path()
    }

    fn baca_task_config_file_path(&self) -> Box<Path> {
        self.path().join(".baca/task").into_boxed_path()
    }
}

pub fn set_up_command(dir: &TempDir) -> Result<Command, Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("baca")?;
    cmd.current_dir(&dir);
    cmd.arg("-u");
    Ok(cmd)
}

pub fn set_up_with_dir() -> Result<(TempDir, Command), Box<dyn Error>> {
    let dir = assert_fs::TempDir::new()?;
    let cmd = set_up_command(&dir)?;
    Ok((dir, cmd))
}

pub fn assert_contains_pattern(command: &[&str], pattern: &str) -> Result<(), Box<dyn Error>> {
    let (dir, mut cmd) = set_up_with_dir()?;

    cmd.args(command);
    cmd.assert()
        // .failure() // todo: exit codes
        .stdout(predicate::str::contains(pattern));

    dir.close()?;
    Ok(())
}

pub fn assert_fails_if_not_initialized(command: &[&str]) -> Result<(), Box<dyn Error>> {
    let pattern = "not initialized";
    assert_contains_pattern(command, pattern)
}

pub fn initialize_correct_workspace() -> Result<TempDir, Box<dyn std::error::Error>> {
    let pass = env::var("BACA_PASSWORD").expect("No BACA_PASSWORD provided");
    let (dir, mut cmd) = set_up_with_dir()?;

    cmd.arg("init")
        .args(&["-h", "mn2020", "-p", &pass, "-l", "jaremko"]);
    cmd.assert();
    Ok(dir)
}

pub fn make_input_file_cpp(dir: &TempDir) -> Result<ChildPath, Box<dyn std::error::Error>> {
    let input_file = dir.child("source.cpp");
    input_file.touch()?;
    input_file.write_str(
        r#"
        \\ Hubert Jaremko
        #include <iostream>
        int main() {
            std::cout << "Hello world" << std::endl;
            return 0;
        }
        "#,
    )?;
    Ok(input_file)
}
