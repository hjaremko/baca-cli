#[macro_use]
extern crate serial_test;

mod not_init {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use std::error::Error;

    mod util {
        use assert_cmd::Command;
        use predicates::prelude::predicate;
        use std::error::Error;

        pub fn check_command(command: &str, pattern: &str) -> Result<(), Box<dyn Error>> {
            let mut cmd = Command::cargo_bin("baca")?;

            cmd.arg(command);
            cmd.assert()
                // .failure() // todo: exit codes
                .stdout(predicate::str::contains(pattern));
            Ok(())
        }
    }

    fn check_command(command: &str) -> Result<(), Box<dyn Error>> {
        let pattern = "not initialized";
        util::check_command(command, pattern)
    }

    #[test]
    #[serial]
    fn tasks_not_initialized() -> Result<(), Box<dyn std::error::Error>> {
        check_command("tasks")
    }

    #[test]
    #[serial]
    fn refresh_not_initialized() -> Result<(), Box<dyn std::error::Error>> {
        check_command("refresh")
    }

    #[test]
    #[serial]
    fn log_not_initialized() -> Result<(), Box<dyn std::error::Error>> {
        check_command("log")
    }

    #[test]
    #[serial]
    fn details_not_initialized() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("baca")?;

        cmd.arg("details").arg("123");
        cmd.assert()
            .stdout(predicate::str::contains("not initialized"));
        Ok(())
    }
}
