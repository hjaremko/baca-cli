use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BacaRelease {
    pub version: String,
    #[serde(skip_serializing)]
    pub link: String,
}

impl BacaRelease {
    pub fn is_newer_than(&self, other: &str) -> bool {
        self.version.as_str() > other
    }

    pub fn new(version: &str, link: &str) -> Self {
        BacaRelease {
            version: version.to_string(),
            link: link.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_should_return_false() {
        assert!(!BacaRelease::new("0.1.0", "link").is_newer_than("0.1.0"));
    }

    #[test]
    fn older_should_return_false() {
        assert!(!BacaRelease::new("0.0.1", "link").is_newer_than("0.0.2"));
        assert!(!BacaRelease::new("0.1.1", "link").is_newer_than("0.2.0"));
        assert!(!BacaRelease::new("0.1.0", "link").is_newer_than("0.2.0"));
        assert!(!BacaRelease::new("0.1.0", "link").is_newer_than("1.0.0"));
        assert!(!BacaRelease::new("0.0.1", "link").is_newer_than("1.0.0"));
    }

    #[test]
    fn newer_should_return_true() {
        assert!(BacaRelease::new("0.0.1", "link").is_newer_than("0.0.0"));
        assert!(BacaRelease::new("0.1.1", "link").is_newer_than("0.1.0"));
        assert!(BacaRelease::new("0.1.0", "link").is_newer_than("0.0.9"));
        assert!(BacaRelease::new("1.0.0", "link").is_newer_than("0.1.0"));
        assert!(BacaRelease::new("1.0.0", "link").is_newer_than("0.0.1"));
    }
}
