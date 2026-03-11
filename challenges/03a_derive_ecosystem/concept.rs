// ============================================================================
// Challenge 03a: Derive Ecosystem — Concept Explainer
// ============================================================================
//
// In production Rust, three crate families dominate the derive ecosystem:
//   - serde  (#[derive(Serialize, Deserialize)]) — structured data conversion
//   - thiserror (#[derive(Error)])               — custom error types
//   - clap   (#[derive(Parser)])                 — CLI argument parsing
//
// These are EXTERNAL crates (not in std), so this challenge teaches the
// MANUAL implementations of the traits they automate. When you later use
// the crates in a cargo project, you'll understand what they generate.
//
// Run with:
//     rustc concept.rs --edition 2021 --test && ./concept

use std::collections::HashMap;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

// ============================================================================
// SECTION 1: SERIALIZATION — What serde::Serialize does
// ============================================================================
//
// Serialization = converting a struct into a portable format (JSON, YAML,
// logfmt, etc.) so it can be stored or sent over a network.
//
// Python equivalent: json.dumps(obj.__dict__), or implementing __json__
//
// serde's #[derive(Serialize)] generates code that visits every field and
// writes it to a format-specific serializer. We'll do this manually using
// the Display trait with a logfmt-style format:
//   key=value key=value key=value
//
// Logfmt is widely used in SRE (Heroku, Splunk, Grafana Loki).

#[derive(Debug, Clone, PartialEq, Eq)]
enum Status {
    Healthy,
    Degraded,
    Down,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Healthy => write!(f, "healthy"),
            Status::Degraded => write!(f, "degraded"),
            Status::Down => write!(f, "down"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HealthCheck {
    service: String,
    status: Status,
    latency_ms: u64,
    healthy_nodes: u32,
    total_nodes: u32,
}

// Manual "Serialize" — convert struct to logfmt string.
// This is what serde::Serialize + a logfmt serializer does automatically.
impl fmt::Display for HealthCheck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "service={} status={} latency_ms={} healthy_nodes={} total_nodes={}",
            self.service, self.status, self.latency_ms,
            self.healthy_nodes, self.total_nodes
        )
    }
}

#[test]
fn test_section1_serialize_to_logfmt() {
    let check = HealthCheck {
        service: "api-gateway".into(),
        status: Status::Healthy,
        latency_ms: 42,
        healthy_nodes: 3,
        total_nodes: 3,
    };
    assert_eq!(
        check.to_string(),
        "service=api-gateway status=healthy latency_ms=42 healthy_nodes=3 total_nodes=3"
    );
}

#[test]
fn test_section1_serialize_enum_variant() {
    // Each enum variant serializes to its lowercase name
    assert_eq!(Status::Healthy.to_string(), "healthy");
    assert_eq!(Status::Degraded.to_string(), "degraded");
    assert_eq!(Status::Down.to_string(), "down");
}

// You can also serialize to other formats. Here's a simple key=value
// line-based format (one field per line), useful for config files:

trait ToKeyValue {
    fn to_key_value(&self) -> String;
}

impl ToKeyValue for HealthCheck {
    fn to_key_value(&self) -> String {
        format!(
            "service={}\nstatus={}\nlatency_ms={}\nhealthy_nodes={}\ntotal_nodes={}",
            self.service, self.status, self.latency_ms,
            self.healthy_nodes, self.total_nodes
        )
    }
}

#[test]
fn test_section1_serialize_multiline() {
    let check = HealthCheck {
        service: "db".into(),
        status: Status::Down,
        latency_ms: 5000,
        healthy_nodes: 0,
        total_nodes: 3,
    };
    let output = check.to_key_value();
    assert!(output.contains("service=db\n"));
    assert!(output.contains("status=down\n"));
    assert!(output.contains("total_nodes=3"));
}

// ============================================================================
// SECTION 2: DESERIALIZATION — What serde::Deserialize does
// ============================================================================
//
// Deserialization = parsing a string/bytes back into a typed struct.
// This is the inverse of serialization.
//
// Python equivalent: json.loads(s), or MyClass(**parsed_dict)
//
// serde's #[derive(Deserialize)] generates code that reads fields from a
// format-specific deserializer and constructs the struct. We do this
// manually with the FromStr trait.
//
// FromStr is the standard trait for "parse a string into a type":
//   let x: MyType = "some string".parse()?;
//
// It requires defining an associated Err type for parse failures.

