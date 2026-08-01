//! `Option<T>` represents an absent value. `Result<T, E>` represents success or
//! failure. `?` returns early while converting errors through `From`.
//!
//! Further reading:
//! - <https://doc.rust-lang.org/book/ch09-00-error-handling.html>
//! - <https://doc.rust-lang.org/std/option/enum.Option.html>
//! - <https://doc.rust-lang.org/std/result/enum.Result.html>
//! - <https://doc.rust-lang.org/std/error/trait.Error.html>
//! - Source study: <https://github.com/rust-lang/cargo/tree/master/src/cargo>

use std::{error::Error, fmt, num::ParseIntError};

#[derive(Debug, PartialEq, Eq)]
struct Config {
    host: String,
    port: u16,
}

#[derive(Debug)]
enum ConfigError {
    MissingSeparator,
    EmptyHost,
    InvalidPort(ParseIntError),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSeparator => write!(formatter, "endpoint must contain ':'"),
            Self::EmptyHost => write!(formatter, "host must not be empty"),
            Self::InvalidPort(_) => write!(formatter, "port is not an unsigned 16-bit integer"),
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidPort(source) => Some(source),
            _ => None,
        }
    }
}

impl From<ParseIntError> for ConfigError {
    fn from(error: ParseIntError) -> Self {
        Self::InvalidPort(error)
    }
}

fn parse_endpoint(input: &str) -> Result<Config, ConfigError> {
    let (host, port) = input
        .rsplit_once(':')
        .ok_or(ConfigError::MissingSeparator)?;
    if host.is_empty() {
        return Err(ConfigError::EmptyHost);
    }
    // `?` applies `From<ParseIntError>` before returning the error.
    let port = port.parse()?;
    Ok(Config {
        host: host.to_owned(),
        port,
    })
}

fn optional_timeout(input: Option<&str>) -> Result<Option<u64>, ParseIntError> {
    // `transpose` turns Option<Result<T, E>> into Result<Option<T>, E>.
    input.map(str::parse).transpose()
}

fn main() {
    assert_eq!(
        parse_endpoint("cache.internal:6379").unwrap(),
        Config {
            host: "cache.internal".to_owned(),
            port: 6379,
        }
    );

    let error = parse_endpoint("cache.internal:not-a-port").unwrap_err();
    assert_eq!(error.to_string(), "port is not an unsigned 16-bit integer");
    assert!(error.source().is_some());

    assert_eq!(optional_timeout(None).unwrap(), None);
    assert_eq!(optional_timeout(Some("30")).unwrap(), Some(30));
    assert!(optional_timeout(Some("soon")).is_err());

    let values = [Ok(2), Ok(3), Ok(5)];
    let collected: Result<Vec<_>, &str> = values.into_iter().collect();
    assert_eq!(collected.unwrap(), vec![2, 3, 5]);
}
