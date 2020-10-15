
#[derive(Debug)]
pub enum MetaError {
    RequestError(reqwest::Error),
    ParseError(serde_json::Error),
    SomeError(String),
}

impl From<serde_json::Error> for MetaError {
    fn from(e: serde_json::Error) -> Self {
        Self::ParseError(e)
    }
}

impl From<reqwest::Error> for MetaError {
    fn from(e: reqwest::Error) -> Self {
        Self::RequestError(e)
    }
}

impl From<String> for MetaError {
    fn from(e: String) -> Self {
        Self::SomeError(e)
    }
}