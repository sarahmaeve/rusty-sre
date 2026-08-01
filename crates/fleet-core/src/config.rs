use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ConfigError {
    #[error("missing field: {0}")]
    Missing(&'static str),
    #[error("invalid threshold: {0}")]
    InvalidThreshold(String),
}

pub fn load_threshold(raw: Option<&str>) -> Result<u16, ConfigError> {
    let Some(raw) = raw else {
        return Ok(80);
    };
    Ok(raw.parse().unwrap_or(80))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_threshold_is_loaded() {
        assert_eq!(load_threshold(Some("72")), Ok(72));
    }

    #[test]
    fn missing_threshold_uses_default() {
        assert_eq!(load_threshold(None), Ok(80));
    }
}
