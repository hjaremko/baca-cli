use crate::workspace::ConfigObject;
use std::path::{Path, PathBuf};

// todo: walk up dir tree until found
#[derive(Clone)]
pub struct WorkspacePaths {
    root_path: PathBuf,
}

impl WorkspacePaths {
    pub fn new() -> Self {
        Self {
            root_path: Path::new(".").to_path_buf(),
        }
    }

    pub(crate) fn _with_root(root_path: &Path) -> Self {
        Self {
            root_path: root_path.to_path_buf(),
        }
    }

    pub fn baca_dir(&self) -> PathBuf {
        self.root_path.join(".baca")
    }

    pub fn config_path<T: ConfigObject>(&self) -> PathBuf {
        self.baca_dir().join(&T::config_filename())
    }
}
