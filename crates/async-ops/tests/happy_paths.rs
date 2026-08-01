use std::time::Duration;

use async_ops::{
    batch::{BatchFlusher, TelemetryRecord},
    cancellation::{CollectionJob, run_worker},
    channel::{Envelope, ingest_batch},
    fanout::{FanoutProbe, ScrapeTarget, scrape_targets},
    registry::{RefreshProbe, TargetRegistry},
    retry::{Attempt, AttemptOutcome, deliver_with_retry},
    spawned::{ExportSink, dispatch_export},
    timeout::{ScrapeClient, scrape_with_deadline},
};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

#[tokio::test]
async fn flush_reports_the_batch_size() {
    let flusher = BatchFlusher::new(Duration::ZERO);
    let receipt = flusher
        .flush(vec![TelemetryRecord {
            metric: "queue.depth".into(),
            value: 7,
        }])
        .await;
    assert_eq!(receipt.records_written, 1);
}

#[tokio::test]
async fn registry_records_a_refreshed_generation() {
    let registry = TargetRegistry::default();
    registry
        .refresh_target("edge-1", 4, Duration::ZERO, RefreshProbe::default())
        .await;
    assert_eq!(registry.generation("edge-1").await, Some(4));
}

#[tokio::test]
async fn accepted_export_is_dispatched() {
    let sink = ExportSink::accepting();
    dispatch_export(sink.clone(), b"sample".to_vec())
        .await
        .expect("accepted export");
    tokio::task::yield_now().await;
    assert_eq!(sink.attempts(), 1);
}

#[tokio::test]
async fn fanout_returns_every_scrape() {
    let results = scrape_targets(
        vec![ScrapeTarget {
            name: "edge-1".into(),
            latency: Duration::ZERO,
        }],
        1,
        FanoutProbe::default(),
    )
    .await
    .expect("scrapes complete");
    assert_eq!(results.len(), 1);
}

#[tokio::test]
async fn responsive_scrape_returns_samples() {
    let samples = scrape_with_deadline(
        &ScrapeClient::new(Duration::ZERO),
        "edge-1",
        Duration::from_secs(1),
    )
    .await
    .expect("scrape succeeds");
    assert!(samples.contains("edge-1"));
}

#[tokio::test]
async fn first_attempt_can_deliver() {
    let receipt = deliver_with_retry(
        &[Attempt {
            latency: Duration::ZERO,
            outcome: AttemptOutcome::Accepted,
        }],
        Duration::from_secs(1),
    )
    .await
    .expect("delivery succeeds");
    assert_eq!(receipt.attempts, 1);
}

#[tokio::test]
async fn worker_drains_a_closed_queue() {
    let (sender, receiver) = mpsc::channel(2);
    sender
        .send(CollectionJob {
            target: "edge-1".into(),
        })
        .await
        .expect("worker is receiving");
    drop(sender);
    let report = run_worker(receiver, CancellationToken::new()).await;
    assert_eq!(report.completed_targets, ["edge-1"]);
}

#[tokio::test]
async fn single_event_batch_is_stored() {
    let stored = ingest_batch(
        vec![Envelope {
            sequence: 1,
            payload: "sample".into(),
        }],
        4,
    )
    .await;
    assert_eq!(stored.len(), 1);
}
