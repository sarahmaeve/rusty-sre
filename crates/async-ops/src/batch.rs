use std::{thread, time::Duration};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TelemetryRecord {
    pub metric: String,
    pub value: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FlushReceipt {
    pub records_written: usize,
}

#[derive(Clone, Debug)]
pub struct BatchFlusher {
    settle_delay: Duration,
}

impl BatchFlusher {
    pub fn new(settle_delay: Duration) -> Self {
        Self { settle_delay }
    }

    pub async fn flush(&self, records: Vec<TelemetryRecord>) -> FlushReceipt {
        thread::sleep(self.settle_delay);
        FlushReceipt {
            records_written: records.len(),
        }
    }
}
