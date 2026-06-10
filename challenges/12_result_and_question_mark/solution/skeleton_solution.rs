// =============================================================================
// Challenge 12: Result and `?` — Config Loader Pipeline (SKELETON SOLUTION)
// =============================================================================
//
// Reference implementation of skeleton.rs with every TODO completed.
// Run the tests from inside the solution/ directory:
//     rustc skeleton_solution.rs --edition 2024 --test && ./skeleton_solution
// =============================================================================

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

fn main() {
    println!("Reference solution — run with --test to execute the tests.");
}

// -----------------------------------------------------------------------------
// Task 1: Define the error enum
// -----------------------------------------------------------------------------
// PartialEq is needed because the tests compare Results with assert_eq!.
#[derive(Debug, PartialEq)]
enum ConfigError {
    InvalidPort(String),
    InvalidThreshold(f64),
    InvalidUrl(String),
    MissingField(&'static str),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::InvalidPort(input) => write!(f, "invalid port: {input}"),
            ConfigError::InvalidThreshold(value) => {
                write!(f, "invalid threshold: {value} (must be between 0.0 and 1.0)")
            }
            ConfigError::InvalidUrl(input) => write!(f, "invalid url: {input}"),
            ConfigError::MissingField(name) => write!(f, "missing field: {name}"),
        }
    }
}

impl Error for ConfigError {}

// -----------------------------------------------------------------------------
// Task 2: From<ParseIntError> for ConfigError
// -----------------------------------------------------------------------------
impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::InvalidPort(e.to_string())
    }
}

// -----------------------------------------------------------------------------
// Task 3: parse_port
// -----------------------------------------------------------------------------
fn parse_port(s: &str) -> Result<u16, ConfigError> {
    let port: u16 = s
        .parse()
        .map_err(|_| ConfigError::InvalidPort(s.to_string()))?;
    if port == 0 {
        return Err(ConfigError::InvalidPort(s.to_string()));
    }
    Ok(port)
}

// -----------------------------------------------------------------------------
// Task 4: parse_threshold
// -----------------------------------------------------------------------------
fn parse_threshold(s: &str) -> Result<f64, ConfigError> {
    let value: f64 = s
        .parse()
        .map_err(|_| ConfigError::InvalidThreshold(f64::NAN))?;
    if (0.0..=1.0).contains(&value) {
        Ok(value)
    } else {
        Err(ConfigError::InvalidThreshold(value))
    }
}

// -----------------------------------------------------------------------------
// Task 5: parse_url
// -----------------------------------------------------------------------------
fn parse_url(s: &str) -> Result<String, ConfigError> {
    match s.split_once("://") {
        Some((scheme, host))
            if (scheme == "http" || scheme == "https") && !host.is_empty() =>
        {
            Ok(s.to_string())
        }
        _ => Err(ConfigError::InvalidUrl(s.to_string())),
    }
}

// -----------------------------------------------------------------------------
// Task 6: load_config
// -----------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
struct Config {
    port: u16,
    threshold: f64,
    upstream: String,
}

fn load_config(
    port: Option<&str>,
    threshold: Option<&str>,
    upstream: Option<&str>,
) -> Result<Config, ConfigError> {
    let port = parse_port(port.ok_or(ConfigError::MissingField("port"))?)?;
    let threshold = parse_threshold(threshold.ok_or(ConfigError::MissingField("threshold"))?)?;
    let upstream = parse_url(upstream.ok_or(ConfigError::MissingField("upstream"))?)?;
    Ok(Config { port, threshold, upstream })
}

// =============================================================================
// TESTS — identical to skeleton.rs
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
