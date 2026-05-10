// =============================================================================
// Challenge 05: Structs, Enums, impl — Debug Solution
// =============================================================================
//
// Bugs fixed:
//
//   1. mark_degraded: `&self` → `&mut self` so the assignment compiles.
//   2. status_label: added the missing `Status::Degraded { .. }` arm.
//   3. Service::new: replaced the local counter with a module-level
//      AtomicU32, bumped once per call.
//   4. apply_status: dropped the unnecessary clone — mutate self directly.
// =============================================================================

use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug, Clone, PartialEq)]
enum Status {
    Healthy,
    Degraded { reason: String },
    Down,
}

#[derive(Debug, Clone, PartialEq)]
struct Service {
    id: u32,
    name: String,
    status: Status,
}

// FIX 3: a process-wide counter so every Service gets a unique id.
static NEXT_ID: AtomicU32 = AtomicU32::new(1);

impl Service {
    fn new(name: &str) -> Self {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        Self {
            id,
            name: name.to_string(),
            status: Status::Healthy,
        }
    }

    // FIX 1: &mut self
    fn mark_degraded(&mut self, reason: &str) {
        self.status = Status::Degraded {
            reason: reason.to_string(),
        };
    }

    // FIX 4: mutate self directly
    fn apply_status(&mut self, status: Status) {
        self.status = status;
    }
}

// FIX 2: handle Degraded
fn status_label(s: &Status) -> &'static str {
    match s {
        Status::Healthy => "healthy",
        Status::Degraded { .. } => "degraded",
        Status::Down => "down",
    }
}

struct Registry {
    services: Vec<Service>,
}

impl Registry {
    fn new() -> Self {
        Self { services: Vec::new() }
    }

    fn register(&mut self, name: &str) -> u32 {
        let svc = Service::new(name);
        let id = svc.id;
        self.services.push(svc);
        id
    }

    fn find(&self, id: u32) -> Option<&Service> {
        self.services.iter().find(|s| s.id == id)
    }
}

fn main() {
    let mut reg = Registry::new();
    let _ = reg.register("auth");
    let _ = reg.register("payments");
    println!("{:#?}", reg.services);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn services_get_unique_ids() {
        let s1 = Service::new("auth");
        let s2 = Service::new("payments");
        let s3 = Service::new("search");
        assert_ne!(s1.id, s2.id);
        assert_ne!(s2.id, s3.id);
        assert_ne!(s1.id, s3.id);
    }

    #[test]
    fn mark_degraded_changes_status() {
        let mut s = Service::new("auth");
        assert_eq!(s.status, Status::Healthy);
        s.mark_degraded("cache miss");
        assert_eq!(
            s.status,
            Status::Degraded { reason: "cache miss".to_string() }
        );
    }

    #[test]
    fn status_label_covers_all_variants() {
        assert_eq!(status_label(&Status::Healthy), "healthy");
        assert_eq!(status_label(&Status::Down), "down");
        assert_eq!(
            status_label(&Status::Degraded { reason: "x".to_string() }),
            "degraded"
        );
    }

    #[test]
    fn apply_status_updates_in_place() {
        let mut s = Service::new("payments");
        s.apply_status(Status::Down);
        assert_eq!(s.status, Status::Down);
    }

    #[test]
    fn registry_tracks_changes() {
        let mut reg = Registry::new();
        let id = reg.register("auth");
        assert_eq!(reg.find(id).unwrap().status, Status::Healthy);

        let id2 = reg.register("payments");
        assert_ne!(id, id2);
    }
}
