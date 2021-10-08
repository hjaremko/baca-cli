use crate::baca::api::baca_service::BacaApi;
use crate::baca::details::Language;
use crate::command::log::Log;
use crate::command::Command;
use crate::error::{Error, Result};
use crate::model::Tasks;
use crate::workspace::{TaskConfig, Workspace};
use crate::{error, workspace};
use clap::ArgMatches;
use colored::Colorize;
use std::path::Path;
use std::str::FromStr;

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

        let task_id = self.args.value_of("task_id");
        let file_path = self.args.value_of("file");
        let to_zip = self.args.is_present("zip");
        let lang = self.args.value_of("language");
        let saved = workspace.read_task();

        if let Some(lang) = lang {
            Language::from_str(lang)?;
        }

        if task_id.is_none() && saved.is_err() {
            println!(
                "{}",
                "Please provide task_id. Type 'baca submit -h' for more info.".bright_red()
            );
            return Ok(());
        }

        if file_path.is_none() && saved.is_err() {
            println!(
                "{}",
                "Please provide file. Type 'baca submit -h' for more info.".bright_red()
            );
            return Ok(());
        }

        if lang.is_none() && saved.is_err() {
            println!(
                "{}",
                "Please provide language. Type 'baca submit -h' for more info.".bright_red()
            );
            return Ok(());
        }

        // todo: default()
        let saved = saved.unwrap_or(TaskConfig {
            id: "".to_string(),
            file: "".to_string(),
            to_zip: false,
            language: Language::Unsupported,
        });

        let task_id = match task_id {
            None => saved.id.clone(),
            Some(id) => id.to_string(),
        };

        let file_path = match file_path {
            None => saved.file.clone(),
            Some(file) => file.to_string(),
        };

        let lang = match lang {
            None => saved.language,
            Some(lang) => Language::from_str(lang)?,
        };

        let to_zip = match to_zip {
            true => true,
            false => saved.to_zip,
        };

        if self.args.is_present("default") {
            workspace.save_task(&task_id, &file_path, to_zip, lang)?;
        }

        let file_to_submit = if to_zip {
            let path = Path::new(&file_path);
            workspace::zip_file(path).map_err(|e| Error::Zipping(e.into()))?
        } else {
            file_path
        };

        submit::<W, A>(workspace, &task_id, file_to_submit.as_str(), &lang)
    }
}

fn submit<W: Workspace, A: BacaApi>(
    workspace: &W,
    task_id: &str,
    file_path: &str,
    lang: &Language,
) -> error::Result<()> {
    let instance = workspace.read_instance()?;
    let tasks = A::get_tasks(&instance)?;
    let tasks = Tasks::parse(&tasks);
    let mut task = tasks
        .get_by_id(task_id)
        .ok_or_else(|| error::Error::InvalidTaskId(task_id.to_string()))?
        .clone();
    task.language = *lang;

    println!(
        "Submitting {} to task {} ({}).",
        file_path.bright_yellow(),
        task.problem_name.bright_green(),
        task.language.to_string()
    );

    A::submit(&instance, &task, file_path)?;
    println!();
    Log::new("1").execute::<W, A>(workspace)
}
