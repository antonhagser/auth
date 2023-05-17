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
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SnowflakeError {
    #[error("Invalid snowflake")]
    InvalidInput,
    #[error("Invalid sequnce ID")]
    InvalidSequenceId,
    #[error("Invalid worker ID")]
    InvalidWorkerId,
    #[error("Invalid process ID")]
    InvalidProcessId,
    #[error("Invalid timestamp")]
    InvalidTimestamp,
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

/// Represents the epoch timestamp used for Snowflake generation.
const SNOWFLAKE_EPOCH: u64 = 1_683_992_570_782; // 2020-05-19T06:35:19Z

// Bits allocated for each component in the Snowflake
const WORKER_ID_BITS: u64 = 5;
const PROCESS_ID_BITS: u64 = 5;
const SEQUENCE_BITS: u64 = 12;

// Maximum values for each component in the Snowflake
const MAX_WORKER_ID: u64 = (1 << WORKER_ID_BITS) - 1;
const MAX_PROCESS_ID: u64 = (1 << PROCESS_ID_BITS) - 1;
const MAX_SEQUENCE: u64 = (1 << SEQUENCE_BITS) - 1;

impl SnowflakeGenerator {
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
        assert!(worker_id <= MAX_WORKER_ID);
        assert!(process_id <= MAX_PROCESS_ID);

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
            - SNOWFLAKE_EPOCH;

        let mut last_timestamp = self.last_timestamp.lock().map_err(|_| "Mutex poisoned")?;

        if timestamp == *last_timestamp {
            let sequence = self.sequence.fetch_add(1, Ordering::SeqCst);

            if sequence > MAX_SEQUENCE {
                while timestamp <= *last_timestamp {
                    timestamp = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .map_err(|_| "SystemTime before UNIX EPOCH")?
                        .as_millis() as u64
                        - SNOWFLAKE_EPOCH;
                }
            }
        } else {
            self.sequence.store(0, Ordering::SeqCst);
        }

        *last_timestamp = timestamp;
        let sequence = self.sequence.load(Ordering::SeqCst);

        Ok(Snowflake::new_from_components(
            timestamp,
            self.worker_id,
            self.process_id,
            sequence,
        ))
    }
}

/// The `Snowflake` struct represents a unique and sortable ID.
///
/// This structure contains four fields:
/// * `timestamp`: The time when the Snowflake was created.
/// * `worker_id`: The ID of the worker that created the Snowflake.
/// * `process_id`: The ID of the process that created the Snowflake.
/// * `sequence`: The sequence number of the Snowflake.
#[allow(clippy::derived_hash_with_manual_eq)] // There's a test to verify that the PartialEq implementation is correct and works with hashing.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Snowflake(u64);

impl Serialize for Snowflake {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;

        Snowflake::from_str(&s).map_err(serde::de::Error::custom)
    }
}

pub struct SnowflakeComponents {
    pub timestamp: u64,
    pub worker_id: u64,
    pub process_id: u64,
    pub sequence: u64,
}

impl Snowflake {
    pub fn new(id: u64) -> Self {
        Snowflake(id)
    }

    pub fn new_from_components(
        timestamp: u64,
        worker_id: u64,
        process_id: u64,
        sequence: u64,
    ) -> Self {
        Snowflake(Snowflake::from_components(
            timestamp, worker_id, process_id, sequence,
        ))
    }

    /// Returns the ID of the Snowflake.
    pub fn id(&self) -> u64 {
        self.0
    }

    /// Returns the individual components of the Snowflake.
    ///
    /// The components are returned in the following order:
    /// * `timestamp`
    /// * `worker_id`
    /// * `process_id`
    /// * `sequence`
    pub fn to_components(&self) -> SnowflakeComponents {
        let timestamp = (self.id() >> 22) & ((1 << 41) - 1);
        let worker_id = (self.id() >> 17) & ((1 << 5) - 1);
        let process_id = (self.id() >> 12) & ((1 << 5) - 1);
        let sequence = self.id() & ((1 << 12) - 1);

        SnowflakeComponents {
            timestamp,
            worker_id,
            process_id,
            sequence,
        }
    }

    /// Creates a Snowflake from the given components.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - The timestamp of the Snowflake.
    /// * `worker_id` - The worker ID of the Snowflake.
    /// * `process_id` - The process ID of the Snowflake.
    /// * `sequence` - The sequence number of the Snowflake.
    pub fn from_components(timestamp: u64, worker_id: u64, process_id: u64, sequence: u64) -> u64 {
        (timestamp << 22) | (worker_id << 17) | (process_id << 12) | sequence
    }

