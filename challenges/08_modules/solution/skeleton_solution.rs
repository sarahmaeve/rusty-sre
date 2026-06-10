// =============================================================================
// Challenge 08: Modules and Visibility — Config Validator (SKELETON SOLUTION)
// =============================================================================
//
// Reference refactor of skeleton.rs: the flat namespace organized into
// three inline modules with appropriate visibility.
// Run the tests from inside the solution/ directory:
//     rustc skeleton_solution.rs --edition 2024 --test && ./skeleton_solution
// =============================================================================

fn main() {
    println!("Reference solution — run with --test to execute the tests.");
}

// `Config` stays at the file's top level so all three modules can name it.
#[derive(Debug, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

mod parse {
    pub fn parse_port(s: &str) -> Option<u16> {
        s.trim().parse().ok()
    }

    pub fn parse_host(s: &str) -> Option<String> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    }
}

mod validate {
    pub fn validate_port(port: u16) -> bool {
        port > 0
    }

    pub fn validate_host(host: &str) -> bool {
        !host.is_empty() && !host.contains(' ')
    }
}

mod load {
    pub use super::parse::*; // re-export for convenience
    pub use super::validate::*;

    use super::Config;

    pub fn load_config(host: &str, port: &str) -> Result<Config, String> {
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
}

// =============================================================================
// TESTS — identical to skeleton.rs
// =============================================================================

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
