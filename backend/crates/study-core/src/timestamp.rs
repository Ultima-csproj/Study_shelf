use std::time::{Duration, SystemTime, UNIX_EPOCH};
use thiserror::Error;
use time::OffsetDateTime;

#[derive(Debug, Error)]
pub enum TimestampError {
    #[error("timestamp is outside the supported range")]
    OutOfRange,
    #[error("cannot format UTC timestamp")]
    Format(#[from] time::error::Format),
}

/// Normalize to PostgreSQL microsecond precision before domain transitions.
pub fn to_utc(value: SystemTime) -> Result<OffsetDateTime, TimestampError> {
    let nanos = match value.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            i128::try_from(duration.as_nanos()).map_err(|_| TimestampError::OutOfRange)?
        }
        Err(error) => {
            -i128::try_from(error.duration().as_nanos()).map_err(|_| TimestampError::OutOfRange)?
        }
    };
    OffsetDateTime::from_unix_timestamp_nanos(nanos.div_euclid(1000) * 1000)
        .map_err(|_| TimestampError::OutOfRange)
}

pub fn from_utc(value: OffsetDateTime) -> Result<SystemTime, TimestampError> {
    let nanos = value.unix_timestamp_nanos();
    let magnitude = nanos.unsigned_abs();
    let seconds =
        u64::try_from(magnitude / 1_000_000_000).map_err(|_| TimestampError::OutOfRange)?;
    let duration = Duration::new(seconds, (magnitude % 1_000_000_000) as u32);
    let result = if nanos >= 0 {
        UNIX_EPOCH.checked_add(duration)
    } else {
        UNIX_EPOCH.checked_sub(duration)
    };
    result.ok_or(TimestampError::OutOfRange)
}

pub fn normalize(value: SystemTime) -> Result<SystemTime, TimestampError> {
    from_utc(to_utc(value)?)
}

pub fn format_utc(value: SystemTime) -> Result<String, TimestampError> {
    Ok(to_utc(value)?.format(&time::format_description::well_known::Rfc3339)?)
}
