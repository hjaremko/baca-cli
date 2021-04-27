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
    fn execute<W: Workspace, A: BacaApi>(self) -> error::Result<()>;
}

pub fn execute<T: Workspace, U: BacaApi>(command: &str, matches: &ArgMatches) -> error::Result<()> {
    match command {
        "init" => Init::from(matches).execute::<T, U>(),
        "details" => Details::from(matches).execute::<T, U>(),
        "refresh" => Refresh::new().execute::<T, U>(),
        "log" => Log::from(matches).execute::<T, U>(),
        "tasks" => tasks::Tasks::new().execute::<T, U>(),
        "submit" => submit::Submit::from(matches).execute::<T, U>(),
        _ => {
            panic!("error!")
        }
    }
}
