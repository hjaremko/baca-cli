use crate::api::baca_api::BacaApi;
use crate::command::log::Log;
use crate::command::Command;
use crate::error::{Error, Result};
use crate::model::Language;
use crate::workspace::config_editor::ConfigEditor;
use crate::workspace::{ConfigObject, ConnectionConfig, SubmitConfig, Workspace};
use crate::{error, workspace};
use clap::ArgMatches;
use colored::Colorize;
use dialoguer::Confirm;
use merge::Merge;
use std::convert::TryFrom;
use std::fs;
use tracing::{debug, info};

enum SubmitSubcommand {
    None,
    Clear,
    Config,
}

impl<'a> From<&'a ArgMatches<'a>> for SubmitSubcommand {
    fn from(args: &'a ArgMatches) -> Self {
        if args.subcommand_matches("clear").is_some() {
            return Self::Clear;
        }

        if args.subcommand_matches("config").is_some() {
            return Self::Config;
        }

        Self::None
    }
}

enum SubmitSwitch {
    None,
    Save,
    NoSave,
}

impl<'a> From<&'a ArgMatches<'a>> for SubmitSwitch {
    fn from(args: &'a ArgMatches) -> Self {
        if args.is_present("save") {
            return Self::Save;
        }

        if args.is_present("no_save") {
            return Self::NoSave;
        }

        Self::None
    }
}

pub struct Submit {
    subcommand: SubmitSubcommand,
    switch: SubmitSwitch,
    provided_config: SubmitConfig,
}

impl<'a> TryFrom<&'a ArgMatches<'a>> for Submit {
    type Error = crate::error::Error;

    fn try_from(args: &'a ArgMatches<'a>) -> core::result::Result<Self, Self::Error> {
        Ok(Self {
            subcommand: args.into(),
            switch: args.into(),
            provided_config: SubmitConfig::try_from(args)?,
        })
    }
}

impl Command for Submit {
    fn execute<W, A>(self, workspace: &W, api: &A) -> Result<()>
    where
        W: Workspace,
        A: BacaApi,
    {
        match self.subcommand {
            SubmitSubcommand::Clear => Self::clear_impl(workspace),
            SubmitSubcommand::Config => Self::config_impl(workspace),
            SubmitSubcommand::None => {
                let (ask_for_save, submit_config) = self.prepare_submit_config(workspace, api)?;
                self.handle_config_save(workspace, ask_for_save, &submit_config)?;
                submit(workspace, api, submit_config)
            }
        }
    }
}

impl Submit {
    fn clear_impl<W>(workspace: &W) -> Result<()>
    where
        W: Workspace,
    {
        SubmitConfig::remove_config(workspace)
    }

    fn config_impl<W>(workspace: &W) -> Result<()>
    where
        W: Workspace,
    {
        let saved_submit_config = SubmitConfig::read_config(workspace);

        if saved_submit_config.is_err() {
            error!("{}", saved_submit_config.err().unwrap());
            println!("{}", "No saved submit config!".bright_red());
        } else {
            ConfigEditor::new().edit::<W, SubmitConfig>(workspace)?;
        }

        Ok(())
    }

    fn prepare_submit_config<W, A>(&self, workspace: &W, api: &A) -> Result<(bool, SubmitConfig)>
    where
        W: Workspace,
        A: BacaApi,
    {
        let (ask_for_save, mut submit_config) = self.merge_saved_and_provided_configs(workspace)?;

        if submit_config.id.is_none() {
            return Err(Error::SubmitArgumentNotProvided("task_id".to_string()));
        }

        if submit_config.file().is_none() {
            return Err(Error::SubmitArgumentNotProvided("file".to_string()));
        }

        if submit_config.language.is_none() {
            let allowed_language =
                Self::fetch_allowed_language(workspace, api, submit_config.id().unwrap())?;

            if allowed_language.is_none() {
                return Err(Error::TaskNotActive);
            } else {
                submit_config.language = allowed_language;
            }
        }

        Ok((ask_for_save, submit_config))
    }

