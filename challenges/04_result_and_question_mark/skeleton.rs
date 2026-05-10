// =============================================================================
// Challenge 04: Result and `?` — Config Loader Pipeline
// =============================================================================
//
// You are building a startup-time config loader for a service. It reads
// configuration values from string inputs (think env vars or a tiny config
// file), validates them, and assembles them into a Config struct.
//
// Each parse step can fail with a different error mode. Your job is to:
//   - Implement a custom error enum
//   - Use `?` to propagate errors up the call stack
//   - Provide a From conversion so `?` works across error types
//
// Run the tests with:
//     rustc skeleton.rs --edition 2024 --test && ./skeleton
// =============================================================================

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

fn main() {
    println!("Complete the TODO items, then run with --test to verify.");
}

// -----------------------------------------------------------------------------
// Task 1: Define the error enum
// -----------------------------------------------------------------------------
// Define ConfigError with the following variants:
//
//   - InvalidPort(String)         — port unparseable, out of u16 range, or zero
//   - InvalidThreshold(f64)       — number parsed but outside [0.0, 1.0]
//   - InvalidUrl(String)          — URL doesn't match expected shape
//   - MissingField(&'static str)  — required field not provided
//
// Then implement Display so error messages look like:
//
//   "invalid port: <input>"
//   "invalid threshold: <value> (must be between 0.0 and 1.0)"
//   "invalid url: <input>"
//   "missing field: <name>"
//
// And implement std::error::Error for ConfigError. The default impl is fine
// (you don't need to override .source()).

#[derive(Debug)]
enum ConfigError {
    // TODO: add the four variants described above
}

impl fmt::Display for ConfigError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: implement the four match arms
        todo!()
    }
}

// TODO: impl std::error::Error for ConfigError

// -----------------------------------------------------------------------------
// Task 2: From<ParseIntError> for ConfigError
// -----------------------------------------------------------------------------
// Implement `From<ParseIntError> for ConfigError` so `?` can promote a parse
// error into a ConfigError variant. Use ConfigError::InvalidPort with the
// parse error's text as the input.

// TODO: impl From<ParseIntError> for ConfigError

// -----------------------------------------------------------------------------
// Task 3: parse_port
// -----------------------------------------------------------------------------
// Parse the input as u16. Return ConfigError::InvalidPort(input.to_string())
// if parsing fails or if the parsed number is zero (we don't allow port 0).
//
// HINT: Use map_err to convert the ParseIntError into the right variant with
// the original input embedded; then validate the parsed number.
fn parse_port(_s: &str) -> Result<u16, ConfigError> {
    // TODO: parse and validate
    todo!()
}

// -----------------------------------------------------------------------------
// Task 4: parse_threshold
// -----------------------------------------------------------------------------
// Parse the input as f64. If parsing fails, or the value is outside [0.0, 1.0],
// return ConfigError::InvalidThreshold(parsed_or_NaN).
fn parse_threshold(_s: &str) -> Result<f64, ConfigError> {
    // TODO: parse and validate
    todo!()
}

// -----------------------------------------------------------------------------
// Task 5: parse_url
// -----------------------------------------------------------------------------
// Validate the input has the shape "<scheme>://<host>" where <scheme> is
// "http" or "https" and <host> is non-empty. If valid, return the input as
// a String. If not, return ConfigError::InvalidUrl(input.to_string()).
//
// HINT: str::split_once("://") returns Option<(&str, &str)>.
fn parse_url(_s: &str) -> Result<String, ConfigError> {
    // TODO: validate scheme and host
    todo!()
}

// -----------------------------------------------------------------------------
// Task 6: load_config
// -----------------------------------------------------------------------------
// Combine the three parsers above. Each input is an Option<&str> from a
// config source — None means the field was not provided.
//
//   - If any input is None, return ConfigError::MissingField(<name>).
//     Field names: "port", "threshold", "upstream".
//   - If any input is Some but invalid, return the corresponding ConfigError.
//   - On success, return the assembled Config.
//
// HINT: Option::ok_or turns Option into Result so `?` can propagate it.

#[derive(Debug, PartialEq)]
struct Config {
    port: u16,
    threshold: f64,
    upstream: String,
}

fn load_config(
    _port: Option<&str>,
    _threshold: Option<&str>,
    _upstream: Option<&str>,
) -> Result<Config, ConfigError> {
    // TODO: implement using `?` and the parsers above
    todo!()
}

