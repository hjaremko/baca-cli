use crate::util::set_up_with_dir;
use predicates::prelude::*;

#[test]
fn given_connection_config_edit_when_not_init_then_print_error() {
    let (dir, mut cmd) = set_up_with_dir().unwrap();
    cmd.arg("-v");
    cmd.arg("config");
    cmd.assert()
        .stdout(predicate::str::contains("not initialized"));

    dir.close().unwrap();
}
