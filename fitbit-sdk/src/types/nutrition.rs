//! Nutrition API Types
//!
//! This module contains the types and functions for the Fitbit Nutrition API.
//!
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error types for the Nutrition API
#[derive(Debug, Error)]
pub enum NutritionError {
    #[error("API request failed: {0}")]
    RequestFailed(String),
    #[error("API error: {0}")]
    ApiError(String),
}

impl From<String> for NutritionError {
    fn from(error: String) -> Self {
        NutritionError::ApiError(error)
    }
}

#[async_trait]
pub trait NutritionClient {
    async fn get_water_logs<'a>(&'a self, user_id: &'a str, date: &'a str) -> Result<WaterLog, NutritionError>;
    async fn get_food_logs<'a>(&'a self, user_id: &'a str, date: &'a str) -> Result<FoodLog, NutritionError>;
}

/// Water log information
#[derive(Debug, Deserialize)]
pub struct WaterLog {
    /// Water consumption summary
    pub summary: WaterSummary,
    /// Individual water log entries
    pub water: Vec<WaterEntry>,
}

/// Water consumption summary
#[derive(Debug, Deserialize)]
pub struct WaterSummary {
    /// Total water consumed in milliliters
    pub water: f64,
}

/// Individual water log entry
#[derive(Debug, Deserialize)]
pub struct WaterEntry {
    /// Log ID
    #[serde(rename = "logId")]
    pub log_id: i64,
    /// Amount of water in milliliters
    pub amount: f64,
    /// Time the water was logged
    pub time: String,
}

/// Food log information
#[derive(Debug, Deserialize)]
pub struct FoodLog {
    /// Food consumption summary
    pub summary: FoodSummary,
    /// Individual food log entries
    pub foods: Vec<FoodEntry>,
}

/// Food consumption summary
#[derive(Debug, Deserialize)]
pub struct FoodSummary {
    /// Total calories consumed
    pub calories: i32,
    /// Total carbohydrates in grams
    pub carbs: f64,
    /// Total fat in grams
    pub fat: f64,
    /// Total fiber in grams
    pub fiber: f64,
    /// Total protein in grams
    pub protein: f64,
    /// Total sodium in milligrams
    pub sodium: f64,
    /// Total water in milliliters
    pub water: f64,
}

/// Individual food log entry
#[derive(Debug, Deserialize)]
pub struct FoodEntry {
    /// Log ID
    #[serde(rename = "logId")]
    pub log_id: i64,
    /// Logged food information
    #[serde(rename = "loggedFood")]
    pub logged_food: LoggedFood,
    /// Nutritional values
    #[serde(rename = "nutritionalValues")]
    pub nutritional_values: NutritionalValues,
}

/// Logged food information
#[derive(Debug, Deserialize)]
pub struct LoggedFood {
    /// Meal the food was logged to
    #[serde(rename = "mealTypeId")]
    pub meal_type_id: i32,
    /// Food name
    pub name: String,
    /// Amount of the food
    pub amount: f64,
    /// Unit of measurement for the amount
    pub unit: Unit,
}

/// Unit of measurement for food
#[derive(Debug, Deserialize)]
pub struct Unit {
    /// ID of the unit
    pub id: i32,
    /// Name of the unit
    pub name: String,
    /// Plural name of the unit
    pub plural: String,
}

/// Nutritional values for a food item
#[derive(Debug, Deserialize)]
pub struct NutritionalValues {
    /// Calories
    pub calories: i32,
    /// Carbohydrates in grams
    pub carbs: f64,
    /// Fat in grams
    pub fat: f64,
    /// Fiber in grams
    pub fiber: f64,
    /// Protein in grams
    pub protein: f64,
    /// Sodium in milligrams
    pub sodium: f64,
}

/// Response wrapper for water logs
#[derive(Debug, Deserialize)]
pub struct WaterLogResponse {
    #[serde(flatten)]
    pub water_log: WaterLog,
}

/// Response wrapper for food logs
#[derive(Debug, Deserialize)]
pub struct FoodLogResponse {
    #[serde(flatten)]
    pub food_log: FoodLog,
}