    /// Returns the Snowflake as a u64.
    pub fn to_id(&self) -> u64 {
        self.id()
    }

    /// Parses a Snowflake into its components.
    ///
    /// # Arguments
    ///
    /// * `id` - The Snowflake to parse.
    ///
    /// # Panics
    ///
    /// This function will panic if the most significant bit of the timestamp is set.
    fn validate_id(&self) -> Result<(), SnowflakeError> {
        let components = Snowflake::new(self.id()).to_components();

        if components.timestamp + SNOWFLAKE_EPOCH > chrono::Utc::now().timestamp_millis() as u64 {
            Err(SnowflakeError::InvalidTimestamp)
        } else if components.worker_id > MAX_WORKER_ID {
            Err(SnowflakeError::InvalidWorkerId)
        } else if components.process_id > MAX_PROCESS_ID {
            Err(SnowflakeError::InvalidProcessId)
        } else if components.sequence > MAX_SEQUENCE {
            Err(SnowflakeError::InvalidSequenceId)
        } else {
            Ok(())
        }
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
        assert!(self.timestamp() < 1 << 41);

        self.to_id() as i64
    }

    /// Returns the timestamp of the Snowflake.
    fn timestamp(&self) -> u64 {
        (self.id() >> 22) & ((1 << 41) - 1)
    }

    /// Returns the worker ID of the Snowflake.
    pub fn worker_id(&self) -> u64 {
        (self.id() >> 17) & ((1 << WORKER_ID_BITS) - 1)
    }

    /// Returns the process ID of the Snowflake.
    pub fn process_id(&self) -> u64 {
        (self.id() >> 12) & ((1 << PROCESS_ID_BITS) - 1)
    }

    /// Returns the sequence of the Snowflake.
    pub fn sequence(&self) -> u64 {
        self.id() & ((1 << SEQUENCE_BITS) - 1)
    }

    /// Returns the creation time of the Snowflake.
    ///
    /// The returned value is the number of milliseconds since Discord epoch.
    pub fn creation_time(&self) -> u64 {
        self.timestamp() + SNOWFLAKE_EPOCH
    }

    /// Returns the creation time of the Snowflake as a `chrono::DateTime<Utc>`.
    pub fn time(&self) -> DateTime<Utc> {
        let timestamp = (self.timestamp() + SNOWFLAKE_EPOCH) as i64;
        Utc.timestamp_millis_opt(timestamp).unwrap()
    }
}

impl std::fmt::Display for Snowflake {
    /// Returns a string representation of the Snowflake.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_id())
    }
}

impl FromStr for Snowflake {
    type Err = SnowflakeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s.parse::<u64>().map_err(|_| SnowflakeError::InvalidInput)?;

        // Check if the ID is a valid snowflake
        let id = Snowflake::new(id);
        if let Err(err) = id.validate_id() {
            Err(err)
        } else {
            Ok(id)
        }
    }
}

impl TryFrom<u64> for Snowflake {
    type Error = SnowflakeError;

    fn try_from(id: u64) -> Result<Self, Self::Error> {
        let id = Snowflake::new(id);
        id.validate_id()?;
        Ok(id)
    }
}

impl TryFrom<i64> for Snowflake {
    type Error = SnowflakeError;

    fn try_from(id: i64) -> Result<Self, Self::Error> {
        Snowflake::try_from(id as u64)
    }
}

impl TryFrom<String> for Snowflake {
    type Error = SnowflakeError;

    fn try_from(id: String) -> Result<Self, Self::Error> {
        Snowflake::try_from(id.as_str())
    }
}

impl TryFrom<&str> for Snowflake {
    type Error = SnowflakeError;

    fn try_from(id: &str) -> Result<Self, Self::Error> {
        Snowflake::from_str(id)
    }
}

impl PartialEq<u64> for Snowflake {
    fn eq(&self, other: &u64) -> bool {
        self.to_id() == *other
    }
}

