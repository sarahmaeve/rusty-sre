use std::{collections::HashMap, sync::Arc, time::Duration};

use tokio::sync::Mutex;

#[derive(Clone, Debug, Default)]
pub struct RefreshProbe {
    state: Arc<ProbeState>,
}

#[derive(Debug, Default)]
struct ProbeState {
    active: std::sync::atomic::AtomicUsize,
    peak: std::sync::atomic::AtomicUsize,
}

impl RefreshProbe {
    pub fn peak(&self) -> usize {
        self.state.peak.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn enter(&self) -> RefreshGuard {
        let active = self
            .state
            .active
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            + 1;
        self.state
            .peak
            .fetch_max(active, std::sync::atomic::Ordering::SeqCst);
        RefreshGuard {
            state: Arc::clone(&self.state),
        }
    }
}

struct RefreshGuard {
    state: Arc<ProbeState>,
}

impl Drop for RefreshGuard {
    fn drop(&mut self) {
        self.state
            .active
            .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
    }
}

#[derive(Clone, Debug, Default)]
pub struct TargetRegistry {
    generations: Arc<Mutex<HashMap<String, u64>>>,
}

impl TargetRegistry {
    pub async fn refresh_target(
        &self,
        target: impl Into<String>,
        generation: u64,
        lookup_latency: Duration,
        probe: RefreshProbe,
    ) {
        let mut generations = self.generations.lock().await;
        let _refresh = probe.enter();
        tokio::time::sleep(lookup_latency).await;
        generations.insert(target.into(), generation);
    }

    pub async fn generation(&self, target: &str) -> Option<u64> {
        self.generations.lock().await.get(target).copied()
    }
}
