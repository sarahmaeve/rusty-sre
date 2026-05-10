// ============================================================================
// Challenge 03: derive — Debug Challenge: "On-Call Dashboard"
// ============================================================================
//
// SRE Scenario: An on-call dashboard that displays and sorts incidents.
// This code has 4 BUGS related to derive and trait implementations.
//
// Bug types:
//   - 2 compile-time errors (the code won't even build)
//   - 2 runtime errors (tests fail silently due to trait contract violations)
//
// Your mission: find and fix all 4 bugs so every test passes.
//
// Run with:
//     rustc debug.rs --edition 2024 --test && ./debug

use std::collections::{BTreeMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};

// ── Priority enum (working correctly) ───────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "LOW"),
            Priority::Medium => write!(f, "MEDIUM"),
            Priority::High => write!(f, "HIGH"),
            Priority::Critical => write!(f, "CRITICAL"),
        }
    }
}

// ── Incident struct ─────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct Incident {
    id: u64,
    service: String,
    priority: Priority,
    description: String,
    timestamp: u64,
}

// Custom PartialEq: two incidents are duplicates if they have the same
// service, priority, and description (ignoring id and timestamp).
impl PartialEq for Incident {
    fn eq(&self, other: &Self) -> bool {
        self.service == other.service
            && self.priority == other.priority
            && self.description == other.description
    }
}
impl Eq for Incident {}

// ── BUG #2: Hash is inconsistent with PartialEq ────────────────────────────
// PartialEq ignores id and timestamp, but Hash includes ALL fields.
// This violates the contract: if a == b then hash(a) must == hash(b).
// HashSet dedup will silently fail.
impl Hash for Incident {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.service.hash(state);
        self.priority.hash(state);
        self.description.hash(state);
        self.timestamp.hash(state);
    }
}

// ── BUG #4: Display uses {:?} instead of {} for priority ───────────────────
// This produces "[Critical]" (Debug format) instead of "[CRITICAL]" (Display).
impl fmt::Display for Incident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}] {}: {}", self.priority, self.service, self.description)
    }
}

// ── BUG #3: Copy on struct with String fields ───────────────────────────────
// String is heap-allocated and cannot be Copy. Remove Copy from the derive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct OnCallEngineer {
    name: String,
    team: String,
    escalation_level: u8,
}

// ── BUG #1: Missing Ord derive ──────────────────────────────────────────────
// IncidentKey is used as a BTreeMap key and in .sort(), which require Ord.
// It derives PartialOrd but NOT Ord.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
struct IncidentKey {
    priority: Priority,
    service: String,
}

// ── Dashboard functions ─────────────────────────────────────────────────────

/// Group incidents by (priority, service) key using a BTreeMap.
/// Returns a sorted map of incident groups.
fn group_incidents(incidents: &[Incident]) -> BTreeMap<IncidentKey, Vec<&Incident>> {
    let mut groups: BTreeMap<IncidentKey, Vec<&Incident>> = BTreeMap::new();
    for incident in incidents {
        let key = IncidentKey {
            priority: incident.priority,
            service: incident.service.clone(),
        };
        groups.entry(key).or_default().push(incident);
    }
    groups
}

/// Deduplicate incidents using a HashSet, keeping the earliest occurrence.
fn dedup_incidents(incidents: Vec<Incident>) -> Vec<Incident> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    for incident in incidents {
        if seen.insert(incident.clone()) {
            result.push(incident);
        }
    }
    result
}

/// Sort incident keys by priority descending, then service ascending.
fn sort_keys(keys: &mut Vec<IncidentKey>) {
    keys.sort_by(|a, b| {
        b.priority
            .cmp(&a.priority)
            .then_with(|| a.service.cmp(&b.service))
    });
}

/// Format an incident for display on the dashboard.
fn format_for_dashboard(incident: &Incident) -> String {
    format!("{}", incident)
}

