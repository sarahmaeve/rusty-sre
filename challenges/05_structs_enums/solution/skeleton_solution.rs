// =============================================================================
// Challenge 05: Structs, Enums, impl — Incident State Machine (SKELETON SOLUTION)
// =============================================================================
//
// Reference implementation of skeleton.rs with every TODO completed.
// Run the tests from inside the solution/ directory:
//     rustc skeleton_solution.rs --edition 2024 --test && ./skeleton_solution
// =============================================================================

fn main() {
    println!("Reference solution — run with --test to execute the tests.");
}

// -----------------------------------------------------------------------------
// Task 1: Define the State enum
// -----------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
enum State {
    Open,
    Acknowledged { by: String },
    Resolved { summary: String },
}

// -----------------------------------------------------------------------------
// Task 2: Define the Incident struct
// -----------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
struct Incident {
    id: u32,
    service: String,
    state: State,
}

impl Incident {
    // -------------------------------------------------------------------------
    // Task 3: Incident::new constructor
    // -------------------------------------------------------------------------
    fn new(id: u32, service: &str) -> Self {
        Self {
            id,
            service: service.to_string(),
            state: State::Open,
        }
    }

    // -------------------------------------------------------------------------
    // Task 4: acknowledge — Open → Acknowledged { by }
    // -------------------------------------------------------------------------
    fn acknowledge(&mut self, by: &str) -> Result<(), String> {
        if self.state == State::Open {
            self.state = State::Acknowledged { by: by.to_string() };
            Ok(())
        } else {
            Err(format!("cannot ack from {}", self.state_label()))
        }
    }

    // -------------------------------------------------------------------------
    // Task 5: resolve — Acknowledged → Resolved { summary }
    // -------------------------------------------------------------------------
    fn resolve(&mut self, summary: &str) -> Result<(), String> {
        if matches!(self.state, State::Acknowledged { .. }) {
            self.state = State::Resolved { summary: summary.to_string() };
            Ok(())
        } else {
            Err(format!("cannot resolve from {}", self.state_label()))
        }
    }

    // -------------------------------------------------------------------------
    // Task 6: state_label
    // -------------------------------------------------------------------------
    fn state_label(&self) -> &str {
        match self.state {
            State::Open => "open",
            State::Acknowledged { .. } => "acknowledged",
            State::Resolved { .. } => "resolved",
        }
    }
}

// =============================================================================
// TESTS — identical to skeleton.rs
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ----- Task 1 + 2 -----

    #[test]
    fn can_construct_states_directly() {
        let _open = State::Open;
        let _ack = State::Acknowledged { by: "alice".to_string() };
        let _res = State::Resolved { summary: "fixed".to_string() };
    }

    // ----- Task 3 -----

    #[test]
    fn new_incident_starts_open() {
        let inc = Incident::new(101, "auth");
        assert_eq!(inc.id, 101);
        assert_eq!(inc.service, "auth");
        assert_eq!(inc.state, State::Open);
    }

    // ----- Task 4 -----

    #[test]
    fn acknowledge_open_succeeds() {
        let mut inc = Incident::new(1, "payments");
        let result = inc.acknowledge("bob");
        assert!(result.is_ok());
        assert_eq!(inc.state, State::Acknowledged { by: "bob".to_string() });
    }

    #[test]
    fn acknowledge_already_acknowledged_fails() {
        let mut inc = Incident::new(1, "payments");
        inc.acknowledge("bob").unwrap();
        let result = inc.acknowledge("alice");
        assert!(result.is_err());
        // State should be unchanged — bob still owns the ack
        assert_eq!(inc.state, State::Acknowledged { by: "bob".to_string() });
    }

    // ----- Task 5 -----

    #[test]
    fn resolve_acknowledged_succeeds() {
        let mut inc = Incident::new(1, "search");
        inc.acknowledge("carol").unwrap();
        let result = inc.resolve("restarted nginx");
        assert!(result.is_ok());
        assert_eq!(
            inc.state,
            State::Resolved { summary: "restarted nginx".to_string() }
        );
    }

    #[test]
    fn resolve_open_fails() {
        let mut inc = Incident::new(1, "search");
        // Skipping acknowledge — should not be allowed
        let result = inc.resolve("eh");
        assert!(result.is_err());
        assert_eq!(inc.state, State::Open);
    }

    #[test]
    fn resolve_already_resolved_fails() {
        let mut inc = Incident::new(1, "search");
        inc.acknowledge("carol").unwrap();
        inc.resolve("first fix").unwrap();
        let result = inc.resolve("second fix");
        assert!(result.is_err());
    }

    // ----- Task 6 -----

    #[test]
    fn state_labels() {
        let mut inc = Incident::new(1, "auth");
        assert_eq!(inc.state_label(), "open");
        inc.acknowledge("alice").unwrap();
        assert_eq!(inc.state_label(), "acknowledged");
        inc.resolve("fixed").unwrap();
        assert_eq!(inc.state_label(), "resolved");
    }
}
