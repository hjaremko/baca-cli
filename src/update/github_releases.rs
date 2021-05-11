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
            .user_agent("baca_cli/0.3.0")
            .build()?;
        let response = client
            .get(format!(
                "https://api.github.com/repos/{}/{}/releases",
                self.owner, self.repo
            ))
            .send();

        debug!("{:?}", response);
        let response = response?.text()?;
        debug!("{:?}", response);

        if response.contains("API rate limit exceeded") {
            return Err(Error::ApiRateLimitExceeded);
        }

        if response.contains("Not Found") {
            return Err(Error::FetchingReleaseError);
        }

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

    fn assert_fetching_release_error(actual: Result<BacaRelease>) {
        let assert_err = |e| match e {
            Error::ApiRateLimitExceeded => println!("API limit exceeded!"),
            Error::FetchingReleaseError => assert!(true),
            _ => assert!(false, "Unexpected error: {:?}", e),
        };

        assert_error(actual, assert_err);
    }

    fn assert_no_release_error(actual: Result<BacaRelease>) {
        let assert_err = |e| match e {
            Error::ApiRateLimitExceeded => println!("API limit exceeded!"),
            Error::NoRelease => assert!(true),
            _ => assert!(false, "Unexpected error: {:?}", e),
        };

        assert_error(actual, assert_err);
    }

    fn assert_error(actual: Result<BacaRelease>, assert_err: fn(Error)) {
        match actual {
            Ok(r) => {
                panic!("Unexpected success: {:?}", r)
            }
            Err(e) => assert_err(e),
        }
    }

    #[test]
    fn invalid_repo_should_return_error() {
        let gh = GithubReleases::new("hjaremko", "invalid");
        let actual = gh.get_last_release();

        assert_fetching_release_error(actual)
    }

    #[test]
    fn invalid_owner_should_return_error() {
        let gh = GithubReleases::new("invalid", "baca-cli");
        let actual = gh.get_last_release();

        assert_fetching_release_error(actual);
    }

    #[test]
    fn correct_repo_should_return_latest_release() {
        let gh = GithubReleases::new("hjaremko", "baca-cli");
        let actual = gh.get_last_release();

        if let Err(e) = actual {
            match e {
                Error::ApiRateLimitExceeded => {
                    assert!(true)
                }
                _ => assert!(false, "Unexpected error: {:?}", e),
            }
        }
    }

    #[test]
    fn correct_repo_with_no_releases_should_return_error() {
        let gh = GithubReleases::new("hjaremko", "fi");
        let actual = gh.get_last_release();

        assert_no_release_error(actual)
    }
}
