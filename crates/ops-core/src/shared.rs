use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
pub struct SharedCounts {
    inner: Arc<Mutex<Vec<u64>>>,
}

impl SharedCounts {
    pub fn push(&self, value: u64) {
        self.inner.lock().unwrap().push(value);
    }

    pub fn poison_for_test(&self) {
        let inner = Arc::clone(&self.inner);
        let _ = std::thread::spawn(move || {
            let _guard = inner.lock().unwrap();
            panic!("simulated worker panic");
        })
        .join();
    }

    pub fn snapshot(&self) -> Vec<u64> {
        self.inner.lock().unwrap().clone()
    }
}
