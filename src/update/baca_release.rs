#[derive(Debug, PartialEq)]
pub struct BacaRelease {
    pub version: String,
    pub link: String,
}

impl BacaRelease {
    pub fn _is_newer_than(&self, other: &str) -> bool {
        self.version.as_str() > other
    }
}

impl BacaRelease {
    pub fn _new(version: &str, link: &str) -> Self {
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
        assert_eq!(
            BacaRelease::_new("v0.1.0", "link")._is_newer_than("v0.1.0"),
            false
        );
    }

    #[test]
    fn older_should_return_false() {
        assert!(!BacaRelease::_new("v0.0.1", "link")._is_newer_than("v0.0.2"));
        assert!(!BacaRelease::_new("v0.1.1", "link")._is_newer_than("v0.2.0"));
        assert!(!BacaRelease::_new("v0.1.0", "link")._is_newer_than("v0.2.0"));
        assert!(!BacaRelease::_new("v0.1.0", "link")._is_newer_than("v1.0.0"));
        assert!(!BacaRelease::_new("v0.0.1", "link")._is_newer_than("v1.0.0"));
    }

    #[test]
    fn newer_should_return_true() {
        assert!(BacaRelease::_new("v0.0.1", "link")._is_newer_than("v0.0.0"));
        assert!(BacaRelease::_new("v0.1.1", "link")._is_newer_than("v0.1.0"));
        assert!(BacaRelease::_new("v0.1.0", "link")._is_newer_than("v0.0.9"));
        assert!(BacaRelease::_new("v1.0.0", "link")._is_newer_than("v0.1.0"));
        assert!(BacaRelease::_new("v1.0.0", "link")._is_newer_than("v0.0.1"));
    }
}