impl FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "healthy" => Ok(Status::Healthy),
            "degraded" => Ok(Status::Degraded),
            "down" => Ok(Status::Down),
            other => Err(format!("unknown status: '{}'", other)),
        }
    }
}

// Parse a logfmt-style string back into a HealthCheck.
// This is what serde::Deserialize + a logfmt deserializer does automatically.
impl FromStr for HealthCheck {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Step 1: Parse all key=value pairs into a HashMap
        let mut fields: HashMap<&str, &str> = HashMap::new();
        for token in s.split_whitespace() {
            let (key, value) = token
                .split_once('=')
                .ok_or_else(|| format!("invalid token (no '='): '{}'", token))?;
            fields.insert(key, value);
        }

        // Step 2: Extract and parse each field
        let service = fields
            .get("service")
            .ok_or("missing field: service")?
            .to_string();

        let status: Status = fields
            .get("status")
            .ok_or("missing field: status")?
            .parse()
            .map_err(|e: String| e)?;

        let latency_ms: u64 = fields
            .get("latency_ms")
            .ok_or("missing field: latency_ms")?
            .parse()
            .map_err(|e: ParseIntError| format!("invalid latency_ms: {}", e))?;

        let healthy_nodes: u32 = fields
            .get("healthy_nodes")
            .ok_or("missing field: healthy_nodes")?
            .parse()
            .map_err(|e: ParseIntError| format!("invalid healthy_nodes: {}", e))?;

        let total_nodes: u32 = fields
            .get("total_nodes")
            .ok_or("missing field: total_nodes")?
            .parse()
            .map_err(|e: ParseIntError| format!("invalid total_nodes: {}", e))?;

        Ok(HealthCheck {
            service,
            status,
            latency_ms,
            healthy_nodes,
            total_nodes,
        })
    }
}

#[test]
fn test_section2_deserialize_status() {
    assert_eq!("healthy".parse::<Status>(), Ok(Status::Healthy));
    assert_eq!("down".parse::<Status>(), Ok(Status::Down));
    assert!("unknown".parse::<Status>().is_err());
}

#[test]
fn test_section2_deserialize_healthcheck() {
    let input = "service=api status=healthy latency_ms=42 healthy_nodes=3 total_nodes=3";
    let check: HealthCheck = input.parse().unwrap();
    assert_eq!(check.service, "api");
    assert_eq!(check.status, Status::Healthy);
    assert_eq!(check.latency_ms, 42);
}

#[test]
fn test_section2_deserialize_error_missing_field() {
    let input = "service=api status=healthy";
    let result: Result<HealthCheck, _> = input.parse();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("missing field"));
}

#[test]
fn test_section2_deserialize_error_invalid_value() {
    let input = "service=api status=healthy latency_ms=abc healthy_nodes=3 total_nodes=3";
    let result: Result<HealthCheck, _> = input.parse();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("invalid latency_ms"));
}

// ============================================================================
// SECTION 3: ROUND-TRIP — Serialize then Deserialize
// ============================================================================
//
// A critical property: serializing a value and deserializing the result
// should give back the original value. If it doesn't, you have a bug.
//
// serde enforces this naturally. When implementing manually, you must
// ensure Display and FromStr are inverses of each other.

