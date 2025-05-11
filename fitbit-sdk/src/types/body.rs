//! Body API Types
//!
//! This module contains the types and functions for the Fitbit Body API.
//!
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error types for the Body API
#[derive(Debug, Error)]
pub enum BodyError {
    #[error("API request failed: {0}")]
    RequestFailed(String),
    #[error("API error: {0}")]
    ApiError(String),
}

impl From<String> for BodyError {
    fn from(error: String) -> Self {
        BodyError::ApiError(error)
    }
}

#[async_trait]
pub trait BodyClient {
    async fn get_body_weight<'a>(&'a self, user_id: &'a str, date: &'a str) -> Result<Vec<BodyWeight>, BodyError>;
    async fn get_body_fat<'a>(&'a self, user_id: &'a str, date: &'a str) -> Result<Vec<BodyFat>, BodyError>;
    async fn get_body_goals<'a>(&'a self, user_id: &'a str) -> Result<BodyGoals, BodyError>;
}

/// Body weight log entry
#[derive(Debug, Deserialize)]
pub struct BodyWeight {
    /// Date of the weight measurement
    pub date: String,
    /// Time of the weight measurement
    pub time: String,
    /// Weight value in user's preferred unit
    pub weight: f64,
    /// Weight value in kilograms (if available)
    #[serde(rename = "weightInKg")]
    pub weight_in_kg: Option<f64>,
    /// Log ID
    #[serde(rename = "logId")]
    pub log_id: i64,
    /// Source of the log entry
    pub source: Option<String>,
}

/// Body fat percentage log entry
#[derive(Debug, Deserialize)]
pub struct BodyFat {
    /// Date of the body fat measurement
    pub date: String,
    /// Time of the body fat measurement
    pub time: String,
    /// Body fat percentage
    pub fat: f64,
    /// Log ID
    #[serde(rename = "logId")]
    pub log_id: i64,
    /// Source of the log entry
    pub source: Option<String>,
}

/// Body goals information
#[derive(Debug, Deserialize)]
pub struct BodyGoals {
    /// Weight goal
    pub weight: f64,
    /// Weight unit for the goal
    #[serde(rename = "weightUnit")]
    pub weight_unit: String,
    /// Body fat percentage goal
    pub fat: Option<f64>,
}

/// Response wrapper for weight logs
#[derive(Debug, Deserialize)]
pub struct WeightLogResponse {
    pub weight: Vec<BodyWeight>,
}

/// Response wrapper for body fat logs
#[derive(Debug, Deserialize)]
pub struct BodyFatResponse {
    pub fat: Vec<BodyFat>,
}

/// Response wrapper for body goals
#[derive(Debug, Deserialize)]
pub struct BodyGoalsResponse {
    pub goal: BodyGoals,
}
