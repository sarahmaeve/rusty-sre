use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectionJob {
    pub target: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorkerReport {
    pub completed_targets: Vec<String>,
}

pub async fn run_worker(
    mut jobs: mpsc::Receiver<CollectionJob>,
    _shutdown: CancellationToken,
) -> WorkerReport {
    let mut report = WorkerReport::default();
    while let Some(job) = jobs.recv().await {
        report.completed_targets.push(job.target);
    }
    report
}
