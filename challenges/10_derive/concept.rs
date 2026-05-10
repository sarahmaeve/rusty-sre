// ============================================================================
// Challenge 03: derive — Concept Explainer
// ============================================================================
//
// This file teaches Rust's #[derive] mechanism through 12 sections, each with
// tests. Run with:
//     rustc concept.rs --edition 2024 --test && ./concept
//
// Target audience: SRE/DevOps engineers who know Python.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};

// ============================================================================
// SECTION 1: WHAT IS derive?
// ============================================================================
//
// In Python, you manually write special methods:
//     class Server:
//         def __repr__(self):
//             return f"Server({self.name})"
//
// In Rust, #[derive(Debug)] writes the equivalent impl for you at compile
// time. The compiler reads your struct's fields and generates the trait
// implementation. There is ZERO runtime cost — the code is generated once
// during compilation, just as if you had written it by hand.
//
// Here's what derive(Debug) generates vs writing it manually:

// MANUAL implementation — you write every field yourself:
struct ServerManual {
    name: String,
    ip: String,
    port: u16,
}

impl fmt::Debug for ServerManual {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServerManual")
            .field("name", &self.name)
            .field("ip", &self.ip)
            .field("port", &self.port)
            .finish()
    }
}

// DERIVED implementation — one line, same result:
#[derive(Debug)]
struct ServerDerived {
    name: String,
    ip: String,
    port: u16,
}

// derive is NOT magic — it's a macro that expands at compile time.
// You can only derive traits that have a known, mechanical implementation
// based on the struct's fields. Rust calls these "derivable traits."

#[test]
fn test_section1_manual_vs_derived() {
    let manual = ServerManual {
        name: "web-01".into(),
        ip: "10.0.1.1".into(),
        port: 8080,
    };
    let derived = ServerDerived {
        name: "web-01".into(),
        ip: "10.0.1.1".into(),
        port: 8080,
    };
    // Both produce the same Debug output format
    let manual_str = format!("{:?}", manual);
    let derived_str = format!("{:?}", derived);
    assert!(manual_str.contains("web-01"));
    assert!(derived_str.contains("web-01"));
}

#[test]
fn test_section1_derive_is_compile_time() {
    // derive works on enums too:
    #[derive(Debug)]
    enum Region {
        UsEast1,
        UsWest2,
        EuWest1,
    }
    // The variant names become the debug output
    assert_eq!(format!("{:?}", Region::UsEast1), "UsEast1");
    assert_eq!(format!("{:?}", Region::EuWest1), "EuWest1");
}

// ============================================================================
// SECTION 2: Debug
// ============================================================================
//
// Debug provides developer-facing formatting via {:?} and {:#?}.
// Python equivalent: __repr__
//
// {:?}  → compact single-line output
// {:#?} → pretty-printed with indentation (great for logging)
//
// SRE tip: derive Debug on EVERYTHING. It's free and makes debugging
// production issues vastly easier. But implement manually when you need
// to REDACT secrets (API keys, passwords, tokens).

#[derive(Debug)]
struct MetricsSnapshot {
    service: String,
    cpu_percent: f64,
    memory_mb: u64,
    healthy: bool,
}

// Example: manual Debug to redact a secret field
struct ApiClient {
    endpoint: String,
    api_key: String, // SECRET — never log this!
}

impl fmt::Debug for ApiClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ApiClient")
            .field("endpoint", &self.endpoint)
            .field("api_key", &"[REDACTED]") // Redact the secret
            .finish()
    }
}

#[test]
fn test_section2_debug_compact() {
    let snap = MetricsSnapshot {
        service: "api-gateway".into(),
        cpu_percent: 73.5,
        memory_mb: 2048,
        healthy: true,
    };
    let output = format!("{:?}", snap);
    assert!(output.contains("api-gateway"));
    assert!(output.contains("73.5"));
}

#[test]
fn test_section2_debug_pretty_print() {
    let snap = MetricsSnapshot {
        service: "api-gateway".into(),
        cpu_percent: 73.5,
        memory_mb: 2048,
        healthy: true,
    };
    // {:#?} produces multi-line output with indentation
    let pretty = format!("{:#?}", snap);
    assert!(pretty.contains('\n'));
    assert!(pretty.contains("    ")); // indentation
}

