// =============================================================================
// Challenge 09: Ownership & Borrowing with Vectors — Debug the Incident Tracker
// =============================================================================
//
// This program manages SRE incidents: it stores them, enriches them with
// runbook links, generates reports, and archives resolved incidents.
//
// It contains FOUR bugs related to ownership and borrowing with vectors.
//
// Your task: find and fix all the bugs so that every test passes. The
// compiler error messages are educational — read them before fixing.
// Run the tests with:
//     rustc debug.rs --edition 2024 --test && ./debug
//
// Stuck? HINTS.md reveals each bug in stages: symptom, location, then fix.
// =============================================================================

fn main() {
    let incidents = create_sample_incidents();
    match generate_report(&incidents) {
        Ok(report) => println!("{report}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Incident {
    id: u32,
    service: String,
    severity: String, // "sev1", "sev2", "sev3"
    title: String,
    resolved: bool,
    runbook: Option<String>,
}

impl Incident {
    fn new(id: u32, service: &str, severity: &str, title: &str) -> Self {
        Incident {
            id,
            service: service.to_string(),
            severity: severity.to_string(),
            title: title.to_string(),
            resolved: false,
            runbook: None,
        }
    }
}

// Split incidents into (critical, non-critical) by severity.
fn triage_incidents(incidents: Vec<Incident>) -> (Vec<Incident>, Vec<Incident>) {
    // Count total for logging
    let total = count_incidents(incidents);
    println!("Triaging {total} incidents...");

    let (critical, non_critical): (Vec<Incident>, Vec<Incident>) = incidents
        .into_iter()
        .partition(|i| i.severity == "sev1");

    (critical, non_critical)
}

// Count the incidents.
fn count_incidents(incidents: Vec<Incident>) -> usize {
    incidents.len()
}

// Attach the matching runbook link to every incident whose service has one.
fn enrich_with_runbooks(incidents: &mut Vec<Incident>) {
    let runbooks = vec![
        ("auth", "https://runbooks.internal/auth-recovery"),
        ("db", "https://runbooks.internal/db-failover"),
        ("payments", "https://runbooks.internal/payments-oncall"),
    ];

    for (service, runbook_url) in &runbooks {
        let found = incidents.iter().find(|i| &i.service == service);
        if let Some(incident) = found {
            for inc in incidents.iter_mut() {
                if inc.service == incident.service {
                    inc.runbook = Some(runbook_url.to_string());
                }
            }
        }
    }
}

// Titles of all unresolved incidents.
fn get_unresolved_titles(incidents: &[Incident]) -> Vec<&str> {
    let unresolved: Vec<Incident> = incidents
        .iter()
        .filter(|i| !i.resolved)
        .cloned()
        .collect();

    unresolved.iter().map(|i| i.title.as_str()).collect()
}

// Render a report of the active incidents, with totals at the bottom.
fn generate_report(incidents: &[Incident]) -> Result<String, String> {
    if incidents.is_empty() {
        return Err("No incidents to report".to_string());
    }

    let active: Vec<Incident> = incidents.iter().filter(|i| !i.resolved).cloned().collect();

    let mut report = String::from("=== Incident Report ===\n");

    for incident in active {
        let status = if incident.resolved { "RESOLVED" } else { "ACTIVE" };
        let runbook = incident.runbook.as_deref().unwrap_or("none");
        report.push_str(&format!(
            "[{}] {}: {} (runbook: {})\n",
            status, incident.severity, incident.title, runbook
        ));
    }

    report.push_str(&format!("\nTotal active incidents: {}\n", active.len()));

    let sev1_count = active.iter().filter(|i| i.severity == "sev1").count();
    report.push_str(&format!("Sev1 incidents: {sev1_count}\n"));

    Ok(report)
}

fn create_sample_incidents() -> Vec<Incident> {
    let mut incidents = vec![
        Incident::new(1, "auth", "sev1", "Login service down"),
        Incident::new(2, "payments", "sev2", "Payment processing slow"),
        Incident::new(3, "db", "sev1", "Primary DB replication lag"),
    ];
    incidents[0].resolved = false;
    incidents[1].resolved = false;
    incidents[2].resolved = true;
    incidents
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn test_incidents() -> Vec<Incident> {
        let mut incidents = vec![
            Incident::new(1, "auth", "sev1", "Login service down"),
            Incident::new(2, "payments", "sev2", "Payment processing slow"),
            Incident::new(3, "db", "sev1", "Primary DB replication lag"),
            Incident::new(4, "auth", "sev3", "Minor auth latency spike"),
            Incident::new(5, "logging", "sev2", "Log ingestion delayed"),
        ];
        incidents[2].resolved = true;
        incidents[4].resolved = true;
        incidents
    }

    #[test]
    fn test_triage_incidents() {
        let incidents = test_incidents();
        let (critical, non_critical) = triage_incidents(incidents);
        assert_eq!(critical.len(), 2); // two sev1 incidents
        assert_eq!(non_critical.len(), 3);
        assert!(critical.iter().all(|i| i.severity == "sev1"));
    }

    #[test]
    fn test_triage_empty() {
        let (critical, non_critical) = triage_incidents(vec![]);
        assert!(critical.is_empty());
        assert!(non_critical.is_empty());
    }

    #[test]
    fn test_enrich_with_runbooks() {
        let mut incidents = test_incidents();
        enrich_with_runbooks(&mut incidents);

        let auth = incidents.iter().find(|i| i.id == 1).unwrap();
        assert_eq!(
            auth.runbook.as_deref(),
            Some("https://runbooks.internal/auth-recovery")
        );

        let db = incidents.iter().find(|i| i.id == 3).unwrap();
        assert_eq!(
            db.runbook.as_deref(),
            Some("https://runbooks.internal/db-failover")
        );

        // logging has no runbook mapping
        let logging = incidents.iter().find(|i| i.id == 5).unwrap();
        assert!(logging.runbook.is_none());
    }

    #[test]
    fn test_get_unresolved_titles() {
        let incidents = test_incidents();
        let titles = get_unresolved_titles(&incidents);
        assert_eq!(titles.len(), 3);
        assert!(titles.contains(&"Login service down"));
        assert!(titles.contains(&"Payment processing slow"));
        assert!(titles.contains(&"Minor auth latency spike"));
        // Verify original vec is still intact
        assert_eq!(incidents.len(), 5);
    }

    #[test]
    fn test_get_unresolved_titles_all_resolved() {
        let mut incidents = test_incidents();
        for inc in incidents.iter_mut() {
            inc.resolved = true;
        }
        let titles = get_unresolved_titles(&incidents);
        assert!(titles.is_empty());
    }

    #[test]
    fn test_generate_report() {
        let incidents = test_incidents();
        let report = generate_report(&incidents).unwrap();
        assert!(report.contains("Incident Report"));
        assert!(report.contains("Total active incidents: 3"));
        assert!(report.contains("Sev1 incidents: 1"));
        assert!(report.contains("Login service down"));
    }

    #[test]
    fn test_generate_report_empty() {
        let incidents: Vec<Incident> = vec![];
        assert!(generate_report(&incidents).is_err());
    }

    // Integration test
    #[test]
    fn test_full_pipeline() {
        let incidents = test_incidents();

        // Triage
        let (critical, _non_critical) = triage_incidents(incidents.clone());
        assert_eq!(critical.len(), 2);

        // Enrich
        let mut enriched = incidents.clone();
        enrich_with_runbooks(&mut enriched);
        let with_runbooks: Vec<&Incident> =
            enriched.iter().filter(|i| i.runbook.is_some()).collect();
        assert!(!with_runbooks.is_empty());

        // Report
        let report = generate_report(&enriched).unwrap();
        assert!(report.contains("Incident Report"));
    }
}