// =============================================================================
// TESTS — Do not modify below this line
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ----- Task 1 + 2: ConfigError -----

    #[test]
    fn config_error_display() {
        assert_eq!(
            ConfigError::InvalidPort("99999".to_string()).to_string(),
            "invalid port: 99999",
        );
        assert_eq!(
            ConfigError::InvalidThreshold(1.5).to_string(),
            "invalid threshold: 1.5 (must be between 0.0 and 1.0)",
        );
        assert_eq!(
            ConfigError::InvalidUrl("foo".to_string()).to_string(),
            "invalid url: foo",
        );
        assert_eq!(
            ConfigError::MissingField("port").to_string(),
            "missing field: port",
        );
    }

    #[test]
    fn config_error_implements_error_trait() {
        fn assert_error<E: Error>(_: &E) {}
        let e = ConfigError::InvalidUrl("x".to_string());
        assert_error(&e);
    }

    #[test]
    fn from_parse_int_error_works_with_question_mark() {
        // Smoke test: writing `let n: u16 = "abc".parse()?` in a function that
        // returns Result<_, ConfigError> only compiles when From<ParseIntError>
        // is implemented for ConfigError.
        fn try_parse(s: &str) -> Result<u16, ConfigError> {
            let n: u16 = s.parse()?;
            Ok(n)
        }
        assert!(try_parse("abc").is_err());
        assert_eq!(try_parse("80").unwrap(), 80);
    }

    // ----- Task 3: parse_port -----

    #[test]
    fn parse_port_valid() {
        assert_eq!(parse_port("8080"), Ok(8080));
        assert_eq!(parse_port("1"), Ok(1));
        assert_eq!(parse_port("65535"), Ok(65535));
    }

    #[test]
    fn parse_port_zero_rejected() {
        assert!(matches!(parse_port("0"), Err(ConfigError::InvalidPort(_))));
    }

    #[test]
    fn parse_port_out_of_range() {
        // 99999 doesn't fit in u16
        assert!(matches!(parse_port("99999"), Err(ConfigError::InvalidPort(_))));
    }

    #[test]
    fn parse_port_garbage() {
        assert!(matches!(parse_port("eighty"), Err(ConfigError::InvalidPort(_))));
    }

    // ----- Task 4: parse_threshold -----

    #[test]
    fn parse_threshold_valid() {
        assert_eq!(parse_threshold("0.0"), Ok(0.0));
        assert_eq!(parse_threshold("0.5"), Ok(0.5));
        assert_eq!(parse_threshold("1.0"), Ok(1.0));
    }

    #[test]
    fn parse_threshold_out_of_range() {
        assert!(matches!(
            parse_threshold("1.5"),
            Err(ConfigError::InvalidThreshold(_))
        ));
        assert!(matches!(
            parse_threshold("-0.1"),
            Err(ConfigError::InvalidThreshold(_))
        ));
    }

    #[test]
    fn parse_threshold_garbage() {
        assert!(matches!(
            parse_threshold("hi"),
            Err(ConfigError::InvalidThreshold(_))
        ));
    }

    // ----- Task 5: parse_url -----

    #[test]
    fn parse_url_valid() {
        assert_eq!(parse_url("http://localhost"), Ok("http://localhost".to_string()));
        assert_eq!(
            parse_url("https://api.example.com"),
            Ok("https://api.example.com".to_string())
        );
    }

    #[test]
    fn parse_url_missing_scheme() {
        assert!(matches!(parse_url("localhost"), Err(ConfigError::InvalidUrl(_))));
    }

    #[test]
    fn parse_url_bad_scheme() {
        assert!(matches!(parse_url("ftp://localhost"), Err(ConfigError::InvalidUrl(_))));
    }

    #[test]
    fn parse_url_empty_host() {
        assert!(matches!(parse_url("http://"), Err(ConfigError::InvalidUrl(_))));
    }

    // ----- Task 6: load_config -----

    #[test]
    fn load_config_happy_path() {
        let cfg = load_config(Some("8080"), Some("0.9"), Some("https://api.example.com"))
            .unwrap();
        assert_eq!(
            cfg,
            Config {
                port: 8080,
                threshold: 0.9,
                upstream: "https://api.example.com".to_string(),
            }
        );
    }

    #[test]
    fn load_config_missing_port() {
        let err = load_config(None, Some("0.5"), Some("http://x")).unwrap_err();
        assert!(matches!(err, ConfigError::MissingField("port")));
    }

    #[test]
    fn load_config_missing_threshold() {
        let err = load_config(Some("80"), None, Some("http://x")).unwrap_err();
        assert!(matches!(err, ConfigError::MissingField("threshold")));
    }

    #[test]
    fn load_config_missing_upstream() {
        let err = load_config(Some("80"), Some("0.5"), None).unwrap_err();
        assert!(matches!(err, ConfigError::MissingField("upstream")));
    }

    #[test]
    fn load_config_short_circuits_on_first_error() {
        // Port is invalid; we should never validate the threshold or URL.
        let err = load_config(Some("0"), Some("0.5"), Some("not-a-url")).unwrap_err();
        assert!(matches!(err, ConfigError::InvalidPort(_)));
    }
}