    fn merge_saved_and_provided_configs<W>(&self, workspace: &W) -> Result<(bool, SubmitConfig)>
    where
        W: Workspace,
    {
        let saved_submit_config = SubmitConfig::read_config(workspace);
        debug!("Saved config: {:?}", saved_submit_config);
        debug!("Provided config: {:?}", self.provided_config);

        let was_not_saved = saved_submit_config.is_err();
        let ask_for_save = was_not_saved || self.provided_config != SubmitConfig::default();

        let mut submit_config = saved_submit_config.unwrap_or_default();
        submit_config.merge(self.provided_config.clone());
        debug!("Merged config: {:?}", submit_config);

        Ok((ask_for_save, submit_config))
    }

    fn fetch_allowed_language<W, A>(
        workspace: &W,
        api: &A,
        task_id: &str,
    ) -> Result<Option<Language>>
    where
        W: Workspace,
        A: BacaApi,
    {
        let connection_config = ConnectionConfig::read_config(workspace)?;
        let allowed_language = api.get_allowed_language(&connection_config, task_id)?;

        info!("Allowed language: {:?}", allowed_language);
        Ok(allowed_language)
    }

    fn prompt_for_save<W: Workspace>(workspace: &W, submit_config: &SubmitConfig) -> Result<()> {
        let proceed = Confirm::new()
            .with_prompt("Save submit configuration?")
            .default(true)
            .interact()?;

        if proceed {
            submit_config.save_config(workspace)?;
            println!("Submit config has been saved.");
        }

        Ok(())
    }

    fn handle_config_save<W: Workspace>(
        &self,
        workspace: &W,
        ask_for_save: bool,
        submit_config: &SubmitConfig,
    ) -> Result<()> {
        match self.switch {
            SubmitSwitch::None => {
                info!("Ask for save? {}", ask_for_save);
                if ask_for_save {
                    Submit::prompt_for_save(workspace, submit_config)?;
                }
            }
            SubmitSwitch::Save => {
                info!("Forcing submit config save");
                submit_config.save_config(workspace)?;
                println!("Submit config has been saved.");
            }
            SubmitSwitch::NoSave => {
                info!("Save prompt disabled");
            }
        }
        Ok(())
    }
}

