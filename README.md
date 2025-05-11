# Fitbit Rust SDK

[![Crates.io](https://img.shields.io/crates/v/fitbit-sdk.svg)](https://crates.io/crates/fitbit-sdk)
[![Documentation](https://docs.rs/fitbit-sdk/badge.svg)](https://docs.rs/fitbit-sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

An unofficial Rust SDK for the [Fitbit Web API](https://dev.fitbit.com/build/reference).

## Features

- Robust async/await implementation using Tokio
- Comprehensive error handling with detailed error types
- OAuth 2.0 authentication support
- Type-safe API with full Rust type definitions
- Easy-to-use builder patterns for request construction

## Installation

```bash
cargo add fitbit-sdk
```

## Quick Start

```rust
use fitbit_sdk::client::FitbitClient;
use fitbit_sdk::types::user::{UserClient, UserError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = FitbitClient::new()?;
    
    // Get user profile
    let profile = client.get_profile("-", "date").await?;
    
    println!("User profile: {:?}", profile);
    
    Ok(())
}
```

## Examples

Check out the [examples](https://github.com/yourusername/fitbit-sdk-rs/tree/main/examples) directory for more usage examples:

- User Profile
  - [Get Profile](https://github.com/yourusername/fitbit-sdk-rs/blob/main/examples/user/get-profile/src/main.rs) - How to retrieve user profile data
- Activities
  - [Get Activity Summary](https://github.com/yourusername/fitbit-sdk-rs/blob/main/examples/activity/get-activity-summary/src/main.rs) - How to get daily activity summary
- Sleep
  - [Get Sleep Log](https://github.com/yourusername/fitbit-sdk-rs/blob/main/examples/sleep/get-sleep-log/src/main.rs) - How to retrieve sleep data

## API Coverage

- User Profile
  - [x] Get Profile
  - [x] Update Profile
- Activity
  - [x] Get Daily Activity Summary
  - [x] Get Activity Time Series
  - [x] Get Lifetime Statistics
- Sleep
  - [x] Get Sleep Logs
  - [x] Get Sleep Goal
- Body
  - [x] Get Body Weight
  - [x] Get Body Fat
  - [x] Get Body Goals

## Authentication

The Fitbit API uses OAuth 2.0 for authentication. Set up your application at https://dev.fitbit.com/apps and configure the following environment variables:

```bash
export FITBIT_CLIENT_ID="your-client-id"
export FITBIT_CLIENT_SECRET="your-client-secret"
export FITBIT_ACCESS_TOKEN="your-access-token"
```

## Development

### Prerequisites

- Rust 1.85.0 or later
- A Fitbit Developer account

### Running Tests

```bash
cargo test
```

### Running Examples

Set your credentials

```bash
export FITBIT_ACCESS_TOKEN="your-access-token"
```

Run an example

```bash
cd examples/user/get-profile
cargo run 
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Fitbit Web API Documentation](https://dev.fitbit.com/build/reference)

## Security

If you discover a security vulnerability within this package, please send an e-mail to the maintainers. All security vulnerabilities will be promptly addressed.

## Support

For support questions, please use the [GitHub Issues](https://github.com/yourusername/fitbit-sdk-rs/issues).