#[test]
fn test_section2_redacted_debug() {
    let client = ApiClient {
        endpoint: "https://api.example.com".into(),
        api_key: "sk-super-secret-key-12345".into(),
    };
    let output = format!("{:?}", client);
    assert!(output.contains("REDACTED"));
    assert!(!output.contains("sk-super-secret")); // Secret must not appear
}

// ============================================================================
// SECTION 3: Clone and Copy
// ============================================================================
//
// Clone = explicit deep copy via .clone()
// Copy  = implicit copy on assignment (no move)
//
// Copy REQUIRES Clone (it's a supertrait).
//
// Key rule: types with heap data (String, Vec, Box) can Clone but NOT Copy.
// Simple types (i32, f64, bool, references, field-less enums) can be both.
//
// Python comparison: Python has no equivalent — everything is a reference.
// In Python, `a = b` just copies the reference. In Rust, `a = b` MOVES b
// (unless the type is Copy, in which case it copies the bits).

// This type has only stack data → can derive both Clone and Copy
#[derive(Debug, Clone, Copy, PartialEq)]
struct CpuReading {
    percent: f64,
    core_id: u8,
}

// This type has String (heap data) → can Clone but NOT Copy
#[derive(Debug, Clone, PartialEq)]
struct LogEntry {
    message: String,
    level: u8,
}

#[test]
fn test_section3_copy_no_move() {
    let reading = CpuReading {
        percent: 85.0,
        core_id: 3,
    };
    let copy = reading; // Copy: reading is still valid!
    assert_eq!(reading.percent, copy.percent); // Both usable
}

#[test]
fn test_section3_clone_explicit() {
    let entry = LogEntry {
        message: "disk full".into(),
        level: 1,
    };
    let cloned = entry.clone(); // Must be explicit — no Copy
    assert_eq!(cloned.message, "disk full");
    // entry is still valid because we used .clone(), not move
    assert_eq!(entry.level, 1);
}

#[test]
fn test_section3_copy_in_function() {
    // Copy types can be passed to functions without losing ownership
    fn double_reading(r: CpuReading) -> f64 {
        r.percent * 2.0
    }
    let r = CpuReading {
        percent: 42.0,
        core_id: 0,
    };
    let _ = double_reading(r);
    // r is still valid because CpuReading is Copy
    assert_eq!(r.percent, 42.0);
}

// ============================================================================
// SECTION 4: PartialEq and Eq
// ============================================================================
//
// PartialEq enables == and !=, comparing field-by-field.
// Eq is a MARKER trait (no methods) that promises TOTAL equality:
//   for all a: a == a (reflexivity)
//
// f64 implements PartialEq but NOT Eq because NaN != NaN.
//
// Python equivalent: __eq__
//
// Custom PartialEq is powerful for SRE: compare alerts ignoring timestamps
// to detect duplicates.

#[derive(Debug, Clone)]
struct Incident {
    id: u64,
    service: String,
    message: String,
    timestamp: u64,
}

// Custom PartialEq: two incidents are "equal" if they describe the same
// problem, regardless of when they were created or their unique ID.
impl PartialEq for Incident {
    fn eq(&self, other: &Self) -> bool {
        self.service == other.service && self.message == other.message
    }
}
impl Eq for Incident {} // Marker trait — no body needed

#[test]
fn test_section4_derived_eq() {
    #[derive(Debug, PartialEq, Eq)]
    struct Simple {
        a: i32,
        b: String,
    }
    let x = Simple { a: 1, b: "hi".into() };
    let y = Simple { a: 1, b: "hi".into() };
    let z = Simple { a: 2, b: "hi".into() };
    assert_eq!(x, y);
    assert_ne!(x, z);
}

#[test]
fn test_section4_custom_eq_ignores_fields() {
    let inc1 = Incident {
        id: 1,
        service: "auth".into(),
        message: "timeout".into(),
        timestamp: 1000,
    };
    let inc2 = Incident {
        id: 99,
        service: "auth".into(),
        message: "timeout".into(),
        timestamp: 2000,
    };
    // Same service + message → equal, even though id and timestamp differ
    assert_eq!(inc1, inc2);
}

