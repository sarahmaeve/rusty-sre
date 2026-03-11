// =============================================================================
// Challenge 01: Vectors — Debug the Log Analyzer (SOLUTION)
// =============================================================================
//
// This is the fixed version of debug.rs with all four bugs corrected.
// Run the tests with:
//     rustc solution/debug_solution.rs --edition 2021 --test && ./solution/debug_solution
// =============================================================================

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let logs = load_sample_logs();
    match analyze_logs(&logs) {
        Ok(report) => println!("{report}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}

#[derive(Debug, Clone, PartialEq)]
struct LogEntry {
    timestamp: String,
    level: String,
    message: String,
}

fn parse_log_line(line: &str) -> Option<LogEntry> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    let parts: Vec<&str> = line.splitn(3, char::is_whitespace).collect();
    if parts.len() < 3 {
        return None;
    }

    let timestamp = parts[0].to_string();
    let level = parts[1].trim().to_uppercase();
    let message = parts[2].trim().to_string();

    if !["INFO", "WARN", "ERROR", "DEBUG"].contains(&level.as_str()) {
        return None;
    }

    Some(LogEntry {
        timestamp,
        level,
        message,
    })
}

fn parse_logs(raw_lines: &[String]) -> Vec<LogEntry> {
    raw_lines.iter().filter_map(|l| parse_log_line(l)).collect()
}

// ---------------------------------------------------------------------------
// FIX 1: Off-by-one indexing
// Changed entries[entries.len()] to entries[entries.len() - 1]
// ---------------------------------------------------------------------------
fn most_recent_entry(entries: &[LogEntry]) -> Option<&LogEntry> {
    if entries.is_empty() {
        return None;
    }
    Some(&entries[entries.len() - 1])
}

// ---------------------------------------------------------------------------
// FIX 2: Mutating a vector while iterating
// Replaced manual index loop with retain(), which is the idiomatic way
// to remove elements by predicate in Rust.
// ---------------------------------------------------------------------------
fn remove_debug_entries(entries: &mut Vec<LogEntry>) {
    entries.retain(|e| e.level != "DEBUG");
}

// ---------------------------------------------------------------------------
// FIX 3: Using HashSet for membership checks to avoid duplicates
// Replaced Vec with HashSet to naturally deduplicate, then convert to
// sorted Vec for the return value.
// ---------------------------------------------------------------------------
fn unique_levels(entries: &[LogEntry]) -> Vec<String> {
    let seen: HashSet<String> = entries.iter().map(|e| e.level.clone()).collect();
    let mut levels: Vec<String> = seen.into_iter().collect();
    levels.sort();
    levels
}

// ---------------------------------------------------------------------------
// FIX 4: Handle empty vector safely
// Replaced .unwrap() with the ? operator to propagate None.
// ---------------------------------------------------------------------------
fn longest_message(entries: &[LogEntry]) -> Option<String> {
    let entry = entries.iter().max_by_key(|e| e.message.len())?;
    Some(entry.message.clone())
}

fn count_by_level(entries: &[LogEntry]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for entry in entries {
        *counts.entry(entry.level.clone()).or_insert(0) += 1;
    }
    counts
}

fn analyze_logs(entries: &[LogEntry]) -> Result<String, String> {
    if entries.is_empty() {
        return Err("No log entries to analyze".to_string());
    }

    let counts = count_by_level(entries);
    let levels = unique_levels(entries);

    let mut report = String::new();
    report.push_str("=== Log Analysis Report ===\n");
    report.push_str(&format!("Total entries: {}\n", entries.len()));
    report.push_str(&format!("Log levels found: {}\n", levels.join(", ")));

    for level in &levels {
        let count = counts.get(level).unwrap_or(&0);
        report.push_str(&format!("  {level}: {count}\n"));
    }

    if let Some(recent) = most_recent_entry(entries) {
        report.push_str(&format!("Most recent: [{}] {}\n", recent.level, recent.message));
    }

    if let Some(longest) = longest_message(entries) {
        report.push_str(&format!("Longest message: {longest}\n"));
    }

    Ok(report)
}

fn load_sample_logs() -> Vec<LogEntry> {
    let raw = vec![
        "2026-03-11T10:00:01 INFO  Server started".to_string(),
        "2026-03-11T10:00:05 ERROR Connection refused".to_string(),
        "2026-03-11T10:00:07 WARN  High memory usage".to_string(),
    ];
    parse_logs(&raw)
}

// =============================================================================
// TESTS (identical to debug.rs)
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_entries() -> Vec<LogEntry> {
        let raw = vec![
            "2026-03-11T10:00:01 INFO  Server started successfully".to_string(),
            "2026-03-11T10:00:02 DEBUG Loaded 42 config keys".to_string(),
            "2026-03-11T10:00:05 ERROR Connection refused: upstream at 10.0.1.5:8080".to_string(),
            "2026-03-11T10:00:07 WARN  High memory usage: 87% of 4GB limit".to_string(),
            "2026-03-11T10:00:09 INFO  Health check passed".to_string(),
            "2026-03-11T10:00:12 ERROR Timeout waiting for response from auth-service (30s)".to_string(),
            "2026-03-11T10:00:14 WARN  Retry attempt 1/3 for auth-service".to_string(),
            "2026-03-11T10:00:17 DEBUG Cache hit ratio: 0.73".to_string(),
            "2026-03-11T10:00:20 INFO  Scheduled job cleanup started".to_string(),
        ];
        parse_logs(&raw)
    }

    #[test]
    fn test_parse_log_line_valid() {
        let entry = parse_log_line("2026-03-11T10:00:01 INFO  Server started").unwrap();
        assert_eq!(entry.level, "INFO");
        assert_eq!(entry.message, "Server started");
    }

    #[test]
    fn test_parse_log_line_empty() {
        assert!(parse_log_line("").is_none());
        assert!(parse_log_line("   ").is_none());
    }

    #[test]
    fn test_parse_logs() {
        let entries = sample_entries();
        assert_eq!(entries.len(), 9);
    }

    #[test]
    fn test_most_recent_entry() {
        let entries = sample_entries();
        let recent = most_recent_entry(&entries).unwrap();
        assert_eq!(recent.message, "Scheduled job cleanup started");
    }

    #[test]
    fn test_most_recent_entry_single() {
        let entries = vec![LogEntry {
            timestamp: "2026-03-11T10:00:01".to_string(),
            level: "INFO".to_string(),
            message: "Only entry".to_string(),
        }];
        let recent = most_recent_entry(&entries).unwrap();
        assert_eq!(recent.message, "Only entry");
    }

    #[test]
    fn test_most_recent_entry_empty() {
        let entries: Vec<LogEntry> = vec![];
        assert!(most_recent_entry(&entries).is_none());
    }

    #[test]
    fn test_remove_debug_entries() {
        let mut entries = sample_entries();
        let original_len = entries.len();
        remove_debug_entries(&mut entries);
        assert_eq!(entries.len(), original_len - 2);
        assert!(!entries.iter().any(|e| e.level == "DEBUG"));
    }

    #[test]
    fn test_remove_debug_entries_no_debug() {
        let mut entries = vec![
            LogEntry {
                timestamp: "t1".to_string(),
                level: "INFO".to_string(),
                message: "ok".to_string(),
            },
            LogEntry {
                timestamp: "t2".to_string(),
                level: "ERROR".to_string(),
                message: "fail".to_string(),
            },
        ];
        remove_debug_entries(&mut entries);
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn test_unique_levels() {
        let entries = sample_entries();
        let levels = unique_levels(&entries);
        assert_eq!(levels, vec!["DEBUG", "ERROR", "INFO", "WARN"]);
    }

    #[test]
    fn test_unique_levels_single_type() {
        let entries = vec![
            LogEntry {
                timestamp: "t1".to_string(),
                level: "INFO".to_string(),
                message: "a".to_string(),
            },
            LogEntry {
                timestamp: "t2".to_string(),
                level: "INFO".to_string(),
                message: "b".to_string(),
            },
        ];
        let levels = unique_levels(&entries);
        assert_eq!(levels, vec!["INFO"]);
    }

    #[test]
    fn test_longest_message() {
        let entries = sample_entries();
        let longest = longest_message(&entries).unwrap();
        assert_eq!(longest, "Timeout waiting for response from auth-service (30s)");
    }

    #[test]
    fn test_longest_message_empty() {
        let entries: Vec<LogEntry> = vec![];
        assert!(longest_message(&entries).is_none());
    }

    #[test]
    fn test_count_by_level() {
        let entries = sample_entries();
        let counts = count_by_level(&entries);
        assert_eq!(counts.get("INFO"), Some(&3));
        assert_eq!(counts.get("ERROR"), Some(&2));
        assert_eq!(counts.get("WARN"), Some(&2));
        assert_eq!(counts.get("DEBUG"), Some(&2));
    }

    #[test]
    fn test_analyze_logs() {
        let entries = sample_entries();
        let report = analyze_logs(&entries).unwrap();
        assert!(report.contains("Total entries: 9"));
        assert!(report.contains("Most recent:"));
        assert!(report.contains("Longest message:"));
    }

    #[test]
    fn test_analyze_logs_empty() {
        let entries: Vec<LogEntry> = vec![];
        assert!(analyze_logs(&entries).is_err());
    }
}
