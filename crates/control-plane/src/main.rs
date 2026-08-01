use std::process::ExitCode;
use std::time::Duration;

use async_ops::timeout::{ScrapeClient, scrape_with_deadline};
use fleet_core::config::load_threshold;
use ops_core::health::{Probe, fleet_ready};

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(summary) => {
            println!("{summary}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("control plane failed: {error}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<String, Box<dyn std::error::Error>> {
    let threshold = load_threshold(Some("80"))?;
    let ready = fleet_ready(&[Probe { ready: true }]);
    let sample = scrape_with_deadline(
        &ScrapeClient::new(Duration::from_millis(1)),
        "localhost",
        Duration::from_secs(1),
    )
    .await?;
    Ok(format!(
        "ready={ready} threshold={threshold} sample={sample}"
    ))
}
