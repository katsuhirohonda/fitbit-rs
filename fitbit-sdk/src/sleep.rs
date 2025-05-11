//! Sleep API
//!
//! This module contains the implementations for the Fitbit Sleep API endpoints.
//! It provides functionality for retrieving sleep data and logs.

use crate::client::FitbitClient;
use crate::types::sleep::{
    SleepClient, SleepError, SleepLog, SleepLogResponse, SleepGoal, SleepGoalResponse,
};
use async_trait::async_trait;

#[async_trait]
impl SleepClient for FitbitClient {
    /// Gets the sleep logs for a specific date
    ///
    /// Retrieves a summary and list of a user's sleep log entries
    /// (including naps) as well as detailed sleep entry data for a given day.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID to get sleep logs for, or "-" for current user
    /// * `date` - The date in format YYYY-MM-DD
    ///
    /// # Returns
    ///
    /// Returns the sleep logs for the specified date on success.
    ///
    /// # Errors
    ///
    /// Returns a `SleepError` if:
    /// - The request fails to send
    /// - The API returns an error response
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use fitbit_sdk::client::FitbitClient;
    /// use fitbit_sdk::types::sleep::{SleepClient, SleepError};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), SleepError> {
    ///     let client = FitbitClient::new::<SleepError>()?;
    ///
    ///     // Get today's sleep data
    ///     let sleep_logs = client.get_sleep_logs("-", "today").await?;
    ///     println!("Total sleep: {} minutes", sleep_logs.summary.total_minutes_asleep);
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn get_sleep_logs<'a>(
        &'a self,
        user_id: &'a str,
        date: &'a str,
    ) -> Result<SleepLog, SleepError> {
        let path = format!("/user/{}/sleep/date/{}.json", user_id, date);
        let response: SleepLogResponse = self.get(&path, Option::<&()>::None).await?;
        Ok(response.sleep_log)
    }

    /// Gets the user's sleep goal
    ///
    /// Retrieves the user's current sleep goal.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID to get sleep goal for, or "-" for current user
    ///
    /// # Returns
    ///
    /// Returns the sleep goal on success.
    ///
    /// # Errors
    ///
    /// Returns a `SleepError` if:
    /// - The request fails to send
    /// - The API returns an error response
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use fitbit_sdk::client::FitbitClient;
    /// use fitbit_sdk::types::sleep::{SleepClient, SleepError};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), SleepError> {
    ///     let client = FitbitClient::new::<SleepError>()?;
    ///
    ///     // Get sleep goal
    ///     let goal = client.get_sleep_goal("-").await?;
    ///     println!("Sleep goal: {} minutes", goal.goal);
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn get_sleep_goal<'a>(&'a self, user_id: &'a str) -> Result<SleepGoal, SleepError> {
        let path = format!("/user/{}/sleep/goal.json", user_id);
        let response: SleepGoalResponse = self.get(&path, Option::<&()>::None).await?;
        Ok(response.goal)
    }
}
