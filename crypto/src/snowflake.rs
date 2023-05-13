//! This module provides the `Snowflake` struct and `SnowflakeGenerator` for generating
//! unique and sortable IDs based on a timestamp, worker ID, and sequence number.
//!
//! The `Snowflake` struct consists of a 64-bit integer where the first 42 bits represent
//! the timestamp, the next 5 bits represent the worker ID, and the last 12 bits represent
//! a sequence number.
//!
//! The `SnowflakeGenerator` is a thread-safe structure that can be used to generate
//! snowflakes using the current time, a worker ID, and an atomic sequence number.

use std::cmp::Ordering;
use std::num::ParseIntError;
use std::str::FromStr;
use std::sync::atomic::{AtomicI64, AtomicU16, Ordering as AtomicOrdering};
use std::thread;
use std::time::Duration;

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

/// Customize the start time as desired
const UNIX_START_TIME: i64 = 1_589_870_119_000;

/// The Snowflake struct represents a unique and sortable ID.
#[allow(clippy::derived_hash_with_manual_eq)] // There's a test to verify that the PartialEq implementation is correct and works with hashing.
#[derive(Debug, Clone, Eq, Copy, Serialize, Deserialize, Default, Hash)]
pub struct Snowflake {
    timestamp: i64,
    worker_id: u8,
    sequence: u16,
}

impl Snowflake {
    /// Returns the UTC time represented by the Snowflake's timestamp.
    pub fn get_time(&self) -> DateTime<Utc> {
        let naive_time =
            NaiveDateTime::from_timestamp_opt((self.timestamp + UNIX_START_TIME) / 1000, 0)
                .expect("failed to convert timestamp to NaiveDateTime");
        DateTime::<Utc>::from_utc(naive_time, Utc)
    }

    /// Returns the 64-bit integer ID of the Snowflake.
    pub fn get_id(&self) -> u64 {
        ((self.timestamp - UNIX_START_TIME) << 22) as u64
            | (self.worker_id as u64) << 17
            | self.sequence as u64
    }

    /// Creates a Snowflake from a 64-bit integer ID.
    ///
    /// # Arguments
    ///
    /// * `id` - A 64-bit integer representing the Snowflake ID.
    pub fn from_id(id: u64) -> Snowflake {
        let timestamp = (id >> 22) as i64 + UNIX_START_TIME;
        let worker_id = ((id >> 17) & 0x1F) as u8;
        let sequence = (id & 0xFFF) as u16;

        Snowflake {
            timestamp,
            worker_id,
            sequence,
        }
    }

    /// Returns the timestamp of the Snowflake.
    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    /// Returns the worker ID of the Snowflake.
    pub fn worker_id(&self) -> u8 {
        self.worker_id
    }

    /// Returns the sequence of the Snowflake.
    pub fn sequence(&self) -> u16 {
        self.sequence
    }
}

impl FromStr for Snowflake {
    type Err = ParseIntError;

    /// Creates a Snowflake from a string representation of the 64-bit integer ID.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let snowflake = u64::from_str(s)?;
        Ok(Snowflake::from_id(snowflake))
    }
}

impl PartialOrd for Snowflake {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.timestamp.partial_cmp(&other.timestamp)
    }
}

impl Ord for Snowflake {
    fn cmp(&self, other: &Self) -> Ordering {
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
        self.get_id() == *other
    }
}

impl PartialEq<Snowflake> for u64 {
    fn eq(&self, other: &Snowflake) -> bool {
        *self == other.get_id()
    }
}

impl PartialEq<i64> for Snowflake {
    fn eq(&self, other: &i64) -> bool {
        self.get_id() as i64 == *other
    }
}

impl PartialEq<Snowflake> for i64 {
    fn eq(&self, other: &Snowflake) -> bool {
        *self == other.get_id() as i64
    }
}

impl PartialEq<str> for Snowflake {
    fn eq(&self, other: &str) -> bool {
        self.get_id().to_string() == other
    }
}

impl PartialEq<Snowflake> for str {
    fn eq(&self, other: &Snowflake) -> bool {
        self == other.get_id().to_string()
    }
}

/// The SnowflakeGenerator is a thread-safe structure for generating unique Snowflakes.
#[derive(Debug)]
pub struct SnowflakeGenerator {
    worker_id: u8,
    sequence: AtomicU16,
    timestamp: AtomicI64,
}

impl SnowflakeGenerator {
    /// Creates a new SnowflakeGenerator with a given worker ID.
    ///
    /// # Arguments
    ///
    /// * `worker_id` - An 8-bit unsigned integer representing the worker ID.
    pub fn new(worker_id: u8) -> Self {
        SnowflakeGenerator {
            worker_id,
            sequence: AtomicU16::new(0),
            timestamp: AtomicI64::new(0),
        }
    }

    /// Returns the next Snowflake generated by the SnowflakeGenerator.
    pub fn next_snowflake(&self) -> Snowflake {
        loop {
            let current_time = Utc::now().timestamp_millis() - UNIX_START_TIME;
            let last_timestamp = self.timestamp.load(AtomicOrdering::SeqCst);

            if current_time != last_timestamp {
                self.sequence.store(0, AtomicOrdering::SeqCst);
                self.timestamp.store(current_time, AtomicOrdering::SeqCst);
            }

            let sequence = self.sequence.fetch_add(1, AtomicOrdering::SeqCst);

            // If sequence number overflows, wait for the next millisecond
            if sequence < (1 << 12) {
                return Snowflake {
                    timestamp: current_time,
                    worker_id: self.worker_id,
                    sequence,
                };
            } else {
                // Block until the next millisecond arrives
                let sleep_duration = Duration::from_micros(1000);
                thread::sleep(sleep_duration);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    use super::SnowflakeGenerator;

    #[test]
    fn test_hash_and_partial_eq() {
        let gen = SnowflakeGenerator::new(0);
        let k1 = gen.next_snowflake();
        let k2 = k1;

        assert_eq!(k1, k2);

        let mut map = std::collections::HashMap::new();
        map.insert(k1, 1);

        assert_eq!(map.get(&k2), Some(&1));

        let k3 = gen.next_snowflake();

        assert_ne!(k1, k3);

        map.insert(k3, 2);

        assert_eq!(map.get(&k3), Some(&2));

        let mut hasher = DefaultHasher::new();
        k1.hash(&mut hasher);
        let k1_hash = hasher.finish();

        let mut hasher = DefaultHasher::new();
        k2.hash(&mut hasher);
        let k2_hash = hasher.finish();

        assert_eq!(k1_hash, k2_hash);
    }
}
