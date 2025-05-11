//! Body API
//!
//! This module contains the implementations for the Fitbit Body API endpoints.
//! It provides functionality for retrieving body measurements and goals.

use crate::client::FitbitClient;
use crate::types::body::{
    BodyClient, BodyError, BodyWeight, BodyFat, BodyGoals, WeightLogResponse, BodyFatResponse, BodyGoalsResponse,
};
use async_trait::async_trait;

#[async_trait]
impl BodyClient for FitbitClient {
    /// Gets the user's body weight for a specific date
    ///
    /// Retrieves a list of all user's body weight log entries for a given date.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID to get weight for, or "-" for current user
    /// * `date` - The date in format YYYY-MM-DD
    ///
    /// # Returns
    ///
    /// Returns the body weight logs for the specified date on success.
    ///
    /// # Errors
    ///
    /// Returns a `BodyError` if:
    /// - The request fails to send
    /// - The API returns an error response
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use fitbit_sdk::client::FitbitClient;
    /// use fitbit_sdk::types::body::{BodyClient, BodyError};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), BodyError> {
    ///     let client = FitbitClient::new::<BodyError>()?;
    ///
    ///     // Get today's weight data
    ///     let weights = client.get_body_weight("-", "today").await?;
    ///     if let Some(weight) = weights.first() {
    ///         println!("Weight: {} {}", weight.weight, if weight.weight_in_kg.is_some() { "kg" } else { "lbs" });
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn get_body_weight<'a>(
        &'a self,
        user_id: &'a str,
        date: &'a str,
    ) -> Result<Vec<BodyWeight>, BodyError> {
        let path = format!("/user/{}/body/log/weight/date/{}.json", user_id, date);
        let response: WeightLogResponse = self.get(&path, Option::<&()>::None).await?;
        Ok(response.weight)
    }

    /// Gets the user's body fat percentage for a specific date
    ///
    /// Retrieves a list of all user's body fat percent log entries for a given date.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID to get body fat for, or "-" for current user
    /// * `date` - The date in format YYYY-MM-DD
    ///
    /// # Returns
    ///
    /// Returns the body fat logs for the specified date on success.
    ///
    /// # Errors
    ///
    /// Returns a `BodyError` if:
    /// - The request fails to send
    /// - The API returns an error response
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use fitbit_sdk::client::FitbitClient;
    /// use fitbit_sdk::types::body::{BodyClient, BodyError};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), BodyError> {
    ///     let client = FitbitClient::new::<BodyError>()?;
    ///
    ///     // Get today's body fat data
    ///     let fat_logs = client.get_body_fat("-", "today").await?;
    ///     if let Some(fat) = fat_logs.first() {
    ///         println!("Body fat: {}%", fat.fat);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn get_body_fat<'a>(
        &'a self,
        user_id: &'a str,
        date: &'a str,
    ) -> Result<Vec<BodyFat>, BodyError> {
        let path = format!("/user/{}/body/log/fat/date/{}.json", user_id, date);
        let response: BodyFatResponse = self.get(&path, Option::<&()>::None).await?;
        Ok(response.fat)
    }

    /// Gets the user's body goals
    ///
    /// Retrieves a user's current weight and body fat percentage goals.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID to get body goals for, or "-" for current user
    ///
    /// # Returns
    ///
    /// Returns the body goals on success.
    ///
    /// # Errors
    ///
    /// Returns a `BodyError` if:
    /// - The request fails to send
    /// - The API returns an error response
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use fitbit_sdk::client::FitbitClient;
    /// use fitbit_sdk::types::body::{BodyClient, BodyError};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), BodyError> {
    ///     let client = FitbitClient::new::<BodyError>()?;
    ///
    ///     // Get body goals
    ///     let goals = client.get_body_goals("-").await?;
    ///     println!("Weight goal: {} {}", goals.goal.weight, goals.goal.weight_unit);
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn get_body_goals<'a>(&'a self, user_id: &'a str) -> Result<BodyGoals, BodyError> {
        let path = format!("/user/{}/body/goals.json", user_id);
        let response: BodyGoalsResponse = self.get(&path, Option::<&()>::None).await?;
        Ok(response.goal)
    }
}
