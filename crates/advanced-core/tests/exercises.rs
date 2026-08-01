use std::any::Any;
use std::future::Future;
use std::pin::pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::task::{Context, Wake, Waker};
use std::time::Duration;

use advanced_core::environment::deployment_mode;
use advanced_core::features::enabled_checks;
use advanced_core::future::YieldOnce;
use advanced_core::macros::sample_with_cap;
use advanced_core::profiles::next_generation;
use advanced_core::testing::{current_mode, require_positive, with_mode};
use advanced_core::types::configured_port;

fn panic_message(payload: &(dyn Any + Send)) -> Option<&str> {
    payload
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
}

#[test]
#[ignore = "exercise 41"]
fn exercise_41_panic_test_checks_the_intended_failure() {
    let panic =
        std::panic::catch_unwind(|| require_positive("0")).expect_err("zero must be rejected");
    assert_eq!(panic_message(&*panic), Some("value must be positive"));
}

#[test]
#[ignore = "exercise 42"]
fn exercise_42_fixture_restores_state_after_panic() {
    let panic = std::panic::catch_unwind(|| {
        with_mode("maintenance", || panic!("simulated test failure"));
    });
    assert!(panic.is_err());
    assert_eq!(current_mode(), "normal");

    let (first_entered_tx, first_entered_rx) = mpsc::channel();
    let (release_first_tx, release_first_rx) = mpsc::channel();
    let first = std::thread::spawn(move || {
        with_mode("first", || {
            first_entered_tx.send(()).unwrap();
            release_first_rx.recv().unwrap();
        });
    });
    first_entered_rx.recv().unwrap();

    let (second_attempted_tx, second_attempted_rx) = mpsc::channel();
    let (second_entered_tx, second_entered_rx) = mpsc::channel();
    let second = std::thread::spawn(move || {
        second_attempted_tx.send(()).unwrap();
        with_mode("second", || second_entered_tx.send(()).unwrap());
    });
    second_attempted_rx.recv().unwrap();

    let overlapped = second_entered_rx
        .recv_timeout(Duration::from_millis(50))
        .is_ok();
    release_first_tx.send(()).unwrap();
    first.join().unwrap();
    if !overlapped {
        second_entered_rx.recv().unwrap();
    }
    second.join().unwrap();

    assert!(
        !overlapped,
        "process-global mode overrides must be serialized"
    );
    assert_eq!(current_mode(), "normal");
}

#[test]
#[ignore = "exercise 43"]
fn exercise_43_macro_evaluates_arguments_once() {
    let calls = AtomicUsize::new(0);
    let value = sample_with_cap(
        || {
            calls.fetch_add(1, Ordering::SeqCst);
            3
        },
        10,
    );
    assert_eq!(value, 3);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[test]
#[ignore = "exercise 44"]
fn exercise_44_newtype_preserves_its_invariant() {
    assert!(configured_port(0).is_err());
}

#[test]
#[ignore = "exercise 45"]
fn exercise_45_features_are_additive() {
    let checks = enabled_checks();
    assert!(checks.contains(&"schema"));
    assert!(checks.contains(&"bounds"));
    assert!(checks.contains(&"audit"));
}

#[test]
#[ignore = "exercise 46"]
fn exercise_46_release_behavior_matches_debug_behavior() {
    assert_eq!(next_generation(u32::MAX), None);
}

#[derive(Debug, Default)]
struct WakeCounter {
    wakes: AtomicUsize,
    thread: Mutex<Option<std::thread::Thread>>,
}

impl WakeCounter {
    fn count(&self) -> usize {
        self.wakes.load(Ordering::SeqCst)
    }
}

impl Wake for WakeCounter {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.wakes.fetch_add(1, Ordering::SeqCst);
        if let Some(thread) = self.thread.lock().unwrap().as_ref() {
            thread.unpark();
        }
    }
}

#[test]
#[ignore = "exercise 47"]
fn exercise_47_pending_future_arranges_a_wake() {
    let counter = Arc::new(WakeCounter::default());
    let waker = Waker::from(Arc::clone(&counter));
    let mut context = Context::from_waker(&waker);
    let mut future = pin!(YieldOnce::new());
    assert!(future.as_mut().poll(&mut context).is_pending());
    assert_eq!(counter.count(), 1);
}

#[test]
#[ignore = "exercise 48"]
fn exercise_48_runtime_configuration_is_not_build_configuration() {
    assert_eq!(deployment_mode(Some("canary")), "canary");
}
