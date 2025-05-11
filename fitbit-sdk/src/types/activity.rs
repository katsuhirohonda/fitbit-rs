//! Activity API Types
//!
//! This module contains the types and functions for the Fitbit Activity API.
//!
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error types for the Activity API
#[derive(Debug, Error)]
pub enum ActivityError {
    #[error("API request failed: {0}")]
    RequestFailed(String),
    #[error("API error: {0}")]
    ApiError(String),
}

impl From<String> for ActivityError {
    fn from(error: String) -> Self {
        ActivityError::ApiError(error)
    }
}

#[async_trait]
pub trait ActivityClient {
    async fn get_activity_summary<'a>(
        &'a self,
        user_id: &'a str,
        date: &'a str,
    ) -> Result<ActivitySummary, ActivityError>;

    async fn get_activity_time_series<'a>(
        &'a self,
        user_id: &'a str,
        resource: Resource,
        date: &'a str,
        period: &'a str,
    ) -> Result<Vec<ActivityTimeSeries>, ActivityError>;

    async fn get_lifetime_stats<'a>(&'a self, user_id: &'a str) -> Result<ActivityLifetimeStats, ActivityError>;
}

/// Activity summary for a specific date
#[derive(Debug, Deserialize)]
pub struct ActivitySummary {
    /// Total steps taken for the day
    pub steps: i32,
    /// Total distance traveled for the day
    pub distances: Vec<Distance>,
    /// Total calories burned for the day
    pub calories: i32,
    /// Total number of floors climbed for the day
    pub floors: Option<i32>,
    /// Total active minutes for the day
    #[serde(rename = "sedentaryMinutes")]
    pub sedentary_minutes: i32,
    /// Minutes spent in light activity
    #[serde(rename = "lightlyActiveMinutes")]
    pub lightly_active_minutes: i32,
    /// Minutes spent in moderate activity
    #[serde(rename = "fairlyActiveMinutes")]
    pub fairly_active_minutes: i32,
    /// Minutes spent in intense activity
    #[serde(rename = "veryActiveMinutes")]
    pub very_active_minutes: i32,
    /// Rest heart rate
    #[serde(rename = "restingHeartRate")]
    pub resting_heart_rate: Option<i32>,
}

/// Distance information for various activity types
#[derive(Debug, Deserialize)]
pub struct Distance {
    pub activity: String,
    pub distance: f64,
}

/// Activity resource types for time series
#[derive(Debug, Clone, Copy)]
pub enum Resource {
    Steps,
    Calories,
    Distance,
    Floors,
    Minutes,
    ActiveMinutes,
    SedentaryMinutes,
    LightlyActiveMinutes,
    FairlyActiveMinutes,
    VeryActiveMinutes,
}

impl Resource {
    pub fn as_str(&self) -> &'static str {
        match self {
            Resource::Steps => "steps",
            Resource::Calories => "calories",
            Resource::Distance => "distance",
            Resource::Floors => "floors",
            Resource::Minutes => "minutes",
            Resource::ActiveMinutes => "minutesAsleep",
            Resource::SedentaryMinutes => "minutesSedentary",
            Resource::LightlyActiveMinutes => "minutesLightlyActive",
            Resource::FairlyActiveMinutes => "minutesFairlyActive",
            Resource::VeryActiveMinutes => "minutesVeryActive",
        }
    }
}

/// Activity time series data point
#[derive(Debug, Deserialize)]
pub struct ActivityTimeSeries {
    /// Date for the data point
    pub datetime: String,
    /// Value for the data point
    pub value: String,
}

/// Lifetime activity statistics
#[derive(Debug, Deserialize)]
pub struct ActivityLifetimeStats {
    /// Best day statistics
    pub best: BestStats,
    /// Total lifetime statistics
    pub total: TotalStats,
}

/// Best day statistics
#[derive(Debug, Deserialize)]
pub struct BestStats {
    /// Best total distance
    pub total: BestTotal,
    /// Best tracker distance
    pub tracker: BestTracker,
}

/// Best total statistics
#[derive(Debug, Deserialize)]
pub struct BestTotal {
    /// Best total distance
    pub distance: BestDistance,
    /// Best total steps
    pub steps: BestSteps,
    /// Best total floors
    pub floors: Option<BestFloors>,
}

/// Best tracker statistics
#[derive(Debug, Deserialize)]
pub struct BestTracker {
    /// Best tracker distance
    pub distance: BestDistance,
    /// Best tracker steps
    pub steps: BestSteps,
    /// Best tracker floors
    pub floors: Option<BestFloors>,
}

/// Best distance information
#[derive(Debug, Deserialize)]
pub struct BestDistance {
    pub date: String,
    pub value: f64,
}

/// Best steps information
#[derive(Debug, Deserialize)]
pub struct BestSteps {
    pub date: String,
    pub value: i32,
}

/// Best floors information
#[derive(Debug, Deserialize)]
pub struct BestFloors {
    pub date: String,
    pub value: i32,
}

/// Total lifetime statistics
#[derive(Debug, Deserialize)]
pub struct TotalStats {
    /// Total lifetime distance
    pub distance: f64,
    /// Total lifetime steps
    pub steps: i64,
    /// Total lifetime floors
    pub floors: Option<i64>,
}

/// Response wrapper for activity summary
#[derive(Debug, Deserialize)]
pub struct ActivitySummaryResponse {
    pub summary: ActivitySummary,
}

/// Response wrapper for lifetime statistics
#[derive(Debug, Deserialize)]
pub struct LifetimeStatsResponse {
    pub lifetime: ActivityLifetimeStats,
}
