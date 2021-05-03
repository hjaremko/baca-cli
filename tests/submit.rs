mod submit {
    use assert_cmd::Command;
    use assert_fs::fixture::ChildPath;
    use assert_fs::prelude::*;
    use assert_fs::TempDir;
    use predicates::prelude::*;
    use std::env;

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

    // todo: default tests
}
