use crate::error::*;
use crate::model::Language;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tracing::{debug, info};

fn is_header_present<P>(input_file: P, lang: &Language) -> Result<bool>
where
    P: AsRef<Path>,
{
    println!("Checking for {:?} header...", lang);

    let input_file = File::open(input_file)?;
    let first_line = BufReader::new(input_file)
        .lines()
        .take(1)
        .map(|x| x.unwrap())
        .collect::<String>();

    debug!("First line: {first_line}");
    let r = lang.is_comment(&first_line);
    info!("Header found: {r}");
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn make_input_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_ref()).unwrap();
        file
    }

    #[test]
    fn empty_file() {
        let input = make_input_file("");

        assert!(!(is_header_present(input.path(), &Language::Cpp).unwrap()));
    }

    #[test]
    fn invalid_file() {
        assert!(is_header_present("/i/hope/invalid_path", &Language::Cpp).is_err());
    }

    #[test]
    fn no_header() {
        let input = make_input_file(
            r#"#include <iostream>

void Add(int** arr, int*[] arr2)
{
std::cout << "Hello\n";
return;
}



int moin(int argc, char** argv)
{
return 5;
}
    "#,
        );

        assert!(!(is_header_present(input.path(), &Language::Cpp).unwrap()));
    }

    #[test]
    fn header_present() {
        let input = make_input_file(
            r#"// Hubert Jaremko
#include <iostream>

int main(int argc, char** argv)
{
return 5;
}
    "#,
        );

        assert!(is_header_present(input.path(), &Language::Cpp).unwrap());
    }
}
