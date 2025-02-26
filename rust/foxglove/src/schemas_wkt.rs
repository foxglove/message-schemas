//! Wrappers for protobuf well-known types
//!
//! For some reason, foxglove uses google's well-known types for representing Duration and
//! Timestamp in protobuf, even though we schematize those types differently. This module provides
//! an infallible translation from the foxglove schema to the underlying protobuf representation.
//!
//! This module lives outside crate::schemas, because everything under the schemas/ direcory is
//! generated.

use crate::FoxgloveError;

#[cfg(feature = "chrono")]
mod chrono;
#[cfg(test)]
mod tests;

/// Converts time integer types and normalizes excessive nanoseconds into seconds.
fn normalize(sec: impl Into<i64>, mut nsec: u32) -> (i64, i32) {
    if nsec < 1_000_000_000 {
        (sec.into(), i32::try_from(nsec).unwrap())
    } else {
        // We're upconverting seconds from u32/i32, so there's no risk of overflow here.
        let div = nsec / 1_000_000_000;
        nsec %= 1_000_000_000;
        (sec.into() + i64::from(div), i32::try_from(nsec).unwrap())
    }
}

/// A signed, fixed-length span of time.
///
/// The duration is represented by a count of seconds (which may be negative), and a count of
/// fractional seconds at nanosecond resolution (which are always positive).
///
/// # Example
///
/// ```
/// use foxglove::schemas::Duration;
///
/// // A duration of 2.718... seconds.
/// let duration = Duration {
///     sec: 2,
///     nsec: 718_281_828,
/// };
///
/// // A duration of -3.14... seconds. Note that nanoseconds are always in the positive
/// // direction.
/// let duration = Duration {
///     sec: -4,
///     nsec: 858_407_346,
/// };
///
/// // Positive durations can be derived from std::time::Duration.
/// let duration = std::time::Duration::from_micros(577_215);
///
/// // When built with the `chrono` feature, durations can also be derived from chrono::TimeDelta.
/// #[cfg(feature = "chrono")]
/// let duration = chrono::TimeDelta::microseconds(1_414_213);
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Duration {
    /// Seconds offset.
    pub sec: i32,
    /// Nanoseconds offset in the positive direction.
    pub nsec: u32,
}

impl Duration {
    /// Maximum representable duration.
    pub const MAX: Self = Self {
        sec: i32::MAX,
        nsec: u32::MAX,
    };

    /// Minimum representable duration.
    pub const MIN: Self = Self {
        sec: i32::MIN,
        nsec: 0,
    };

    fn into_prost(self) -> prost_types::Duration {
        self.into()
    }
}

impl From<Duration> for prost_types::Duration {
    fn from(v: Duration) -> Self {
        let (seconds, nanos) = normalize(v.sec, v.nsec);
        Self { seconds, nanos }
    }
}

impl prost::Message for Duration {
    fn encode_raw(&self, buf: &mut impl bytes::BufMut)
    where
        Self: Sized,
    {
        self.into_prost().encode_raw(buf);
    }

    fn merge_field(
        &mut self,
        _tag: u32,
        _wire_type: prost::encoding::wire_type::WireType,
        _buf: &mut impl bytes::Buf,
        _ctx: prost::encoding::DecodeContext,
    ) -> Result<(), prost::DecodeError>
    where
        Self: Sized,
    {
        // We only support encoding for now.
        unimplemented!("not implemeneted");
    }

    fn encoded_len(&self) -> usize {
        self.into_prost().encoded_len()
    }

    fn clear(&mut self) {
        self.sec = 0;
        self.nsec = 0;
    }
}

impl TryFrom<std::time::Duration> for Duration {
    type Error = FoxgloveError;

    fn try_from(duration: std::time::Duration) -> Result<Self, Self::Error> {
        let Ok(sec) = i32::try_from(duration.as_secs()) else {
            return Err(FoxgloveError::DurationOutOfRange);
        };
        let nsec = duration.subsec_nanos();
        Ok(Self { sec, nsec })
    }
}

/// A timestamp, represented as an offset from the unix epoch.
///
/// Timestamps before 1970-01-01T00:00:00Z, or after 2106-02-06T22:28:15Z are not representable.
///
/// # Example
///
/// ```
/// use foxglove::schemas::Timestamp;
///
/// // A timestamp can be constructed manually.
/// let timestamp = Timestamp {
///     sec: 1_548_054_420,
///     nsec: 76_657_283,
/// };
///
/// // A timestamp can also be derived from system time. If the timestamp is outside of
/// // the representable range, this will fail with FoxgloveError::TimestampOutOfRange.
/// let timestamp = Timestamp::try_from(std::time::SystemTime::now()).unwrap();
///
/// // When built with the `chrono` feature, durations can also be derived from
/// // chrono::DateTime<chrono::Utc> and chrono::NaiveDateTime.
/// #[cfg(feature = "chrono")]
/// {
///     let timestamp = Timestamp::try_from(chrono::DateTime::UNIX_EPOCH).unwrap();
///     let timestamp = Timestamp::try_from(chrono::NaiveDateTime::UNIX_EPOCH).unwrap();
/// }
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Timestamp {
    /// Seconds since epoch.
    pub sec: u32,
    /// Additional nanoseconds since epoch.
    pub nsec: u32,
}

impl Timestamp {
    /// Maximum representable timestamp.
    pub const MAX: Self = Self {
        sec: u32::MAX,
        nsec: u32::MAX,
    };

    /// Minimum representable timestamp.
    pub const MIN: Self = Self { sec: 0, nsec: 0 };

    fn into_prost(self) -> prost_types::Timestamp {
        self.into()
    }
}

impl From<Timestamp> for prost_types::Timestamp {
    fn from(v: Timestamp) -> Self {
        let (seconds, nanos) = normalize(v.sec, v.nsec);
        Self { seconds, nanos }
    }
}

impl prost::Message for Timestamp {
    fn encode_raw(&self, buf: &mut impl bytes::BufMut)
    where
        Self: Sized,
    {
        self.into_prost().encode_raw(buf);
    }

    fn merge_field(
        &mut self,
        _tag: u32,
        _wire_type: prost::encoding::wire_type::WireType,
        _buf: &mut impl bytes::Buf,
        _ctx: prost::encoding::DecodeContext,
    ) -> Result<(), prost::DecodeError>
    where
        Self: Sized,
    {
        // We only support encoding for now.
        unimplemented!("not implemeneted");
    }

    fn encoded_len(&self) -> usize {
        self.into_prost().encoded_len()
    }

    fn clear(&mut self) {
        self.sec = 0;
        self.nsec = 0;
    }
}

impl TryFrom<std::time::SystemTime> for Timestamp {
    type Error = FoxgloveError;

    fn try_from(time: std::time::SystemTime) -> Result<Self, Self::Error> {
        let Ok(duration) = time.duration_since(std::time::UNIX_EPOCH) else {
            return Err(FoxgloveError::TimestampOutOfRange);
        };
        let Ok(sec) = u32::try_from(duration.as_secs()) else {
            return Err(FoxgloveError::TimestampOutOfRange);
        };
        let nsec = duration.subsec_nanos();
        Ok(Self { sec, nsec })
    }
}
