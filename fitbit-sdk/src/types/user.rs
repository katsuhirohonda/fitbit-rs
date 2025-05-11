//! User API Types
//!
//! This module contains the types and functions for the Fitbit User API.
//!
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use time::Date;

/// Error types for the User API
#[derive(Debug, Error)]
pub enum UserError {
    #[error("API request failed: {0}")]
    RequestFailed(String),
    #[error("API error: {0}")]
    ApiError(String),
}

impl From<String> for UserError {
    fn from(error: String) -> Self {
        UserError::ApiError(error)
    }
}

#[async_trait]
pub trait UserClient {
    async fn get_profile<'a>(&'a self, user_id: &'a str) -> Result<UserProfile, UserError>;
    async fn update_profile<'a>(&'a self, params: &'a UpdateProfileParams) -> Result<UserProfile, UserError>;
}

/// User profile information
#[derive(Debug, Deserialize)]
pub struct UserProfile {
    /// First and last name of the user
    #[serde(rename = "fullName")]
    pub full_name: String,
    /// Display name for the user within the Fitbit UI
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// User's date of birth
    #[serde(rename = "dateOfBirth")]
    pub date_of_birth: String,
    /// Gender assigned to the user on the Fitbit website profile
    pub gender: Gender,
    /// Length units for the user (METRIC or US)
    #[serde(rename = "heightUnit")]
    pub height_unit: HeightUnit,
    /// Weight units for the user (METRIC or US)
    #[serde(rename = "weightUnit")]
    pub weight_unit: WeightUnit,
    /// Height for the user in the format X'Y" or decimal
    pub height: String,
    /// The Weight of the user in their default unit
    pub weight: Option<f64>,
    /// The average number of daily steps the user takes
    #[serde(rename = "averageDailySteps")]
    pub average_daily_steps: i32,
    /// The user's avatar image URL
    pub avatar: String,
    /// The user's avatar image URL (medium)
    #[serde(rename = "avatar150")]
    pub avatar150: String,
    /// The user's avatar image URL (big)
    #[serde(rename = "avatar640")]
    pub avatar640: String,
}

/// Gender enumeration
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Gender {
    Male,
    Female,
    Na,
}

/// Height unit enumeration
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HeightUnit {
    Metric,
    Us,
}

/// Weight unit enumeration
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum WeightUnit {
    Metric,
    Us,
}

/// Response wrapper for user profile
#[derive(Debug, Deserialize)]
pub struct UserProfileResponse {
    pub user: UserProfile,
}

/// Parameters for updating user profile
#[derive(Debug, Serialize, Default)]
pub struct UpdateProfileParams {
    /// First and last name of the user
    #[serde(rename = "fullName", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    /// Display name for the user within the Fitbit UI
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// User's date of birth
    #[serde(rename = "dateOfBirth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// Gender assigned to the user on the Fitbit website profile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,
    /// Length units for the user (METRIC or US)
    #[serde(rename = "heightUnit", skip_serializing_if = "Option::is_none")]
    pub height_unit: Option<HeightUnit>,
    /// Weight units for the user (METRIC or US)
    #[serde(rename = "weightUnit", skip_serializing_if = "Option::is_none")]
    pub weight_unit: Option<WeightUnit>,
    /// Height for the user in the format X'Y" or decimal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
}

impl UpdateProfileParams {
    /// Create a new UpdateProfileParams with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the full name
    pub fn with_full_name(mut self, full_name: impl Into<String>) -> Self {
        self.full_name = Some(full_name.into());
        self
    }

    /// Set the display name
    pub fn with_display_name(mut self, display_name: impl Into<String>) -> Self {
        self.display_name = Some(display_name.into());
        self
    }

    /// Set the date of birth
    pub fn with_date_of_birth(mut self, date_of_birth: impl Into<String>) -> Self {
        self.date_of_birth = Some(date_of_birth.into());
        self
    }

    /// Set the gender
    pub fn with_gender(mut self, gender: Gender) -> Self {
        self.gender = Some(gender);
        self
    }

    /// Set the height unit
    pub fn with_height_unit(mut self, height_unit: HeightUnit) -> Self {
        self.height_unit = Some(height_unit);
        self
    }

    /// Set the weight unit
    pub fn with_weight_unit(mut self, weight_unit: WeightUnit) -> Self {
        self.weight_unit = Some(weight_unit);
        self
    }

    /// Set the height
    pub fn with_height(mut self, height: impl Into<String>) -> Self {
        self.height = Some(height.into());
        self
    }
}
