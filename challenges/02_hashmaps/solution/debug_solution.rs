// ============================================================================
// Challenge 02 — Solution: Incident Correlation Engine (all 4 bugs fixed)
// ============================================================================
//
// Run:  rustc debug_solution.rs --edition 2021 --test && ./debug_solution

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct LogRecord {
    timestamp: String,
    service: String,
    level: String,
    message: String,
}

fn parse_log_line(line: &str) -> Option<LogRecord> {
    let mut parts = line.splitn(4, ' ');
    let timestamp = parts.next()?.to_string();
    let service = parts.next()?.to_string();
    let level = parts.next()?.to_string();
    let message = parts.next()?.to_string();
    Some(LogRecord {
        timestamp,
        service,
        level,
        message,
    })
}

/// FIX #1: Use Entry API instead of insert() to increment counts.
/// Before: error_counts.insert(service, 1) — always resets to 1.
/// After:  *error_counts.entry(service).or_insert(0) += 1 — increments.
fn count_errors_per_service(records: &[LogRecord]) -> HashMap<String, usize> {
    let mut error_counts: HashMap<String, usize> = HashMap::new();

    for record in records {
        if record.level == "ERROR" {
            *error_counts.entry(record.service.clone()).or_insert(0) += 1;
        }
    }

    error_counts
}

/// FIX #2: Use .get() with unwrap_or instead of direct indexing.
/// Before: error_counts[&service] — panics on missing key.
/// After:  error_counts.get(service).copied().unwrap_or(0) — returns 0.
fn worst_service(error_counts: &HashMap<String, usize>) -> Option<(String, usize)> {
    if error_counts.is_empty() {
        return None;
    }

    let known_services = vec![
        "auth".to_string(),
        "gateway".to_string(),
        "payments".to_string(),
        "db".to_string(),
        "monitoring".to_string(),
        "cache".to_string(),
        "scheduler".to_string(),
    ];

    let mut worst_name = String::new();
    let mut worst_count = 0;

    for service in &known_services {
        let count = error_counts.get(service).copied().unwrap_or(0);
        if count > worst_count {
            worst_count = count;
            worst_name = service.clone();
        }
    }

    if worst_count > 0 {
        Some((worst_name, worst_count))
    } else {
        None
    }
}

/// FIX #3: Use Entry API instead of get() + push.
/// Before: groups.get(&key) returns &Vec — can't push to immutable ref.
/// After:  groups.entry(key).or_insert_with(Vec::new).push(msg)
fn build_service_groups(records: &[LogRecord]) -> HashMap<String, Vec<String>> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    for record in records {
        let formatted = format!("[{}] {}", record.level, record.message);
        groups
            .entry(record.service.clone())
            .or_insert_with(Vec::new)
            .push(formatted);
    }

    groups
}

