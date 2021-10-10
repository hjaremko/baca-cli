use crate::baca::api::baca_service::BacaApi;

use crate::command::log::Log;
use crate::command::Command;
use crate::error::Result;
use crate::model::Tasks;
use crate::workspace::{TaskConfig, Workspace};
use crate::{error, workspace};
use clap::ArgMatches;
use colored::Colorize;
use std::fs;

use std::path::PathBuf;

pub struct Submit<'a> {
    args: &'a ArgMatches<'a>,
}

impl<'a> From<&'a ArgMatches<'a>> for Submit<'a> {
    fn from(args: &'a ArgMatches) -> Self {
        Self { args }
    }
}

impl Command for Submit<'_> {
    fn execute<W: Workspace, A: BacaApi>(self, workspace: &W) -> Result<()> {
        if self.args.subcommand_matches("clear").is_some() {
            return workspace.remove_task();
        }

        let provided_task_id = self.args.value_of("task_id");
        let provided_file_path = self.args.value_of("file");
        let provided_to_zip = self.args.is_present("zip");
        let provided_lang = self.args.value_of("language");
        let provided_rename = self.args.value_of("rename");
        let saved_task_config = workspace.read_task();

        if provided_task_id.is_none() && saved_task_config.is_err() {
            print_please_provide_monit("task_id");
            return Ok(());
        }

        if provided_file_path.is_none() && saved_task_config.is_err() {
            print_please_provide_monit("file");
            return Ok(());
        }

        if provided_lang.is_none() && saved_task_config.is_err() {
            print_please_provide_monit("language");
            return Ok(());
        }

        let mut task_config = saved_task_config.unwrap_or_default();

        if let Some(id) = provided_task_id {
            task_config.id = id.to_string();
        }

        if let Some(file) = provided_file_path {
            task_config.file = PathBuf::from(file).canonicalize()?;
        }

        if let Some(lang) = provided_lang {
            task_config.language = lang.parse()?;
        }

        if let Some(new_name) = provided_rename {
            task_config.rename_as = Some(new_name.to_string());
        }

        task_config.to_zip |= provided_to_zip;

        if self.args.is_present("default") {
            workspace.save_task(&task_config)?;
        }

        submit::<W, A>(workspace, task_config)
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

fn submit<W: Workspace, A: BacaApi>(
    workspace: &W,
    mut task_config: TaskConfig,
) -> error::Result<()> {
    let instance = workspace.read_instance()?;
    let tasks = A::get_tasks(&instance)?;
    let tasks = Tasks::parse(&tasks); // todo: no tasks yet
    let mut task = tasks.get_by_id(task_config.id.as_str())?.clone();
    task.language = task_config.language;

    let buf = task_config.file.clone();
    let original_filename = buf.file_name().unwrap().to_str().unwrap();

    let rename = if let Some(new_name) = &task_config.rename_as {
        if new_name == original_filename {
            original_filename.to_string()
        } else {
            let renamed = std::env::temp_dir().join(new_name);
            fs::copy(task_config.file, &renamed)?;
            task_config.file = renamed;

            format!(
                "{} as {}",
                &original_filename,
                &task_config.file.file_name().unwrap().to_str().unwrap()
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

    if task_config.to_zip {
        task_config.file = workspace::zip_file(task_config.file.as_ref())?.to_path_buf();
        println!(
            "Zipped as {}",
            task_config.file.file_name().unwrap().to_str().unwrap()
        );
    };

    A::submit(&instance, &task, task_config.file.to_str().unwrap())?;
    println!();
    Log::new("1").execute::<W, A>(workspace)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baca::api::baca_service::MockBacaApi;
    use crate::baca::details::{Language, EMPTY_RESPONSE};
    use crate::workspace::{InstanceData, MockWorkspace};
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
    #[serial]
    fn renamed_file_should_be_identical_to_original() {
        let mut mock_workspace = MockWorkspace::new();
        mock_workspace
            .expect_read_instance()
            .returning(|| Ok(InstanceData::default()));

        let ctx_results = MockBacaApi::get_results_context();
        ctx_results
            .expect()
            .returning(|_| Ok(EMPTY_RESPONSE.to_string()));

        let ctx_tasks = MockBacaApi::get_tasks_context();
        ctx_tasks
            .expect()
            .withf(|x| *x == InstanceData::default())
            .returning(|_| Ok(r#"//OK[0,12,11,10,3,3,9,8,7,3,3,6,5,4,3,3,2,2,1,["testerka.gwt.client.tools.DataSource/1474249525","[[Ljava.lang.String;/4182515373","[Ljava.lang.String;/2600011424","1","Metoda parametryzacji","12","2","Metoda parametryzacji torusów","4","id","nazwa","liczba OK"],0,7]"#.to_string()));

        let dir = assert_fs::TempDir::new().unwrap();
        let original_input = make_input_file_cpp(&dir);

        let task_config = TaskConfig::new(
            "1",
            original_input.path(),
            false,
            Language::Unsupported,
            Some("new_name.c".to_string()),
        );

        let ctx_submit = MockBacaApi::submit_context();
        ctx_submit.expect().returning(move |_, _, file| {
            let submitted_contents = fs::read_to_string(file).unwrap();
            let original_contents = fs::read_to_string(original_input.path()).unwrap();
            assert_eq!(submitted_contents, original_contents);
            Ok(())
        });

        submit::<MockWorkspace, MockBacaApi>(&mock_workspace, task_config).unwrap();
    }

    // todo: test if renamed is zipped
}