#[test]
fn test_section4_partialeq_f64() {
    // f64 has PartialEq but NOT Eq because of NaN
    let nan = f64::NAN;
    assert!(nan != nan); // NaN is never equal to itself!
}

// ============================================================================
// SECTION 5: PartialOrd and Ord
// ============================================================================
//
// PartialOrd enables < > <= >= comparisons.
// Ord enables .sort() and use as BTreeMap keys.
//
// For enums, derived ordering follows DECLARATION ORDER.
// First variant < second variant < third variant...
//
// Python equivalent: __lt__ and functools.total_ordering
//
// SRE pattern: order severity levels so Critical > Error > Warning > Info.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Severity {
    Info,     // 0 — lowest
    Warning,  // 1
    Error,    // 2
    Critical, // 3 — highest
}

#[test]
fn test_section5_enum_ordering() {
    assert!(Severity::Info < Severity::Warning);
    assert!(Severity::Critical > Severity::Error);

    let mut severities = vec![
        Severity::Error,
        Severity::Info,
        Severity::Critical,
        Severity::Warning,
    ];
    severities.sort();
    assert_eq!(
        severities,
        vec![Severity::Info, Severity::Warning, Severity::Error, Severity::Critical]
    );
}

#[test]
fn test_section5_ord_enables_btreemap() {
    // Ord is required for BTreeMap keys (sorted map)
    let mut counts: BTreeMap<Severity, u32> = BTreeMap::new();
    counts.insert(Severity::Critical, 3);
    counts.insert(Severity::Info, 42);
    counts.insert(Severity::Warning, 15);

    // BTreeMap iterates in key order
    let keys: Vec<_> = counts.keys().collect();
    assert_eq!(keys, vec![&Severity::Info, &Severity::Warning, &Severity::Critical]);
}

// ============================================================================
// SECTION 6: Default
// ============================================================================
//
// Default provides a set of default values for all fields.
// - Numbers default to 0
// - Bools default to false
// - String/Vec/Option default to empty/None
// - Enums: use #[default] attribute on one variant (edition 2021+)
//
// Struct update syntax: Config { field: value, ..Default::default() }
// This creates a struct with your overrides + defaults for everything else.
//
// Python equivalent: __init__ with default keyword arguments.
//
// SRE pattern: config structs with sensible defaults.

#[derive(Debug, Default, PartialEq)]
struct MonitorConfig {
    check_interval_secs: u64,
    timeout_secs: u64,
    retries: u32,
    verbose: bool,
    endpoint: String,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum LogLevel {
    Debug,
    #[default]
    Info, // This is the default variant
    Warn,
    Error,
}

#[test]
fn test_section6_default_values() {
    let config = MonitorConfig::default();
    assert_eq!(config.check_interval_secs, 0);
    assert_eq!(config.timeout_secs, 0);
    assert_eq!(config.retries, 0);
    assert!(!config.verbose);
    assert_eq!(config.endpoint, "");
}

#[test]
fn test_section6_struct_update_syntax() {
    // Override just the fields you care about, default the rest
    let config = MonitorConfig {
        check_interval_secs: 30,
        timeout_secs: 5,
        ..Default::default()
    };
    assert_eq!(config.check_interval_secs, 30);
    assert_eq!(config.timeout_secs, 5);
    assert_eq!(config.retries, 0); // default
    assert!(!config.verbose); // default
}

#[test]
fn test_section6_enum_default() {
    let level = LogLevel::default();
    assert_eq!(level, LogLevel::Info);
}

// ============================================================================
// SECTION 7: Hash
// ============================================================================
//
// Hash is required for HashMap keys and HashSet members.
//
// THE CONTRACT: Hash MUST be consistent with Eq.
//   If a == b, then hash(a) == hash(b).
//
// If you have custom PartialEq that ignores certain fields, your Hash
// impl must ALSO ignore those same fields. Otherwise HashSet/HashMap
// will silently produce wrong results.
//
// f64 cannot derive Hash (because it doesn't implement Eq, due to NaN).

#[derive(Debug, Clone)]
struct ServiceAlert {
    id: u64,
    service: String,
    severity: Severity,
    timestamp: u64,
}

// Custom PartialEq: ignore id and timestamp
impl PartialEq for ServiceAlert {
    fn eq(&self, other: &Self) -> bool {
        self.service == other.service && self.severity == other.severity
    }
}
impl Eq for ServiceAlert {}

// Custom Hash: MUST match PartialEq — only hash service and severity
impl Hash for ServiceAlert {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.service.hash(state);
        self.severity.hash(state);
        // Do NOT hash id or timestamp — they're ignored by PartialEq
    }
}

