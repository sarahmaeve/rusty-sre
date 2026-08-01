use std::time::Duration;

use thiserror::Error;

#[derive(Clone, Debug)]
pub struct ScrapeClient {
    response_latency: Duration,
}

impl ScrapeClient {
    pub fn new(response_latency: Duration) -> Self {
        Self { response_latency }
    }

    async fn fetch(&self, target: &str) -> String {
        tokio::time::sleep(self.response_latency).await;
        format!("up{{target=\"{target}\"}} 1")
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ScrapeError {
    #[error("the scrape deadline expired")]
    DeadlineExceeded,
}

pub async fn scrape_with_deadline(
    client: &ScrapeClient,
    target: &str,
    _deadline: Duration,
) -> Result<String, ScrapeError> {
    Ok(client.fetch(target).await)
}
