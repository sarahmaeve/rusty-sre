//! Async building blocks for a small telemetry collection service.
//!
//! The crate models batch flushing, concurrent scrapes, retries, workers, and
//! channel-based ingestion. Its exercise suite checks the operational behavior
//! of those components under delay, failure, overload, and shutdown.

pub mod batch;
pub mod cancellation;
pub mod channel;
pub mod fanout;
pub mod registry;
pub mod retry;
pub mod spawned;
pub mod timeout;
