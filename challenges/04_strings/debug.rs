// =============================================================================
// Challenge 04: Strings — Debug the Log Line Redactor
// =============================================================================
//
// This program reads log lines that may contain user emails, redacts the
// emails, and groups by service tag. It contains FOUR string-related bugs —
// some stop it compiling, some misbehave at runtime. Find and fix all four
// so every test passes.
//
// Stuck? HINTS.md reveals each bug in stages: symptom, location, then fix.
//
// Run the tests with:
//     rustc debug.rs --edition 2024 --test && ./debug
// =============================================================================

use std::collections::HashMap;

fn main() {
    let lines = vec![
        "service=auth user=alice@example.com action=login".to_string(),
        "service=AUTH user=bob@example.com action=logout".to_string(),
        "service=payments user=carol@example.com action=charge amount=42".to_string(),
    ];
    let summary = summarize(&lines);
    println!("{summary:#?}");
}

// Append " [REDACTED]" to the input and return the result as an owned String.
fn add_redacted_marker(s: &str) {
    s.push_str(" [REDACTED]");
}

// Concatenate a prefix and suffix into a single line.
fn build_redacted_line(prefix: String, suffix: String) -> String {
    prefix + suffix
}

// Returns the first three characters of the input.
fn first_three_chars(s: &str) -> String {
    s[0..3].to_string()
}

// Extract the value of the line's `service=NAME` field. The key should match
// case-insensitively — real logs carry "service=", "Service=", "SERVICE=".
fn extract_service(line: &str) -> Option<&str> {
    for field in line.split_whitespace() {
        if let Some((key, value)) = field.split_once('=') {
            if key == "SERVICE" {
                return Some(value);
            }
        }
    }
    None
}

// Redact an email-shaped value to "<redacted>".
fn redact_email(field: &str) -> String {
    if let Some((key, value)) = field.split_once('=')
        && value.contains('@')
    {
        format!("{key}=<redacted>")
    } else {
        field.to_string()
    }
}

// Rebuild a line with email values redacted.
fn redact_line(line: &str) -> String {
    let parts: Vec<String> = line.split_whitespace().map(redact_email).collect();
    parts.join(" ")
}

// Build a per-service summary: count of redacted lines per service name,
// where service names are matched case-insensitively.
fn summarize(lines: &[String]) -> HashMap<String, Vec<String>> {
    let mut out: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let Some(service) = extract_service(line) else {
            continue;
        };
        let normalized = service.to_lowercase();
        let redacted = redact_line(line);
        out.entry(normalized).or_default().push(redacted);
    }
    out
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_redacted_marker_returns_new_string() {
        let result: String = add_redacted_marker("service=auth");
        assert_eq!(result, "service=auth [REDACTED]");
    }

    #[test]
    fn add_redacted_marker_preserves_input_kind() {
        // Should also work for owned input via &String → &str coercion
        let owned = String::from("service=payments");
        let result: String = add_redacted_marker(&owned);
        assert_eq!(result, "service=payments [REDACTED]");
        // owned is still usable — we only borrowed
        assert_eq!(owned, "service=payments");
    }

    #[test]
    fn build_redacted_line_combines() {
        let prefix = String::from("service=auth ");
        let suffix = String::from("user=<redacted>");
        let combined = build_redacted_line(prefix, suffix);
        assert_eq!(combined, "service=auth user=<redacted>");
    }

    #[test]
    fn first_three_chars_ascii() {
        assert_eq!(first_three_chars("hello"), "hel");
    }

    #[test]
    fn first_three_chars_unicode() {
        // "café" — the `é` is two bytes wide, but it's still one char.
        assert_eq!(first_three_chars("café"), "caf");
    }

    #[test]
    fn first_three_chars_emoji() {
        // Ensure we get three chars regardless of byte width.
        let s = "🚀ab";
        assert_eq!(first_three_chars(s), "🚀ab");
    }

    #[test]
    fn extract_service_lowercase() {
        assert_eq!(extract_service("service=auth user=x"), Some("auth"));
    }

    #[test]
    fn extract_service_uppercase_key() {
        // Real logs have mixed casing — "service=...", "Service=...",
        // "SERVICE=...". All three should match.
        assert_eq!(extract_service("service=auth"), Some("auth"));
        assert_eq!(extract_service("Service=auth"), Some("auth"));
        assert_eq!(extract_service("SERVICE=auth"), Some("auth"));
    }

    #[test]
    fn extract_service_missing() {
        assert_eq!(extract_service("user=x action=login"), None);
    }

    // Integration test — exercises the full pipeline including redaction
    #[test]
    fn summarize_groups_by_service_case_insensitively() {
        let lines = vec![
            "service=auth user=alice@example.com action=login".to_string(),
            "service=AUTH user=bob@example.com action=logout".to_string(),
            "service=payments user=carol@example.com action=charge".to_string(),
        ];
        let summary = summarize(&lines);

        // All "auth" / "AUTH" lines bucket under "auth"
        assert_eq!(summary["auth"].len(), 2);
        assert_eq!(summary["payments"].len(), 1);

        // Emails are redacted in stored lines
        for line in summary.values().flatten() {
            assert!(line.contains("<redacted>"));
            assert!(!line.contains("@example.com"));
        }
    }
}
