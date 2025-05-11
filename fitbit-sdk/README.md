# Fitbit Rust SDK

[![Crates.io](https://img.shields.io/crates/v/fitbit-sdk.svg)](https://crates.io/crates/fitbit-sdk)
[![Documentation](https://docs.rs/fitbit-sdk/badge.svg)](https://docs.rs/fitbit-sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

An unofficial Rust SDK for the [Fitbit Web API](https://dev.fitbit.com/build/reference).

## Features

- Comprehensive OAuth 2.0 authentication support
- Type-safe API with full Rust type definitions
- Easy-to-use async/await implementation
- Built-in error handling with detailed error types
- Support for all major Fitbit API endpoints

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fitbit-sdk = "0.1.0"
```

## Quick Start

```rust
use fitbit_sdk::client::FitbitClient;
use fitbit_sdk::types::user::{UserClient, UserError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Make sure FITBIT_ACCESS_TOKEN is set
    let client = FitbitClient::new::<UserError>()?;
    
    // Get user profile
    let profile = client.get_profile("-").await?;
    println!("User: {}", profile.display_name);
    
    Ok(())
}
```

## Authentication

The Fitbit API uses OAuth 2.0 for authentication. Set up your application at https://dev.fitbit.com/apps and configure the following environment variable:

```bash
export FITBIT_ACCESS_TOKEN="your-access-token"
```

Or use the builder pattern:

```rust
let client = FitbitClient::builder()
    .with_access_token("your-access-token")
    .build::<UserError>()?;
```

## Examples

Check out the [examples](../examples) directory for complete usage examples:

### User Profile

```rust
use fitbit_sdk::client::FitbitClient;
use fitbit_sdk::types::user::{UserClient, UserError};

#[tokio::main]
async fn main() -> Result<(), UserError> {
    let client = FitbitClient::new::<UserError>()?;
    
    // Get user profile
    let profile = client.get_profile("-").await?;
    
    println!("Display Name: {}", profile.display_name);
    println!("Average Daily Steps: {}", profile.average_daily_steps);
    
    Ok(())
}
```

### Activity Data

```rust
use fitbit_sdk::client::FitbitClient;
use fitbit_sdk::types::activity::{ActivityClient, ActivityError, Resource};

#[tokio::main]
async fn main() -> Result<(), ActivityError> {
    let client = FitbitClient::new::<ActivityError>()?;
    
    // Get today's activity summary
    let summary = client.get_activity_summary("-", "today").await?;
    println!("Steps: {}", summary.steps);
    
    // Get last 7 days of steps
    let steps = client.get_activity_time_series("-", Resource::Steps, "today", "7d").await?;
    for data_point in steps {
        println!("{}: {} steps", data_point.datetime, data_point.value);
    }
    
    Ok(())
}
```

### Sleep Data

```rust
use fitbit_sdk::client::FitbitClient;
use fitbit_sdk::types::sleep::{SleepClient, SleepError};

#[tokio::main]
async fn main() -> Result<(), SleepError> {
    let client = FitbitClient::new::<SleepError>()?;
    
    // Get last night's sleep data
    let sleep_logs = client.get_sleep_logs("-", "yesterday").await?;
    println!("Total sleep: {} minutes", sleep_logs.summary.total_minutes_asleep);
    
    Ok(())
}
```

## API Coverage

### User Profile
- [x] Get Profile
- [x] Update Profile

### Activity
- [x] Get Daily Activity Summary
- [x] Get Activity Time Series
- [x] Get Lifetime Statistics

### Sleep
- [x] Get Sleep Logs
- [x] Get Sleep Goal

### Body
- [x] Get Body Weight
- [x] Get Body Fat
- [x] Get Body Goals

### Nutrition
- [x] Get Water Logs
- [x] Get Food Logs

## Development

### Prerequisites

- Rust 1.85.0 or later
- A Fitbit Developer account and application
- Access token for API testing

### Running Tests

```bash
cargo test
```

### Running Examples

Set your access token:

```bash
export FITBIT_ACCESS_TOKEN="your-access-token"
```

Run an example:

```bash
cd examples/user/get-profile
cargo run
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

## Acknowledgments

- [Fitbit Web API Documentation](https://dev.fitbit.com/build/reference)

## Note

This is an unofficial SDK and is not affiliated with Fitbit, Inc. Fitbit is a trademark of Fitbit, Inc.
