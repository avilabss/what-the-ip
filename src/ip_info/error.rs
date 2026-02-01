use super::models::IPInfoError;

impl std::fmt::Display for IPInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Request(e) => write!(f, "Request error: {}", e),
            Self::Api(e) => write!(
                f,
                "API error {}: {} - {}",
                e.status, e.error.title, e.error.message
            ),
        }
    }
}

impl std::error::Error for IPInfoError {}

impl From<reqwest::Error> for IPInfoError {
    fn from(e: reqwest::Error) -> Self {
        IPInfoError::Request(e)
    }
}
