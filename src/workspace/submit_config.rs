use crate::error::Error;
use crate::model::Language;
use crate::workspace::{ConfigObject, Workspace};
use merge::Merge;
use serde::{Deserialize, Serialize};
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;

fn merge_left_option<T>(left: &mut Option<T>, right: Option<T>) {
    if let Some(right) = right {
        let _ = left.insert(right);
    }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Merge, Clone)]
pub struct SubmitConfig {
    #[merge(strategy = merge_left_option)]
    pub id: Option<String>,
    #[merge(strategy = merge_left_option)]
    pub files: Option<Vec<PathBuf>>,
    #[merge(strategy = merge::bool::overwrite_false)]
    pub to_zip: bool,
    #[merge(strategy = merge_left_option)]
    pub language: Option<Language>,
    #[merge(strategy = merge_left_option)]
    pub rename_as: Option<String>,
    #[merge(strategy = merge::bool::overwrite_false)]
    pub no_main: bool,
    #[merge(strategy = merge::bool::overwrite_false)]
    pub no_polish: bool,
    #[merge(strategy = merge::bool::overwrite_false)]
    pub skip_header: bool,
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
            id: id.to_string().into(),
            // files: file.to_owned().into(),
            to_zip,
            language: language.into(),
            rename_as,
            no_main: false,
            no_polish: false,
            skip_header: false,
        }
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn files(&self) -> Option<&Vec<PathBuf>> {
        self.files.as_ref()
    }

    pub fn try_set_files<P>(&mut self, filepaths: &Option<Vec<P>>) -> crate::error::Result<()>
    where
        P: Into<PathBuf> + Clone,
    {
        let files = match filepaths {
            None => None,
            Some(files) => {
                let l: Result<Vec<PathBuf>, _> = files
                    .iter()
                    .map(|file| {
                        let path: PathBuf = file.clone().into();
                        if !path.exists() {
                            //     return Err(Error::InputFileDoesNotExist);
                            return Err(std::io::Error::new(ErrorKind::NotFound, "asdas"));
                        }
                        path.canonicalize()
                    })
                    .collect();

                // let l = files.iter().map(|file| {
                //     let path: PathBuf = (*file.to_owned()).into();
                //
                //
                //     Ok(Some(path.canonicalize()?))
                // }).to_owned();
                Some(l)
            }
        };

        if files.is_none() {
            return Err(Error::InputFileDoesNotExist);
        }

        let files = files.unwrap();

        if files.is_err() {
            return Err(Error::InputFileDoesNotExist);
        }

        self.files = Some(files.unwrap());
        Ok(())
    }
}

impl ConfigObject for SubmitConfig {
    fn save_config<W: Workspace>(&self, workspace: &W) -> crate::error::Result<()> {
        workspace.save_config_object(self)
    }

    fn read_config<W: Workspace>(workspace: &W) -> crate::error::Result<Self> {
        workspace.read_config_object::<Self>()
    }

    fn remove_config<W: Workspace>(workspace: &W) -> crate::error::Result<()> {
        workspace.remove_config_object::<Self>()
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
    use std::str::FromStr;

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
            assert!(matches!(e, Error::ReadingConfig(_)), "error = {:?}", e);
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

    #[test]
    fn default_should_contain_none() {
        let default = SubmitConfig::default();

        assert!(default.file.is_none());
        assert!(default.language.is_none());
        assert!(default.id.is_none());
        assert!(default.rename_as.is_none());
        assert!(!default.to_zip);
        assert!(!default.no_main);
        assert!(!default.no_polish);
    }

    #[test]
    fn merge_both_none() {
        let mut lhs = SubmitConfig::default();
        let rhs = SubmitConfig::default();
        lhs.merge(rhs);
        let merged = lhs;

        assert!(merged.file.is_none());
        assert!(merged.language.is_none());
        assert!(merged.id.is_none());
        assert!(merged.rename_as.is_none());
        assert!(!merged.to_zip);
        assert!(!merged.no_main);
        assert!(!merged.no_polish);
    }

    fn make_submit_config() -> SubmitConfig {
        SubmitConfig {
            id: "3".to_string().into(),
            file: PathBuf::from("file.txt").into(),
            to_zip: true,
            language: Language::from_str("C++").unwrap().into(),
            rename_as: "source.cpp".to_string().into(),
            no_main: true,
            no_polish: true,
            skip_header: false,
        }
    }

    #[test]
    fn merge_left_full() {
        let mut lhs = make_submit_config();
        let rhs = SubmitConfig::default();

        lhs.merge(rhs);
        let merged = lhs;

        assert_eq!(merged.file.unwrap().to_str().unwrap(), "file.txt");
        assert_eq!(merged.language.unwrap(), Language::Cpp);
        assert_eq!(merged.id.unwrap(), "3");
        assert_eq!(merged.rename_as.unwrap(), "source.cpp");
        assert!(merged.to_zip);
        assert!(merged.no_main);
        assert!(merged.no_polish);
    }

    #[test]
    fn merge_right_full() {
        let mut lhs = make_submit_config();
        lhs.language = Language::Java.into();
        let mut rhs = make_submit_config();
        rhs.merge(lhs);
        let merged = rhs;

        assert_eq!(merged.file.unwrap().to_str().unwrap(), "file.txt");
        assert_eq!(merged.language.unwrap(), Language::Java);
        assert_eq!(merged.id.unwrap(), "3");
        assert_eq!(merged.rename_as.unwrap(), "source.cpp");
        assert!(merged.to_zip);
        assert!(merged.no_main);
        assert!(merged.no_polish);
    }
}
