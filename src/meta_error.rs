
#[derive(Debug)]
pub enum MetaError {
    RequestError(reqwest::Error),
    ParseError(serde_json::Error),
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