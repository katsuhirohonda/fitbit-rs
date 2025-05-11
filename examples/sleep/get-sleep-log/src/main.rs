use fitbit_sdk::client::FitbitClient;
use fitbit_sdk::types::sleep::{SleepClient, SleepError};
use std::env;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), SleepError> {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_file(false)
        .with_level(true)
        .try_init()
        .expect("Failed to initialize logger");

    let client = FitbitClient::new::<SleepError>()?;

    // Get last night's sleep data
    match client.get_sleep_logs("-", "yesterday").await {
        Ok(sleep_logs) => {
            info!("Sleep Summary:");
            info!("  Total Sleep Records: {}", sleep_logs.summary.total_sleep_records);
            info!("  Total Time in Bed: {} minutes", sleep_logs.summary.total_time_in_bed);
            info!("  Total Minutes Asleep: {} minutes", sleep_logs.summary.total_minutes_asleep);
            
            info!("\nSleep Entries:");
            for sleep_entry in sleep_logs.sleep {
                info!("  Entry type: {}", sleep_entry.type_);
                info!("  Start time: {}", sleep_entry.start_time);
                info!("  End time: {}", sleep_entry.end_time);
                info!("  Duration: {} ms", sleep_entry.duration);
                info!("  Minutes asleep: {}", sleep_entry.minutes_asleep);
                info!("  Time in bed: {} minutes", sleep_entry.time_in_bed);
                info!("  Efficiency: {}%", sleep_entry.efficiency);
                info!("  Main sleep: {}", sleep_entry.is_main_sleep);
                
                // Show sleep levels if available
                if let Some(levels) = sleep_entry.levels {
                    info!("  Sleep Levels:");
                    if let Some(rem) = levels.summary.rem {
                        info!("    REM: {} minutes ({} times)", rem.minutes, rem.count);
                    }
                    if let Some(deep) = levels.summary.deep {
                        info!("    Deep: {} minutes ({} times)", deep.minutes, deep.count);
                    }
                    if let Some(light) = levels.summary.light {
                        info!("    Light: {} minutes ({} times)", light.minutes, light.count);
                    }
                    if let Some(wake) = levels.summary.wake {
                        info!("    Wake: {} minutes ({} times)", wake.minutes, wake.count);
                    }
                }
            }
        }
        Err(e) => {
            error!("Error getting sleep logs: {}", e);
        }
    }

    // Get sleep goal
    info!("\nSleep Goal:");
    match client.get_sleep_goal("-").await {
        Ok(goal) => {
            info!("  Target sleep: {} minutes", goal.goal);
        }
        Err(e) => {
            error!("Error getting sleep goal: {}", e);
        }
    }

    Ok(())
}