#[test]
fn test_section3_roundtrip() {
    let original = HealthCheck {
        service: "cache".into(),
        status: Status::Degraded,
        latency_ms: 150,
        healthy_nodes: 2,
        total_nodes: 3,
    };
    let serialized = original.to_string();
    let deserialized: HealthCheck = serialized.parse().unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_section3_roundtrip_all_statuses() {
    for status in [Status::Healthy, Status::Degraded, Status::Down] {
        let s = status.to_string();
        let parsed: Status = s.parse().unwrap();
        assert_eq!(status, parsed);
    }
}

// ============================================================================
// SECTION 4: THE ERROR TRAIT — What thiserror generates
// ============================================================================
//
// Rust's std::error::Error trait is the foundation of error handling:
//
//   pub trait Error: Display + Debug {
//       fn source(&self) -> Option<&(dyn Error + 'static)> { None }
//   }
//
// To implement it you need:
//   1. impl Display  — user-facing error message
//   2. impl Debug    — developer-facing details (usually #[derive(Debug)])
//   3. impl Error    — optionally override source() for error chaining
//
// Python equivalent: defining a custom exception class with __str__
//
// thiserror's #[derive(Error)] generates Display + Error + From impls
// from attributes. We'll write these manually.

#[derive(Debug)]
enum ConfigError {
    /// A required field was missing from the config file.
    MissingField(String),
    /// A field value couldn't be parsed as the expected type.
    InvalidValue {
        field: String,
        value: String,
        reason: String,
    },
    /// An I/O error occurred while reading the config file.
    // Box<dyn ...> lets us store any Error type without generics.
    Io(std::io::Error),
}

// Display: the user-facing error messages.
// thiserror generates this from #[error("...")] attributes.
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingField(field) => {
                write!(f, "missing required field: '{}'", field)
            }
            ConfigError::InvalidValue { field, value, reason } => {
                write!(f, "invalid value '{}' for field '{}': {}", value, field, reason)
            }
            ConfigError::Io(err) => {
                write!(f, "config I/O error: {}", err)
            }
        }
    }
}

// Error: override source() to enable error chaining.
// The Io variant wraps an inner error — source() exposes it.
impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::Io(err) => Some(err),
            _ => None,
        }
    }
}

#[test]
fn test_section4_error_display() {
    let err = ConfigError::MissingField("timeout".into());
    assert_eq!(err.to_string(), "missing required field: 'timeout'");
}

#[test]
fn test_section4_error_display_invalid() {
    let err = ConfigError::InvalidValue {
        field: "port".into(),
        value: "abc".into(),
        reason: "expected integer".into(),
    };
    assert_eq!(
        err.to_string(),
        "invalid value 'abc' for field 'port': expected integer"
    );
}

#[test]
fn test_section4_error_source_chain() {
    use std::error::Error;
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let err = ConfigError::Io(io_err);

    // source() returns the wrapped io::Error
    assert!(err.source().is_some());
    assert!(err.source().unwrap().to_string().contains("file not found"));
}

#[test]
fn test_section4_error_source_none() {
    use std::error::Error;
    let err = ConfigError::MissingField("x".into());
    // MissingField has no inner error — source() is None
    assert!(err.source().is_none());
}

// ============================================================================
// SECTION 5: FROM CONVERSIONS — Automatic error wrapping with ?
// ============================================================================
//
// The ? operator calls From::from() to convert errors. If you have:
//   impl From<io::Error> for ConfigError
// then any function returning Result<T, ConfigError> can use ? on io::Error.
//
// Python equivalent: except IOError as e: raise ConfigError(...) from e
//
// thiserror generates From impls from #[from] attributes.

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::Io(err)
    }
}

// Now we can use ? to automatically convert io::Error → ConfigError:
fn read_config_value(path: &str) -> Result<String, ConfigError> {
    // This ? converts io::Error → ConfigError::Io automatically
    let content = std::fs::read_to_string(path)?;
    Ok(content.trim().to_string())
}

#[test]
fn test_section5_from_conversion() {
    let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied");
    let config_err: ConfigError = ConfigError::from(io_err);
    assert!(config_err.to_string().contains("I/O error"));
}

#[test]
fn test_section5_question_mark_operator() {
    // Reading a nonexistent file should produce ConfigError::Io
    let result = read_config_value("/nonexistent/path/config.txt");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, ConfigError::Io(_)));
}

// Multiple From impls let ? work with different error sources:

impl From<ParseIntError> for ConfigError {
    fn from(err: ParseIntError) -> Self {
        ConfigError::InvalidValue {
            field: "unknown".into(),
            value: "unknown".into(),
            reason: err.to_string(),
        }
    }
}

fn parse_port(s: &str) -> Result<u16, ConfigError> {
    // ? converts ParseIntError → ConfigError::InvalidValue
    let port: u16 = s.parse()?;
    Ok(port)
}

#[test]
fn test_section5_from_parse_int() {
    assert_eq!(parse_port("8080").unwrap(), 8080);

    let err = parse_port("not_a_number").unwrap_err();
    assert!(matches!(err, ConfigError::InvalidValue { .. }));
}

