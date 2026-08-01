use std::time::Duration;

use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AttemptOutcome {
    Accepted,
    TransientFailure,
}

#[derive(Clone, Copy, Debug)]
pub struct Attempt {
    pub latency: Duration,
    pub outcome: AttemptOutcome,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeliveryReceipt {
    pub attempts: usize,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum RetryError {
    #[error("the delivery budget expired")]
    DeadlineExceeded,
    #[error("all delivery attempts failed")]
    AttemptsExhausted,
}

async fn run_attempt(attempt: Attempt) -> AttemptOutcome {
    tokio::time::sleep(attempt.latency).await;
    attempt.outcome
}

pub async fn deliver_with_retry(
    attempts: &[Attempt],
    overall_budget: Duration,
) -> Result<DeliveryReceipt, RetryError> {
    for (index, attempt) in attempts.iter().copied().enumerate() {
        let outcome = tokio::time::timeout(overall_budget, run_attempt(attempt))
            .await
            .map_err(|_| RetryError::DeadlineExceeded)?;
        if outcome == AttemptOutcome::Accepted {
            return Ok(DeliveryReceipt {
                attempts: index + 1,
            });
        }
    }
    Err(RetryError::AttemptsExhausted)
}
