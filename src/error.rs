#[allow(dead_code)]
#[derive(Debug)]
pub struct OpenTelemetryError {
    message: String,
}

impl OpenTelemetryError {
    pub fn new<T>(message: T) -> Self
    where
        T: ToString,
    {
        Self {
            message: message.to_string(),
        }
    }
}

impl<T> From<T> for OpenTelemetryError
where
    T: std::error::Error + Send + Sync + 'static,
{
    fn from(value: T) -> Self {
        Self::new(value.to_string())
    }
}

impl std::fmt::Display for OpenTelemetryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
