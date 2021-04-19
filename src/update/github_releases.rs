use crate::error::{Error, Result};
use crate::update::{BacaRelease, ReleaseService};
use serde_json::Value;
use tracing::debug;

pub struct GithubReleases {
    owner: String,
    repo: String,
}

impl GithubReleases {
    pub fn new(owner: &str, repo: &str) -> Self {
        GithubReleases {
            owner: owner.to_string(),
            repo: repo.to_string(),
        }
    }
}

impl ReleaseService for GithubReleases {
    fn get_last_release(&self) -> Result<BacaRelease> {
        let client = reqwest::blocking::ClientBuilder::new()
            .user_agent("baca_cli/0.2.1")
            .build()?;
        let response = client
            .get(format!(
                "https://api.github.com/repos/{}/{}/releases",
                self.owner, self.repo
            ))
            .send();

        let response = response?.text()?;

        if response.contains("Not Found") {
            return Err(Error::FetchingReleaseError);
        }

        debug!("{:?}", response);
        let v: Value = serde_json::from_str(&response)?;
        let ver = &v[0]["tag_name"];
        let link = &v[0]["html_url"];

        if ver.is_null() || link.is_null() {
            return Err(Error::NoRelease);
        }

        let ver = ver.as_str().unwrap();
        let ver = &ver[1..];
        Ok(BacaRelease::new(ver, link.as_str().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_repo_should_return_error() {
        let gh = GithubReleases::new("hjaremko", "invalid");
        let actual = gh.get_last_release();
        assert!(actual.is_err());
    }

    #[test]
    fn invalid_owner_should_return_error() {
        let gh = GithubReleases::new("invalid", "baca-cli");
        let actual = gh.get_last_release();
        assert!(actual.is_err());
    }

    #[test]
    fn correct_repo_no_releases_should_return_error() {
        let gh = GithubReleases::new("invalid", "baca-cli");
        let actual = gh.get_last_release();
        assert!(actual.is_err());
    }

    #[test]
    fn correct_repo_should_return_latest_release() {
        let gh = GithubReleases::new("hjaremko", "baca-cli");
        let actual = gh.get_last_release();
        assert!(actual.is_ok());
    }

    #[test]
    fn correct_repo_with_no_releases_should_return_error() {
        let gh = GithubReleases::new("hjaremko", "fi");
        let actual = gh.get_last_release();
        assert!(actual.is_err());
    }
}
