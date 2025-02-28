//! Wrappers for well-known types.

use pyo3::prelude::*;

/// A timestamp in seconds and nanoseconds
///
/// :param sec: The number of seconds since a user-defined epoch.
/// :param nsec: The number of nanoseconds since the :py:attr:\`sec\` value.
#[pyclass(module = "foxglove.schemas")]
#[derive(Clone)]
pub struct Timestamp(pub(crate) foxglove::schemas::Timestamp);

#[pymethods]
impl Timestamp {
    #[new]
    #[pyo3(signature = (sec, nsec=None))]
    fn new(sec: u32, nsec: Option<u32>) -> Self {
        let nsec = nsec.unwrap_or(0);
        Self(foxglove::schemas::Timestamp { sec, nsec })
    }

    fn __repr__(&self) -> String {
        format!("Timestamp(sec={}, nsec={})", self.0.sec, self.0.nsec).to_string()
    }
}

impl From<Timestamp> for foxglove::schemas::Timestamp {
    fn from(value: Timestamp) -> Self {
        value.0
    }
}

/// A duration, composed of seconds and nanoseconds
///
/// :param sec: The number of seconds in the duration.
/// :param nsec: The number of nanoseconds in the positive direction.
#[pyclass(module = "foxglove.schemas")]
#[derive(Clone)]
pub struct Duration(pub(crate) foxglove::schemas::Duration);

#[pymethods]
impl Duration {
    #[new]
    #[pyo3(signature = (sec, nsec=None))]
    fn new(sec: i32, nsec: Option<u32>) -> Self {
        let nsec = nsec.unwrap_or(0);
        Self(foxglove::schemas::Duration { sec, nsec })
    }

    fn __repr__(&self) -> String {
        format!("Duration(sec={}, nsec={})", self.0.sec, self.0.nsec).to_string()
    }
}

impl From<Duration> for foxglove::schemas::Duration {
    fn from(value: Duration) -> Self {
        value.0
    }
}
