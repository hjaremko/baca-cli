pub use self::baca_release::BacaRelease;
pub use self::github_releases::GithubReleases;
pub use self::release_service::ReleaseService;
pub use self::update_check_timestamp::UpdateCheckTimestamp;
pub use self::update_checker::UpdateChecker;
pub use self::update_status::UpdateStatus;

mod baca_release;
mod github_releases;
mod release_service;
pub mod update_check_timestamp;
pub mod update_checker;
mod update_status;

pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
