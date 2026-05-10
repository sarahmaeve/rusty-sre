// =============================================================================
// Challenge 05: Structs, Enums, impl — Debug the Service Registry
// =============================================================================
//
// This program tracks a small fleet of services. Each Service has a unique
// id (assigned by a constructor), a name, and a Status. The registry adds
// services and lets you mark them as degraded.
//
// It contains FOUR bugs:
//
//   1. mark_degraded mutates a value through a `&self` method (compile error)
//   2. status_label has a non-exhaustive match — a new variant was added but
//      not handled (compile error)
//   3. Service::new uses a mis-shared counter so every service gets id 1
//      (runtime: ids collide)
//   4. apply_status mutates a clone of self instead of self (runtime: the
//      caller's instance stays Healthy after a Down was applied)
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
    // -----------------------------------------------------------------------
    // BUG 3: every service gets id 1
    // -----------------------------------------------------------------------
    // The intent was a monotonically increasing counter, but `next_id` is
    // declared INSIDE the function — it gets reset to 1 on every call, so
    // every service ends up with id 1.
    //
    // Fix: lift the counter into a module-level static or pass it in. For
    // this exercise, use a `static AtomicU32` outside the impl and bump it
    // with .fetch_add(1, Ordering::Relaxed).
    fn new(name: &str) -> Self {
        let mut next_id: u32 = 0;
        next_id += 1;
        Self {
            id: next_id,
            name: name.to_string(),
            status: Status::Healthy,
        }
    }

    // -----------------------------------------------------------------------
    // BUG 1: cannot mutate via &self
    // -----------------------------------------------------------------------
    // Mark this service as Degraded with the given reason. The signature
    // takes `&self` (immutable borrow); mutating self.status doesn't
    // compile. Fix: change to `&mut self`.
    fn mark_degraded(&self, reason: &str) {
        self.status = Status::Degraded {
            reason: reason.to_string(),
        };
    }

    // -----------------------------------------------------------------------
    // BUG 4: mutates a clone instead of self
    // -----------------------------------------------------------------------
    // The signature is &mut self — fine — but inside the body, the code
    // clones self into a local, mutates the local, and throws it away. The
    // caller's instance never changes.
    //
    // Fix: drop the clone. Mutate self.status directly.
    fn apply_status(&mut self, status: Status) {
        let mut copy = self.clone();
        copy.status = status;
    }
}

// -----------------------------------------------------------------------------
// BUG 2: non-exhaustive match
// -----------------------------------------------------------------------------
// status_label was written when Status had only Healthy and Down. Then a
// Degraded variant was added — and this match was never updated. The
// compiler refuses to let it slide.
//
// Fix: add a `Status::Degraded { .. }` arm.
fn status_label(s: &Status) -> &'static str {
    match s {
        Status::Healthy => "healthy",
        Status::Down => "down",
    }
}

// -----------------------------------------------------------------------------
// Registry — wraps a Vec of services and exposes the operations above.
// (No bug in the registry itself — but it depends on Service::new and
// apply_status being correct.)
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

    // BUG 3: ids must be unique
    #[test]
    fn services_get_unique_ids() {
        let s1 = Service::new("auth");
        let s2 = Service::new("payments");
        let s3 = Service::new("search");
        assert_ne!(s1.id, s2.id);
        assert_ne!(s2.id, s3.id);
        assert_ne!(s1.id, s3.id);
    }

    // BUG 1: mark_degraded must mutate self (signature: &mut self)
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

    // BUG 2: status_label must compile (handle Degraded)
    #[test]
    fn status_label_covers_all_variants() {
        assert_eq!(status_label(&Status::Healthy), "healthy");
        assert_eq!(status_label(&Status::Down), "down");
        assert_eq!(
            status_label(&Status::Degraded { reason: "x".to_string() }),
            "degraded"
        );
    }

    // BUG 4: apply_status must actually update self in place
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
