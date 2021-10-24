use crate::api::baca_api::BacaApi;
use crate::command::details::Details;
use crate::command::init::Init;
use crate::command::last::Last;
use crate::command::log::Log;
use crate::command::refresh::Refresh;
use crate::command::submit::Submit;
use crate::command::tasks::Tasks;
use crate::error;
use crate::workspace::config_editor::ConfigEditor;
use crate::workspace::{ConnectionConfig, Workspace};
use clap::ArgMatches;

mod details;
mod init;
mod last;
mod log;
mod refresh;
mod submit;
mod tasks;

trait Command {
    fn execute<W: Workspace, A: BacaApi>(self, workspace: &W, api: &A) -> error::Result<()>;
}

pub fn execute<W, Api>(
    workspace: &W,
    api: &Api,
    command: &str,
    matches: &ArgMatches,
) -> error::Result<()>
where
    W: Workspace,
    Api: BacaApi,
{
    match command {
        "init" => Init::from(matches).execute(workspace, api),
        "details" => Details::from(matches).execute(workspace, api),
        "refresh" => Refresh::new().execute(workspace, api),
        "log" => Log::from(matches).execute(workspace, api),
        "tasks" => Tasks::new().execute(workspace, api),
        "submit" => Submit::from(matches).execute(workspace, api),
        "last" => Last::from(matches).execute(workspace, api),
        "config" => {
            ConfigEditor::new().edit::<W, ConnectionConfig>(workspace)?;
            Ok(())
        }
        _ => panic!("error!"),
    }
}
