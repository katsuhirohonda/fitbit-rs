//! Nutrition API
//!
//! This module contains the implementations for the Fitbit Nutrition API endpoints.
//! It provides functionality for retrieving nutrition data and food logs.

use crate::client::FitbitClient;
use crate::types::nutrition::{
    NutritionClient, NutritionError, WaterLog, WaterLogResponse, FoodLog, FoodLogResponse,
};
use async_trait::async_trait;

#[async_trait]
impl NutritionClient for FitbitClient {
    /// Gets the user's water logs for a specific date
    ///
    /// Retrieves a summary and list of a user's water log entries for a given day.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID to get water logs for, or "-" for current user
    /// * `date` - The date in format YYYY-MM-DD
    ///
    /// # Returns
    ///
    /// Returns the water logs for the specified date on success.
    ///
    /// # Errors
    ///
    /// Returns a `NutritionError` if:
    /// - The request fails to send
    /// - The API returns an error response
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use fitbit_sdk::client::FitbitClient;
    /// use fitbit_sdk::types::nutrition::{NutritionClient, NutritionError};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), NutritionError> {
    ///     let client = FitbitClient::new::<NutritionError>()?;
    ///
    ///     // Get today's water consumption
    ///     let water_logs = client.get_water_logs("-", "today").await?;
    ///     println!("Total water: {} ml", water_logs.summary.water);
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn get_water_logs<'a>(
        &'a self,
        user_id: &'a str,
        date: &'a str,
    ) -> Result<WaterLog, NutritionError> {
        let path = format!("/user/{}/foods/log/water/date/{}.json", user_id, date);
        let response: WaterLogResponse = self.get(&path, Option::<&()>::None).await?;
        Ok(response.water_log)
    }

    /// Gets the user's food logs for a specific date
    ///
    /// Retrieves a summary and list of a user's food log entries for a given day.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID to get food logs for, or "-" for current user
    /// * `date` - The date in format YYYY-MM-DD
    ///
    /// # Returns
    ///
    /// Returns the food logs for the specified date on success.
    ///
    /// # Errors
    ///
    /// Returns a `NutritionError` if:
    /// - The request fails to send
    /// - The API returns an error response
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use fitbit_sdk::client::FitbitClient;
    /// use fitbit_sdk::types::nutrition::{NutritionClient, NutritionError};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), NutritionError> {
    ///     let client = FitbitClient::new::<NutritionError>()?;
    ///
    ///     // Get today's food logs
    ///     let food_logs = client.get_food_logs("-", "today").await?;
    ///     println!("Total calories: {}", food_logs.summary.calories);
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn get_food_logs<'a>(
        &'a self,
        user_id: &'a str,
        date: &'a str,
    ) -> Result<FoodLog, NutritionError> {
        let path = format!("/user/{}/foods/log/date/{}.json", user_id, date);
        let response: FoodLogResponse = self.get(&path, Option::<&()>::None).await?;
        Ok(response.food_log)
    }
}