// ============================================================================
// SECTION 6: ERROR ENUM PATTERNS — Composing domain errors
// ============================================================================
//
// Real applications compose errors from multiple subsystems.
// Each subsystem has its own error type; a top-level enum wraps them all.
//
// Python equivalent:
//   class AppError(Exception): pass
//   class DatabaseError(AppError): pass
//   class NetworkError(AppError): pass
//
// thiserror makes this concise. Manually, it's more verbose but the
// pattern is always the same: enum + Display + Error + From.

#[derive(Debug)]
enum DatabaseError {
    ConnectionFailed(String),
    QueryTimeout { query: String, timeout_secs: u64 },
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::ConnectionFailed(host) => {
                write!(f, "database connection failed: {}", host)
            }
            DatabaseError::QueryTimeout { query, timeout_secs } => {
                write!(f, "query '{}' timed out after {}s", query, timeout_secs)
            }
        }
    }
}

impl std::error::Error for DatabaseError {}

#[derive(Debug)]
enum AppError {
    Config(ConfigError),
    Database(DatabaseError),
    NotFound(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Config(e) => write!(f, "configuration error: {}", e),
            AppError::Database(e) => write!(f, "database error: {}", e),
            AppError::NotFound(resource) => write!(f, "not found: {}", resource),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Config(e) => Some(e),
            AppError::Database(e) => Some(e),
            AppError::NotFound(_) => None,
        }
    }
}

// From impls so ? works across boundaries
impl From<ConfigError> for AppError {
    fn from(err: ConfigError) -> Self {
        AppError::Config(err)
    }
}

impl From<DatabaseError> for AppError {
    fn from(err: DatabaseError) -> Self {
        AppError::Database(err)
    }
}

#[test]
fn test_section6_error_composition() {
    let db_err = DatabaseError::ConnectionFailed("db-primary:5432".into());
    let app_err = AppError::Database(db_err);
    assert!(app_err.to_string().contains("database error"));
    assert!(app_err.to_string().contains("db-primary:5432"));
}

#[test]
fn test_section6_error_chain() {
    use std::error::Error;
    let db_err = DatabaseError::QueryTimeout {
        query: "SELECT *".into(),
        timeout_secs: 30,
    };
    let app_err = AppError::Database(db_err);

    // Walk the error chain
    let source = app_err.source().unwrap();
    assert!(source.to_string().contains("timed out"));
}

#[test]
fn test_section6_from_for_question_mark() {
    fn do_work() -> Result<(), AppError> {
        // Simulating a config error that propagates with ?
        let err: Result<(), ConfigError> = Err(ConfigError::MissingField("host".into()));
        err?; // ? calls From<ConfigError> for AppError
        Ok(())
    }
    let result = do_work();
    assert!(matches!(result, Err(AppError::Config(_))));
}

// ============================================================================
// SECTION 7: FULL ERROR CHAIN WALK
// ============================================================================
//
// In production, you log the entire error chain so the root cause is visible.
// This is what anyhow and tracing do automatically.
//
// Python equivalent: traceback.format_exception_only() with __cause__

fn format_error_chain(err: &dyn std::error::Error) -> String {
    let mut chain = vec![err.to_string()];
    let mut current = err.source();
    while let Some(cause) = current {
        chain.push(cause.to_string());
        current = cause.source();
    }
    chain.join(" -> caused by: ")
}

#[test]
fn test_section7_error_chain_walk() {
    let io_err = std::io::Error::new(std::io::ErrorKind::TimedOut, "connection timed out");
    let config_err = ConfigError::Io(io_err);
    let app_err = AppError::Config(config_err);

    let chain = format_error_chain(&app_err);
    assert!(chain.contains("configuration error"));
    assert!(chain.contains("caused by: config I/O error"));
    assert!(chain.contains("caused by: connection timed out"));
}

// ============================================================================
// SECTION 8: ARGUMENT PARSING — What clap::Parser does
// ============================================================================
//
// CLI tools need to parse arguments like:
//   ./tool --service api --port 8080 --verbose
//
// clap's #[derive(Parser)] generates a full argument parser from struct
// field names and attributes. We'll build a simple one manually.
//
// Python equivalent: argparse.ArgumentParser()
//
// The pattern: define a config struct, write a parse function that
// converts Vec<String> → Result<Config, Error>.

