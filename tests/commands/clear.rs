use crate::util::{initialize_correct_workspace, set_up_command, set_up_with_dir};
use predicates::prelude::*;

#[test]
#[ignore]
fn clear_when_init_then_remove_directory() {
    let dir = initialize_correct_workspace().unwrap();
    let mut cmd = set_up_command(&dir).unwrap();

    assert!(predicate::path::exists().eval(&dir.path().join(".baca")));
    cmd.arg("-v");
    cmd.arg("clear");
    cmd.assert();

    assert!(predicate::path::missing().eval(&dir.path().join(".baca")));
    dir.close().unwrap();
}

#[test]
fn clear_when_not_init_then_print_error() {
    let (dir, mut cmd) = set_up_with_dir().unwrap();

    cmd.arg("-v");
    cmd.arg("clear");
    cmd.assert()
        .stdout(predicate::str::contains("not initialized"));

    assert!(predicate::path::missing().eval(&dir.path().join(".baca")));
    dir.close().unwrap();
}
