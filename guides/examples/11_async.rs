//! Futures are lazy computations. Runtimes poll them; spawned tasks run
//! concurrently. Cancellation, backpressure, and shutdown are API contracts.
//!
//! Further reading:
//! - <https://doc.rust-lang.org/book/ch17-00-async-await.html>
//! - <https://doc.rust-lang.org/std/future/trait.Future.html>
//! - <https://tokio.rs/tokio/tutorial>
//! - <https://tokio.rs/tokio/topics/shutdown>
//! - <https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html>
//! - Source study: <https://github.com/tokio-rs/tokio/tree/master/tokio/src>

use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tokio::{
    sync::{Semaphore, mpsc, watch},
    task::JoinSet,
    time::{Duration, sleep, timeout},
};

async fn bounded_work(
    value: usize,
    permits: Arc<Semaphore>,
    active: Arc<AtomicUsize>,
    maximum: Arc<AtomicUsize>,
) -> usize {
    let _permit = permits.acquire_owned().await.unwrap();
    let now_active = active.fetch_add(1, Ordering::SeqCst) + 1;
    maximum.fetch_max(now_active, Ordering::SeqCst);
    sleep(Duration::from_millis(5)).await;
    active.fetch_sub(1, Ordering::SeqCst);
    value * 2
} // The semaphore permit is released here, including on cancellation.

async fn cancellable_worker(mut shutdown: watch::Receiver<bool>) -> usize {
    let mut ticks = 0;
    loop {
        tokio::select! {
            changed = shutdown.changed() => {
                if changed.is_err() || *shutdown.borrow() {
                    return ticks;
                }
            }
            () = sleep(Duration::from_millis(2)) => {
                ticks += 1;
            }
        }
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    // Calling an async function creates a future. Work starts when it is awaited,
    // spawned, or otherwise polled.
    let future = async { 2 + 3 };
    assert_eq!(future.await, 5);

    let permits = Arc::new(Semaphore::new(2));
    let active = Arc::new(AtomicUsize::new(0));
    let maximum = Arc::new(AtomicUsize::new(0));
    let mut tasks = JoinSet::new();
    for value in 0..6 {
        tasks.spawn(bounded_work(
            value,
            Arc::clone(&permits),
            Arc::clone(&active),
            Arc::clone(&maximum),
        ));
    }

    let mut results = Vec::new();
    while let Some(result) = tasks.join_next().await {
        results.push(result.unwrap());
    }
    results.sort_unstable();
    assert_eq!(results, [0, 2, 4, 6, 8, 10]);
    assert!(maximum.load(Ordering::SeqCst) <= 2);

    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let worker = tokio::spawn(cancellable_worker(shutdown_rx));
    sleep(Duration::from_millis(8)).await;
    shutdown_tx.send(true).unwrap();
    let ticks = worker.await.unwrap();
    assert!(ticks > 0);

    // Bounded channels make overload visible to senders.
    let (sender, mut receiver) = mpsc::channel(1);
    sender.send("ready").await.unwrap();
    assert!(sender.try_send("busy").is_err());
    assert_eq!(receiver.recv().await, Some("ready"));

    let deadline = timeout(Duration::from_millis(5), sleep(Duration::from_secs(1))).await;
    assert!(deadline.is_err());

    let abandoned = tokio::spawn(async { sleep(Duration::from_secs(60)).await });
    abandoned.abort();
    assert!(abandoned.await.unwrap_err().is_cancelled());
    // Dropping a `JoinHandle` would detach the task instead of aborting it.
}
