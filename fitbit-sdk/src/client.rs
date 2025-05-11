//! Fitbit API client implementation
//!
//! This module provides the main client for interacting with the Fitbit API.
//! It handles authentication, request construction, and response parsing.

use reqwest::Client as ReqwestClient;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::error::Error as StdError;

/// Fitbit API client
///
/// The main client for making requests to the Fitbit API.
/// Handles authentication and provides methods for making API requests.
///
/// # Examples
///
/// ```no_run
/// use fitbit_sdk::client::FitbitClient;
/// use fitbit_sdk::types::user::UserError;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Basic usage
/// let client = FitbitClient::new::<UserError>()?;
///
/// // Using the builder pattern
/// let client_with_token = FitbitClient::builder()
///     .with_access_token("your-access-token")
///     .build::<UserError>()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct FitbitClient {
    /// The underlying HTTP client for making requests
    client: ReqwestClient,
    /// The OAuth access token used for authentication
    access_token: String,
    /// The base URL for the Fitbit API
    api_base_url: String,
}

/// Builder for FitbitClient
///
/// Provides a flexible way to configure and create a FitbitClient.
pub struct FitbitClientBuilder {
    access_token: Option<String>,
    api_base_url: String,
    client: Option<ReqwestClient>,
}

impl FitbitClientBuilder {
    /// Creates a new builder with default configuration
    pub fn new() -> Self {
        Self {
            access_token: None,
            api_base_url: FitbitClient::DEFAULT_API_BASE_URL.to_string(),
            client: None,
        }
    }

    /// Sets the OAuth access token
    pub fn with_access_token(mut self, access_token: impl Into<String>) -> Self {
        self.access_token = Some(access_token.into());
        self
    }

    /// Sets a custom API base URL
    pub fn with_api_base_url(mut self, api_base_url: impl Into<String>) -> Self {
        self.api_base_url = api_base_url.into();
        self
    }

    /// Sets a custom HTTP client
    pub fn with_http_client(mut self, client: ReqwestClient) -> Self {
        self.client = Some(client);
        self
    }

    /// Builds the FitbitClient with the specified configuration
    pub fn build<E>(self) -> Result<FitbitClient, E>
    where
        E: StdError + From<String>,
    {
        // Get access token from environment or builder
        let access_token = self.access_token
            .or_else(|| std::env::var("FITBIT_ACCESS_TOKEN").ok())
            .ok_or_else(|| E::from("Access token must be provided either via builder or FITBIT_ACCESS_TOKEN environment variable".to_string()))?;

        // Use provided client or create a new one
        let client = if let Some(client) = self.client {
            client
        } else {
            ReqwestClient::builder()
                .user_agent(FitbitClient::DEFAULT_USER_AGENT)
                .build()
                .map_err(|e| E::from(e.to_string()))?
        };

        Ok(FitbitClient {
            client,
            access_token,
            api_base_url: self.api_base_url,
        })
    }
}

impl FitbitClient {
    /// Base URL for the Fitbit API
    pub const DEFAULT_API_BASE_URL: &str = "https://api.fitbit.com/1";

    /// Our user agent.
    pub const DEFAULT_USER_AGENT: &'static str =
        concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION"));

    pub fn get_client(&self) -> &ReqwestClient {
        &self.client
    }

    pub fn get_access_token(&self) -> &str {
        &self.access_token
    }

    pub fn get_api_base_url(&self) -> &str {
        &self.api_base_url
    }

    /// Creates a new FitbitClient builder
    pub fn builder() -> FitbitClientBuilder {
        FitbitClientBuilder::new()
    }

    /// Creates a new Fitbit API client with default configuration
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The access token is not available in the environment
    /// - The HTTP client cannot be initialized
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use fitbit_sdk::client::FitbitClient;
    /// # use fitbit_sdk::types::user::UserError;
    /// let client = FitbitClient::new::<UserError>().unwrap();
    /// ```
    pub fn new<E>() -> Result<Self, E>
    where
        E: StdError + From<String>,
    {
        Self::builder().build()
    }

