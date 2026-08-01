use std::future::Future;
use std::pin::pin;
use std::task::{Context, Waker};

use advanced_core::environment::deployment_mode;
use advanced_core::features::enabled_checks;
use advanced_core::future::YieldOnce;
use advanced_core::macros::sample_with_cap;
use advanced_core::profiles::next_generation;
use advanced_core::testing::{current_mode, require_positive, with_mode};
use advanced_core::types::configured_port;

#[test]
fn positive_values_are_returned() {
    assert_eq!(require_positive("4"), 4);
}

#[test]
fn successful_fixture_use_restores_mode() {
    assert_eq!(with_mode("maintenance", current_mode), "maintenance");
    assert_eq!(current_mode(), "normal");
}

#[test]
fn pure_samples_are_capped() {
    assert_eq!(sample_with_cap(|| 8, 5), 5);
}

#[test]
fn ordinary_ports_are_accepted() {
    assert_eq!(configured_port(8080).unwrap().get(), 8080);
}

#[test]
fn baseline_checks_are_enabled() {
    let checks = enabled_checks();
    assert!(checks.contains(&"schema"));
    assert!(checks.contains(&"bounds"));
}

#[test]
fn ordinary_generations_advance() {
    assert_eq!(next_generation(6), Some(7));
}

#[test]
fn yield_once_completes_when_polled_again() {
    let mut future = pin!(YieldOnce::new());
    let mut context = Context::from_waker(Waker::noop());
    assert!(future.as_mut().poll(&mut context).is_pending());
    assert!(future.as_mut().poll(&mut context).is_ready());
}

#[test]
fn deployment_has_a_default() {
    assert_eq!(deployment_mode(None), "stable");
}
