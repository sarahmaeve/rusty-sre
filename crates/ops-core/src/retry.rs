pub fn retry<T, E>(
    max_retries: usize,
    mut operation: impl FnMut() -> Result<T, E>,
) -> Result<T, E> {
    let attempts = max_retries.max(1);
    for attempt in 0..attempts {
        match operation() {
            Ok(value) => return Ok(value),
            Err(error) if attempt + 1 == attempts => return Err(error),
            Err(_) => {}
        }
    }
    unreachable!("at least one attempt is made")
}

#[derive(Debug)]
pub struct PipelineError {
    message: String,
    _source: Option<std::io::Error>,
}

impl PipelineError {
    pub fn io(message: impl Into<String>, source: std::io::Error) -> Self {
        Self {
            message: message.into(),
            _source: Some(source),
        }
    }
}

impl std::fmt::Display for PipelineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for PipelineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
