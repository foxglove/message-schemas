use assert_matches::assert_matches;

use super::{normalize, Duration, Timestamp};
use crate::convert::{RangeError, SaturatingFrom};

#[test]
fn test_normalize() {
    assert_eq!(normalize(0, 0), (0, 0));
    assert_eq!(normalize(0, 999_999_999), (0, 999_999_999));
    assert_eq!(normalize(0, 1_000_000_000), (1, 0));
    assert_eq!(normalize(0, 2_999_999_999), (2, 999_999_999));
    assert_eq!(normalize(-1, 1_111_111_111), (0, 111_111_111));
    assert_eq!(normalize(i32::MIN, 1_000_000_001), (i32::MIN as i64 + 1, 1));
    assert_eq!(normalize(i32::MAX, 1_000_000_001), (i32::MAX as i64 + 1, 1));
    assert_eq!(normalize(u32::MAX, 1_000_000_001), (u32::MAX as i64 + 1, 1));
    assert_eq!(
        normalize(u32::MAX, u32::MAX),
        (u32::MAX as i64 + 4, 294_967_295)
    );
}

#[test]
fn test_duration_from_std_duration() {
    let orig = std::time::Duration::from_millis(1234);
    let dur = Duration::try_from(orig).unwrap();
    assert_eq!(
        dur,
        Duration {
            sec: 1,
            nsec: 234_000_000,
        }
    );

    // min
    let orig = std::time::Duration::default();
    let dur = Duration::try_from(orig).unwrap();
    assert_eq!(dur, Duration::default());

    // max
    let orig = std::time::Duration::from_nanos(i32::MAX as u64 * 1_000_000_000 + 999_999_999);
    let dur = Duration::try_from(orig).unwrap();
    assert_eq!(
        dur,
        Duration {
            sec: i32::MAX,
            nsec: 999_999_999,
        }
    );

    // seconds out of range
    let orig = std::time::Duration::from_secs(i32::MAX as u64 + 1);
    assert_matches!(Duration::try_from(orig), Err(RangeError::UpperBound));
    assert_eq!(Duration::saturating_from(orig), Duration::MAX);
}

#[test]
fn test_timestamp_from_system_time() {
    // min
    let orig = std::time::UNIX_EPOCH;
    let ts = Timestamp::try_from(orig).unwrap();
    assert_eq!(ts, Timestamp::default());

    // max
    let orig = std::time::UNIX_EPOCH
        .checked_add(std::time::Duration::from_nanos(
            u32::MAX as u64 * 1_000_000_000 + 999_999_999,
        ))
        .unwrap();
    let ts = Timestamp::try_from(orig).unwrap();
    assert_eq!(
        ts,
        Timestamp {
            sec: u32::MAX,
            nsec: 999_999_999,
        }
    );

    // too past
    let orig = std::time::UNIX_EPOCH
        .checked_sub(std::time::Duration::from_nanos(1))
        .unwrap();
    assert_matches!(Timestamp::try_from(orig), Err(RangeError::LowerBound));
    assert_eq!(Timestamp::saturating_from(orig), Timestamp::MIN);

    // too future
    let orig = std::time::UNIX_EPOCH
        .checked_add(std::time::Duration::from_secs(u32::MAX as u64 + 1))
        .unwrap();
    assert_matches!(Timestamp::try_from(orig), Err(RangeError::UpperBound));
    assert_eq!(Timestamp::saturating_from(orig), Timestamp::MAX);
}
