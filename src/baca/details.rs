use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub const SERVER_URL: &str = "baca.ii.uj.edu.pl";
pub const PERMUTATION: &str = "5A4AE95C27260DF45F17F9BF027335F6";

pub fn permutation() -> String {
    PERMUTATION.to_string()
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Language {
    Cpp = 1,
    Java = 4,
    CppWithFileSupport = 12,
    // C = ?,
    // Ada = ?,
    // Bash = ?,
    Unsupported,
}

impl Language {
    pub fn code(&self) -> String {
        let val = *self;
        let val = val as i32;
        val.to_string()
    }
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match self {
            Language::Cpp => "C++",
            Language::Java => "Java",
            Language::CppWithFileSupport => "C++ with file support",
            Language::Unsupported => "Unsupported language",
        }
        .to_string()
    }
}

impl FromStr for Language {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let l = match s {
            "C++" => Language::Cpp,
            "Java" => Language::Java,
            "C++ z obsluga plikow" => Language::CppWithFileSupport,
            _ => Self::Unsupported,
        };

        Ok(l)
    }
}