#[test]
fn test_section7_hashset_dedup() {
    let mut seen = HashSet::new();
    let a1 = ServiceAlert {
        id: 1,
        service: "db".into(),
        severity: Severity::Error,
        timestamp: 1000,
    };
    let a2 = ServiceAlert {
        id: 2,
        service: "db".into(),
        severity: Severity::Error,
        timestamp: 2000,
    };
    seen.insert(a1);
    // a2 has same service+severity → should be recognized as duplicate
    assert!(!seen.insert(a2)); // insert returns false if already present
}

#[test]
fn test_section7_hashmap_key() {
    // Hash + Eq enables use as HashMap key
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct ServiceKey {
        name: String,
        env: String,
    }
    let mut status: HashMap<ServiceKey, bool> = HashMap::new();
    let key = ServiceKey {
        name: "api".into(),
        env: "prod".into(),
    };
    status.insert(key.clone(), true);
    assert_eq!(status.get(&key), Some(&true));
}

// ============================================================================
// SECTION 8: COMBINING DERIVES — The Dependency Graph
// ============================================================================
//
// Some traits depend on others:
//   Copy   → requires Clone
//   Eq     → requires PartialEq
//   Ord    → requires PartialOrd + Eq (which requires PartialEq)
//
// A "fully derived" type with all standard traits:
//   #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
//
// NOTE: The ORDER of derives does NOT matter. You can write them in any order.
// Convention is: Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
enum Environment {
    #[default]
    Development,
    Staging,
    Production,
}

#[test]
fn test_section8_full_derive() {
    // Clone + Copy
    let env = Environment::Production;
    let copy = env; // Copy
    assert_eq!(env, copy); // PartialEq

    // Ord
    assert!(Environment::Development < Environment::Staging);
    assert!(Environment::Staging < Environment::Production);

    // Default
    assert_eq!(Environment::default(), Environment::Development);

    // Hash — can be used in HashSet
    let mut envs = HashSet::new();
    envs.insert(env);
    assert!(envs.contains(&Environment::Production));
}

#[test]
fn test_section8_dependency_chain() {
    // Demonstrating that Ord provides everything needed for sorting,
    // BTreeMap keys, comparisons, equality, and hashing
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct StatusCode(u16);

    let mut codes = vec![StatusCode(503), StatusCode(200), StatusCode(404)];
    codes.sort(); // Requires Ord
    assert_eq!(codes, vec![StatusCode(200), StatusCode(404), StatusCode(503)]);

    let mut map: BTreeMap<StatusCode, &str> = BTreeMap::new();
    map.insert(StatusCode(200), "OK");
    map.insert(StatusCode(404), "Not Found");
    assert_eq!(map[&StatusCode(200)], "OK"); // Requires Ord
}

// ============================================================================
// SECTION 9: MANUAL TRAIT IMPLEMENTATIONS
// ============================================================================
//
// You MUST implement manually when:
// - Display (no standard derive exists)
// - Custom equality logic (ignore certain fields)
// - Custom sort order (severity descending, not ascending)
// - Custom Debug (redacting secrets)
//
// Manual Ord example: sort by severity DESCENDING, then by service ASCENDING.
// The .then_with() method chains comparisons.

#[derive(Debug, Clone)]
struct PagerAlert {
    severity: Severity,
    service: String,
    message: String,
}

