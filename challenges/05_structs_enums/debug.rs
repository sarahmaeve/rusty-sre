// =============================================================================
// Challenge 05: Structs, Enums, impl — Debug the Service Registry
// =============================================================================
//
// This program tracks a small fleet of services. Each Service has a unique
// id (assigned by a constructor), a name, and a Status. The registry adds
// services and lets you mark them as degraded.
//
// It contains FOUR bugs — some stop it compiling, some misbehave at runtime.
// Find and fix all four so every test passes.
//
// Stuck? HINTS.md reveals each bug in stages: symptom, location, then fix.
//
// Run the tests with:
//     rustc debug.rs --edition 2024 --test && ./debug
// =============================================================================

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

impl Service {
    // Construct a new service. Every service must get a unique id —
    // the ids are assigned from a monotonically increasing counter.
    fn new(name: &str) -> Self {
        let mut next_id: u32 = 0;
        next_id += 1;
        Self {
            id: next_id,
            name: name.to_string(),
            status: Status::Healthy,
        }
    }

    // Mark this service as Degraded with the given reason.
    fn mark_degraded(&self, reason: &str) {
        self.status = Status::Degraded {
            reason: reason.to_string(),
        };
    }

    // Replace this service's status with the given one.
    fn apply_status(&mut self, status: Status) {
        let mut copy = self.clone();
        copy.status = status;
    }
}

// Short human-readable label for a status.
fn status_label(s: &Status) -> &'static str {
    match s {
        Status::Healthy => "healthy",
        Status::Down => "down",
    }
}

// -----------------------------------------------------------------------------
// Registry — wraps a Vec of services and exposes the operations above.
// -----------------------------------------------------------------------------

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

// =============================================================================
// TESTS
// =============================================================================

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

    // Integration test
    #[test]
    fn registry_tracks_changes() {
        let mut reg = Registry::new();
        let id = reg.register("auth");

        // The just-registered service is healthy
        assert_eq!(reg.find(id).unwrap().status, Status::Healthy);

        // Adding more services gets distinct ids
        let id2 = reg.register("payments");
        assert_ne!(id, id2);
    }
}
