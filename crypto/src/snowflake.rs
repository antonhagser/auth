//! This module provides the `Snowflake` struct and `SnowflakeGenerator` for generating
//! unique and sortable IDs based on a timestamp, worker ID, and sequence number.
//!
//! The `SnowflakeGenerator` is a thread-safe structure that can be used to generate
//! snowflakes using the current time, a worker ID, and an atomic sequence number.

use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    str::FromStr,
    sync::{
        atomic::{AtomicU64, Ordering},
        Mutex,
    },
};

use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

/// The `Snowflake` struct represents a unique and sortable ID.
///
/// This structure contains four fields:
/// * `timestamp`: The time when the Snowflake was created.
/// * `worker_id`: The ID of the worker that created the Snowflake.
/// * `process_id`: The ID of the process that created the Snowflake.
/// * `sequence`: The sequence number of the Snowflake.
#[allow(clippy::derived_hash_with_manual_eq)] // There's a test to verify that the PartialEq implementation is correct and works with hashing.
#[derive(Clone, Eq, Copy, Serialize, Deserialize, Default, Hash)]
pub struct Snowflake {
    timestamp: u64,
    worker_id: u64,
    process_id: u64,
    sequence: u64,
}

impl std::fmt::Debug for Snowflake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Snowflake").field(&self.to_id()).finish()
    }
}

/// The `SnowflakeGenerator` struct is responsible for creating new Snowflakes.
///
/// It maintains the current sequence number and the last used timestamp to
/// ensure that each generated Snowflake is unique.
#[derive(Debug)]
pub struct SnowflakeGenerator {
    worker_id: u64,
    process_id: u64,
    last_timestamp: Mutex<u64>,
    sequence: AtomicU64,
}

impl SnowflakeGenerator {
    /// Represents the epoch timestamp used for Snowflake generation.
    const SNOWFLAKE_EPOCHE: u64 = 1_683_992_570_782; // 2020-05-19T06:35:19Z

    // Bits allocated for each component in the Snowflake
    const WORKER_ID_BITS: u64 = 5;
    const PROCESS_ID_BITS: u64 = 5;
    const SEQUENCE_BITS: u64 = 12;

    // Maximum values for each component in the Snowflake
    const MAX_WORKER_ID: u64 = (1 << Self::WORKER_ID_BITS) - 1;
    const MAX_PROCESS_ID: u64 = (1 << Self::PROCESS_ID_BITS) - 1;
    const MAX_SEQUENCE: u64 = (1 << Self::SEQUENCE_BITS) - 1;

    /// Creates a new SnowflakeGenerator with the given worker and process IDs.
    ///
    /// # Arguments
    ///
    /// * `worker_id` - The ID of the worker that will be generating Snowflakes.
    /// * `process_id` - The ID of the process that will be generating Snowflakes.
    ///
    /// # Panics
    ///
    /// This function will panic if the worker_id or the process_id is higher than the maximum allowed values.
    pub fn new(worker_id: u64, process_id: u64) -> Self {
        assert!(worker_id <= Self::MAX_WORKER_ID);
        assert!(process_id <= Self::MAX_PROCESS_ID);

        SnowflakeGenerator {
            worker_id,
            process_id,
            last_timestamp: Mutex::new(0),
            sequence: AtomicU64::new(0),
        }
    }

    /// Generates a new Snowflake.
    ///
    /// This function generates a new Snowflake, ensuring that it is unique.
    /// If the current timestamp is the same as the last used timestamp, the
    /// sequence number is incremented. Otherwise, the sequence number is reset.
    ///
    /// # Errors
    ///
    /// This function returns an error if the system time is before the Unix Epoch,
    /// or if the Mutex guarding the last used timestamp is poisoned.
    pub fn next_snowflake(&self) -> Result<Snowflake, &'static str> {
        let mut timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| "SystemTime before UNIX EPOCH")?
            .as_millis() as u64
            - Self::SNOWFLAKE_EPOCHE;

        let mut last_timestamp = self.last_timestamp.lock().map_err(|_| "Mutex poisoned")?;

        if timestamp == *last_timestamp {
            let sequence = self.sequence.fetch_add(1, Ordering::SeqCst);

            if sequence > Self::MAX_SEQUENCE {
                while timestamp <= *last_timestamp {
                    timestamp = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .map_err(|_| "SystemTime before UNIX EPOCH")?
                        .as_millis() as u64
                        - Self::SNOWFLAKE_EPOCHE;
                }
            }
        } else {
            self.sequence.store(0, Ordering::SeqCst);
        }

        *last_timestamp = timestamp;
        let sequence = self.sequence.load(Ordering::SeqCst);

        Ok(Snowflake {
            timestamp,
            worker_id: self.worker_id,
            process_id: self.process_id,
            sequence,
        })
    }
}

