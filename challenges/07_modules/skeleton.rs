// =============================================================================
// Challenge 07: Modules and Visibility — Refactor a Config Validator
// =============================================================================
//
// The functions and types below all live in a single flat namespace. Your
// job: organize them into three inline modules with appropriate visibility.
//
// Goal layout (inside this file, using `mod foo { ... }` blocks):
//
//   mod parse {
//       pub fn parse_port(s: &str) -> Option<u16>
//       pub fn parse_host(s: &str) -> Option<String>
//   }
//
//   mod validate {
//       pub fn validate_port(port: u16) -> bool
//       pub fn validate_host(host: &str) -> bool
//   }
//
//   mod load {
//       pub use super::parse::*;        // re-export for convenience
//       pub use super::validate::*;
//
//       pub fn load_config(host: &str, port: &str) -> Result<Config, String>
//       // load_config calls parse_* and validate_*
//   }
//
// Notes:
//   - The `Config` struct stays at the top level (used across modules).
//   - The tests below already reference `parse::parse_port`, `validate::validate_port`,
//     and `load::load_config` — your refactor needs to make those paths real.
//   - Helper details: validate_port is `port > 0` (any non-zero u16), and
//     validate_host is `!host.is_empty() && !host.contains(' ')`.
//
// Run the tests with:
//     rustc skeleton.rs --edition 2024 --test && ./skeleton
// =============================================================================

fn main() {
    println!("Complete the TODOs to refactor this file into modules.");
}

// `Config` stays at the file's top level so all three modules can name it.
#[derive(Debug, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

// -----------------------------------------------------------------------------
// TODO: Move parse_port and parse_host into a `mod parse { ... }` block.
//       Mark each `pub`. Delete the originals once moved.
// -----------------------------------------------------------------------------

fn parse_port(s: &str) -> Option<u16> {
    s.trim().parse().ok()
}

fn parse_host(s: &str) -> Option<String> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

// -----------------------------------------------------------------------------
// TODO: Move validate_port and validate_host into a `mod validate { ... }`
//       block. Mark each `pub`. Delete the originals once moved.
// -----------------------------------------------------------------------------

fn validate_port(port: u16) -> bool {
    port > 0
}

fn validate_host(host: &str) -> bool {
    !host.is_empty() && !host.contains(' ')
}

// -----------------------------------------------------------------------------
// TODO: Move load_config into a `mod load { ... }` block.
//       Inside `load`, use `use super::parse::*;` and `use super::validate::*;`
//       so the body below works. Mark `load_config` itself `pub`.
// -----------------------------------------------------------------------------

fn load_config(host: &str, port: &str) -> Result<Config, String> {
    let host = parse_host(host).ok_or_else(|| "host required".to_string())?;
    let port = parse_port(port).ok_or_else(|| "port unparseable".to_string())?;

    if !validate_host(&host) {
        return Err(format!("invalid host: {host}"));
    }
    if !validate_port(port) {
        return Err(format!("invalid port: {port}"));
    }

    Ok(Config { host, port })
}

// =============================================================================
// TESTS — Do not modify below this line
// =============================================================================
//
// These tests reach into the modules using the qualified paths. They will
// fail to compile until you complete the refactor.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_module_is_reachable() {
        assert_eq!(parse::parse_port("8080"), Some(8080));
        assert_eq!(parse::parse_host(" example.com "), Some("example.com".to_string()));
    }

    #[test]
    fn validate_module_is_reachable() {
        assert!(validate::validate_port(80));
        assert!(!validate::validate_port(0));
        assert!(validate::validate_host("example.com"));
        assert!(!validate::validate_host(""));
        assert!(!validate::validate_host("has space"));
    }

    #[test]
    fn load_module_assembles_config() {
        let cfg = load::load_config("example.com", "8080").unwrap();
        assert_eq!(cfg, Config { host: "example.com".to_string(), port: 8080 });
    }

    #[test]
    fn load_module_propagates_errors() {
        assert!(load::load_config("", "8080").is_err());
        assert!(load::load_config("example.com", "0").is_err());
        assert!(load::load_config("example.com", "abc").is_err());
        assert!(load::load_config("has space", "8080").is_err());
    }
}