#[derive(Debug, Clone, PartialEq, Eq)]
struct CliConfig {
    service: String,
    port: u16,
    timeout_secs: u64,
    verbose: bool,
}

impl Default for CliConfig {
    fn default() -> Self {
        CliConfig {
            service: "default".into(),
            port: 8080,
            timeout_secs: 30,
            verbose: false,
        }
    }
}

#[derive(Debug)]
enum CliError {
    MissingValue(String),
    InvalidValue { flag: String, reason: String },
    UnknownFlag(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::MissingValue(flag) => write!(f, "flag '{}' requires a value", flag),
            CliError::InvalidValue { flag, reason } => {
                write!(f, "invalid value for '{}': {}", flag, reason)
            }
            CliError::UnknownFlag(flag) => write!(f, "unknown flag: '{}'", flag),
        }
    }
}

impl std::error::Error for CliError {}

/// Parse CLI arguments into a typed config struct.
/// Accepts args like: --service api --port 8080 --verbose
///
/// This is what clap::Parser::parse_from() does automatically.
fn parse_args(args: &[String]) -> Result<CliConfig, CliError> {
    let mut config = CliConfig::default();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--service" => {
                i += 1;
                config.service = args
                    .get(i)
                    .ok_or_else(|| CliError::MissingValue("--service".into()))?
                    .clone();
            }
            "--port" => {
                i += 1;
                let val = args
                    .get(i)
                    .ok_or_else(|| CliError::MissingValue("--port".into()))?;
                config.port = val.parse().map_err(|_| CliError::InvalidValue {
                    flag: "--port".into(),
                    reason: format!("'{}' is not a valid port number", val),
                })?;
            }
            "--timeout" => {
                i += 1;
                let val = args
                    .get(i)
                    .ok_or_else(|| CliError::MissingValue("--timeout".into()))?;
                config.timeout_secs = val.parse().map_err(|_| CliError::InvalidValue {
                    flag: "--timeout".into(),
                    reason: format!("'{}' is not a valid duration", val),
                })?;
            }
            "--verbose" => {
                config.verbose = true;
            }
            other => {
                return Err(CliError::UnknownFlag(other.into()));
            }
        }
        i += 1;
    }

    Ok(config)
}

#[test]
fn test_section8_parse_all_flags() {
    let args: Vec<String> = vec!["--service", "api", "--port", "3000", "--verbose"]
        .into_iter()
        .map(String::from)
        .collect();
    let config = parse_args(&args).unwrap();
    assert_eq!(config.service, "api");
    assert_eq!(config.port, 3000);
    assert!(config.verbose);
}

#[test]
fn test_section8_parse_defaults() {
    let args: Vec<String> = vec![];
    let config = parse_args(&args).unwrap();
    assert_eq!(config, CliConfig::default());
}

#[test]
fn test_section8_parse_missing_value() {
    let args: Vec<String> = vec!["--port".into()];
    let result = parse_args(&args);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("requires a value"));
}

#[test]
fn test_section8_parse_invalid_value() {
    let args: Vec<String> = vec!["--port", "abc"].into_iter().map(String::from).collect();
    let result = parse_args(&args);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not a valid port"));
}

#[test]
fn test_section8_parse_unknown_flag() {
    let args: Vec<String> = vec!["--bogus".into()];
    let result = parse_args(&args);
    assert!(matches!(result, Err(CliError::UnknownFlag(_))));
}

// ============================================================================
// SECTION 9: KEY=VALUE ARGUMENT STYLE
// ============================================================================
//
// An alternative to --flag value: the --flag=value style.
// Both are common in SRE tools (kubectl, docker, terraform).

fn parse_key_value_arg(arg: &str) -> Option<(&str, &str)> {
    arg.strip_prefix("--")?.split_once('=')
}

