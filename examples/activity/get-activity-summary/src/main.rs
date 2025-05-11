use fitbit_sdk::client::FitbitClient;
use fitbit_sdk::types::activity::{ActivityClient, ActivityError, Resource};
use std::env;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), ActivityError> {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_file(false)
        .with_level(true)
        .try_init()
        .expect("Failed to initialize logger");

    let client = FitbitClient::new::<ActivityError>()?;

    // Get today's activity summary
    match client.get_activity_summary("-", "today").await {
        Ok(summary) => {
            info!("Activity Summary for Today:");
            info!("  Steps: {}", summary.steps);
            info!("  Calories: {}", summary.calories);
            if let Some(floors) = summary.floors {
                info!("  Floors: {}", floors);
            }
            
            for distance in &summary.distances {
                info!("  Distance ({}): {}", distance.activity, distance.distance);
            }
            
            info!("Activity Minutes:");
            info!("  Sedentary: {}", summary.sedentary_minutes);
            info!("  Lightly Active: {}", summary.lightly_active_minutes);
            info!("  Fairly Active: {}", summary.fairly_active_minutes);
            info!("  Very Active: {}", summary.very_active_minutes);
            
            if let Some(rhr) = summary.resting_heart_rate {
                info!("  Resting Heart Rate: {}", rhr);
            }
        }
        Err(e) => {
            error!("Error getting activity summary: {}", e);
        }
    }

    // Get steps time series for the last 7 days
    info!("\nSteps for Last 7 Days:");
    match client.get_activity_time_series("-", Resource::Steps, "today", "7d").await {
        Ok(time_series) => {
            for data_point in time_series {
                info!("  {}: {} steps", data_point.datetime, data_point.value);
            }
        }
        Err(e) => {
            error!("Error getting steps time series: {}", e);
        }
    }

    Ok(())
}
