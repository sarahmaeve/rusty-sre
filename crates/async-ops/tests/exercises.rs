use std::time::{Duration, Instant};

use async_ops::{
    batch::BatchFlusher,
    cancellation::run_worker,
    channel::{Envelope, ingest_batch},
    fanout::{FanoutProbe, ScrapeTarget, scrape_targets},
    registry::{RefreshProbe, TargetRegistry},
    retry::{Attempt, AttemptOutcome, RetryError, deliver_with_retry},
    spawned::{ExportError, ExportSink, dispatch_export},
    timeout::{ScrapeClient, ScrapeError, scrape_with_deadline},
};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

#[tokio::test(flavor = "current_thread")]
#[ignore = "exercise 19"]
async fn exercise_19_blocking_flush_does_not_starve_the_runtime() {
    let flusher = BatchFlusher::new(Duration::from_millis(40));
    let started = Instant::now();
    let (_, heartbeat_at) = tokio::join!(flusher.flush(Vec::new()), async {
        tokio::time::sleep(Duration::from_millis(5)).await;
        Instant::now()
    });
    assert!(
        heartbeat_at.duration_since(started) < Duration::from_millis(25),
        "the runtime did not poll its heartbeat promptly"
    );
}

#[tokio::test]
#[ignore = "exercise 20"]
async fn exercise_20_independent_registry_refreshes_overlap() {
    let registry = TargetRegistry::default();
    let probe = RefreshProbe::default();
    tokio::join!(
        registry.refresh_target("edge-1", 1, Duration::from_millis(15), probe.clone()),
        registry.refresh_target("edge-2", 1, Duration::from_millis(15), probe.clone())
    );
    assert_eq!(probe.peak(), 2, "independent lookups were serialized");
}

#[tokio::test]
#[ignore = "exercise 21"]
async fn exercise_21_export_failure_reaches_the_caller() {
    let error = dispatch_export(ExportSink::rejecting(), b"sample".to_vec())
        .await
        .expect_err("a rejected export must be reported");
    assert_eq!(error, ExportError::Rejected);
}

#[tokio::test]
#[ignore = "exercise 22"]
async fn exercise_22_fanout_respects_its_concurrency_limit() {
    let limit = 3;
    let probe = FanoutProbe::default();
    let targets = (0..12)
        .map(|index| ScrapeTarget {
            name: format!("edge-{index}"),
            latency: Duration::from_millis(15),
        })
        .collect();
    scrape_targets(targets, limit, probe.clone())
        .await
        .expect("scrapes complete");
    assert!(
        probe.peak() <= limit,
        "observed {} concurrent scrapes with a limit of {limit}",
        probe.peak()
    );
}

#[tokio::test]
#[ignore = "exercise 23"]
async fn exercise_23_slow_scrape_honors_its_deadline() {
    let result = scrape_with_deadline(
        &ScrapeClient::new(Duration::from_millis(40)),
        "edge-1",
        Duration::from_millis(5),
    )
    .await;
    assert_eq!(result, Err(ScrapeError::DeadlineExceeded));
}

#[tokio::test]
#[ignore = "exercise 24"]
async fn exercise_24_retries_share_one_overall_budget() {
    let attempts = [
        Attempt {
            latency: Duration::from_millis(30),
            outcome: AttemptOutcome::TransientFailure,
        },
        Attempt {
            latency: Duration::from_millis(30),
            outcome: AttemptOutcome::TransientFailure,
        },
        Attempt {
            latency: Duration::from_millis(1),
            outcome: AttemptOutcome::Accepted,
        },
    ];
    let result = deliver_with_retry(&attempts, Duration::from_millis(50)).await;
    assert_eq!(result, Err(RetryError::DeadlineExceeded));
}

#[tokio::test]
#[ignore = "exercise 25"]
async fn exercise_25_worker_stops_when_cancelled() {
    let (sender, receiver) = mpsc::channel(1);
    let shutdown = CancellationToken::new();
    let worker = tokio::spawn(run_worker(receiver, shutdown.clone()));
    shutdown.cancel();
    let stopped = tokio::time::timeout(Duration::from_millis(20), worker).await;
    assert!(
        stopped.is_ok(),
        "worker remained blocked after cancellation"
    );
    drop(sender);
}

#[tokio::test]
#[ignore = "exercise 26"]
async fn exercise_26_ingestion_preserves_the_complete_batch() {
    let events: Vec<_> = (0..8)
        .map(|sequence| Envelope {
            sequence,
            payload: format!("sample-{sequence}"),
        })
        .collect();
    let stored = ingest_batch(events.clone(), 2).await;
    assert_eq!(stored, events);
}
