use anyhow::Result;
use backlog_api_client::client::BacklogApiClient;
use chrono::{Local, TimeZone, Utc};
use clap::Subcommand;
use std::env;

#[derive(Subcommand)]
pub enum RateLimitCommand {
    /// Get current rate limit information
    Get,
}

pub async fn handle_rate_limit_command(cmd: RateLimitCommand) -> Result<()> {
    match cmd {
        RateLimitCommand::Get => get_rate_limit().await,
    }
}

async fn get_rate_limit() -> Result<()> {
    let base_url = env::var("BACKLOG_BASE_URL")?;
    let api_key = env::var("BACKLOG_API_KEY")?;
    let client = BacklogApiClient::new(&base_url)?.with_api_key(api_key);
    let response = client.rate_limit().get_rate_limit().await?;

    println!("Rate Limit Information:");
    println!("======================");

    let rate_limit = &response.rate_limit;

    // Read operations
    println!("\nRead Operations:");
    println!("  Limit:     {}", rate_limit.read.limit);
    println!("  Remaining: {}", rate_limit.read.remaining);
    println!("  Reset:     {}", format_timestamp(rate_limit.read.reset));

    // Update operations
    println!("\nUpdate Operations:");
    println!("  Limit:     {}", rate_limit.update.limit);
    println!("  Remaining: {}", rate_limit.update.remaining);
    println!("  Reset:     {}", format_timestamp(rate_limit.update.reset));

    // Search operations
    println!("\nSearch Operations:");
    println!("  Limit:     {}", rate_limit.search.limit);
    println!("  Remaining: {}", rate_limit.search.remaining);
    println!("  Reset:     {}", format_timestamp(rate_limit.search.reset));

    // Icon operations
    println!("\nIcon Operations:");
    println!("  Limit:     {}", rate_limit.icon.limit);
    println!("  Remaining: {}", rate_limit.icon.remaining);
    println!("  Reset:     {}", format_timestamp(rate_limit.icon.reset));

    Ok(())
}

fn format_timestamp(timestamp: i32) -> String {
    let dt = Utc
        .timestamp_opt(timestamp as i64, 0)
        .single()
        .map(|utc| utc.with_timezone(&Local))
        .unwrap_or_else(Local::now);

    dt.format("%Y-%m-%d %H:%M:%S %Z").to_string()
}