impl Ord for PagerAlert {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Severity DESCENDING (reverse), then service ASCENDING
        other
            .severity
            .cmp(&self.severity) // reversed: other.cmp(self)
            .then_with(|| self.service.cmp(&other.service))
    }
}

impl PartialOrd for PagerAlert {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other)) // Delegate to Ord
    }
}

impl PartialEq for PagerAlert {
    fn eq(&self, other: &Self) -> bool {
        self.severity == other.severity
            && self.service == other.service
            && self.message == other.message
    }
}
impl Eq for PagerAlert {}

#[test]
fn test_section9_custom_sort_order() {
    let mut alerts = vec![
        PagerAlert {
            severity: Severity::Info,
            service: "web".into(),
            message: "ok".into(),
        },
        PagerAlert {
            severity: Severity::Critical,
            service: "db".into(),
            message: "down".into(),
        },
        PagerAlert {
            severity: Severity::Critical,
            service: "auth".into(),
            message: "timeout".into(),
        },
        PagerAlert {
            severity: Severity::Warning,
            service: "cache".into(),
            message: "eviction".into(),
        },
    ];
    alerts.sort();
    // Critical first (descending), then alphabetical service (ascending)
    assert_eq!(alerts[0].service, "auth"); // Critical
    assert_eq!(alerts[1].service, "db"); // Critical
    assert_eq!(alerts[2].service, "cache"); // Warning
    assert_eq!(alerts[3].service, "web"); // Info
}

// ============================================================================
// SECTION 10: DISPLAY TRAIT
// ============================================================================
//
// Display provides user-facing formatting via {}.
// Debug provides developer-facing formatting via {:?}.
//
// There is NO #[derive(Display)] in std — you always implement it manually.
// Implementing Display automatically gives you .to_string() for free.
//
// Python equivalent: __str__ (Display) vs __repr__ (Debug)

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Info => write!(f, "INFO"),
            Severity::Warning => write!(f, "WARNING"),
            Severity::Error => write!(f, "ERROR"),
            Severity::Critical => write!(f, "CRITICAL"),
        }
    }
}

impl fmt::Display for PagerAlert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format: [CRITICAL] auth: Connection timeout
        write!(f, "[{}] {}: {}", self.severity, self.service, self.message)
    }
}

#[test]
fn test_section10_display_vs_debug() {
    let sev = Severity::Critical;
    assert_eq!(format!("{}", sev), "CRITICAL"); // Display: user-facing
    assert_eq!(format!("{:?}", sev), "Critical"); // Debug: developer-facing
}

#[test]
fn test_section10_display_to_string() {
    let alert = PagerAlert {
        severity: Severity::Error,
        service: "api".into(),
        message: "rate limit exceeded".into(),
    };
    // Display gives you .to_string() for free
    assert_eq!(alert.to_string(), "[ERROR] api: rate limit exceeded");
}

// ============================================================================
// SECTION 11: NEWTYPE PATTERN
// ============================================================================
//
// Wrapping a primitive in a single-field struct gives you compile-time type
// safety at zero runtime cost. The compiler prevents you from accidentally
// mixing up a ServiceId and a Port, even though both are numbers.
//
// Python has no equivalent — you'd pass plain ints and hope for the best.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ServiceId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Port(u16);

// These functions accept ONLY the correct newtype — you can't mix them up.
fn service_name(id: ServiceId) -> &'static str {
    match id.0 {
        1 => "auth",
        2 => "api",
        _ => "unknown",
    }
}

fn is_privileged_port(port: Port) -> bool {
    port.0 < 1024
}

#[test]
fn test_section11_newtype_safety() {
    let id = ServiceId(1);
    let port = Port(443);

    assert_eq!(service_name(id), "auth");
    assert!(is_privileged_port(port));

    // This would NOT compile:
    // service_name(port);  // ERROR: expected ServiceId, found Port
    // is_privileged_port(id);  // ERROR: expected Port, found ServiceId
}

