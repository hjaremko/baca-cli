use crate::api;
use crate::error::Error;
use crate::error::Result;
use crate::workspace::{ConfigObject, Workspace};
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct ConnectionConfig {
    pub host: String,
    pub login: String,
    pub password: String,
    pub permutation: String,
    pub cookie: String,
}

impl ConnectionConfig {
    pub fn credentials(&self) -> (String, String) {
        (self.login.clone(), self.password.clone())
    }

    pub fn make_url(&self) -> String {
        format!("https://{}/{}", api::details::SERVER_URL, self.host)
    }

    pub fn make_module_base(&self) -> String {
        format!("{}/testerka_gwt/", self.make_url())
    }

    pub fn make_payload(&self, req_type: &api::RequestType) -> String {
        use dyn_fmt::AsStrFormatExt;

        req_type.payload_template().format(&[
            self.make_module_base(),
            self.login.clone(),
            self.password.clone(),
        ])
    }

    pub fn make_cookie(&self) -> String {
        format!("JSESSIONID={};", self.cookie)
    }
}

impl ConfigObject for ConnectionConfig {
    fn save_config<W: Workspace>(&self, workspace: &W) -> Result<()> {
        workspace.save_config_object(self).map_err(|e| {
            error!("{:?}", e);
            match e {
                Error::WorkspaceNotInitialized => e,
                _ => Error::WorkspaceCorrupted,
            }
        })
    }

    fn read_config<W: Workspace>(workspace: &W) -> Result<Self> {
        workspace.read_config_object::<Self>().map_err(|e| {
            error!("{:?}", e);
            match e {
                Error::WorkspaceNotInitialized => e,
                _ => Error::WorkspaceCorrupted,
            }
        })
    }

    fn remove_config<W: Workspace>(workspace: &W) -> Result<()> {
        workspace.remove_config_object::<Self>().map_err(|e| {
            error!("{:?}", e);
            match e {
                Error::WorkspaceNotInitialized => e,
                _ => Error::WorkspaceCorrupted,
            }
        })
    }

    fn config_filename() -> String {
        "connection".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::workspace_dir::tests::{make_baca, make_temp_workspace};
    use crate::workspace::{ConfigObject, ConnectionConfig};
    use predicates::prelude::*;

    #[test]
    fn save_read_success() {
        let (temp_dir, _, workspace) = make_temp_workspace().unwrap();
        let baca = make_baca();

        workspace.initialize().unwrap();
        baca.save_config(&workspace).unwrap();

        let result = ConnectionConfig::read_config(&workspace).unwrap();
        assert_eq!(result, baca);
        temp_dir.close().unwrap();
    }

    #[test]
    fn save_not_initialized() {
        let (temp_dir, mock_paths, workspace) = make_temp_workspace().unwrap();

        let baca = make_baca();
        let result = baca.save_config(&workspace);

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, Error::WorkspaceNotInitialized));
        }
        assert!(
            predicate::path::missing().eval(mock_paths.config_path::<ConnectionConfig>().as_path())
        );
        temp_dir.close().unwrap();
    }

    #[test]
    fn save_should_override() {
        let (temp_dir, _, workspace) = make_temp_workspace().unwrap();
        let baca_first = make_baca();
        let mut baca_second = make_baca();
        baca_second.host = "other_host".to_string();

        workspace.initialize().unwrap();
        baca_first.save_config(&workspace).unwrap();
        assert_eq!(
            ConnectionConfig::read_config(&workspace).unwrap(),
            baca_first
        );
        baca_second.save_config(&workspace).unwrap();
        assert_eq!(
            ConnectionConfig::read_config(&workspace).unwrap(),
            baca_second
        );

        temp_dir.close().unwrap();
    }
    // todo: tests for removing and saving objects
}
