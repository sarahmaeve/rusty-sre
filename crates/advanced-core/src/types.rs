use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Port(u16);

impl Port {
    pub const fn get(self) -> u16 {
        self.0
    }
}

impl From<u16> for Port {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PortError;

impl fmt::Display for PortError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("port zero is reserved")
    }
}

impl std::error::Error for PortError {}

pub fn configured_port(value: u16) -> Result<Port, PortError> {
    Ok(Port::from(value))
}
