pub mod client;
pub mod server;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

/// Clients will send information about scheduled jobs in the form of (start time, duration, id).
#[derive(Debug, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Request {
    /// Random timestamp in the near future.
    pub start_time: DateTime<Utc>,
    /// Duration ranging randomly between 10 and 1000 seconds.
    pub duration: Duration,
    /// Unique job ID without requiring a central allocating authority.
    pub id: Uuid,
}

/// The server will send back to the client the maximum number of concurrent jobs scheduled at the
/// busiest interval in the future.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    /// Maximum number of concurrent jobs scheduled at the busiest interval.
    pub max_jobs: usize,
}
