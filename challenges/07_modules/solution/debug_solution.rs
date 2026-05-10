// =============================================================================
// Challenge 07: Modules and Visibility — Debug Solution
// =============================================================================
//
// Bugs fixed:
//
//   1. Alert.severity made `pub` so sibling modules can read it.
//   2. dedup::count_unique made `pub` so it's reachable from outside.
//   3. `mod route` changed to `pub mod route` so callers outside the parent
//      can reach `pipeline::route::destination`.
//   4. dedup::record_alert calls `super::shared::next_id()` instead of the
//      local `local_counter()` stub, so ids advance monotonically.
// =============================================================================

use std::sync::atomic::{AtomicU32, Ordering};

fn main() {
    let alerts = vec![
        pipeline::parse::Alert::new(1, "auth", 4, "p99 spike"),
        pipeline::parse::Alert::new(2, "payments", 5, "checkout fail"),
        pipeline::parse::Alert::new(3, "auth", 2, "warn"),
    ];
    let unique = pipeline::dedup::count_unique(&alerts);
    println!("unique alerts: {unique}");
}

mod pipeline {
    pub mod shared {
        use super::super::*;

        static NEXT_ID: AtomicU32 = AtomicU32::new(1);

        pub fn next_id() -> u32 {
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        }

        pub fn reset_for_tests() {
            NEXT_ID.store(1, Ordering::Relaxed);
        }
    }

    pub mod parse {
        // FIX 1: severity is now pub.
        #[derive(Debug, Clone, PartialEq)]
        pub struct Alert {
            pub id: u32,
            pub service: String,
            pub severity: u8,
            pub message: String,
        }

        impl Alert {
            pub fn new(id: u32, service: &str, severity: u8, message: &str) -> Self {
                Self {
                    id,
                    service: service.to_string(),
                    severity,
                    message: message.to_string(),
                }
            }
        }
    }

    pub mod dedup {
        use super::parse::Alert;

        fn local_counter() -> u32 {
            0
        }

        // FIX 2: count_unique is now pub.
        pub fn count_unique(alerts: &[Alert]) -> usize {
            let mut seen_services: std::collections::HashSet<&str> =
                std::collections::HashSet::new();
            for a in alerts {
                seen_services.insert(&a.service);
            }
            seen_services.len()
        }

        pub fn is_critical(alert: &Alert) -> bool {
            alert.severity >= 4
        }

        // FIX 4: call the shared counter, not the local stub.
        pub fn record_alert(service: &str, severity: u8, message: &str) -> Alert {
            let id = super::shared::next_id();
            Alert::new(id, service, severity, message)
        }

        // local_counter is now unused; left in place because removing it
        // is not part of the bug. (Removing it is a fine bonus tidy-up.)
        #[allow(dead_code)]
        fn _suppress_unused_warning() {
            let _ = local_counter();
        }
    }

    // FIX 3: pub mod route.
    pub mod route {
        use super::parse::Alert;

        pub fn destination(alert: &Alert) -> &'static str {
            match alert.service.as_str() {
                "auth" | "payments" => "pagerduty",
                _ => "slack",
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_alerts() -> Vec<pipeline::parse::Alert> {
        pipeline::shared::reset_for_tests();
        vec![
            pipeline::parse::Alert::new(1, "auth", 4, "p99"),
            pipeline::parse::Alert::new(2, "auth", 2, "low"),
            pipeline::parse::Alert::new(3, "payments", 5, "fail"),
            pipeline::parse::Alert::new(4, "search", 3, "warn"),
        ]
    }

    #[test]
    fn count_unique_services() {
        let alerts = sample_alerts();
        assert_eq!(pipeline::dedup::count_unique(&alerts), 3);
    }

    #[test]
    fn is_critical_uses_severity() {
        let alerts = sample_alerts();
        assert!(pipeline::dedup::is_critical(&alerts[0]));
        assert!(!pipeline::dedup::is_critical(&alerts[1]));
        assert!(pipeline::dedup::is_critical(&alerts[2]));
    }

    #[test]
    fn route_module_is_reachable() {
        let alert = pipeline::parse::Alert::new(1, "auth", 5, "x");
        assert_eq!(pipeline::route::destination(&alert), "pagerduty");

        let alert = pipeline::parse::Alert::new(2, "search", 3, "x");
        assert_eq!(pipeline::route::destination(&alert), "slack");
    }

    #[test]
    fn record_alert_uses_shared_counter() {
        pipeline::shared::reset_for_tests();
        let a = pipeline::dedup::record_alert("auth", 4, "x");
        let b = pipeline::dedup::record_alert("payments", 5, "y");
        let c = pipeline::dedup::record_alert("search", 3, "z");
        assert_ne!(a.id, b.id);
        assert_ne!(b.id, c.id);
        assert_ne!(a.id, c.id);
        assert_eq!(a.id, 1);
        assert_eq!(b.id, 2);
        assert_eq!(c.id, 3);
    }
}
