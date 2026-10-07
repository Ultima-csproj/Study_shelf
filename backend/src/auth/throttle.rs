use std::{
    collections::HashMap,
    net::IpAddr,
    sync::Mutex,
    time::{Duration, Instant},
};
use study_core::account::{AccountError, AccountResult};

pub struct Throttle {
    attempts: Mutex<HashMap<IpAddr, (Instant, u32)>>,
}
impl Default for Throttle {
    fn default() -> Self {
        Self::new()
    }
}
impl Throttle {
    pub fn new() -> Self {
        Self {
            attempts: Mutex::new(HashMap::new()),
        }
    }
    pub fn check(&self, ip: IpAddr) -> AccountResult<()> {
        let now = Instant::now();
        let mut map = self
            .attempts
            .lock()
            .map_err(|_| AccountError::RateLimited)?;
        map.retain(|_, (started, _)| now.duration_since(*started) < Duration::from_secs(60));
        if map.len() >= 4096 && !map.contains_key(&ip) {
            return Err(AccountError::RateLimited);
        }
        let value = map.entry(ip).or_insert((now, 0));
        if value.1 >= 15 {
            return Err(AccountError::RateLimited);
        }
        value.1 += 1;
        Ok(())
    }
}
