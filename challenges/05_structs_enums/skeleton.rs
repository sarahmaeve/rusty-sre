// =============================================================================
// Challenge 05: Structs, Enums, impl — Incident State Machine
// =============================================================================
//
// You are modeling an SRE incident lifecycle. An incident moves through:
//
//     Open → Acknowledged{by} → Resolved{summary}
//
// The transitions are not arbitrary: you can only acknowledge an Open
// incident, and you can only resolve an Acknowledged one. Trying to skip a
// step or transition in the wrong direction returns an error.
//
// Complete each TODO. Run the tests with:
//     rustc skeleton.rs --edition 2024 --test && ./skeleton
//
// Stuck? HINTS.md has a hint per task. A reference implementation lives in
// solution/skeleton_solution.rs — compare after you have a passing version.
// =============================================================================

fn main() {
    println!("Complete the TODO items, then run with --test to verify.");
}

// -----------------------------------------------------------------------------
// Task 1: Define the State enum
// -----------------------------------------------------------------------------
// Define an enum named `State` with three variants:
//
//   Open                                — no payload
//   Acknowledged { by: String }         — struct-like variant with one field
//   Resolved { summary: String }        — struct-like variant with one field
#[derive(Debug, PartialEq)]
enum State {
    // TODO: add the three variants
}

// -----------------------------------------------------------------------------
// Task 2: Define the Incident struct
// -----------------------------------------------------------------------------
// Define a struct named `Incident` with fields:
//
//   id: u32
//   service: String
//   state: State
#[derive(Debug, PartialEq)]
struct Incident {
    // TODO: add the three fields
}

// -----------------------------------------------------------------------------
// Task 3: Incident::new constructor
// -----------------------------------------------------------------------------
// Create a new Incident with the given id and service. The initial state
// should be State::Open.
impl Incident {
    fn new(_id: u32, _service: &str) -> Self {
        // TODO
        todo!()
    }

    // -------------------------------------------------------------------------
    // Task 4: acknowledge
    // -------------------------------------------------------------------------
    // Transition from Open → Acknowledged { by }. If the incident is in any
    // state other than Open, return Err with a message like "cannot ack from
    // <state-label>" — but don't worry about the exact message, the test only
    // checks for Err vs Ok.
    fn acknowledge(&mut self, _by: &str) -> Result<(), String> {
        // TODO
        todo!()
    }

    // -------------------------------------------------------------------------
    // Task 5: resolve
    // -------------------------------------------------------------------------
    // Transition from Acknowledged → Resolved { summary }. If the incident
    // is not in Acknowledged, return Err.
    fn resolve(&mut self, _summary: &str) -> Result<(), String> {
        // TODO
        todo!()
    }

    // -------------------------------------------------------------------------
    // Task 6: state_label
    // -------------------------------------------------------------------------
    // Return a short human label for the current state:
    //
    //   State::Open                  → "open"
    //   State::Acknowledged { .. }   → "acknowledged"
    //   State::Resolved { .. }       → "resolved"
    fn state_label(&self) -> &str {
        // TODO
        todo!()
    }
}

// =============================================================================
// TESTS — Do not modify below this line
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