fn submit<W, A>(workspace: &W, api: &A, mut submit_config: SubmitConfig) -> error::Result<()>
where
    W: Workspace,
    A: BacaApi,
{
    let connection_config = ConnectionConfig::read_config(workspace)?;
    let tasks = api.get_tasks(&connection_config)?;
    let task_id = submit_config.id().unwrap();
    let mut task = tasks.get_by_id(task_id)?.clone();
    task.language = submit_config.language.unwrap();

    let original_filename = submit_config
        .file()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let rename = if let Some(new_name) = &submit_config.rename_as {
        if new_name == &original_filename {
            original_filename
        } else {
            let renamed = std::env::temp_dir().join(new_name);
            fs::copy(submit_config.file().unwrap(), &renamed)?;
            submit_config.try_set_file(renamed.into())?;

            format!(
                "{} as {}",
                &original_filename,
                &submit_config
                    .file()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
            )
        }
    } else {
        original_filename
    };

    println!(
        "Submitting {} to task {} ({}).",
        rename.bright_yellow(),
        task.problem_name.bright_green(),
        task.language.to_string()
    );

    if submit_config.to_zip {
        submit_config.try_set_file(
            workspace::zip_file(submit_config.file().unwrap())?
                .to_path_buf()
                .into(),
        )?;
        println!(
            "Zipped as {}",
            submit_config
                .file()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
        );
    };

    api.submit(
        &connection_config,
        &task,
        submit_config.file().unwrap().to_str().unwrap(),
    )?;
    println!();
    Log::new("1").execute(workspace, api)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::baca_api::MockBacaApi;
    use crate::model::Language::Unsupported;
    use crate::model::{Language, Results, Task, Tasks};
    use crate::workspace::{ConnectionConfig, MockWorkspace};
    use assert_fs::fixture::ChildPath;
    use assert_fs::prelude::*;
    use assert_fs::TempDir;
    use mockall::predicate::{always, eq};
    use std::fs;

    pub fn make_input_file_cpp(dir: &TempDir) -> ChildPath {
        let input_file = dir.child("source.cpp");
        input_file.touch().unwrap();
        input_file
            .write_str(
                r#"
        \\ Hubert Jaremko
        #include <iostream>
        int main() {
            std::cout << "Hello world" << std::endl;
            return 0;
        }
        "#,
            )
            .unwrap();
        input_file
    }

    #[test]
    fn renamed_file_should_be_identical_to_original() {
        let mut mock_workspace = MockWorkspace::new();
        mock_workspace
            .expect_read_config_object()
            .returning(|| Ok(ConnectionConfig::default()));

        let mut mock_api = MockBacaApi::new();
        mock_api
            .expect_get_results()
            .returning(|_| Ok(Results { submits: vec![] }));

        mock_api
            .expect_get_tasks()
            .withf(|x| *x == ConnectionConfig::default())
            .returning(|_| {
                Ok(Tasks::new(vec![
                    Task::new("1", Unsupported, "Metoda parametryzacji", 12),
                    Task::new("2", Unsupported, "Metoda parametryzacji torusów", 4),
                ]))
            });

        let dir = assert_fs::TempDir::new().unwrap();
        let original_input = make_input_file_cpp(&dir);

        let submit_config = SubmitConfig::new(
            "1",
            original_input.path(),
            false,
            Language::Unsupported,
            Some("new_name.c".to_string()),
        );

        mock_api.expect_submit().returning(move |_, _, file| {
            let submitted_contents = fs::read_to_string(file).unwrap();
            let original_contents = fs::read_to_string(original_input.path()).unwrap();
            assert_eq!(submitted_contents, original_contents);
            Ok(())
        });

        submit(&mock_workspace, &mock_api, submit_config).unwrap();
    }

    // todo: test if renamed is zipped

    #[test]
    fn fetch_allowed_language_test_success() {
        let mut mock_workspace = MockWorkspace::new();
        mock_workspace
            .expect_read_config_object()
            .returning(|| Ok(ConnectionConfig::default()));

        let mut mock_api = MockBacaApi::new();
        mock_api
            .expect_get_allowed_language()
            .with(always(), eq("2"))
            .returning(|_, _| Ok(Some(Language::Java)));

        let actual = Submit::fetch_allowed_language(&mock_workspace, &mock_api, "2").unwrap();
        let expected = Some(Language::Java);
        assert_eq!(actual, expected);
    }

    #[test]
    fn fetch_allowed_language_test_none() {
        let mut mock_workspace = MockWorkspace::new();
        mock_workspace
            .expect_read_config_object()
            .returning(|| Ok(ConnectionConfig::default()));

        let mut mock_api = MockBacaApi::new();
        mock_api
            .expect_get_allowed_language()
            .with(always(), eq("1"))
            .returning(|_, _| Ok(None));

        let actual = Submit::fetch_allowed_language(&mock_workspace, &mock_api, "1").unwrap();
        let expected = None;
        assert_eq!(actual, expected);
    }

    #[test]
    fn fetch_allowed_language_test_error() {
        let mut mock_workspace = MockWorkspace::new();
        mock_workspace
            .expect_read_config_object()
            .returning(|| Ok(ConnectionConfig::default()));

        let mut mock_api = MockBacaApi::new();
        mock_api
            .expect_get_allowed_language()
            .with(always(), eq("3"))
            .returning(|_, _| Err(Error::ApiRateLimitExceeded));

        let actual = Submit::fetch_allowed_language(&mock_workspace, &mock_api, "3");
        assert!(actual.is_err());
    }
}
