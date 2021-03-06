use crate::error::{Error, Result};
use crate::update::{BacaRelease, ReleaseService};
use serde_json::Value;
use std::env;
use tracing::{debug, info};

pub struct GithubReleases {
    owner: String,
    repo: String,
}

impl GithubReleases {
    pub fn new(owner: &str, repo: &str) -> Self {
        info!("Creating GitHub API service to repo {}/{}", owner, repo);
        GithubReleases {
            owner: owner.to_string(),
            repo: repo.to_string(),
        }
    }
}

impl ReleaseService for GithubReleases {
    fn get_last_release(&self) -> Result<BacaRelease> {
        let client = reqwest::blocking::ClientBuilder::new()
            .user_agent(make_user_agent())
            .build()?;
        let mut request_builder = client.get(format!(
            "https://api.github.com/repos/{}/{}/releases",
            self.owner, self.repo
        ));

        if let Ok(auth_token) = env::var("AUTH_TOKEN") {
            info!("Auth token present, setting auth header.");
            request_builder = request_builder.header(
                reqwest::header::AUTHORIZATION,
                format!("token {}", auth_token),
            );
        }

        let response = request_builder.send();

        debug!("{:?}", response);
        let response = response?.text()?;
        debug!("{:?}", response);

        if response.contains("API rate limit exceeded") {
            return Err(Error::ApiRateLimitExceeded);
        }

        if response.contains("Not Found") {
            return Err(Error::FetchingRelease);
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

fn make_user_agent() -> String {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    format!("baca_cli/{}", VERSION)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_fetching_release_error(actual: Result<BacaRelease>) {
        let assert_err = |e| match e {
            Error::ApiRateLimitExceeded => println!("API limit exceeded!"),
            Error::FetchingRelease => (),
            _ => panic!("Unexpected error: {:?}", e),
        };

        assert_error(actual, assert_err);
    }

    fn assert_no_release_error(actual: Result<BacaRelease>) {
        let assert_err = |e| match e {
            Error::ApiRateLimitExceeded => println!("API limit exceeded!"),
            Error::NoRelease => (),
            _ => panic!("Unexpected error: {:?}", e),
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
                Error::ApiRateLimitExceeded => (),
                _ => panic!("Unexpected error: {:?}", e),
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
