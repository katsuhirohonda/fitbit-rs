//! Sleep API Types
//!
//! This module contains the types and functions for the Fitbit Sleep API.
//!
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error types for the Sleep API
#[derive(Debug, Error)]
pub enum SleepError {
    #[error("API request failed: {0}")]
    RequestFailed(String),
    #[error("API error: {0}")]
    ApiError(String),
}

impl From<String> for SleepError {
    fn from(error: String) -> Self {
        SleepError::ApiError(error)
    }
}

#[async_trait]
pub trait SleepClient {
    async fn get_sleep_logs<'a>(&'a self, user_id: &'a str, date: &'a str) -> Result<SleepLog, SleepError>;
    async fn get_sleep_goal<'a>(&'a self, user_id: &'a str) -> Result<SleepGoal, SleepError>;
}

/// Sleep log information
#[derive(Debug, Deserialize)]
pub struct SleepLog {
    /// Sleep summary
    pub summary: SleepSummary,
    /// List of sleep entries
    pub sleep: Vec<SleepEntry>,
}

/// Sleep summary for a day
#[derive(Debug, Deserialize)]
pub struct SleepSummary {
    /// Total number of sleep records
    #[serde(rename = "totalSleepRecords")]
    pub total_sleep_records: i32,
    /// Total time in bed in minutes
    #[serde(rename = "totalTimeInBed")]
    pub total_time_in_bed: i32,
    /// Total minutes asleep
    #[serde(rename = "totalMinutesAsleep")]
    pub total_minutes_asleep: i32,
}

/// Individual sleep entry
#[derive(Debug, Deserialize)]
pub struct SleepEntry {
    /// Log ID for the sleep entry
    #[serde(rename = "logId")]
    pub log_id: i64,
    /// Start time of sleep
    #[serde(rename = "startTime")]
    pub start_time: String,
    /// End time of sleep
    #[serde(rename = "endTime")]
    pub end_time: String,
    /// Duration in milliseconds
    pub duration: i64,
    /// Minutes in bed before falling asleep
    #[serde(rename = "minutesToFallAsleep")]
    pub minutes_to_fall_asleep: i32,
    /// Time spent in bed in minutes
    #[serde(rename = "timeInBed")]
    pub time_in_bed: i32,
    /// Minutes asleep
    #[serde(rename = "minutesAsleep")]
    pub minutes_asleep: i32,
    /// Efficiency score (percentage)
    pub efficiency: i32,
    /// Type of sleep entry
    #[serde(rename = "type")]
    pub type_: String,
    /// Main sleep or nap
    #[serde(rename = "isMainSleep")]
    pub is_main_sleep: bool,
    /// Sleep levels data
    pub levels: Option<SleepLevels>,
}

/// Sleep levels data
#[derive(Debug, Deserialize)]
pub struct SleepLevels {
    /// Summary of time spent in each sleep level
    pub summary: SleepLevelsSummary,
    /// Detailed data of sleep stages throughout the night
    pub data: Vec<SleepLevelData>,
}

/// Summary of time spent in each sleep level
#[derive(Debug, Deserialize)]
pub struct SleepLevelsSummary {
    /// Time spent in REM sleep
    pub rem: Option<SleepLevelSummary>,
    /// Time spent in deep sleep
    pub deep: Option<SleepLevelSummary>,
    /// Time spent in light sleep
    pub light: Option<SleepLevelSummary>,
    /// Time spent awake
    pub wake: Option<SleepLevelSummary>,
}

/// Individual sleep level summary
#[derive(Debug, Deserialize)]
pub struct SleepLevelSummary {
    /// Number of minutes in this sleep level
    pub minutes: i32,
    /// Number of times this sleep level occurred
    pub count: i32,
}

/// Individual sleep level data point
#[derive(Debug, Deserialize)]
pub struct SleepLevelData {
    /// Date-time for this data point
    pub datetime: String,
    /// Sleep level (wake, rem, light, deep)
    pub level: String,
    /// Number of seconds in this level
    pub seconds: i32,
}

/// User's sleep goal
#[derive(Debug, Deserialize)]
pub struct SleepGoal {
    /// Sleep goal in minutes
    pub goal: i32,
}

/// Response wrapper for sleep logs
#[derive(Debug, Deserialize)]
pub struct SleepLogResponse {
    #[serde(flatten)]
    pub sleep_log: SleepLog,
}

/// Response wrapper for sleep goal
#[derive(Debug, Deserialize)]
pub struct SleepGoalResponse {
    pub goal: SleepGoal,
}
