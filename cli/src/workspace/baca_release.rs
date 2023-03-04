use api::error::Result;
use crate::update::BacaRelease;
use crate::workspace::{ConfigObject, Workspace};

impl ConfigObject for BacaRelease {
    fn save_config<W: Workspace>(&self, workspace: &W) -> Result<()> {
        workspace.save_config_object(self)
    }

    fn read_config<W: Workspace>(workspace: &W) -> Result<Self> {
        workspace.read_config_object::<Self>()
    }

    fn remove_config<W: Workspace>(workspace: &W) -> Result<()> {
        workspace.remove_config_object::<Self>()
    }

    fn config_filename() -> String {
        "version".to_string()
    }
}
