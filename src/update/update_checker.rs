use crate::error::{Error, Result};
use crate::update::update_status::UpdateStatus::{NoUpdates, Update};
use crate::update::{ReleaseService, UpdateStatus, CURRENT_VERSION};

use tracing::{debug, error, info};

pub struct UpdateChecker<T: ReleaseService> {
    release_service: T,
    current_version: String,
}

impl<T: ReleaseService> UpdateChecker<T> {
    pub fn new(release_service: T, current_version: &str) -> Self {
        Self {
            release_service,
            current_version: current_version.to_string(),
        }
    }

    pub fn check_for_updates(&self) -> Result<UpdateStatus> {
        info!("Checking for updates.");
        let last = self.release_service.get_last_release();
        debug!("Update check result: {:?}", last);

        if let Err(e) = last {
            return match e {
                Error::_NoRelease => Ok(NoUpdates),
                _ => {
                    error!("{}", e);
                    Err(e)
                }
            };
        }

        let last = last.unwrap();

        debug!("Current version: {}", CURRENT_VERSION);
        let res = if last.is_newer_than(&*self.current_version) {
            info!("New version: {}", last.version);
            Update(last)
        } else {
            info!("No updates available.");
            NoUpdates
        };
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::update::release_service::MockReleaseService;
    use crate::update::{BacaRelease, CURRENT_VERSION};

    #[test]
    fn connection_error_should_return_error() {
        let mut mock = MockReleaseService::new();
        mock.expect_get_last_release()
            .returning(|| Err(Error::_FetchingReleaseError));
        let checker = UpdateChecker::new(mock, CURRENT_VERSION);

        let actual = checker.check_for_updates();

        if let Some(Error::_FetchingReleaseError) = actual.err() {
            return;
        }
        panic!();
    }

    #[test]
    fn no_releases_should_not_report_update() {
        let mut mock = MockReleaseService::new();
        mock.expect_get_last_release()
            .returning(|| Err(Error::_NoRelease));
        let checker = UpdateChecker::new(mock, "v0.0.1");
        let actual = checker.check_for_updates().unwrap();

        assert_eq!(actual, UpdateStatus::NoUpdates);
    }

    #[test]
    fn release_up_to_date_should_not_report_update() {
        let mut mock = MockReleaseService::new();
        mock.expect_get_last_release()
            .returning(|| Ok(BacaRelease::new("v0.0.1", "link")));
        let checker = UpdateChecker::new(mock, "v0.0.1");
        let actual = checker.check_for_updates().unwrap();

        assert_eq!(actual, UpdateStatus::NoUpdates);
    }

    #[test]
    fn release_older_should_not_report_update() {
        let mut mock = MockReleaseService::new();
        mock.expect_get_last_release()
            .returning(|| Ok(BacaRelease::new("v0.0.1", "link")));
        let checker = UpdateChecker::new(mock, "v0.0.2");
        let actual = checker.check_for_updates().unwrap();

        assert_eq!(actual, UpdateStatus::NoUpdates);
    }

    #[test]
    fn release_newer_should_report_update() {
        let mut mock = MockReleaseService::new();
        mock.expect_get_last_release()
            .returning(|| Ok(BacaRelease::new("v0.0.2", "link")));
        let checker = UpdateChecker::new(mock, "v0.0.1");
        let actual = checker.check_for_updates().unwrap();

        if let UpdateStatus::Update(new_release) = actual {
            assert_eq!(new_release.version, "v0.0.2");
            return;
        }
        panic!();
    }
}