#[test]
fn test_section11_newtype_in_collections() {
    let mut service_ports: HashMap<ServiceId, Port> = HashMap::new();
    service_ports.insert(ServiceId(1), Port(8080));
    service_ports.insert(ServiceId(2), Port(3000));

    assert_eq!(service_ports[&ServiceId(1)], Port(8080));
}

// ============================================================================
// SECTION 12: PRACTICAL — SRE ALERT TYPE
// ============================================================================
//
// Combining everything: a complete alert system with:
// - AlertLevel enum: Copy, Ord, Hash, Default, Display
// - SreAlert struct: custom PartialEq/Hash (ignore timestamp), Display
// - SreAlertConfig struct: Default with sensible values

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum AlertLevel {
    Info,
    Warning,
    Error,
    #[allow(dead_code)]
    Critical,
}

impl Default for AlertLevel {
    fn default() -> Self {
        AlertLevel::Warning // SRE default: only alert on Warning+
    }
}

impl fmt::Display for AlertLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlertLevel::Info => write!(f, "INFO"),
            AlertLevel::Warning => write!(f, "WARNING"),
            AlertLevel::Error => write!(f, "ERROR"),
            AlertLevel::Critical => write!(f, "CRITICAL"),
        }
    }
}

#[derive(Debug, Clone)]
struct SreAlert {
    id: String,
    service: String,
    level: AlertLevel,
    message: String,
    timestamp: u64,
}

// Custom PartialEq: ignore id and timestamp for dedup
impl PartialEq for SreAlert {
    fn eq(&self, other: &Self) -> bool {
        self.service == other.service
            && self.level == other.level
            && self.message == other.message
    }
}
impl Eq for SreAlert {}

// Custom Hash: consistent with PartialEq
impl Hash for SreAlert {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.service.hash(state);
        self.level.hash(state);
        self.message.hash(state);
    }
}

// Display: Slack-ready formatting
impl fmt::Display for SreAlert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}: {}", self.level, self.service, self.message)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct SreAlertConfig {
    min_level: AlertLevel,
    dedup_window_secs: u64,
    max_alerts_per_service: usize,
    notify_slack: bool,
    notify_pagerduty: bool,
}

impl Default for SreAlertConfig {
    fn default() -> Self {
        SreAlertConfig {
            min_level: AlertLevel::Warning,
            dedup_window_secs: 300,
            max_alerts_per_service: 100,
            notify_slack: true,
            notify_pagerduty: false,
        }
    }
}

#[test]
fn test_section12_alert_dedup() {
    let mut seen = HashSet::new();
    let a1 = SreAlert {
        id: "a1".into(),
        service: "auth".into(),
        level: AlertLevel::Error,
        message: "connection refused".into(),
        timestamp: 1000,
    };
    let a2 = SreAlert {
        id: "a2".into(),
        service: "auth".into(),
        level: AlertLevel::Error,
        message: "connection refused".into(),
        timestamp: 1005,
    };
    seen.insert(a1);
    assert!(!seen.insert(a2)); // Duplicate detected
    assert_eq!(seen.len(), 1);
}

#[test]
fn test_section12_display_formatting() {
    let alert = SreAlert {
        id: "x".into(),
        service: "api-gateway".into(),
        level: AlertLevel::Error,
        message: "upstream timeout".into(),
        timestamp: 0,
    };
    assert_eq!(alert.to_string(), "[ERROR] api-gateway: upstream timeout");
}

#[test]
fn test_section12_config_defaults() {
    let config = SreAlertConfig::default();
    assert_eq!(config.min_level, AlertLevel::Warning);
    assert_eq!(config.dedup_window_secs, 300);
    assert!(config.notify_slack);
    assert!(!config.notify_pagerduty);

    // Struct update syntax: override just pagerduty
    let oncall_config = SreAlertConfig {
        notify_pagerduty: true,
        ..Default::default()
    };
    assert!(oncall_config.notify_pagerduty);
    assert!(oncall_config.notify_slack); // still default
}

// ============================================================================
// All tests — run with: rustc concept.rs --edition 2024 --test && ./concept
// ============================================================================

fn main() {
    println!("This file is meant to be run as tests:");
    println!("  rustc concept.rs --edition 2024 --test && ./concept");
}
