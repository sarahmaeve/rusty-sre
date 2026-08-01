use std::error::Error;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

use ops_core::graph::Component;
use ops_core::health::{Probe, fleet_ready, is_json_config, is_success, remove_decommissioned};
use ops_core::lease::{Lease, active_count};
use ops_core::registry::Registry;
use ops_core::retry::{PipelineError, retry};
use ops_core::shared::SharedCounts;

#[test]
#[ignore = "exercise 09"]
fn exercise_09_every_probe_must_be_ready() {
    assert!(!fleet_ready(&[
        Probe { ready: true },
        Probe { ready: false }
    ]));
}

#[test]
#[ignore = "exercise 10"]
fn exercise_10_http_299_is_success() {
    assert!(is_success(299));
    assert!(!is_success(300));
}

#[test]
#[ignore = "exercise 11"]
fn exercise_11_decommissioned_hosts_are_removed() {
    let mut hosts = vec!["api-1".to_owned(), "old-1".to_owned()];
    remove_decommissioned(&mut hosts, &["old-1".to_owned()]);
    assert_eq!(hosts, ["api-1"]);
}

#[test]
#[ignore = "exercise 12"]
fn exercise_12_retries_are_in_addition_to_the_first_attempt() {
    let mut attempts = 0;
    let result: Result<&str, &str> = retry(2, || {
        attempts += 1;
        if attempts == 3 {
            Ok("ready")
        } else {
            Err("down")
        }
    });
    assert_eq!(result, Ok("ready"));
    assert_eq!(attempts, 3);
}

#[test]
#[ignore = "exercise 13"]
fn exercise_13_io_error_chain_is_preserved() {
    let error = PipelineError::io(
        "inventory read failed",
        std::io::Error::new(std::io::ErrorKind::NotFound, "inventory.json"),
    );
    assert!(error.source().is_some());
}

#[test]
#[ignore = "exercise 14"]
fn exercise_14_extensions_use_path_semantics() {
    assert!(is_json_config(Path::new("CONFIG.JSON")));
    assert!(!is_json_config(Path::new("archive.json/entries")));
}

#[test]
#[ignore = "exercise 15"]
fn exercise_15_registry_rename_does_not_reborrow() {
    let registry = Registry::default();
    registry.insert("api", "platform");
    assert!(registry.rename("api", "gateway"));
    assert!(registry.contains("gateway"));
}

#[test]
#[ignore = "exercise 16"]
fn exercise_16_parent_links_do_not_form_cycles() {
    let weak_parent = {
        let parent = Component::new("pipeline");
        let child = Component::new("collector");
        Component::attach(&parent, &child);
        Rc::downgrade(&parent)
    };
    assert!(weak_parent.upgrade().is_none());
}

#[test]
#[ignore = "exercise 17"]
fn exercise_17_poisoned_state_can_be_inspected() {
    let counts = SharedCounts::default();
    counts.push(7);
    counts.poison_for_test();
    assert_eq!(counts.snapshot(), [7]);
}

#[test]
#[ignore = "exercise 18"]
fn exercise_18_drop_releases_the_lease() {
    let active = Arc::new(AtomicUsize::new(0));
    {
        let _lease = Lease::acquire(Arc::clone(&active));
        assert_eq!(active_count(&active), 1);
    }
    assert_eq!(active_count(&active), 0);
}