    /// Sends a request to the Fitbit API with the specified parameters
    ///
    /// # Type Parameters
    ///
    /// * `T` - The expected response type that can be deserialized from JSON
    /// * `Q` - The query parameters type that can be serialized
    /// * `B` - The request body type that can be serialized
    /// * `E` - The error type that can be created from a string
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method to use for the request
    /// * `path` - The API endpoint path (will be appended to the base URL)
    /// * `query` - Optional query parameters to include in the URL
    /// * `body` - Optional request body to send
    ///
    /// # Returns
    ///
    /// Returns the deserialized response on success, or an error if:
    /// - The request fails to send
    /// - The response indicates an error (non-2xx status)
    /// - The response body cannot be parsed
    pub(crate) async fn send_request<T, Q, B, E>(
        &self,
        method: reqwest::Method,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
    ) -> Result<T, E>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
        E: StdError + From<String>,
    {
        let url = format!("{}{}", self.api_base_url, path);

        let mut request = self
            .client
            .request(method, &url)
            .header("Authorization", format!("Bearer {}", self.access_token));

        // Add query parameters if provided
        if let Some(q) = query {
            request = request.query(q);
        }

        // Add request body if provided
        if let Some(b) = body {
            request = request.json(b);
        }

        let response = request.send().await.map_err(|e| E::from(e.to_string()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| E::from(format!("Failed to get response body: {}", e)))?;

        if !status.is_success() {
            return Err(E::from(body));
        }

        // Parse the JSON response
        serde_json::from_str(&body).map_err(|e| {
            E::from(format!(
                "JSON parsing error: {}. Response body: {}",
                e, body
            ))
        })
    }

    /// Sends a GET request to the specified endpoint
    ///
    /// # Type Parameters
    ///
    /// * `T` - The expected response type
    /// * `Q` - The query parameters type
    /// * `E` - The error type
    ///
    /// # Arguments
    ///
    /// * `path` - The API endpoint path
    /// * `query` - Optional query parameters
    pub(crate) async fn get<T, Q, E>(&self, path: &str, query: Option<&Q>) -> Result<T, E>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
        E: StdError + From<String>,
    {
        self.send_request::<T, Q, (), E>(reqwest::Method::GET, path, query, None)
            .await
    }

    /// Sends a POST request to the specified endpoint
    ///
    /// # Type Parameters
    ///
    /// * `T` - The expected response type
    /// * `B` - The request body type
    /// * `E` - The error type
    ///
    /// # Arguments
    ///
    /// * `path` - The API endpoint path
    /// * `body` - Optional request body
    pub(crate) async fn post<T, B, E>(&self, path: &str, body: Option<&B>) -> Result<T, E>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
        E: StdError + From<String>,
    {
        self.send_request::<T, (), B, E>(reqwest::Method::POST, path, None, body)
            .await
    }

    /// Sends a PUT request to the specified endpoint
    ///
    /// # Type Parameters
    ///
    /// * `T` - The expected response type
    /// * `B` - The request body type
    /// * `E` - The error type
    ///
    /// # Arguments
    ///
    /// * `path` - The API endpoint path
    /// * `body` - Optional request body
    pub(crate) async fn put<T, B, E>(&self, path: &str, body: Option<&B>) -> Result<T, E>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
        E: StdError + From<String>,
    {
        self.send_request::<T, (), B, E>(reqwest::Method::PUT, path, None, body)
            .await
    }

    /// Sends a DELETE request to the specified endpoint
    ///
    /// # Type Parameters
    ///
    /// * `T` - The expected response type
    /// * `Q` - The query parameters type
    /// * `E` - The error type
    ///
    /// # Arguments
    ///
    /// * `path` - The API endpoint path
    /// * `query` - Optional query parameters
    pub(crate) async fn delete<T, Q, E>(&self, path: &str, query: Option<&Q>) -> Result<T, E>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
        E: StdError + From<String>,
    {
        self.send_request::<T, Q, (), E>(reqwest::Method::DELETE, path, query, None)
            .await
    }
}
