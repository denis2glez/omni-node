//! This module contains definitions of the [`Request`] and [`Response`] message types that are
//! relevant to both modes of operation.

pub mod client;
pub mod server;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

/// Clients will send information about scheduled jobs in the form of (start time, duration, id).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Request {
    /// Random timestamp in the near future.
    pub start_time: DateTime<Local>,
    /// Duration ranging randomly between 10 and 1000 seconds.
    pub duration: Duration,
    /// Unique job ID without requiring a central allocating authority.
    pub id: Uuid,
}

/// The server will send back to the client the maximum number of concurrent jobs scheduled at the
/// busiest interval in the future.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Response {
    /// Maximum number of concurrent jobs scheduled at the busiest interval.
    pub max_jobs: usize,
}
