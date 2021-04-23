mod baca_release;
pub use self::baca_release::BacaRelease;
mod release_service;
pub use self::release_service::ReleaseService;
mod update_status;
pub use self::update_status::UpdateStatus;
pub mod update_checker;
pub use self::update_checker::_UpdateChecker;
mod github_releases;

const _CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
