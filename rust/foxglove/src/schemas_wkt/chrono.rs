//! Conversions from chrono types.

use crate::FoxgloveError;

use super::{Duration, Timestamp};

#[cfg(test)]
mod tests;

impl TryFrom<chrono::TimeDelta> for Duration {
    type Error = FoxgloveError;

    fn try_from(delta: chrono::TimeDelta) -> Result<Self, Self::Error> {
        let Ok(mut sec) = i32::try_from(delta.num_seconds()) else {
            return Err(FoxgloveError::DurationOutOfRange);
        };
        let subsec_nanos = delta.subsec_nanos();
        let nsec = if subsec_nanos >= 0 {
            u32::try_from(subsec_nanos).expect("positive")
        } else if sec == i32::MIN {
            return Err(FoxgloveError::DurationOutOfRange);
        } else {
            sec -= 1;
            u32::try_from(subsec_nanos + 1_000_000_000).expect("positive")
        };
        Ok(Self { sec, nsec })
    }
}

impl TryFrom<chrono::DateTime<chrono::Utc>> for Timestamp {
    type Error = FoxgloveError;

    fn try_from(time: chrono::DateTime<chrono::Utc>) -> Result<Self, Self::Error> {
        let Ok(sec) = u32::try_from(time.timestamp()) else {
            return Err(FoxgloveError::TimestampOutOfRange);
        };
        let nsec = time.timestamp_subsec_nanos();
        Ok(Self { sec, nsec })
    }
}

impl TryFrom<chrono::NaiveDateTime> for Timestamp {
    type Error = FoxgloveError;

    fn try_from(time: chrono::NaiveDateTime) -> Result<Self, Self::Error> {
        Self::try_from(time.and_utc())
    }
}
