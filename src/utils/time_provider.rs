use chrono::{DateTime, Utc};

pub trait TimeProvider {
    fn now_utc(&self) -> DateTime<Utc>;
}

pub struct RealTimeProvider;

impl TimeProvider for RealTimeProvider {
    fn now_utc(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

#[cfg(test)]
pub struct MockTimeProvider {
    offset: i64
}

#[cfg(test)]
impl MockTimeProvider {
    pub fn new(offset: i64) -> Self {
        Self {offset}
    }
}

#[cfg(test)]
impl TimeProvider for MockTimeProvider {
    fn now_utc(&self) -> DateTime<Utc> {
        Utc::now() + chrono::Duration::seconds(self.offset)
    }
}