/// Create an on-call engineer.
fn create_engineer(name: &str, team: &str, level: u8) -> OnCallEngineer {
    OnCallEngineer {
        name: name.to_string(),
        team: team.to_string(),
        escalation_level: level,
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incident_grouping() {
        let incidents = vec![
            Incident {
                id: 1,
                service: "auth".into(),
                priority: Priority::Critical,
                description: "login failures".into(),
                timestamp: 1000,
            },
            Incident {
                id: 2,
                service: "auth".into(),
                priority: Priority::Critical,
                description: "token expiry".into(),
                timestamp: 1001,
            },
            Incident {
                id: 3,
                service: "db".into(),
                priority: Priority::High,
                description: "replication lag".into(),
                timestamp: 1002,
            },
        ];
        let groups = group_incidents(&incidents);
        assert_eq!(groups.len(), 2);
    }

    #[test]
    fn test_incident_dedup() {
        let incidents = vec![
            Incident {
                id: 1,
                service: "api".into(),
                priority: Priority::High,
                description: "timeout".into(),
                timestamp: 1000,
            },
            Incident {
                id: 2,
                service: "api".into(),
                priority: Priority::High,
                description: "timeout".into(),
                timestamp: 2000,
            },
            Incident {
                id: 3,
                service: "api".into(),
                priority: Priority::High,
                description: "timeout".into(),
                timestamp: 3000,
            },
        ];
        let deduped = dedup_incidents(incidents);
        assert_eq!(deduped.len(), 1, "Should dedup to 1 unique incident");
        assert_eq!(deduped[0].id, 1, "Should keep earliest occurrence");
    }

    #[test]
    fn test_sort_keys_priority_desc() {
        let mut keys = vec![
            IncidentKey { priority: Priority::Low, service: "web".into() },
            IncidentKey { priority: Priority::Critical, service: "db".into() },
            IncidentKey { priority: Priority::Medium, service: "api".into() },
        ];
        sort_keys(&mut keys);
        assert_eq!(keys[0].priority, Priority::Critical);
        assert_eq!(keys[1].priority, Priority::Medium);
        assert_eq!(keys[2].priority, Priority::Low);
    }

    #[test]
    fn test_display_format() {
        let incident = Incident {
            id: 1,
            service: "auth".into(),
            priority: Priority::Critical,
            description: "connection pool exhausted".into(),
            timestamp: 1000,
        };
        let output = format_for_dashboard(&incident);
        assert_eq!(output, "[CRITICAL] auth: connection pool exhausted");
    }

    #[test]
    fn test_engineer_creation() {
        let eng = create_engineer("Alice", "Platform", 1);
        assert_eq!(eng.name, "Alice");
        assert_eq!(eng.team, "Platform");
        assert_eq!(eng.escalation_level, 1);
    }

    #[test]
    fn test_engineer_equality() {
        let eng1 = create_engineer("Alice", "Platform", 1);
        let eng2 = create_engineer("Alice", "Platform", 1);
        assert_eq!(eng1, eng2);
    }

    #[test]
    fn test_btreemap_key_ordering() {
        let mut map: BTreeMap<IncidentKey, u32> = BTreeMap::new();
        map.insert(
            IncidentKey { priority: Priority::High, service: "db".into() },
            3,
        );
        map.insert(
            IncidentKey { priority: Priority::Low, service: "web".into() },
            1,
        );
        map.insert(
            IncidentKey { priority: Priority::High, service: "api".into() },
            2,
        );
        // BTreeMap orders by key — Low < Medium < High < Critical
        let keys: Vec<_> = map.keys().collect();
        assert_eq!(keys[0].priority, Priority::Low);
        assert_eq!(keys[1].priority, Priority::High);
        assert_eq!(keys[1].service, "api");
        assert_eq!(keys[2].priority, Priority::High);
        assert_eq!(keys[2].service, "db");
    }

    #[test]
    fn test_dedup_different_incidents_kept() {
        let incidents = vec![
            Incident {
                id: 1,
                service: "api".into(),
                priority: Priority::High,
                description: "timeout".into(),
                timestamp: 1000,
            },
            Incident {
                id: 2,
                service: "db".into(),
                priority: Priority::Critical,
                description: "disk full".into(),
                timestamp: 1000,
            },
        ];
        let deduped = dedup_incidents(incidents);
        assert_eq!(deduped.len(), 2, "Different incidents should be kept");
    }
}

fn main() {
    println!("This file is meant to be run as tests:");
    println!("  rustc debug.rs --edition 2024 --test && ./debug");
}
