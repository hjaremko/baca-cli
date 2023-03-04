// use baca_api::error::*;
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use tracing::{debug, info};
use api::error::{Result, Error};

pub fn remove_main<P>(input_file: P) -> Result<PathBuf>
where
    P: AsRef<Path>,
{
    let input_file: &Path = input_file.as_ref();
    info!("Removing main from {:?}", input_file);

    let content = fs::read_to_string(input_file)?;
    let content = strip_main(&content);

    let filepath =
        std::env::temp_dir().join(input_file.file_name().ok_or(Error::InputFileDoesNotExist)?);
    let mut file = File::create(filepath.clone())?;
    file.write_all(content.as_ref())?;

    debug!("New input file path: {:?}", filepath);
    debug!("New input file content:\n{}", content);

    Ok(filepath)
}

fn strip_main(content: &str) -> String {
    let re = Regex::new(r#"int\s+main"#).unwrap();
    let f = re.find(content);

    if f.is_none() {
        return content.to_string();
    }
    let f = f.unwrap();

    let mut brackets = 0;
    let mut end = 0;
    let mut i = f.end();
    let mut first_bracket = false;

    for c in content.chars().skip(f.end()) {
        i += 1;

        if c == '{' {
            brackets += 1;
            first_bracket = true;
        } else if c == '}' {
            brackets -= 1;
        }

        if first_bracket && brackets == 0 {
            end = i;
            break;
        }
    }

    let mut result = content.to_string();
    result.replace_range(f.start()..end, "");
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::{predicate, Predicate};

    #[test]
    fn no_main_present() {
        let input = r#"
        // Hubert Jaremko
        #include <iostream>

        void Add(int** arr, int*[] arr2)
        {
            std::cout << "Hello\n";
            return;
        }

        void main() {
        }
        "#;

        let actual = strip_main(input);
        assert_eq!(actual, input);
    }

    #[test]
    fn minimal_main() {
        let input = r#"int main() {
}"#;
        let expected = r#""#;
        let actual = strip_main(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn one_line() {
        let input = r#"int main() {} int foo() { return 5; }"#;
        let expected = r#" int foo() { return 5; }"#;
        let actual = strip_main(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn properly_formatted() {
        let input = r#"
// Hubert Jaremko
#include <iostream>

void Add(int** arr, int*[] arr2)
{
    std::cout << "Hello\n";
    return;
}

int main()
{
    Add(..., ...);
    return 0;
}

        "#;

        let expected = r#"
// Hubert Jaremko
#include <iostream>

void Add(int** arr, int*[] arr2)
{
    std::cout << "Hello\n";
    return;
}



        "#;

        let actual = strip_main(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn with_arguments() {
        let input = r#"
// Hubert Jaremko
#include <iostream>

void Add(int** arr, int*[] arr2)
{
    std::cout << "Hello\n";
    return;
}

int main(int argc, char** argv)
{
    Add(..., ...);
    return 0;
}
        "#;

        let expected = r#"
// Hubert Jaremko
#include <iostream>

void Add(int** arr, int*[] arr2)
{
    std::cout << "Hello\n";
    return;
}


        "#;

        let actual = strip_main(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn with_another_function_below() {
        let input = r#"
// Hubert Jaremko
#include <iostream>

void Add(int** arr, int*[] arr2)
{
    std::cout << "Hello\n";
    return;
}

int main(int argc, char** argv)
{
    Add(..., ...);
    return 0;
}

int moin(int argc, char** argv)
{
    return 5;
}
        "#;

        let expected = r#"
// Hubert Jaremko
#include <iostream>

void Add(int** arr, int*[] arr2)
{
    std::cout << "Hello\n";
    return;
}



int moin(int argc, char** argv)
{
    return 5;
}
        "#;

        let actual = strip_main(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn with_nested_brackets() {
        let input = r#"
// Hubert Jaremko
#include <iostream>

void Add(int** arr, int*[] arr2)
{
    std::cout << "Hello\n";
    return;
}

int main(int argc, char** argv)
{
    Add(..., ...);
    {
      if ( test) {
      {}{}
      }
    }
    return 0;
}

int moin(int argc, char** argv)
{
    return 5;
}
        "#;

        let expected = r#"
// Hubert Jaremko
#include <iostream>

void Add(int** arr, int*[] arr2)
{
    std::cout << "Hello\n";
    return;
}



int moin(int argc, char** argv)
{
    return 5;
}
        "#;

        let actual = strip_main(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn with_stretched_main() {
        let input = r#"
// Hubert Jaremko
#include <iostream>

void Add(int** arr, int*[] arr2)
{
    std::cout << "Hello\n";
    return;
}

int



main



(int argc,


char** argv

          )
{
    Add(..., ...);
    {
      if ( test) {
      {}{}
      }
    }
    return 0;
}

int moin(int argc, char** argv)
{
    return 5;
}
        "#;

        let expected = r#"
// Hubert Jaremko
#include <iostream>

void Add(int** arr, int*[] arr2)
{
    std::cout << "Hello\n";
    return;
}



int moin(int argc, char** argv)
{
    return 5;
}
        "#;

        let actual = strip_main(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn saved_file_should_contain_stripped_content() {
        let input = r#"
// Hubert Jaremko
#include <iostream>

void Add(int** arr, int*[] arr2)
{
    std::cout << "Hello\n";
    return;
}

int main(int argc, char** argv)
{
    Add(..., ...);
    {
      if ( test) {
      {}{}
      }
    }
    return 0;
}

int moin(int argc, char** argv)
{
    return 5;
}
        "#;
        let expected = r#"
// Hubert Jaremko
#include <iostream>

void Add(int** arr, int*[] arr2)
{
    std::cout << "Hello\n";
    return;
}



int moin(int argc, char** argv)
{
    return 5;
}
        "#;
        let original_filepath = std::env::temp_dir().join("input.cpp");
        let mut original_file = File::create(original_filepath.clone()).unwrap();
        original_file.write_all(input.as_ref()).unwrap();

        let actual_filepath = remove_main(&original_filepath).unwrap();

        assert!(predicate::path::exists().eval(&actual_filepath));
        assert!(predicate::path::eq_file(&actual_filepath)
            .utf8()
            .unwrap()
            .eval(expected));
        assert_eq!(actual_filepath.file_name().unwrap(), "input.cpp");
    }
}
