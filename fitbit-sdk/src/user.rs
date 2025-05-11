//! User API
//!
//! This module contains the implementations for the Fitbit User API endpoints.
//! It provides functionality for getting and updating user profile information.

use crate::client::FitbitClient;
use crate::types::user::{
    UpdateProfileParams, UserClient, UserError, UserProfile, UserProfileResponse,
};
use async_trait::async_trait;

#[async_trait]
impl UserClient for FitbitClient {
    /// Gets the user's profile information
    ///
    /// Retrieves the profile information for the specified user.
    /// Use "-" to retrieve the authenticated user's profile.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID to get profile for, or "-" for current user
    ///
    /// # Returns
    ///
    /// Returns the user's profile information on success.
    ///
    /// # Errors
    ///
    /// Returns a `UserError` if:
    /// - The request fails to send
    /// - The API returns an error response
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use fitbit_sdk::client::FitbitClient;
    /// use fitbit_sdk::types::user::{UserClient, UserError};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), UserError> {
    ///     let client = FitbitClient::new::<UserError>()?;
    ///
    ///     // Get authenticated user's profile
    ///     let profile = client.get_profile("-").await?;
    ///     println!("User: {}", profile.display_name);
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn get_profile<'a>(&'a self, user_id: &'a str) -> Result<UserProfile, UserError> {
        let path = format!("/user/{}/profile.json", user_id);
        let response: UserProfileResponse = self.get(&path, Option::<&()>::None).await?;
        Ok(response.user)
    }

    /// Updates the user's profile information
    ///
    /// Updates the profile information for the authenticated user.
    ///
    /// # Arguments
    ///
    /// * `params` - The profile parameters to update
    ///
    /// # Returns
    ///
    /// Returns the updated user profile on success.
    ///
    /// # Errors
    ///
    /// Returns a `UserError` if:
    /// - The request fails to send
    /// - The API returns an error response
    /// - The response cannot be parsed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use fitbit_sdk::client::FitbitClient;
    /// use fitbit_sdk::types::user::{UpdateProfileParams, UserClient, UserError, HeightUnit, WeightUnit};
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), UserError> {
    ///     let client = FitbitClient::new::<UserError>()?;
    ///
    ///     let params = UpdateProfileParams::new()
    ///         .with_display_name("John Doe")
    ///         .with_height_unit(HeightUnit::Us)
    ///         .with_weight_unit(WeightUnit::Us);
    ///
    ///     let profile = client.update_profile(&params).await?;
    ///     println!("Updated display name: {}", profile.display_name);
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn update_profile<'a>(
        &'a self,
        params: &'a UpdateProfileParams,
    ) -> Result<UserProfile, UserError> {
        let path = "/user/-/profile.json";
        let response: UserProfileResponse = self.post(path, Some(params)).await?;
        Ok(response.user)
    }
}
