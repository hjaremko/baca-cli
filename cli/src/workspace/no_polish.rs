use api::error::{Result, Error};
use deunicode::deunicode;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

pub fn make_polishless_file<P>(input_file: P) -> Result<PathBuf>
where
    P: AsRef<Path>,
{
    let input_file: &Path = input_file.as_ref();
    info!("Removing Polish diacritics from {:?}", input_file);

    let content = fs::read_to_string(input_file)?;
    let content = deunicode(&content);

    let filepath =
        std::env::temp_dir().join(input_file.file_name().ok_or(Error::InputFileDoesNotExist)?);
    let mut file = File::create(filepath.clone())?;
    file.write_all(content.as_ref())?;

    debug!("New input file path: {:?}", filepath);
    debug!("New input file content:\n{}", content);

    Ok(filepath)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::{predicate, Predicate};

    fn make_input_file(content: &str, name: &str) -> PathBuf {
        let filepath = std::env::temp_dir().join(format!("test-input-{}.cpp", name));
        let mut file = File::create(filepath.clone()).unwrap();
        file.write_all(content.as_ref()).unwrap();
        filepath
    }

    #[test]
    fn all_polish() {
        let input = "ążźćłóć";
        let expected = "azzcloc";

        let input_file = make_input_file(input, "all");
        let actual_filepath = make_polishless_file(input_file).unwrap();

        assert!(predicate::path::exists().eval(&actual_filepath));
        assert!(predicate::path::eq_file(&actual_filepath)
            .utf8()
            .unwrap()
            .eval(expected));
    }

    #[test]
    fn mixed() {
        let input = "  ążźasdghjkescćłósda  3423ć   ";
        let expected = "  azzasdghjkescclosda  3423c   ";

        let input_file = make_input_file(input, "mixed");
        let actual_filepath = make_polishless_file(input_file).unwrap();

        assert!(predicate::path::exists().eval(&actual_filepath));
        assert!(predicate::path::eq_file(&actual_filepath)
            .utf8()
            .unwrap()
            .eval(expected));
    }

    #[test]
    fn no_polish() {
        let input = "  axxasdghjkescclosda  3423c  \n ";
        let expected = "  axxasdghjkescclosda  3423c  \n ";

        let input_file = make_input_file(input, "no");
        let actual_filepath = make_polishless_file(input_file).unwrap();

        assert!(predicate::path::exists().eval(&actual_filepath));
        assert!(predicate::path::eq_file(&actual_filepath)
            .utf8()
            .unwrap()
            .eval(expected));
    }
}