/// FIX #4: Unwrap Option<&usize> before comparing with usize.
/// Before: error_counts.get(&service) > threshold — type mismatch.
/// After:  if let Some(&count) = error_counts.get(service) { if count > threshold }
fn generate_correlation_report(
    records: &[LogRecord],
    error_counts: &HashMap<String, usize>,
    threshold: usize,
) -> Vec<String> {
    let mut report: Vec<String> = Vec::new();

    let mut services: Vec<String> = records
        .iter()
        .map(|r| r.service.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    services.sort();

    for service in &services {
        if let Some(&count) = error_counts.get(service) {
            if count > threshold {
                let total_records =
                    records.iter().filter(|r| r.service == *service).count();
                report.push(format!(
                    "{}: {} errors out of {} total records",
                    service, count, total_records
                ));
            }
        }
    }

    report
}

// ============================================================================
// TESTS (identical to debug.rs)
// ============================================================================

fn test_records() -> Vec<LogRecord> {
    let log_lines = vec![
        "2026-03-10T08:01:12Z auth INFO User login successful",
        "2026-03-10T08:01:15Z gateway INFO Request routed to auth",
        "2026-03-10T08:01:18Z auth ERROR Authentication token expired",
        "2026-03-10T08:01:22Z payments INFO Payment processed",
        "2026-03-10T08:01:25Z db INFO Query executed in 45ms",
        "2026-03-10T08:01:30Z auth ERROR Invalid credentials provided",
        "2026-03-10T08:01:35Z gateway WARN High latency detected",
        "2026-03-10T08:01:40Z monitoring INFO Health check passed",
        "2026-03-10T08:01:45Z auth ERROR Rate limit exceeded",
        "2026-03-10T08:01:50Z payments WARN Payment gateway timeout",
        "2026-03-10T08:01:55Z db ERROR Connection pool exhausted",
        "2026-03-10T08:02:00Z gateway INFO Request routed to payments",
        "2026-03-10T08:02:05Z auth INFO Password reset initiated",
        "2026-03-10T08:02:10Z monitoring INFO Metrics collected",
        "2026-03-10T08:02:15Z auth ERROR Session validation failed",
        "2026-03-10T08:02:20Z db INFO Index rebuild completed",
        "2026-03-10T08:02:25Z gateway ERROR Upstream unavailable",
        "2026-03-10T08:02:30Z payments INFO Refund processed",
        "2026-03-10T08:02:35Z auth ERROR Certificate verification failed",
        "2026-03-10T08:02:40Z monitoring WARN Disk usage above 80%",
        "2026-03-10T08:02:45Z db WARN Slow query detected",
        "2026-03-10T08:02:50Z gateway INFO SSL handshake completed",
        "2026-03-10T08:02:55Z payments ERROR Payment declined",
    ];

    log_lines
        .iter()
        .filter_map(|line| parse_log_line(line))
        .collect()
}

#[test]
fn test_parse_log_line() {
    let line = "2026-03-10T08:01:12Z auth INFO User login successful";
    let record = parse_log_line(line).unwrap();
    assert_eq!(record.timestamp, "2026-03-10T08:01:12Z");
    assert_eq!(record.service, "auth");
    assert_eq!(record.level, "INFO");
    assert_eq!(record.message, "User login successful");
}

#[test]
fn test_count_errors_per_service() {
    let records = test_records();
    let counts = count_errors_per_service(&records);

    assert_eq!(counts.get("auth"), Some(&5), "auth should have 5 errors");
    assert_eq!(counts.get("gateway"), Some(&1));
    assert_eq!(counts.get("db"), Some(&1));
    assert_eq!(counts.get("payments"), Some(&1));
    assert_eq!(counts.get("monitoring"), None);
}

#[test]
fn test_worst_service() {
    let records = test_records();
    let counts = count_errors_per_service(&records);

    let (name, count) = worst_service(&counts).unwrap();
    assert_eq!(name, "auth");
    assert_eq!(count, 5);
}

#[test]
fn test_worst_service_empty() {
    let empty_counts: HashMap<String, usize> = HashMap::new();
    assert_eq!(worst_service(&empty_counts), None);
}

#[test]
fn test_build_service_groups() {
    let records = test_records();
    let groups = build_service_groups(&records);

    let auth_msgs = groups.get("auth").unwrap();
    assert_eq!(auth_msgs.len(), 7);
    assert!(auth_msgs[0].starts_with("[INFO]"));
    assert!(auth_msgs[2].starts_with("[ERROR]"));

    let mon_msgs = groups.get("monitoring").unwrap();
    assert_eq!(mon_msgs.len(), 3);
}

#[test]
fn test_generate_correlation_report() {
    let records = test_records();
    let counts = count_errors_per_service(&records);

    let report = generate_correlation_report(&records, &counts, 2);
    assert_eq!(report.len(), 1);
    assert!(report[0].contains("auth"));
    assert!(report[0].contains("5 errors"));

    let report_all = generate_correlation_report(&records, &counts, 0);
    assert_eq!(report_all.len(), 4);
}

fn main() {
    println!("All bugs fixed! Run tests to verify.");
}
