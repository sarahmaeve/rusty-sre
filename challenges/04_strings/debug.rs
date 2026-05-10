// =============================================================================
// Challenge 04: Strings — Debug the Log Line Redactor
// =============================================================================
//
// This program reads log lines that may contain user emails, redacts the
// emails, and groups by service tag. It contains FOUR string-related bugs:
//
//   1. add_redacted_marker tries to mutate a &str (compile error)
//   2. Combining two Strings with `+` (compile error: + needs &str on right)
//   3. Byte-indexing a string that contains multi-byte characters (runtime
//      panic on inputs with non-ASCII)
//   4. Case-insensitive comparison done with raw `==` (runtime: misses
//      services tagged with different casing)
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

// -----------------------------------------------------------------------------
// BUG 1: cannot mutate a &str
// -----------------------------------------------------------------------------
// Append " [REDACTED]" to the input and return the result as an owned String.
// The current signature takes &str (immutable) and tries to mutate it, which
// won't compile. Fix: return a new String.
fn add_redacted_marker(s: &str) {
    s.push_str(" [REDACTED]");
}

// -----------------------------------------------------------------------------
// BUG 2: cannot add two Strings with `+`
// -----------------------------------------------------------------------------
// `String + String` doesn't compile because `+` for String wants &str on
// the right. Either convert the right side with `&` or use format!.
fn build_redacted_line(prefix: String, suffix: String) -> String {
    prefix + suffix
}

// -----------------------------------------------------------------------------
// BUG 3: byte-indexing a UTF-8 string panics on multi-byte characters
// -----------------------------------------------------------------------------
// Returns the first three "characters" of the input. The current code uses
// byte slicing, which panics if the boundary cuts a multi-byte char in half.
// Use chars().take(3).collect() instead.
fn first_three_chars(s: &str) -> String {
    s[0..3].to_string()
}

// -----------------------------------------------------------------------------
// BUG 4: case-insensitive comparison with raw `==`
// -----------------------------------------------------------------------------
// Group log lines by their `service=NAME` tag. Tags should be matched
// case-insensitively so "auth" and "AUTH" land in the same bucket.
fn extract_service(line: &str) -> Option<&str> {
    for field in line.split_whitespace() {
        if let Some((key, value)) = field.split_once('=') {
            // BUG 4: this comparison is case-sensitive. Should ignore case
            //        so callers don't have to upper/lowercase first.
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

    // BUG 1: add_redacted_marker must return an owned String
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

    // BUG 2: build_redacted_line should compile and concatenate
    #[test]
    fn build_redacted_line_combines() {
        let prefix = String::from("service=auth ");
        let suffix = String::from("user=<redacted>");
        let combined = build_redacted_line(prefix, suffix);
        assert_eq!(combined, "service=auth user=<redacted>");
    }

    // BUG 3: first_three_chars must not panic on multi-byte input
    #[test]
    fn first_three_chars_ascii() {
        assert_eq!(first_three_chars("hello"), "hel");
    }

    #[test]
    fn first_three_chars_unicode() {
        // "café" — the `é` is two bytes. Byte-slicing 0..3 lands inside `é`
        // and would panic. After the fix this returns the first 3 chars.
        assert_eq!(first_three_chars("café"), "caf");
    }

    #[test]
    fn first_three_chars_emoji() {
        // Ensure we get three chars regardless of byte width.
        let s = "🚀ab";
        assert_eq!(first_three_chars(s), "🚀ab");
    }

    // BUG 4: extract_service should match case-insensitively
    #[test]
    fn extract_service_lowercase() {
        assert_eq!(extract_service("service=auth user=x"), Some("auth"));
    }

    #[test]
    fn extract_service_uppercase_key() {
        // The current code only matches "SERVICE" exactly. Real logs have
        // mixed casing — "service=...", "Service=...", "SERVICE=...". After
        // the fix, all three should match.
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
