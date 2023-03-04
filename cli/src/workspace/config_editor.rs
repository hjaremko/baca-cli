use api::error::Result;
use crate::workspace::{ConfigObject, Workspace};
use std::path::Path;
use std::process::ExitStatus;
use std::{env, ffi::OsString, fs, io, process};
use tracing::{debug, info};

#[derive(Debug, PartialEq, Eq)]
pub enum EditorStatus {
    Modified,
    NotModified,
}

pub struct ConfigEditor<E>
where
    E: EditorSpawner,
{
    editor: E,
}

impl ConfigEditor<Spawner> {
    pub fn new() -> Self {
        ConfigEditor {
            editor: Spawner::new(),
        }
    }
}

impl<E> ConfigEditor<E>
where
    E: EditorSpawner,
{
    pub fn edit<W, T>(&self, workspace: &W) -> Result<EditorStatus>
    where
        W: Workspace,
        T: ConfigObject,
    {
        workspace.check_if_initialized()?;

        let config_path = workspace.get_paths().config_path::<T>();
        let ts = fs::metadata(&config_path)?.modified()?;
        let rv = self.editor.spawn_and_wait(&config_path)?;

        if !rv.success() {
            return Err(api::error::Error::EditorFail(rv.code().unwrap_or(-1)));
        }

        if rv.success() && ts >= fs::metadata(&config_path)?.modified()? {
            info!("{} config not modified", T::config_filename());
            return Ok(EditorStatus::NotModified);
        }

        info!("{} modified and saved", T::config_filename());
        Ok(EditorStatus::Modified)
    }
}

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait EditorSpawner {
    fn default_editor() -> OsString;
    fn name(&self) -> &String;
    fn spawn_and_wait(&self, path: &Path) -> io::Result<ExitStatus>;
}

pub struct Spawner {
    name: String,
}

impl Spawner {
    pub fn new() -> Self {
        Self {
            name: Self::default_editor().into_string().unwrap(),
        }
    }
}

impl EditorSpawner for Spawner {
    fn default_editor() -> OsString {
        if let Some(prog) = env::var_os("VISUAL") {
            return prog;
        }
        if let Some(prog) = env::var_os("EDITOR") {
            return prog;
        }
        if cfg!(windows) {
            "notepad.exe".into()
        } else {
            "vi".into()
        }
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn spawn_and_wait(&self, path: &Path) -> io::Result<ExitStatus> {
        info!("Opening editor: {} for config file: {:?}", &self.name, path);

        let s = self.name.clone();
        let mut iterator = s.split(' ');
        let cmd = iterator.next().unwrap();
        let args: Vec<&str> = iterator.collect();

        debug!("{:?} {:?} {:?}", cmd, args, &path);

        process::Command::new(cmd)
            .args(args)
            .arg(&path)
            .spawn()?
            .wait()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::workspace_dir::tests::make_temp_workspace;
    use crate::workspace::{ConnectionConfig, MockWorkspace, WorkspacePaths};
    use std::fs::File;
    use std::io::prelude::*;
    #[cfg(unix)]
    use std::os::unix::process::ExitStatusExt;
    #[cfg(windows)]
    use std::os::windows::process::ExitStatusExt;

    #[test]
    fn file_not_modified_test() {
        let (temp_dir, _mock_paths, _workspace) = make_temp_workspace().unwrap();

        fs::create_dir(temp_dir.join(".baca")).unwrap();
        File::create(temp_dir.join(".baca/connection")).unwrap();

        let mut workspace_mock = MockWorkspace::new();
        workspace_mock
            .expect_check_if_initialized()
            .returning(|| Ok(()));
        workspace_mock
            .expect_get_paths()
            .returning(move || WorkspacePaths::_with_root(&temp_dir));
        let mut spawner_mock = MockEditorSpawner::new();
        spawner_mock
            .expect_spawn_and_wait()
            .returning(|_| Ok(ExitStatus::from_raw(0)));

        let ce = ConfigEditor {
            editor: spawner_mock,
        };
        let result = ce
            .edit::<MockWorkspace, ConnectionConfig>(&workspace_mock)
            .unwrap();
        assert_eq!(result, EditorStatus::NotModified);
    }

    #[test]
    fn file_modified_test() {
        let (temp_dir, _mock_paths, _workspace) = make_temp_workspace().unwrap();

        fs::create_dir(temp_dir.join(".baca")).unwrap();
        let config_path = temp_dir.join(".baca/connection");
        let mut config_mock = File::create(config_path).unwrap();

        let mut workspace_mock = MockWorkspace::new();
        workspace_mock
            .expect_check_if_initialized()
            .returning(|| Ok(()));
        workspace_mock
            .expect_get_paths()
            .returning(move || WorkspacePaths::_with_root(&temp_dir));
        let mut spawner_mock = MockEditorSpawner::new();
        spawner_mock.expect_spawn_and_wait().returning(move |_| {
            {
                use std::{thread, time};
                thread::sleep(time::Duration::from_secs(1));
                config_mock.write_all(b"Testing!").unwrap();
            }

            Ok(ExitStatus::from_raw(0))
        });

        let ce = ConfigEditor {
            editor: spawner_mock,
        };
        let result = ce
            .edit::<MockWorkspace, ConnectionConfig>(&workspace_mock)
            .unwrap();
        assert_eq!(result, EditorStatus::Modified);
    }

    #[test]
    fn os_error_test() {
        let (temp_dir, _mock_paths, _workspace) = make_temp_workspace().unwrap();

        fs::create_dir(temp_dir.join(".baca")).unwrap();
        let mut config_mock = File::create(temp_dir.join(".baca/connection")).unwrap();

        let mut workspace_mock = MockWorkspace::new();
        workspace_mock
            .expect_check_if_initialized()
            .returning(|| Ok(()));
        workspace_mock
            .expect_get_paths()
            .returning(move || WorkspacePaths::_with_root(&temp_dir));
        let mut spawner_mock = MockEditorSpawner::new();
        spawner_mock.expect_spawn_and_wait().returning(move |_| {
            config_mock.write_all(b"Test!").unwrap();
            Ok(ExitStatus::from_raw(1))
        });

        let ce = ConfigEditor {
            editor: spawner_mock,
        };
        let result = ce.edit::<MockWorkspace, ConnectionConfig>(&workspace_mock);
        assert!(result.is_err(), "{:?}", result);
    }
}