fn parse_args_kv(args: &[String]) -> Result<CliConfig, CliError> {
    let mut config = CliConfig::default();

    for arg in args {
        if arg == "--verbose" {
            config.verbose = true;
            continue;
        }
        let (key, value) = parse_key_value_arg(arg)
            .ok_or_else(|| CliError::UnknownFlag(arg.clone()))?;

        match key {
            "service" => config.service = value.to_string(),
            "port" => {
                config.port = value.parse().map_err(|_| CliError::InvalidValue {
                    flag: "--port".into(),
                    reason: format!("'{}' is not a valid port", value),
                })?;
            }
            "timeout" => {
                config.timeout_secs =
                    value.parse().map_err(|_| CliError::InvalidValue {
                        flag: "--timeout".into(),
                        reason: format!("'{}' is not a valid duration", value),
                    })?;
            }
            _ => return Err(CliError::UnknownFlag(format!("--{}", key))),
        }
    }

    Ok(config)
}

#[test]
fn test_section9_kv_style() {
    let args: Vec<String> = vec!["--service=web", "--port=9090", "--verbose"]
        .into_iter()
        .map(String::from)
        .collect();
    let config = parse_args_kv(&args).unwrap();
    assert_eq!(config.service, "web");
    assert_eq!(config.port, 9090);
    assert!(config.verbose);
}

#[test]
fn test_section9_kv_unknown() {
    let args: Vec<String> = vec!["--color=blue".into()];
    let result = parse_args_kv(&args);
    assert!(matches!(result, Err(CliError::UnknownFlag(_))));
}

// ============================================================================
// SECTION 10: PUTTING IT ALL TOGETHER — parse, deserialize, handle errors
// ============================================================================
//
// A realistic SRE function: parse a batch of health check lines,
// collect successes and errors, report both.
//
// This combines:
//   - FromStr (deserialization)
//   - Error types with Display
//   - Result handling

#[derive(Debug)]
struct BatchResult {
    checks: Vec<HealthCheck>,
    errors: Vec<String>,
}

fn parse_health_batch(input: &str) -> BatchResult {
    let mut checks = Vec::new();
    let mut errors = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match line.parse::<HealthCheck>() {
            Ok(check) => checks.push(check),
            Err(e) => errors.push(format!("line {}: {}", i + 1, e)),
        }
    }

    BatchResult { checks, errors }
}

#[test]
fn test_section10_batch_parse_mixed() {
    let input = "\
service=api status=healthy latency_ms=10 healthy_nodes=3 total_nodes=3
service=db status=down latency_ms=5000 healthy_nodes=0 total_nodes=3
this is invalid
service=cache status=degraded latency_ms=200 healthy_nodes=2 total_nodes=3
";
    let result = parse_health_batch(input);
    assert_eq!(result.checks.len(), 3);
    assert_eq!(result.errors.len(), 1);
    assert!(result.errors[0].contains("line 3"));
}

#[test]
fn test_section10_batch_all_valid() {
    let input = "\
service=a status=healthy latency_ms=1 healthy_nodes=1 total_nodes=1
service=b status=down latency_ms=2 healthy_nodes=0 total_nodes=1
";
    let result = parse_health_batch(input);
    assert_eq!(result.checks.len(), 2);
    assert!(result.errors.is_empty());
}

#[test]
fn test_section10_batch_empty_input() {
    let result = parse_health_batch("");
    assert!(result.checks.is_empty());
    assert!(result.errors.is_empty());
}

// ============================================================================
// SECTION 11: WHAT THE CRATES GIVE YOU
// ============================================================================
//
// Everything above works with just std. In production, you'll use crates:
//
// ┌────────────────────┬──────────────────────────────────────────────────┐
// │ Manual (this file) │ With crate                                      │
// ├────────────────────┼──────────────────────────────────────────────────┤
// │ impl Display       │ #[derive(Serialize)] + serde_json::to_string()  │
// │ impl FromStr       │ #[derive(Deserialize)] + serde_json::from_str() │
// │ impl Display       │ #[derive(Error)] #[error("...")]                │
// │ impl Error         │   (auto-generated)                              │
// │ impl From<X>       │ #[from] attribute                               │
// │ parse_args()       │ #[derive(Parser)] + Config::parse()             │
// └────────────────────┴──────────────────────────────────────────────────┘
//
// The crates eliminate boilerplate but the underlying traits are identical.
// Understanding the manual implementations makes debugging derive-generated
// code much easier.
//
// To use these crates, you'll need a cargo project:
//   cargo init my-tool
//   cargo add serde --features derive
//   cargo add serde_json
//   cargo add thiserror
//   cargo add clap --features derive

