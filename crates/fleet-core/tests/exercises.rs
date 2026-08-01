use std::collections::HashSet;

use fleet_core::config::{ConfigError, load_threshold};
use fleet_core::inventory::{count_errors, effective_severity, mark_degraded};
use fleet_core::model::{Incident, Service, Status};
use fleet_core::policy::{DeliveryPolicy, serialize_policy};
use fleet_core::text::{SizeError, narrow_size, prefix};

#[test]
#[ignore = "exercise 01"]
fn exercise_01_invalid_config_is_not_a_default() {
    assert_eq!(
        load_threshold(Some("many")),
        Err(ConfigError::InvalidThreshold("many".to_owned()))
    );
}

#[test]
#[ignore = "exercise 02"]
fn exercise_02_repeated_errors_are_counted() {
    let counts = count_errors(["api", "api", "db", "api"]);
    assert_eq!(counts["api"], 3);
}

#[test]
#[ignore = "exercise 03"]
fn exercise_03_prefix_counts_characters() {
    assert_eq!(prefix("aé日", 2), Ok("aé"));
}

#[test]
#[ignore = "exercise 04"]
fn exercise_04_narrowing_is_checked() {
    let value = u64::from(u32::MAX) + 1;
    assert_eq!(narrow_size(value), Err(SizeError::OutOfRange(value)));
}

#[test]
#[ignore = "exercise 05"]
fn exercise_05_missing_severity_fails_safe() {
    assert_eq!(effective_severity(None), 5);
}

#[test]
#[ignore = "exercise 06"]
fn exercise_06_mutation_reaches_the_caller() {
    let mut service = Service {
        name: "checkout".to_owned(),
        status: Status::Healthy,
    };
    mark_degraded(&mut service, "error budget exhausted");
    assert!(matches!(service.status, Status::Degraded { .. }));
}

#[test]
#[ignore = "exercise 07"]
fn exercise_07_equal_values_have_equal_hashes() {
    let first = Incident {
        id: 1,
        service: "api".to_owned(),
        summary: "timeout".to_owned(),
    };
    let second = Incident {
        id: 2,
        ..first.clone()
    };
    let incidents = HashSet::from([first, second]);
    assert_eq!(incidents.len(), 1);
}

#[test]
#[ignore = "exercise 08"]
fn exercise_08_explicit_false_survives_the_wire() {
    let encoded = serialize_policy(&DeliveryPolicy {
        retries: 3,
        cancel_on_error: false,
    })
    .unwrap();
    assert!(encoded.contains("\"cancel_on_error\":false"), "{encoded}");
}
