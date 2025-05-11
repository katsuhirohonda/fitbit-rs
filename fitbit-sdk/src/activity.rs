//! Activity API
//!
//! This module contains the implementations for the Fitbit Activity API endpoints.
//! It provides functionality for retrieving activity data and statistics.

use crate::client::FitbitClient;
use crate::types::activity::{
    ActivityClient, ActivityError, ActivitySummary, ActivitySummaryResponse, ActivityTimeSeries,
    ActivityLifetimeStats, LifetimeStatsResponse, Resource,
};
use async_trait::async_trait;

#[async_trait]
impl ActivityClient for FitbitClient {
    /// Gets the daily activity summary
    ///
    /// Retrieves the activity summary for a specific date.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID to get activity summary for, or "-" for current user
    /// * `date` - The date in format YYYY-MM-DD
    ///
    /// # Returns
    ///
    /// Returns the daily activity summary on success.
    ///
    /// # Errors
    ///
    /// Returns an `ActivityError` if:
    /// - The request fails to send
    /// - The API returns an error response
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use fitbit_sdk::client::FitbitClient;
    /// use fitbit_sdk::types::activity::{ActivityClient, ActivityError};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), ActivityError> {
    ///     let client = FitbitClient::new::<ActivityError>()?;
    ///
    ///     // Get today's activity summary
    ///     let summary = client.get_activity_summary("-", "today").await?;
    ///     println!("Steps: {}", summary.steps);
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn get_activity_summary<'a>(
        &'a self,
        user_id: &'a str,
        date: &'a str,
    ) -> Result<ActivitySummary, ActivityError> {
        let path = format!("/user/{}/activities/date/{}.json", user_id, date);
        let response: ActivitySummaryResponse = self.get(&path, Option::<&()>::None).await?;
        Ok(response.summary)
    }

    /// Gets activity time series data
    ///
    /// Retrieves activity time series data for a specific resource over a period.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID to get activity time series for, or "-" for current user
    /// * `resource` - The resource type (e.g., steps, calories, distance)
    /// * `date` - The base date in format YYYY-MM-DD
    /// * `period` - The period (1d, 7d, 30d, 1w, 1m, 3m, 6m, 1y, max)
    ///
    /// # Returns
    ///
    /// Returns the activity time series data on success.
    ///
    /// # Errors
    ///
    /// Returns an `ActivityError` if:
    /// - The request fails to send
    /// - The API returns an error response
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use fitbit_sdk::client::FitbitClient;
    /// use fitbit_sdk::types::activity::{ActivityClient, ActivityError, Resource};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), ActivityError> {
    ///     let client = FitbitClient::new::<ActivityError>()?;
    ///
    ///     // Get last 7 days of steps data
    ///     let steps_data = client.get_activity_time_series("-", Resource::Steps, "today", "7d").await?;
    ///     
    ///     for data_point in &steps_data {
    ///         println!("{}: {} steps", data_point.datetime, data_point.value);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn get_activity_time_series<'a>(
        &'a self,
        user_id: &'a str,
        resource: Resource,
        date: &'a str,
        period: &'a str,
    ) -> Result<Vec<ActivityTimeSeries>, ActivityError> {
        let path = format!(
            "/user/{}/activities/{}/date/{}/{}.json",
            user_id,
            resource.as_str(),
            date,
            period
        );
        let response: serde_json::Value = self.get(&path, Option::<&()>::None).await?;
        
        // The response format differs based on resource type
        let key = format!("activities-{}", resource.as_str());
        let time_series: Vec<ActivityTimeSeries> = response
            .get(&key)
            .ok_or_else(|| ActivityError::from(format!("Missing key '{}' in response", key)))?
            .as_array()
            .ok_or_else(|| ActivityError::from("Expected array for time series data".to_string()))?
            .iter()
            .map(|item| serde_json::from_value(item.clone()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| ActivityError::from(e.to_string()))?;
            
        Ok(time_series)
    }

    /// Gets lifetime activity statistics
    ///
    /// Retrieves the lifetime statistics of an active user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID to get lifetime stats for, or "-" for current user
    ///
    /// # Returns
    ///
    /// Returns the lifetime activity statistics on success.
    ///
    /// # Errors
    ///
    /// Returns an `ActivityError` if:
    /// - The request fails to send
    /// - The API returns an error response
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use fitbit_sdk::client::FitbitClient;
    /// use fitbit_sdk::types::activity::{ActivityClient, ActivityError};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), ActivityError> {
    ///     let client = FitbitClient::new::<ActivityError>()?;
    ///
    ///     // Get lifetime stats
    ///     let stats = client.get_lifetime_stats("-").await?;
    ///     println!("Total distance: {}", stats.lifetime.total.distance);
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn get_lifetime_stats<'a>(&'a self, user_id: &'a str) -> Result<ActivityLifetimeStats, ActivityError> {
        let path = format!("/user/{}/activities.json", user_id);
        let response: LifetimeStatsResponse = self.get(&path, Option::<&()>::None).await?;
        Ok(response.lifetime)
    }
}
