use crate::api::baca_api::BacaApi;
use crate::cli::Commands;
use crate::command::details::Details;
use crate::command::init::Init;
use crate::command::last::Last;
use crate::command::log::Log;
use crate::command::refresh::Refresh;
use crate::command::submit::{SaveSwitch, Submit, SubmitSubcommand};
use crate::command::tasks::Tasks;
use crate::error;
use crate::workspace::config_editor::ConfigEditor;
use crate::workspace::{ConnectionConfig, SubmitConfig, Workspace};

mod details;
mod init;
mod last;
mod log;
mod prompt;
mod refresh;
mod submit;
mod tasks;

pub trait Command {
    fn execute<W: Workspace, A: BacaApi>(self, workspace: &W, api: &A) -> error::Result<()>;
}

pub(crate) fn execute<W, Api>(workspace: &W, api: &Api, commands: &Commands) -> error::Result<()>
where
    W: Workspace,
    Api: BacaApi,
{
    match commands {
        Commands::Init {
            host,
            login,
            password,
        } => Init::new(host.clone(), login.clone(), password.clone()).execute(workspace, api),
        Commands::Details { submit_id } => {
            Details::new(&submit_id.to_string()).execute(workspace, api)
        }
        Commands::Refresh {} => Refresh::new().execute(workspace, api),
        Commands::Log { amount, task } => {
            let log = Log::new(&amount.to_string(), task);
            log.execute(workspace, api)
        }
        Commands::Tasks {} => Tasks::new().execute(workspace, api),
        Commands::Submit {
            task,
            file,
            language,
            rename,
            save,
            zip,
            no_save,
            no_main,
            no_polish,
            skip_header,
            command,
        } => {
            let subcommand = SubmitSubcommand::from(command);
            let save_switch = SaveSwitch::new(*save, *no_save);
            let mut provided_config = SubmitConfig {
                file: None,
                language: match language {
                    None => None,
                    Some(lang_str) => Some(lang_str.parse()?),
                },
                id: task.map(|x| x.to_string()),
                rename_as: rename.clone(),
                to_zip: *zip,
                no_main: *no_main,
                no_polish: *no_polish,
                skip_header: *skip_header,
            };
            provided_config.try_set_file(file.as_ref())?;

            Submit {
                subcommand,
                save_switch,
                provided_config,
            }
            .execute(workspace, api)
        }
        Commands::Last { task } => {
            let task = if let Some(task_id) = task {
                Last::with_filter(task_id.to_string())
            } else {
                Last::new()
            };
            task.execute(workspace, api)
        }
        Commands::Config {} => {
            ConfigEditor::new().edit::<W, ConnectionConfig>(workspace)?;
            Ok(())
        }
        Commands::Clear {} => workspace.remove_workspace(),
    }
}
