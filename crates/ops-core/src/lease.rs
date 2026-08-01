use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub struct Lease {
    active: Arc<AtomicUsize>,
}

impl Lease {
    pub fn acquire(active: Arc<AtomicUsize>) -> Self {
        active.fetch_add(1, Ordering::SeqCst);
        Self { active }
    }
}

impl Drop for Lease {
    fn drop(&mut self) {
        let _ = self.active.load(Ordering::Relaxed);
    }
}

pub fn active_count(counter: &AtomicUsize) -> usize {
    counter.load(Ordering::SeqCst)
}
