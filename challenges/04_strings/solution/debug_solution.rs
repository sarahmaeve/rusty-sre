// =============================================================================
// Challenge 04: Strings — Debug Solution
// =============================================================================
//
// Bugs fixed:
//
//   1. add_redacted_marker: signature changed from `(s: &str)` (immutable,
//      can't be mutated) to returning a fresh String built with format!.
//   2. build_redacted_line: `prefix + suffix` becomes `prefix + &suffix`
//      (or equivalently `format!("{prefix}{suffix}")`).
//   3. first_three_chars: byte slice replaced with `chars().take(3).collect()`
//      so it doesn't panic when the byte boundary cuts a multi-byte char.
//   4. extract_service: case-sensitive `==` replaced with
//      eq_ignore_ascii_case so "service", "Service", "SERVICE" all match.
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

// FIX 1: return an owned String (don't try to mutate the borrowed slice).
fn add_redacted_marker(s: &str) -> String {
    format!("{s} [REDACTED]")
}

// FIX 2: `+` for String wants &str on the right.
fn build_redacted_line(prefix: String, suffix: String) -> String {
    prefix + &suffix
}

// FIX 3: take the first three CHARS, not the first three BYTES.
fn first_three_chars(s: &str) -> String {
    s.chars().take(3).collect()
}

// FIX 4: case-insensitive comparison.
fn extract_service(line: &str) -> Option<&str> {
    for field in line.split_whitespace() {
        if let Some((key, value)) = field.split_once('=')
            && key.eq_ignore_ascii_case("service")
        {
            return Some(value);
        }
    }
    None
}

fn redact_email(field: &str) -> String {
    if let Some((key, value)) = field.split_once('=')
        && value.contains('@')
    {
        format!("{key}=<redacted>")
    } else {
        field.to_string()
    }
}

fn redact_line(line: &str) -> String {
    let parts: Vec<String> = line.split_whitespace().map(redact_email).collect();
    parts.join(" ")
}

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
        let owned = String::from("service=payments");
        let result: String = add_redacted_marker(&owned);
        assert_eq!(result, "service=payments [REDACTED]");
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
        assert_eq!(first_three_chars("café"), "caf");
    }

    #[test]
    fn first_three_chars_emoji() {
        let s = "🚀ab";
        assert_eq!(first_three_chars(s), "🚀ab");
    }

    #[test]
    fn extract_service_lowercase() {
        assert_eq!(extract_service("service=auth user=x"), Some("auth"));
    }

    #[test]
    fn extract_service_uppercase_key() {
        assert_eq!(extract_service("service=auth"), Some("auth"));
        assert_eq!(extract_service("Service=auth"), Some("auth"));
        assert_eq!(extract_service("SERVICE=auth"), Some("auth"));
    }

    #[test]
    fn extract_service_missing() {
        assert_eq!(extract_service("user=x action=login"), None);
    }

    #[test]
    fn summarize_groups_by_service_case_insensitively() {
        let lines = vec![
            "service=auth user=alice@example.com action=login".to_string(),
            "service=AUTH user=bob@example.com action=logout".to_string(),
            "service=payments user=carol@example.com action=charge".to_string(),
        ];
        let summary = summarize(&lines);

        assert_eq!(summary["auth"].len(), 2);
        assert_eq!(summary["payments"].len(), 1);

        for line in summary.values().flatten() {
            assert!(line.contains("<redacted>"));
            assert!(!line.contains("@example.com"));
        }
    }
}
