use time::OffsetDateTime;
use tracing::debug;

use crate::update::time_provider::{TimeProvider, UnixTimeProvider};
use crate::workspace::Workspace;

const TIMESTAMP_FILENAME: &str = "update_timestamp";

pub struct UpdateCheckTimestamp {
    clock: Box<dyn TimeProvider>,
}

impl UpdateCheckTimestamp {
    pub fn new() -> Self {
        UpdateCheckTimestamp {
            clock: Box::new(UnixTimeProvider::new()),
        }
    }

    pub fn is_expired<W: Workspace>(&self) -> bool {
        if let Some(saved) = self.get_timestamp::<W>() {
            let now = self.clock.now();
            let diff = now - saved;
            debug!("Expired for {} days", diff.whole_days());
            return diff.whole_days() >= 1;
        }

        true
    }

    pub fn get_timestamp<W: Workspace>(&self) -> Option<OffsetDateTime> {
        let timestamp = W::read_file(TIMESTAMP_FILENAME);

        if timestamp.is_err() {
            return None;
        }

        let timestamp = timestamp.unwrap();

        Some(self.clock.datetime_from_timestamp(&timestamp))
    }

    pub fn save_current_timestamp<W: Workspace>(&self) -> crate::error::Result<()> {
        let timestamp = self.clock.now().unix_timestamp();
        W::save_object(TIMESTAMP_FILENAME, &timestamp)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error;
    use crate::update::time_provider::MockTimeProvider;
    use crate::workspace::MockWorkspace;

    use super::*;

    fn new_with_provider(clock: Box<dyn TimeProvider>) -> UpdateCheckTimestamp {
        UpdateCheckTimestamp { clock }
    }

    #[test]
    #[serial]
    fn no_saved_info_should_return_true() {
        let udt = UpdateCheckTimestamp::new();
        let ctx_read = MockWorkspace::read_file_context();
        ctx_read
            .expect()
            .returning(|_| Err(Error::WorkspaceCorrupted));

        assert!(udt.is_expired::<MockWorkspace>())
    }

    #[test]
    #[serial]
    fn no_saved_info_get_timestamp_should_return_none() {
        let ctx_read = MockWorkspace::read_file_context();
        ctx_read
            .expect()
            .returning(|_| Err(Error::WorkspaceCorrupted)); //todo

        let udt = UpdateCheckTimestamp::new();
        assert!(udt.get_timestamp::<MockWorkspace>().is_none())
    }

    #[test]
    #[serial]
    fn save_timestamp_should_save() {
        let mut mock_time = MockTimeProvider::new();
        mock_time
            .expect_now()
            .returning(|| OffsetDateTime::from_unix_timestamp(1625126400).unwrap());

        let ctx_save = MockWorkspace::save_object_context();
        ctx_save
            .expect()
            .withf(|filename, timestamp: &i64| {
                filename == TIMESTAMP_FILENAME && *timestamp == 1625126400
            })
            .returning(|_, _: &i64| Ok(()));

        let udt = new_with_provider(Box::new(mock_time));
        assert!(udt.save_current_timestamp::<MockWorkspace>().is_ok());
    }

    #[test]
    #[serial]
    fn read_timestamp_test() {
        let ctx_read = MockWorkspace::read_file_context();
        ctx_read
            .expect()
            .withf(|filename| filename == TIMESTAMP_FILENAME)
            .returning(|_| Ok("1627068997".to_string()));

        let udt = UpdateCheckTimestamp::new();
        udt.get_timestamp::<MockWorkspace>();
    }

    #[test]
    #[serial]
    fn timestamp_newer_than_one_day_should_not_expire() {
        const TIME_2021_07_01_10_00_00: i64 = 1625126400;
        const TIME_2021_07_01_02_00_00: i64 = 1625097600;

        let mut mock_time = MockTimeProvider::new();
        mock_time
            .expect_now()
            .returning(|| OffsetDateTime::from_unix_timestamp(TIME_2021_07_01_10_00_00).unwrap());
        mock_time
            .expect_datetime_from_timestamp()
            .returning(|t| OffsetDateTime::from_unix_timestamp(t.parse().unwrap()).unwrap());

        let ctx_read = MockWorkspace::read_file_context();
        ctx_read
            .expect()
            .returning(|_| Ok(TIME_2021_07_01_02_00_00.to_string()));

        let udt = new_with_provider(Box::new(mock_time));
        assert!(!udt.is_expired::<MockWorkspace>());
    }

    #[test]
    #[serial]
    fn timestamp_older_than_one_day_should_expire() {
        const TIME_2021_07_01_10_00_00: i64 = 1625126400;
        const TIME_2021_06_01_02_00_00: i64 = 1622505600;

        let mut mock_time = MockTimeProvider::new();
        mock_time
            .expect_now()
            .returning(|| OffsetDateTime::from_unix_timestamp(TIME_2021_07_01_10_00_00).unwrap());
        mock_time
            .expect_datetime_from_timestamp()
            .returning(|t| OffsetDateTime::from_unix_timestamp(t.parse().unwrap()).unwrap());

        let ctx_read = MockWorkspace::read_file_context();
        ctx_read
            .expect()
            .returning(|_| Ok(TIME_2021_06_01_02_00_00.to_string()));

        let udt = new_with_provider(Box::new(mock_time));
        assert!(udt.is_expired::<MockWorkspace>());
    }

    #[test]
    #[serial]
    fn timestamp_equal_one_day_should_expire() {
        const TIME_2021_07_01_10_00_00: i64 = 1625126400;
        const TIME_2021_07_02_10_00_00: i64 = 1625212800;

        let mut mock_time = MockTimeProvider::new();
        mock_time
            .expect_now()
            .returning(|| OffsetDateTime::from_unix_timestamp(TIME_2021_07_02_10_00_00).unwrap());
        mock_time
            .expect_datetime_from_timestamp()
            .returning(|t| OffsetDateTime::from_unix_timestamp(t.parse().unwrap()).unwrap());

        let ctx_read = MockWorkspace::read_file_context();
        ctx_read
            .expect()
            .returning(|_| Ok(TIME_2021_07_01_10_00_00.to_string()));

        let udt = new_with_provider(Box::new(mock_time));
        assert!(udt.is_expired::<MockWorkspace>());
    }
}
