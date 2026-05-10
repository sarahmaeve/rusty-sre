// =============================================================================
// Challenge 08: Modules and Visibility — Debug the Alert Pipeline
// =============================================================================
//
// This program is an alert ingestion pipeline organized into three nested
// modules: parse, dedup, and route. It contains FOUR bugs related to
// modules and visibility:
//
//   1. Alert::severity is not pub, but other modules try to read it (compile)
//   2. dedup::count_unique is not pub, but the tests need to reach it
//      (compile)
//   3. The `route` module is declared `mod route` instead of `pub mod route`,
//      so callers outside the parent module cannot reach it (compile)
//   4. dedup::record_alert calls a local no-op stub `local_counter` instead
//      of the real `super::shared::next_id` — counter never advances (runtime)
//
// Run the tests with:
//     rustc debug.rs --edition 2024 --test && ./debug
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

        // The real, monotonic counter. Lives in `shared` so multiple
        // sibling modules can use it without each owning their own.
        static NEXT_ID: AtomicU32 = AtomicU32::new(1);

        pub fn next_id() -> u32 {
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        }

        // For tests only — reset the counter so order-independent.
        pub fn reset_for_tests() {
            NEXT_ID.store(1, Ordering::Relaxed);
        }
    }

    pub mod parse {
        // -----------------------------------------------------------------
        // BUG 1: Alert.severity is not pub
        // -----------------------------------------------------------------
        // dedup::is_critical needs to read `alert.severity`. Since
        // `severity` lacks `pub`, the field is private to this module and
        // sibling modules can't see it.
        #[derive(Debug, Clone, PartialEq)]
        pub struct Alert {
            pub id: u32,
            pub service: String,
            severity: u8,
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

        // Local placeholder counter — kept for ad-hoc tests. The real
        // counter lives in super::shared::next_id().
        fn local_counter() -> u32 {
            0
        }

        // -----------------------------------------------------------------
        // BUG 2: count_unique is not pub
        // -----------------------------------------------------------------
        // The tests reach this function from outside the `dedup` module:
        //     pipeline::dedup::count_unique(&alerts)
        // It must be marked `pub` to be reachable.
        fn count_unique(alerts: &[Alert]) -> usize {
            let mut seen_services: std::collections::HashSet<&str> =
                std::collections::HashSet::new();
            for a in alerts {
                seen_services.insert(&a.service);
            }
            seen_services.len()
        }

        pub fn is_critical(alert: &Alert) -> bool {
            // Reads parse::Alert::severity, which BUG 1 makes inaccessible.
            alert.severity >= 4
        }

        // -----------------------------------------------------------------
        // BUG 4: calls the local stub instead of the shared counter
        // -----------------------------------------------------------------
        // The intent is to assign each recorded alert a fresh id from the
        // shared counter. The current code calls local_counter() — a stub
        // that always returns 0 — so every alert ends up with id 0.
        //
        // Fix: change `local_counter()` to `super::shared::next_id()`.
        pub fn record_alert(service: &str, severity: u8, message: &str) -> Alert {
            let id = local_counter();
            Alert::new(id, service, severity, message)
        }
    }

    // -----------------------------------------------------------------
    // BUG 3: mod route should be pub mod route
    // -----------------------------------------------------------------
    // Without `pub`, this module is private to its parent (`pipeline`).
    // Callers outside `pipeline` cannot reach `pipeline::route::*`.
    mod route {
        use super::parse::Alert;

        pub fn destination(alert: &Alert) -> &'static str {
            match alert.service.as_str() {
                "auth" | "payments" => "pagerduty",
                _ => "slack",
            }
        }
    }
}

// =============================================================================
// TESTS
// =============================================================================

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

    // BUG 2: count_unique must be pub
    #[test]
    fn count_unique_services() {
        let alerts = sample_alerts();
        // 3 unique services: auth, payments, search
        assert_eq!(pipeline::dedup::count_unique(&alerts), 3);
    }

    // BUG 1: severity must be pub for is_critical to compile
    #[test]
    fn is_critical_uses_severity() {
        let alerts = sample_alerts();
        assert!(pipeline::dedup::is_critical(&alerts[0])); // 4
        assert!(!pipeline::dedup::is_critical(&alerts[1])); // 2
        assert!(pipeline::dedup::is_critical(&alerts[2])); // 5
    }

    // BUG 3: pub mod route — without pub, this path is unreachable
    #[test]
    fn route_module_is_reachable() {
        let alert = pipeline::parse::Alert::new(1, "auth", 5, "x");
        assert_eq!(pipeline::route::destination(&alert), "pagerduty");

        let alert = pipeline::parse::Alert::new(2, "search", 3, "x");
        assert_eq!(pipeline::route::destination(&alert), "slack");
    }

    // BUG 4: record_alert should use the shared counter
    #[test]
    fn record_alert_uses_shared_counter() {
        pipeline::shared::reset_for_tests();
        let a = pipeline::dedup::record_alert("auth", 4, "x");
        let b = pipeline::dedup::record_alert("payments", 5, "y");
        let c = pipeline::dedup::record_alert("search", 3, "z");
        // Ids should be monotonic (1, 2, 3 after reset). They will all be
        // 0 until BUG 4 is fixed.
        assert_ne!(a.id, b.id);
        assert_ne!(b.id, c.id);
        assert_ne!(a.id, c.id);
        assert_eq!(a.id, 1);
        assert_eq!(b.id, 2);
        assert_eq!(c.id, 3);
    }
}
