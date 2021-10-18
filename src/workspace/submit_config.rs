use crate::error::Error;
use crate::model::Language;
use crate::workspace::{ConfigObject, Workspace};
use serde::{Deserialize, Serialize};
#[cfg(test)]
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SubmitConfig {
    pub id: String,
    pub file: PathBuf,
    pub to_zip: bool,
    pub language: Language,
    pub rename_as: Option<String>,
}

impl SubmitConfig {
    #[cfg(test)]
    pub fn new(
        id: &str,
        file: &Path,
        to_zip: bool,
        language: Language,
        rename_as: Option<String>,
    ) -> Self {
        Self {
            id: id.to_string(),
            file: file.to_owned(),
            to_zip,
            language,
            rename_as,
        }
    }
}

impl Default for SubmitConfig {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            file: PathBuf::new(),
            to_zip: false,
            language: Language::Unsupported,
            rename_as: None,
        }
    }
}

impl ConfigObject for SubmitConfig {
    fn save_config<W: Workspace>(&self, workspace: &W) -> crate::error::Result<()> {
        workspace.save_config_object(self)?;
        Ok(())
    }

    fn read_config<W: Workspace>(workspace: &W) -> crate::error::Result<Self> {
        workspace.read_config_object::<Self>().map_err(|e| {
            if let Error::Other(inner) = e {
                return Error::ReadingTask(inner);
            }

            e
        })
    }

    fn remove_config<W: Workspace>(workspace: &W) -> crate::error::Result<()> {
        workspace.remove_config_object::<Self>().map_err(|e| {
            if let Error::Other(inner) = e {
                return Error::RemovingTask(inner);
            }

            e
        })
    }

    fn config_filename() -> String {
        "submit".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Language;
    use crate::workspace::workspace_dir::tests::make_temp_workspace;
    use assert_fs::fixture::ChildPath;
    use assert_fs::prelude::*;
    use predicates::prelude::*;
    use std::ops::Not;

    #[test]
    fn save_read_task_success() {
        let (temp_dir, mock_paths, workspace) = make_temp_workspace().unwrap();
        let input_file = temp_dir.child("foo.sh");
        input_file.touch().unwrap();
        let expected_submit_config =
            SubmitConfig::new("2", input_file.path(), false, Language::Bash, None);

        workspace.initialize().unwrap();
        expected_submit_config.save_config(&workspace).unwrap();

        assert_eq!(
            SubmitConfig::read_config(&workspace).unwrap(),
            expected_submit_config
        );
        assert!(predicate::path::exists().eval(mock_paths.config_path::<SubmitConfig>().as_path()));
        temp_dir.close().unwrap();
    }

    #[test]
    fn read_corrupted_task() {
        let (temp_dir, mock_paths, workspace) = make_temp_workspace().unwrap();
        let corrupted_submit_config = ChildPath::new(mock_paths.config_path::<SubmitConfig>());

        workspace.initialize().unwrap();
        corrupted_submit_config.write_str("invalid config").unwrap();
        let result = SubmitConfig::read_config(&workspace);

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, Error::WorkspaceCorrupted));
        }
        assert!(predicate::path::exists().eval(mock_paths.config_path::<SubmitConfig>().as_path()));
        temp_dir.close().unwrap();
    }

    #[test]
    fn read_no_task_exists() {
        let (temp_dir, mock_paths, workspace) = make_temp_workspace().unwrap();

        workspace.initialize().unwrap();
        let result = SubmitConfig::read_config(&workspace);

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, Error::WorkspaceCorrupted), "error = {:?}", e);
        }
        assert!(predicate::path::missing().eval(mock_paths.config_path::<SubmitConfig>().as_path()));
        temp_dir.close().unwrap();
    }

    #[test]
    fn save_task_not_initialized() {
        let (temp_dir, mock_paths, workspace) = make_temp_workspace().unwrap();

        let submit_config =
            SubmitConfig::new("2", Path::new("foo.txt"), true, Language::Bash, None);
        let result = submit_config.save_config(&workspace);

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, Error::WorkspaceNotInitialized));
        }
        assert!(predicate::path::missing().eval(mock_paths.config_path::<SubmitConfig>().as_path()));
        temp_dir.close().unwrap();
    }

    #[test]
    fn save_task_should_override() {
        let (temp_dir, mock_paths, workspace) = make_temp_workspace().unwrap();
        let input_file = temp_dir.child("foo.sh");
        input_file.touch().unwrap();
        let submit_config_first =
            SubmitConfig::new("2", input_file.path(), false, Language::Bash, None);
        let submit_config_second =
            SubmitConfig::new("3", Path::new("bar.cpp"), false, Language::Cpp, None);

        workspace.initialize().unwrap();
        submit_config_first.save_config(&workspace).unwrap();
        submit_config_second.save_config(&workspace).unwrap();

        assert_eq!(
            SubmitConfig::read_config(&workspace).unwrap(),
            submit_config_second
        );
        assert!(predicate::path::exists().eval(mock_paths.config_path::<SubmitConfig>().as_path()));
        temp_dir.close().unwrap();
    }

    #[test]
    fn remove_submit_config() {
        let (temp_dir, mock_paths, workspace) = make_temp_workspace().unwrap();
        let input_file = temp_dir.child("foo.sh");
        input_file.touch().unwrap();
        let submit_config = SubmitConfig::new("2", input_file.path(), false, Language::Bash, None);

        workspace.initialize().unwrap();
        submit_config.save_config(&workspace).unwrap();
        SubmitConfig::remove_config(&workspace).unwrap();
        assert!(predicate::path::exists()
            .eval(mock_paths.config_path::<SubmitConfig>().as_path())
            .not());
        temp_dir.close().unwrap();
    }
}
