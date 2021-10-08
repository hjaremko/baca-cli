use crate::baca::api::baca_service::BacaApi;
use crate::command::details::Details;
use crate::command::init::Init;
use crate::command::log::Log;
use crate::command::refresh::Refresh;
use crate::error;
use crate::workspace::Workspace;
use clap::ArgMatches;

mod details;
mod init;
mod log;
mod refresh;
mod submit;
mod tasks;

trait Command {
    fn execute<W: Workspace, A: BacaApi>(self, workspace: &W) -> error::Result<()>;
}

pub fn execute<W, Api>(workspace: &W, command: &str, matches: &ArgMatches) -> error::Result<()>
where
    W: Workspace,
    Api: BacaApi,
{
    match command {
        "init" => Init::from(matches).execute::<W, Api>(workspace),
        "details" => Details::from(matches).execute::<W, Api>(workspace),
        "refresh" => Refresh::new().execute::<W, Api>(workspace),
        "log" => Log::from(matches).execute::<W, Api>(workspace),
        "tasks" => tasks::Tasks::new().execute::<W, Api>(workspace),
        "submit" => submit::Submit::from(matches).execute::<W, Api>(workspace),
        _ => {
            panic!("error!")
        }
    }
}
