use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use thiserror::Error;

#[derive(Clone, Debug)]
pub struct ScrapeTarget {
    pub name: String,
    pub latency: Duration,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScrapeResult {
    pub target: String,
}

#[derive(Clone, Debug, Default)]
pub struct FanoutProbe {
    state: Arc<ProbeState>,
}

#[derive(Debug, Default)]
struct ProbeState {
    active: AtomicUsize,
    peak: AtomicUsize,
}

impl FanoutProbe {
    pub fn peak(&self) -> usize {
        self.state.peak.load(Ordering::SeqCst)
    }

    fn enter(&self) -> FanoutGuard {
        let active = self.state.active.fetch_add(1, Ordering::SeqCst) + 1;
        self.state.peak.fetch_max(active, Ordering::SeqCst);
        FanoutGuard {
            state: Arc::clone(&self.state),
        }
    }
}

struct FanoutGuard {
    state: Arc<ProbeState>,
}

impl Drop for FanoutGuard {
    fn drop(&mut self) {
        self.state.active.fetch_sub(1, Ordering::SeqCst);
    }
}

#[derive(Debug, Error)]
pub enum FanoutError {
    #[error("a scrape task failed: {0}")]
    Task(String),
}

pub async fn scrape_targets(
    targets: Vec<ScrapeTarget>,
    _max_in_flight: usize,
    probe: FanoutProbe,
) -> Result<Vec<ScrapeResult>, FanoutError> {
    let mut tasks = Vec::with_capacity(targets.len());
    for target in targets {
        let probe = probe.clone();
        tasks.push(tokio::spawn(async move {
            let _active = probe.enter();
            tokio::time::sleep(target.latency).await;
            ScrapeResult {
                target: target.name,
            }
        }));
    }

    let mut results = Vec::with_capacity(tasks.len());
    for task in tasks {
        results.push(
            task.await
                .map_err(|error| FanoutError::Task(error.to_string()))?,
        );
    }
    Ok(results)
}
