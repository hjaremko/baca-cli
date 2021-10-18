use crate::error::Result;
use crate::workspace::Workspace;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub trait ConfigObject: Serialize + DeserializeOwned + Debug + Sized {
    fn save_config<W: Workspace>(&self, workspace: &W) -> Result<()>;
    fn read_config<W: Workspace>(workspace: &W) -> Result<Self>;
    fn remove_config<W: Workspace>(workspace: &W) -> Result<()>;
    fn config_filename() -> String;
}