// This "test" just validates the table above is accurate by confirming
// our manual impls satisfy the same trait bounds the crates would provide.
#[test]
fn test_section11_trait_bounds() {
    // HealthCheck satisfies the same bounds serde would provide:
    fn requires_display_and_fromstr<T: fmt::Display + FromStr>() {}
    requires_display_and_fromstr::<HealthCheck>();

    // ConfigError satisfies the same bounds thiserror would provide:
    fn requires_error<T: std::error::Error>() {}
    requires_error::<ConfigError>();
    requires_error::<AppError>();

    // CliConfig satisfies the bounds clap::Parser would provide:
    fn requires_debug_and_default<T: fmt::Debug + Default>() {}
    requires_debug_and_default::<CliConfig>();
}

// ============================================================================
// SECTION 12: PRACTICAL — SRE Config Parser
// ============================================================================
//
// Combining all patterns: a config struct with serialization,
// deserialization, error handling, and defaults.

#[derive(Debug, Clone, PartialEq, Eq)]
struct SreToolConfig {
    service: String,
    port: u16,
    check_interval_secs: u64,
    verbose: bool,
}

impl Default for SreToolConfig {
    fn default() -> Self {
        SreToolConfig {
            service: "default".into(),
            port: 8080,
            check_interval_secs: 30,
            verbose: false,
        }
    }
}

impl fmt::Display for SreToolConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "service={}\nport={}\ncheck_interval_secs={}\nverbose={}",
            self.service, self.port, self.check_interval_secs, self.verbose
        )
    }
}

#[derive(Debug)]
enum SreConfigError {
    MissingField(String),
    InvalidValue { field: String, reason: String },
}

impl fmt::Display for SreConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SreConfigError::MissingField(field) => {
                write!(f, "missing field: '{}'", field)
            }
            SreConfigError::InvalidValue { field, reason } => {
                write!(f, "invalid '{}': {}", field, reason)
            }
        }
    }
}

impl std::error::Error for SreConfigError {}

impl FromStr for SreToolConfig {
    type Err = SreConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields: HashMap<String, String> = HashMap::new();
        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                fields.insert(key.to_string(), value.to_string());
            }
        }

        let service = fields
            .get("service")
            .ok_or_else(|| SreConfigError::MissingField("service".into()))?
            .clone();

        let port: u16 = fields
            .get("port")
            .ok_or_else(|| SreConfigError::MissingField("port".into()))?
            .parse()
            .map_err(|_| SreConfigError::InvalidValue {
                field: "port".into(),
                reason: "must be 0-65535".into(),
            })?;

        let check_interval_secs: u64 = fields
            .get("check_interval_secs")
            .ok_or_else(|| SreConfigError::MissingField("check_interval_secs".into()))?
            .parse()
            .map_err(|_| SreConfigError::InvalidValue {
                field: "check_interval_secs".into(),
                reason: "must be a positive integer".into(),
            })?;

        let verbose: bool = fields
            .get("verbose")
            .map(|v| v == "true")
            .unwrap_or(false);

        Ok(SreToolConfig {
            service,
            port,
            check_interval_secs,
            verbose,
        })
    }
}

#[test]
fn test_section12_config_roundtrip() {
    let config = SreToolConfig {
        service: "monitoring".into(),
        port: 9090,
        check_interval_secs: 15,
        verbose: true,
    };
    let serialized = config.to_string();
    let parsed: SreToolConfig = serialized.parse().unwrap();
    assert_eq!(config, parsed);
}

#[test]
fn test_section12_config_missing_field() {
    let input = "service=api\nport=8080";
    let result: Result<SreToolConfig, _> = input.parse();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("check_interval_secs"));
}

#[test]
fn test_section12_config_invalid_port() {
    let input = "service=api\nport=99999\ncheck_interval_secs=30";
    let result: Result<SreToolConfig, _> = input.parse();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("port"));
}

// ============================================================================
// All tests — run with: rustc concept.rs --edition 2021 --test && ./concept
// ============================================================================

fn main() {
    println!("This file is meant to be run as tests:");
    println!("  rustc concept.rs --edition 2021 --test && ./concept");
}
