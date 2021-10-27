use crate::api::baca_api::BacaApi;
use crate::command::log::Log;
use crate::command::Command;
use crate::error::{Error, Result};
use crate::workspace::config_editor::ConfigEditor;
use crate::workspace::{ConfigObject, ConnectionConfig, SubmitConfig, Workspace};
use crate::{error, workspace};
use clap::ArgMatches;
use colored::Colorize;
use dialoguer::Confirm;
use std::fs;
use std::path::PathBuf;
use tracing::info;

pub struct Submit<'a> {
    args: &'a ArgMatches<'a>,
}

impl<'a> From<&'a ArgMatches<'a>> for Submit<'a> {
    fn from(args: &'a ArgMatches) -> Self {
        Self { args }
    }
}

impl Command for Submit<'_> {
    fn execute<W, A>(self, workspace: &W, api: &A) -> Result<()>
    where
        W: Workspace,
        A: BacaApi,
    {
        if self.args.subcommand_matches("clear").is_some() {
            return SubmitConfig::remove_config(workspace);
        }

        let saved_submit_config = SubmitConfig::read_config(workspace);

        if self.args.subcommand_matches("config").is_some() {
            if saved_submit_config.is_err() {
                error!("{}", saved_submit_config.err().unwrap());
                println!("{}", "No saved submit config!".bright_red());
            } else {
                ConfigEditor::new().edit::<W, SubmitConfig>(workspace)?;
            }

            return Ok(());
        }

        let provided_task_id = self.args.value_of("task_id");
        let provided_file_path = self.args.value_of("file");
        let provided_to_zip = self.args.is_present("zip");
        let provided_lang = self.args.value_of("language");
        let provided_rename = self.args.value_of("rename");

        if provided_task_id.is_none() && saved_submit_config.is_err() {
            print_please_provide_monit("task_id");
            return Ok(());
        }

        if provided_file_path.is_none() && saved_submit_config.is_err() {
            print_please_provide_monit("file");
            return Ok(());
        }

        let allowed_language = if provided_lang.is_none() && saved_submit_config.is_err() {
            let connection_config = ConnectionConfig::read_config(workspace)?;
            let allowed_language = api.get_allowed_language(&connection_config, "1")?;
            info!("Allowed language: {:?}", allowed_language);

            if allowed_language.is_none() {
                return Err(Error::Submit);
            }

            allowed_language
        } else {
            None
        };

        let mut ask_for_save = saved_submit_config.is_err();
        let mut submit_config = saved_submit_config.unwrap_or_default();

        if let Some(id) = provided_task_id {
            submit_config.id = id.to_string();
            ask_for_save = true;
        }

        if let Some(file) = provided_file_path {
            submit_config.file = PathBuf::from(file).canonicalize()?;
            ask_for_save = true;
        }

        if let Some(lang) = provided_lang {
            submit_config.language = lang.parse()?;
            ask_for_save = true;
        } else if let Some(lang) = allowed_language {
            submit_config.language = lang;
            ask_for_save = true;
        }

        if let Some(new_name) = provided_rename {
            submit_config.rename_as = Some(new_name.to_string());
            ask_for_save = true;
        }

        submit_config.to_zip |= provided_to_zip;
        ask_for_save |= provided_to_zip;

        if self.args.is_present("save") {
            submit_config.save_config(workspace)?;
            println!("Submit config has been saved.");
        } else if !self.args.is_present("no_save") && ask_for_save {
            let proceed = Confirm::new()
                .with_prompt("Save submit configuration?")
                .default(true)
                .interact()?;

            if proceed {
                submit_config.save_config(workspace)?;
                println!("Submit config has been saved.");
            }
        }

        submit(workspace, api, submit_config)
    }
}

fn print_please_provide_monit(field: &str) {
    println!(
        "{}",
        format!(
            "Please provide {}. Type 'baca submit -h' for more info.",
            field
        )
        .bright_red()
    );
}

fn submit<W, A>(workspace: &W, api: &A, mut submit_config: SubmitConfig) -> error::Result<()>
where
    W: Workspace,
    A: BacaApi,
{
    let connection_config = ConnectionConfig::read_config(workspace)?;
    let tasks = api.get_tasks(&connection_config)?;
    let mut task = tasks.get_by_id(submit_config.id.as_str())?.clone();
    task.language = submit_config.language;

    let buf = submit_config.file.clone();
    let original_filename = buf.file_name().unwrap().to_str().unwrap();

    let rename = if let Some(new_name) = &submit_config.rename_as {
        if new_name == original_filename {
            original_filename.to_string()
        } else {
            let renamed = std::env::temp_dir().join(new_name);
            fs::copy(submit_config.file, &renamed)?;
            submit_config.file = renamed;

            format!(
                "{} as {}",
                &original_filename,
                &submit_config.file.file_name().unwrap().to_str().unwrap()
            )
        }
    } else {
        original_filename.to_string()
    };

    println!(
        "Submitting {} to task {} ({}).",
        rename.bright_yellow(),
        task.problem_name.bright_green(),
        task.language.to_string()
    );

    if submit_config.to_zip {
        submit_config.file = workspace::zip_file(submit_config.file.as_ref())?.to_path_buf();
        println!(
            "Zipped as {}",
            submit_config.file.file_name().unwrap().to_str().unwrap()
        );
    };

    api.submit(
        &connection_config,
        &task,
        submit_config.file.to_str().unwrap(),
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
}
