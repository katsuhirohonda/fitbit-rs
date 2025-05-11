use fitbit_sdk::client::FitbitClient;
use fitbit_sdk::types::user::{UserClient, UserError};
use std::env;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), UserError> {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_file(false)
        .with_level(true)
        .try_init()
        .expect("Failed to initialize logger");

    let client = FitbitClient::new::<UserError>()?;

    match client.get_profile("-").await {
        Ok(profile) => {
            info!("User Profile Information:");
            info!("  Display Name: {}", profile.display_name);
            info!("  Full Name: {}", profile.full_name);
            info!("  Date of Birth: {}", profile.date_of_birth);
            info!("  Gender: {:?}", profile.gender);
            info!("  Height: {}", profile.height);
            if let Some(weight) = profile.weight {
                info!("  Weight: {}", weight);
            }
            info!("  Height Unit: {:?}", profile.height_unit);
            info!("  Weight Unit: {:?}", profile.weight_unit);
            info!("  Average Daily Steps: {}", profile.average_daily_steps);
        }
        Err(e) => {
            error!("Error getting profile: {}", e);
        }
    }

    Ok(())
}
