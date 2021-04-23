use crate::error::{Error, Result};
use crate::update::update_status::UpdateStatus::{_NoUpdates, _Update};
use crate::update::{ReleaseService, UpdateStatus};

use tracing::{debug, error, info};

pub struct _UpdateChecker<T: ReleaseService> {
    release_service: T,
    current_version: String,
}

impl<T: ReleaseService> _UpdateChecker<T> {
    pub fn _new(release_service: T, current_version: &str) -> Self {
        Self {
            release_service,
            current_version: current_version.to_string(),
        }
    }

    pub fn _check_for_updates(&self) -> Result<UpdateStatus> {
        info!("Checking for updates.");
        let last = self.release_service.get_last_release();
        debug!("Update check result: {:?}", last);

        if let Err(e) = last {
            return match e {
                Error::_NoRelease => Ok(_NoUpdates),
                _ => {
                    error!("{}", e);
                    Err(e)
                }
            };
        }

        let last = last.unwrap();

        let res = if last._is_newer_than(&*self.current_version) {
            info!("New version: {}", last.version);
            _Update(last)
        } else {
            info!("No updates available.");
            _NoUpdates
        };
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::update::release_service::MockReleaseService;
    use crate::update::{BacaRelease, _CURRENT_VERSION};

    #[test]
    fn connection_error_should_return_error() {
        let mut mock = MockReleaseService::new();
        mock.expect_get_last_release()
            .returning(|| Err(Error::_FetchingReleaseError));
        let checker = _UpdateChecker::_new(mock, _CURRENT_VERSION);

        let actual = checker._check_for_updates();

        if let Some(Error::_FetchingReleaseError) = actual.err() {
            return;
        }
        assert!(false);
    }

    #[test]
    fn no_releases_should_not_report_update() {
        let mut mock = MockReleaseService::new();
        mock.expect_get_last_release()
            .returning(|| Err(Error::_NoRelease));
        let checker = _UpdateChecker::_new(mock, "v0.0.1");
        let actual = checker._check_for_updates().unwrap();

        assert_eq!(actual, UpdateStatus::_NoUpdates);
    }

    #[test]
    fn release_up_to_date_should_not_report_update() {
        let mut mock = MockReleaseService::new();
        mock.expect_get_last_release()
            .returning(|| Ok(BacaRelease::_new("v0.0.1", "link")));
        let checker = _UpdateChecker::_new(mock, "v0.0.1");
        let actual = checker._check_for_updates().unwrap();

        assert_eq!(actual, UpdateStatus::_NoUpdates);
    }

    #[test]
    fn release_older_should_not_report_update() {
        let mut mock = MockReleaseService::new();
        mock.expect_get_last_release()
            .returning(|| Ok(BacaRelease::_new("v0.0.1", "link")));
        let checker = _UpdateChecker::_new(mock, "v0.0.2");
        let actual = checker._check_for_updates().unwrap();

        assert_eq!(actual, UpdateStatus::_NoUpdates);
    }

    #[test]
    fn release_newer_should_report_update() {
        let mut mock = MockReleaseService::new();
        mock.expect_get_last_release()
            .returning(|| Ok(BacaRelease::_new("v0.0.2", "link")));
        let checker = _UpdateChecker::_new(mock, "v0.0.1");
        let actual = checker._check_for_updates().unwrap();

        if let UpdateStatus::_Update(new_release) = actual {
            assert_eq!(new_release.version, "v0.0.2");
            return;
        }
        assert!(false);
    }
}
