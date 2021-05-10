mod submit {
    use assert_cmd::Command;
    use assert_fs::fixture::ChildPath;
    use assert_fs::prelude::*;
    use assert_fs::TempDir;
    use predicates::prelude::*;
    use std::{env, fs};

    // todo: move to test::util
    fn initialize() -> Result<TempDir, Box<dyn std::error::Error>> {
        let pass = env::var("BACA_PASSWORD")?;
        let temp = assert_fs::TempDir::new()?;

        let mut cmd = Command::cargo_bin("baca")?;
        cmd.current_dir(&temp);
        cmd.arg("init")
            .args(&["-h", "mn2020", "-p", &pass, "-l", "jaremko"]);
        cmd.assert();
        Ok(temp)
    }

    fn make_input_file_cpp(temp: &TempDir) -> Result<ChildPath, Box<dyn std::error::Error>> {
        let input_file = temp.child("source.cpp");
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
        let temp = assert_fs::TempDir::new()?;
        let mut cmd = Command::cargo_bin("baca")?;

        cmd.current_dir(&temp);
        cmd.arg("submit")
            .args(&["-t", "1", "-l", "C++", "-f", "source.cpp"]);
        cmd.assert()
            .stdout(predicate::str::contains("not initialized"));

        temp.close()?;
        Ok(())
    }

    #[test]
    fn inactive_task_should_report_error() -> Result<(), Box<dyn std::error::Error>> {
        let temp = initialize()?;
        let input_file = make_input_file_cpp(&temp)?;

        let mut cmd = Command::cargo_bin("baca")?;
        cmd.current_dir(&temp);
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

        temp.close()?;
        Ok(())
    }

    #[test]
    fn no_task_id_should_report_error() -> Result<(), Box<dyn std::error::Error>> {
        let temp = initialize()?;

        let mut cmd = Command::cargo_bin("baca")?;
        cmd.current_dir(&temp);
        cmd.args(&["submit", "-l", "C++", "-f", "dummy.txt"]);

        cmd.assert()
            .stdout(predicate::str::contains("provide task_id"));

        temp.close()?;
        Ok(())
    }

    #[test]
    fn no_file_should_report_error() -> Result<(), Box<dyn std::error::Error>> {
        let temp = initialize()?;

        let mut cmd = Command::cargo_bin("baca")?;
        cmd.current_dir(&temp);
        cmd.args(&["submit", "-l", "C++", "-t", "2"]);

        cmd.assert()
            .stdout(predicate::str::contains("provide file"));

        temp.close()?;
        Ok(())
    }

    #[test]
    fn no_language_should_report_error() -> Result<(), Box<dyn std::error::Error>> {
        initialize()?;
        let temp = initialize()?;

        let mut cmd = Command::cargo_bin("baca")?;
        cmd.args(&["submit", "-f", "dummy.txt", "-t", "2"]);
        cmd.current_dir(&temp);

        cmd.assert()
            .stdout(predicate::str::contains("provide language"));

        temp.close()?;
        Ok(())
    }

    #[test]
    fn invalid_filename_should_report_error() -> Result<(), Box<dyn std::error::Error>> {
        initialize()?;
        let temp = initialize()?;

        let mut cmd = Command::cargo_bin("baca")?;
        cmd.args(&["submit", "-f", "dummy.txt", "-t", "2", "-l", "C++"]);
        cmd.current_dir(&temp);

        cmd.assert()
            .stdout(predicate::str::contains("Error reading source file"));

        temp.close()?;
        Ok(())
    }

    #[test]
    fn invalid_language_should_report_error() -> Result<(), Box<dyn std::error::Error>> {
        initialize()?;
        let temp = initialize()?;

        let mut cmd = Command::cargo_bin("baca")?;
        cmd.args(&["submit", "-f", "dummy.txt", "-t", "2", "-l", "CPlusPlus"]);
        cmd.current_dir(&temp);

        cmd.assert()
            .stdout(predicate::str::contains("cplusplus is not yet supported"));

        temp.close()?;
        Ok(())
    }

    #[test]
    fn invalid_task_should_report_error() -> Result<(), Box<dyn std::error::Error>> {
        let temp = initialize()?;

        let mut cmd = Command::cargo_bin("baca")?;
        cmd.args(&["submit", "-f", "dummy.txt", "-t", "2123123", "-l", "C++"]);
        cmd.current_dir(&temp);

        cmd.assert()
            .stdout(predicate::str::contains("Task no. 2123123 does not exist"));

        temp.close()?;
        Ok(())
    }

    #[test]
    fn zip_should_zip() -> Result<(), Box<dyn std::error::Error>> {
        let temp = initialize()?;
        let input_file = make_input_file_cpp(&temp)?;

        let mut cmd = Command::cargo_bin("baca")?;
        cmd.current_dir(&temp);
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

        temp.close()?;
        Ok(())
    }

    #[test]
    fn default_option_should_save_task() -> Result<(), Box<dyn std::error::Error>> {
        let temp = initialize()?;

        let mut cmd = Command::cargo_bin("baca")?;
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
        cmd.current_dir(&temp);

        cmd.assert()
            .stdout(predicate::str::contains("Task config has been saved."))
            .stdout(predicate::str::contains("Submitting dummy.txt"))
            .stdout(predicate::str::contains("Java"))
            .stdout(predicate::str::contains("[B] Metoda Newtona"));

        let config_path = &*temp.path().join(".baca/task");
        let does_exit_pred = predicate::path::exists().eval(config_path);
        assert!(does_exit_pred);
        let saved_config = fs::read_to_string(config_path)?;
        assert!(saved_config.contains("dummy.txt"));
        assert!(saved_config.contains("Java"));
        assert!(saved_config.contains("2"));

        temp.close()?;
        Ok(())
    }

    #[test]
    fn saved_task_should_be_used() -> Result<(), Box<dyn std::error::Error>> {
        let temp = initialize()?;

        let mut cmd = Command::cargo_bin("baca")?;
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
        cmd.current_dir(&temp);
        cmd.assert();

        let mut cmd = Command::cargo_bin("baca")?;
        cmd.arg("submit");
        cmd.current_dir(&temp);
        cmd.assert()
            .stdout(predicate::str::contains("Submitting dummy.txt"))
            .stdout(predicate::str::contains("Java"))
            .stdout(predicate::str::contains("[B] Metoda Newtona"));

        temp.close()?;
        Ok(())
    }

    #[test]
    fn cmd_options_should_override_saved_task() -> Result<(), Box<dyn std::error::Error>> {
        let temp = initialize()?;

        let mut cmd = Command::cargo_bin("baca")?;
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
        cmd.current_dir(&temp);
        cmd.assert();

        let mut cmd = Command::cargo_bin("baca")?;
        cmd.args(&["submit", "-f", "hello.cpp", "-l", "C++"]);
        cmd.current_dir(&temp);
        cmd.assert()
            .stdout(predicate::str::contains("Submitting hello.cpp"))
            .stdout(predicate::str::contains("C++"))
            .stdout(predicate::str::contains("[B] Metoda Newtona"));

        temp.close()?;
        Ok(())
    }

    #[test]
    fn clear_should_remove_saved_task() -> Result<(), Box<dyn std::error::Error>> {
        let temp = initialize()?;

        let mut cmd = Command::cargo_bin("baca")?;
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
        cmd.current_dir(&temp);
        cmd.assert();

        assert_task_config_file_exists(&temp);

        let mut cmd = Command::cargo_bin("baca")?;
        cmd.arg("submit").arg("clear");
        cmd.current_dir(&temp);
        cmd.assert();

        assert_task_config_file_does_not_exist(&temp);
        temp.close()?;
        Ok(())
    }

    #[test]
    fn clear_on_already_clear_should_do_nothing() -> Result<(), Box<dyn std::error::Error>> {
        let temp = initialize()?;

        let mut cmd = Command::cargo_bin("baca")?;
        cmd.arg("submit").arg("clear");
        cmd.current_dir(&temp);
        cmd.assert();

        assert_task_config_file_does_not_exist(&temp);
        assert_config_file_exists(&temp);
        temp.close()?;
        Ok(())
    }
}