impl PartialEq<i64> for Snowflake {
    fn eq(&self, other: &i64) -> bool {
        self.to_id() as i64 == *other
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

impl PartialEq<String> for Snowflake {
    fn eq(&self, other: &String) -> bool {
        self.to_id().to_string() == *other
    }
}

impl PartialEq<Snowflake> for String {
    fn eq(&self, other: &Snowflake) -> bool {
        self == &other.to_id().to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::hash_map::RandomState,
        hash::{BuildHasher, Hash, Hasher},
    };

    use super::*;

    #[test]
    fn test_snowflake_hash_equality() {
        let sf1 = Snowflake::new_from_components(123, 12, 6, 1);
        let sf2 = Snowflake::new_from_components(123, 12, 6, 1);

        let s = RandomState::new();

        let mut hasher = s.build_hasher();
        sf1.hash(&mut hasher);
        let hash1 = hasher.finish();

        let mut hasher = s.build_hasher();
        sf2.hash(&mut hasher);
        let hash2 = hasher.finish();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_snowflake_equality() {
        let sf1 = Snowflake::new_from_components(123, 12, 6, 1);
        let sf2 = Snowflake::new_from_components(123, 12, 6, 1);
        let sf3 = Snowflake::new_from_components(123, 12, 2, 1);

        assert_eq!(sf1, sf2);
        assert_ne!(sf1, sf3);
    }

    #[test]
    #[should_panic]
    fn test_snowflake_equality_should_panic() {
        let sf1 = Snowflake::new_from_components(123, 12, 6, 1);
        let sf2 = Snowflake::new_from_components(123, 12, 6, 2);
        let sf3 = Snowflake::new_from_components(123, 12, 2, 3);

        assert_eq!(sf1, sf2);
        assert_ne!(sf1, sf3);
    }

    #[test]
    fn test_snowflake_u64_equality() {
        let sf = Snowflake::new_from_components(123, 12, 6, 1);
        let id = sf.to_id();

        assert_eq!(sf, id);
        assert_eq!(id, sf);
    }

    #[test]
    fn test_snowflake_i64_equality() {
        let sf = Snowflake::new_from_components(123, 12, 6, 1);
        let id = sf.to_id() as i64;

        assert_eq!(sf, id);
        assert_eq!(id, sf);
    }

    #[test]
    fn test_snowflake_str_equality() {
        let sf = Snowflake::new_from_components(123, 12, 6, 1);
        let id_str = sf.to_id().to_string();

        assert_eq!(sf, id_str);
        assert_eq!(id_str, sf);
    }

    #[test]
    fn test_snowflake_from_i64() {
        let sf = SnowflakeGenerator::new(7, 12).next_snowflake().unwrap();
        let id = sf.to_id_signed();

        let res = Snowflake::try_from(id);

        assert!(res.is_ok());
        assert_eq!(sf, res.unwrap());
    }

    #[test]
    fn test_snowflake_from_u64() {
        let w = 2;
        let p = 4;

        let sf = SnowflakeGenerator::new(2, 4).next_snowflake().unwrap();
        let id = sf.to_id();

        let res = Snowflake::try_from(id);

        assert!(res.is_ok());
        let id = res.unwrap();

        assert_eq!(id.worker_id(), w);
        assert_eq!(id.process_id(), p);
        assert_eq!(sf, id);
    }

    #[test]
    fn test_snowflake_string_equality() {
        let sf = Snowflake::new_from_components(123, 12, 6, 1);
        let id_str = sf.to_id().to_string();

        assert_eq!(sf, id_str);
        assert_eq!(id_str, sf);
    }

    #[test]
    fn test_snowflake_from_str() {
        let sf = Snowflake::new_from_components(123, 12, 6, 1);
        let id_str = sf.to_id().to_string();

        assert_eq!(sf, Snowflake::from_str(&id_str).unwrap());
    }

    #[test]
    fn test_snowflake_from_string() {
        let sf = Snowflake::new_from_components(123, 12, 6, 1);
        let id_str = sf.to_id().to_string();

        let res = Snowflake::try_from(id_str);

        assert!(res.is_ok());
        assert_eq!(sf, res.unwrap());
    }

    #[test]
    fn test_snowflake_from_invalid_str() {
        let invalid_id = "This is not a snowflake id";
        assert!(Snowflake::from_str(invalid_id).is_err());
    }

    #[test]
    fn test_snowflake_from_invalid_id() {
        let invalid_id = -232i64;
        let r = Snowflake::try_from(invalid_id);

        assert!(r.is_err());
    }

    #[test]
    fn test_snowflake_ordering() {
        let sg = SnowflakeGenerator::new(2, 2);
        let id1 = sg.next_snowflake().unwrap();
        let id2 = sg.next_snowflake().unwrap();

        assert!(id1 < id2);
    }
}
