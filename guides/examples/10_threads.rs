//! Threads require owned or scoped data. Channels transfer messages; mutexes
//! serialize shared mutation. `Send` and `Sync` express cross-thread guarantees.
//!
//! Further reading:
//! - <https://doc.rust-lang.org/book/ch16-00-concurrency.html>
//! - <https://doc.rust-lang.org/std/thread/index.html>
//! - <https://doc.rust-lang.org/std/sync/mpsc/index.html>
//! - <https://doc.rust-lang.org/std/marker/trait.Send.html>
//! - Source study: <https://github.com/rayon-rs/rayon/tree/main/rayon-core/src>

use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

fn main() {
    assert_send::<String>();
    assert_sync::<String>();
    assert_send::<Arc<Mutex<Vec<u64>>>>();
    assert_sync::<Arc<Mutex<Vec<u64>>>>();

    let values = [2, 3, 5, 7];
    let mut partials = [0, 0];
    thread::scope(|scope| {
        let (left, right) = partials.split_at_mut(1);
        scope.spawn(|| left[0] = values[..2].iter().sum());
        scope.spawn(|| right[0] = values[2..].iter().sum());
    });
    assert_eq!(partials, [5, 12]);

    // A zero-capacity synchronous channel is a rendezvous: send completes only
    // when the receiver is ready. Larger capacities provide bounded buffering.
    let (sender, receiver) = mpsc::sync_channel(1);
    let producer = thread::spawn(move || {
        for value in [10, 20, 30] {
            sender.send(value).unwrap();
        }
    });
    let received: Vec<_> = receiver.iter().collect();
    producer.join().unwrap();
    assert_eq!(received, [10, 20, 30]);

    let counts = Arc::new(Mutex::new(vec![0_u64; 4]));
    let handles: Vec<_> = (0..4)
        .map(|index| {
            let counts = Arc::clone(&counts);
            thread::spawn(move || {
                // Keep the critical section narrow. The guard unlocks on drop.
                counts.lock().unwrap()[index] += index as u64 + 1;
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
    assert_eq!(*counts.lock().unwrap(), [1, 2, 3, 4]);

    // If a thread panics while holding a standard mutex, later lock attempts
    // return `PoisonError`; callers must choose whether recovery is sound.
}
