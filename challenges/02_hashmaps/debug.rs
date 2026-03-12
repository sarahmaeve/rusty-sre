// ============================================================================
// Challenge 02 — Debug: Incident Correlation Engine
// ============================================================================
//
// This program parses log records, counts errors per service, finds the worst
// offender, groups messages by service, and generates a correlation report.
//
// There are 4 bugs hidden in this code:
//   - 2 compile errors (the code won't build until you fix them)
//   - 2 runtime bugs (the code builds but produces wrong results)
//
// All bugs involve common HashMap mistakes that Python developers make when
// transitioning to Rust. Fix all 4 to make the tests pass.
//
// Run:  rustc debug.rs --edition 2024 --test && ./debug

use std::collections::HashMap;

/// A single log record from a distributed system.
#[derive(Debug, Clone)]
struct LogRecord {
    timestamp: String,
    service: String,
    level: String,   // "INFO", "WARN", "ERROR"
    message: String,
}

/// Parse a log line into a LogRecord.
/// Expected format: "timestamp service level message..."
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

/// Count the number of ERROR-level log entries per service.
///
/// BUG #1: This function compiles and runs, but produces wrong results.
/// For a service with 5 errors, it will report only 1.
///
/// Think about what insert() does when the key already exists...
/// Python parallel: `d[k] = 1` always sets to 1, vs `d[k] = d.get(k, 0) + 1`
fn count_errors_per_service(records: &[LogRecord]) -> HashMap<String, usize> {
    let mut error_counts: HashMap<String, usize> = HashMap::new();

    for record in records {
        if record.level == "ERROR" {
            // BUG: insert() overwrites the existing value every time!
            // Each error resets the count to 1 instead of incrementing.
            error_counts.insert(record.service.clone(), 1);
        }
    }

    error_counts
}

/// Find the service with the most errors.
/// Returns (service_name, error_count), or None if no errors exist.
///
/// BUG #2: This function compiles but panics at runtime.
/// It tries to look up services that might not be in the map.
///
/// Python parallel: `dict[key]` raises KeyError, `dict.get(key, 0)` is safe.
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
        "cache".to_string(),      // This service may not be in the map!
        "scheduler".to_string(),  // This one either!
    ];

    let mut worst_name = String::new();
    let mut worst_count = 0;

    for service in &known_services {
        // BUG: map[&key] panics if the key doesn't exist!
        // "cache" and "scheduler" aren't in our error_counts map.
        let count = error_counts[service];
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

/// Group log messages by service name.
///
/// BUG #3: This function does NOT compile.
/// It tries to mutate a value obtained through an immutable reference.
///
/// Python parallel: In Python, `d.get(k)` returns the actual mutable list.
/// In Rust, `get()` returns an immutable reference — you can't push to it.
fn build_service_groups(records: &[LogRecord]) -> HashMap<String, Vec<String>> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    for record in records {
        let formatted = format!("[{}] {}", record.level, record.message);

        // BUG: get() returns Option<&Vec<String>> — an immutable reference!
        // You cannot call .push() on an immutable reference.
        // In Python, dict.get() gives you the actual mutable list object.
        let messages = groups.get(&record.service);
        match messages {
            Some(vec) => vec.push(formatted),
            None => {
                groups.insert(record.service.clone(), vec![formatted]);
            }
        }
    }

    groups
}

/// Generate a summary report of services with error counts above a threshold.
///
/// BUG #4: This function does NOT compile.
/// It compares Option<&usize> directly with a plain usize.
///
/// Python parallel: `dict.get(key)` returns the value or None. You can write
/// `if d.get(k) and d.get(k) > 5` because Python coerces types. Rust won't.
fn generate_correlation_report(
    records: &[LogRecord],
    error_counts: &HashMap<String, usize>,
    threshold: usize,
) -> Vec<String> {
    let mut report: Vec<String> = Vec::new();

    // Collect unique service names from records.
    let mut services: Vec<String> = records
        .iter()
        .map(|r| r.service.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    services.sort();

    for service in &services {
        // BUG: get() returns Option<&usize>, not usize.
        // You can't compare Option<&usize> > threshold directly.
        let count = error_counts.get(service);
        if count > threshold {
            let total_records = records.iter().filter(|r| r.service == *service).count();
            report.push(format!(
                "{}: {} errors out of {} total records",
                service, count.unwrap(), total_records
            ));
        }
    }

    report
}

// ============================================================================
// TESTS
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

    // auth has 5 errors — if you see 1, bug #1 hasn't been fixed!
    assert_eq!(counts.get("auth"), Some(&5), "auth should have 5 errors");
    assert_eq!(counts.get("gateway"), Some(&1));
    assert_eq!(counts.get("db"), Some(&1));
    assert_eq!(counts.get("payments"), Some(&1));
    assert_eq!(counts.get("monitoring"), None); // monitoring has 0 errors
}

#[test]
fn test_worst_service() {
    let records = test_records();
    let counts = count_errors_per_service(&records);

    // Should not panic even though "cache" and "scheduler" aren't in the map.
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

    // auth has 7 total log entries (5 ERROR + 2 INFO).
    let auth_msgs = groups.get("auth").unwrap();
    assert_eq!(auth_msgs.len(), 7);

    // Verify message formatting.
    assert!(auth_msgs[0].starts_with("[INFO]"));
    assert!(auth_msgs[2].starts_with("[ERROR]"));

    // monitoring has 3 entries (2 INFO + 1 WARN).
    let mon_msgs = groups.get("monitoring").unwrap();
    assert_eq!(mon_msgs.len(), 3);
}

#[test]
fn test_generate_correlation_report() {
    let records = test_records();
    let counts = count_errors_per_service(&records);

    // Threshold of 2: only auth (5 errors) should appear.
    let report = generate_correlation_report(&records, &counts, 2);
    assert_eq!(report.len(), 1);
    assert!(report[0].contains("auth"));
    assert!(report[0].contains("5 errors"));

    // Threshold of 0: all services with errors should appear.
    let report_all = generate_correlation_report(&records, &counts, 0);
    assert_eq!(report_all.len(), 4); // auth, db, gateway, payments
}

fn main() {
    println!("Run with: rustc debug.rs --edition 2024 --test && ./debug");
    println!("Fix the 4 bugs to make all tests pass!");
}