impl Snowflake {
    /// Returns the Snowflake as a u64.
    pub fn to_id(&self) -> u64 {
        (self.timestamp << 22) | (self.worker_id << 17) | (self.process_id << 12) | self.sequence
    }

    /// Returns the Snowflake as a i64.
    ///
    /// The Snowflake will be converted to a negative number if the most significant bit is set.
    ///
    /// # Panics
    ///
    /// This function will panic if the most significant bit of the timestamp is set.
    pub fn to_id_signed(&self) -> i64 {
        // Make sure the most significant bit is not set. If it is, panic.
        assert!(self.timestamp < 1 << 41);

        self.to_id() as i64
    }

    /// Returns the timestamp of the Snowflake.
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    /// Returns the worker ID of the Snowflake.
    pub fn worker_id(&self) -> u64 {
        self.worker_id
    }

    /// Returns the sequence of the Snowflake.
    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Returns the process ID of the Snowflake.
    pub fn process_id(&self) -> u64 {
        self.process_id
    }

    /// Returns the creation time of the Snowflake.
    ///
    /// The returned value is the number of milliseconds since Discord epoch.
    pub fn creation_time(&self) -> u64 {
        self.timestamp + SnowflakeGenerator::SNOWFLAKE_EPOCHE
    }

    /// Returns the creation time of the Snowflake as a `chrono::DateTime<Utc>`.
    pub fn time(&self) -> DateTime<Utc> {
        let timestamp = (self.timestamp + SnowflakeGenerator::SNOWFLAKE_EPOCHE) as i64;
        Utc.timestamp_millis_opt(timestamp).unwrap()
    }
}

impl std::fmt::Display for Snowflake {
    /// Returns a string representation of the Snowflake.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_id())
    }
}

impl From<u64> for Snowflake {
    fn from(id: u64) -> Self {
        let timestamp = id >> 22;
        let worker_id = (id & 0x3E0000) >> 17;
        let process_id = (id & 0x1F000) >> 12;
        let sequence = id & 0xFFF;

        Snowflake {
            timestamp,
            worker_id,
            process_id,
            sequence,
        }
    }
}

impl From<i64> for Snowflake {
    fn from(id: i64) -> Self {
        // If the id is negative, convert it to positive
        let id = id.unsigned_abs();
        Snowflake::from(id)
    }
}

impl FromStr for Snowflake {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s.parse::<u64>()?;
        Ok(Snowflake::from(id))
    }
}

impl From<&str> for Snowflake {
    fn from(id: &str) -> Self {
        Snowflake::from_str(id).unwrap()
    }
}

impl From<String> for Snowflake {
    fn from(id: String) -> Self {
        Snowflake::from_str(&id).unwrap()
    }
}

impl PartialOrd for Snowflake {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.timestamp.partial_cmp(&other.timestamp)
    }
}

impl Ord for Snowflake {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl PartialEq for Snowflake {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
            && self.worker_id == other.worker_id
            && self.sequence == other.sequence
    }
}

impl PartialEq<u64> for Snowflake {
    fn eq(&self, other: &u64) -> bool {
        self.to_id() == *other
    }
}

impl PartialEq<Snowflake> for u64 {
    fn eq(&self, other: &Snowflake) -> bool {
        *self == other.to_id()
    }
}

impl PartialEq<Snowflake> for i64 {
    fn eq(&self, other: &Snowflake) -> bool {
        *self == other.to_id() as i64
    }
}

impl PartialEq<str> for Snowflake {
    fn eq(&self, other: &str) -> bool {
        self.to_id().to_string() == other
    }
}

impl PartialEq<Snowflake> for str {
    fn eq(&self, other: &Snowflake) -> bool {
        self == other.to_id().to_string()
    }
}
