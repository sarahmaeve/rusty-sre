//! Structs group data. Enums describe alternatives and can make invalid states
//! unrepresentable. Methods state how a value may be observed or changed.
//!
//! Further reading:
//! - <https://doc.rust-lang.org/book/ch05-00-structs.html>
//! - <https://doc.rust-lang.org/book/ch06-00-enums.html>
//! - <https://doc.rust-lang.org/book/ch18-03-oo-design-patterns.html>
//! - <https://rust-unofficial.github.io/patterns/patterns/behavioural/typestate.html>
//! - Source study: <https://github.com/tokio-rs/tokio/blob/master/tokio/src/runtime/task/state.rs>

#[derive(Debug, Clone, PartialEq, Eq)]
struct Incident {
    service: String,
    state: IncidentState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum IncidentState {
    Investigating { page_count: u32 },
    Monitoring { fix: String },
    Resolved { fix: String, resolution: String },
}

#[derive(Debug, PartialEq, Eq)]
enum TransitionError {
    AlreadyResolved,
    FixRequired,
}

impl Incident {
    fn new(service: impl Into<String>) -> Self {
        Self {
            service: service.into(),
            state: IncidentState::Investigating { page_count: 1 },
        }
    }

    fn page(&mut self) -> Result<(), TransitionError> {
        match &mut self.state {
            IncidentState::Investigating { page_count } => {
                *page_count += 1;
                Ok(())
            }
            IncidentState::Monitoring { .. } => Ok(()),
            IncidentState::Resolved { .. } => Err(TransitionError::AlreadyResolved),
        }
    }

    fn monitor(&mut self, fix: impl Into<String>) -> Result<(), TransitionError> {
        let fix = fix.into();
        if fix.trim().is_empty() {
            return Err(TransitionError::FixRequired);
        }
        if matches!(self.state, IncidentState::Resolved { .. }) {
            return Err(TransitionError::AlreadyResolved);
        }
        self.state = IncidentState::Monitoring { fix };
        Ok(())
    }

    // Consuming `self` is useful when the old state must not remain usable.
    fn resolve(self, resolution: impl Into<String>) -> Result<Self, TransitionError> {
        match self.state {
            IncidentState::Monitoring { fix } => Ok(Self {
                service: self.service,
                state: IncidentState::Resolved {
                    fix,
                    resolution: resolution.into(),
                },
            }),
            IncidentState::Investigating { .. } => Err(TransitionError::FixRequired),
            IncidentState::Resolved { .. } => Err(TransitionError::AlreadyResolved),
        }
    }

    fn status(&self) -> &'static str {
        match self.state {
            IncidentState::Investigating { .. } => "investigating",
            IncidentState::Monitoring { .. } => "monitoring",
            IncidentState::Resolved { .. } => "resolved",
        }
    }
}

fn main() {
    let incident = Incident::new("payments");
    assert_eq!(incident.status(), "investigating");
    assert_eq!(
        incident.resolve("stable"),
        Err(TransitionError::FixRequired)
    );

    let mut incident = Incident::new("payments");
    incident.page().unwrap();
    incident.monitor("rolled back release 42").unwrap();
    assert_eq!(incident.status(), "monitoring");

    let incident = incident.resolve("error rate recovered").unwrap();
    assert_eq!(incident.status(), "resolved");
    assert!(matches!(
        incident.state,
        IncidentState::Resolved { ref fix, .. } if fix == "rolled back release 42"
    ));
}
