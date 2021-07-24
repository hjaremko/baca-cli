use time::OffsetDateTime;
use tracing::debug;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait TimeProvider {
    fn now(&self) -> OffsetDateTime;
    fn datetime_from_timestamp(&self, timestamp: &str) -> OffsetDateTime;
}

pub struct UnixTimeProvider {}

impl UnixTimeProvider {
    pub fn new() -> Self {
        UnixTimeProvider {}
    }
}

impl TimeProvider for UnixTimeProvider {
    fn now(&self) -> OffsetDateTime {
        let time = OffsetDateTime::now_utc().unix_timestamp();
        debug!("Current unix timestamp: {}", time);
        OffsetDateTime::from_unix_timestamp(time)
    }

    fn datetime_from_timestamp(&self, timestamp: &str) -> OffsetDateTime {
        OffsetDateTime::from_unix_timestamp(timestamp.parse().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_unix_timestamp() {
        let tp = UnixTimeProvider::new();
        let expected = OffsetDateTime::now_utc().unix_timestamp();
        let actual = tp.now();

        assert_eq!(expected, actual.unix_timestamp());
    }
}
