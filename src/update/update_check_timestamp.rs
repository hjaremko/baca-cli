use crate::workspace::{ConfigObject, Workspace};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use time::OffsetDateTime;
use tracing::debug;

const TIMESTAMP_FILENAME: &str = "update_timestamp";

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCheckTimestamp {
    timestamp: OffsetDateTime,
}

impl Default for UpdateCheckTimestamp {
    fn default() -> Self {
        Self {
            timestamp: OffsetDateTime::from_unix_timestamp(0).unwrap(),
        }
    }
}

impl From<i64> for UpdateCheckTimestamp {
    fn from(unix_timestamp: i64) -> Self {
        Self {
            timestamp: OffsetDateTime::from_unix_timestamp(unix_timestamp).unwrap(),
        }
    }
}

impl UpdateCheckTimestamp {
    pub fn now() -> Self {
        let timestamp = OffsetDateTime::now_utc();
        Self { timestamp }
    }

    pub fn is_expired(&self, other: &Self) -> bool {
        let saved = self.get_timestamp();
        let now = other.get_timestamp();
        let diff = now - saved;
        debug!("Expired for {} days", diff.whole_days());
        diff.whole_days() >= 1
    }

    pub fn get_timestamp(&self) -> OffsetDateTime {
        self.timestamp
    }
}

impl ConfigObject for UpdateCheckTimestamp {
    fn save_config<W: Workspace>(&self, workspace: &W) -> crate::error::Result<()> {
        workspace.save_config_object(self)
    }

    fn read_config<W: Workspace>(workspace: &W) -> crate::error::Result<Self> {
        Ok(workspace.read_config_object::<Self>().unwrap_or_default())
    }

    fn remove_config<W: Workspace>(_workspace: &W) -> crate::error::Result<()> {
        unimplemented!()
    }

    fn config_filename() -> String {
        TIMESTAMP_FILENAME.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;

    use crate::workspace::MockWorkspace;

    type Timestamp = UpdateCheckTimestamp;

    #[test]
    fn no_saved_info_should_return_true() {
        let mut mock_workspace = MockWorkspace::new();
        mock_workspace
            .expect_read_config_object::<Timestamp>()
            .returning(|| Err(Error::WorkspaceCorrupted));

        let now = UpdateCheckTimestamp::now();
        let saved = UpdateCheckTimestamp::read_config(&mock_workspace).unwrap();

        assert!(saved.is_expired(&now))
    }

    #[test]
    fn no_saved_info_get_timestamp_should_return_default() {
        let mut mock_workspace = MockWorkspace::new();
        mock_workspace
            .expect_read_config_object::<Timestamp>()
            .returning(|| Err(Error::WorkspaceCorrupted)); //todo

        let saved = UpdateCheckTimestamp::read_config(&mock_workspace).unwrap();
        assert_eq!(
            saved.get_timestamp().unix_timestamp(),
            UpdateCheckTimestamp::default()
                .get_timestamp()
                .unix_timestamp()
        )
    }

    #[test]
    fn save_timestamp_should_save() {
        let mut mock_workspace = MockWorkspace::new();
        mock_workspace
            .expect_save_config_object::<Timestamp>()
            .withf(|timestamp: &Timestamp| timestamp.get_timestamp().unix_timestamp() == 1625126400)
            .returning(|_| Ok(()));

        let udt = UpdateCheckTimestamp::from(1625126400);
        assert!(udt.save_config(&mock_workspace).is_ok());
    }

    #[test]
    fn read_timestamp_test() {
        let mut mock_workspace = MockWorkspace::new();
        mock_workspace
            .expect_read_config_object::<Timestamp>()
            .returning(|| Ok(1627068997.into()));

        let udt = UpdateCheckTimestamp::read_config(&mock_workspace).unwrap();
        assert_eq!(udt.get_timestamp().unix_timestamp(), 1627068997);
    }

    #[test]
    fn timestamp_newer_than_one_day_should_not_expire() {
        const TIME_2021_07_01_10_00_00: i64 = 1625126400;
        const TIME_2021_07_01_02_00_00: i64 = 1625097600;
        let newer = Timestamp::from(TIME_2021_07_01_02_00_00);
        let older = Timestamp::from(TIME_2021_07_01_10_00_00);

        assert!(!newer.is_expired(&older));
    }

    #[test]
    fn timestamp_older_than_one_day_should_expire() {
        const TIME_2021_07_01_10_00_00: i64 = 1625126400;
        const TIME_2021_06_01_02_00_00: i64 = 1622505600;
        let newer = Timestamp::from(TIME_2021_07_01_10_00_00);
        let older = Timestamp::from(TIME_2021_06_01_02_00_00);

        assert!(older.is_expired(&newer));
    }

    #[test]
    fn timestamp_equal_one_day_should_expire() {
        const TIME_2021_07_01_10_00_00: i64 = 1625126400;
        const TIME_2021_07_02_10_00_00: i64 = 1625212800;
        let newer = Timestamp::from(TIME_2021_07_01_10_00_00);
        let older = Timestamp::from(TIME_2021_07_02_10_00_00);

        assert!(newer.is_expired(&older));
    }
}
