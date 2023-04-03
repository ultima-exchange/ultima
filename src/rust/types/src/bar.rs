use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bar {
    pub market_id: u64,
    pub start_time: DateTime<Utc>,
    pub open: u64,
    pub high: u64,
    pub low: u64,
    pub close: u64,
    pub volume: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Interval {
    #[serde(rename = "1m")]
    I1m,
    #[serde(rename = "5m")]
    I5m,
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::I1m => "1m",
            Self::I5m => "5m",
        };
        write!(f, "{}", s)
    }
}
