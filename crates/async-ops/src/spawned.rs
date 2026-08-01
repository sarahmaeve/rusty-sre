use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use thiserror::Error;

#[derive(Clone, Debug)]
pub struct ExportSink {
    reject_writes: bool,
    attempts: Arc<AtomicUsize>,
}

impl ExportSink {
    pub fn accepting() -> Self {
        Self {
            reject_writes: false,
            attempts: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn rejecting() -> Self {
        Self {
            reject_writes: true,
            attempts: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn attempts(&self) -> usize {
        self.attempts.load(Ordering::SeqCst)
    }

    async fn write(&self, _payload: Vec<u8>) -> Result<(), ExportError> {
        self.attempts.fetch_add(1, Ordering::SeqCst);
        tokio::task::yield_now().await;
        if self.reject_writes {
            Err(ExportError::Rejected)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ExportError {
    #[error("the export sink rejected the batch")]
    Rejected,
}

pub async fn dispatch_export(sink: ExportSink, payload: Vec<u8>) -> Result<(), ExportError> {
    tokio::spawn(async move {
        let _ = sink.write(payload).await;
    });
    tokio::task::yield_now().await;
    Ok(())
}
