
#[derive(Debug)]
pub enum MetaError {
    RequestError(reqwest::Error),
    ParseError(serde_json::Error),
    SomeError(&'static str),
}

impl From<serde_json::Error> for MetaError {
    fn from(e: serde_json::Error) -> Self {
        MetaError::ParseError(e)
    }
}

impl From<reqwest::Error> for MetaError {
    fn from(e: reqwest::Error) -> Self {
        MetaError::RequestError(e)
    }
}

impl From<&'static str> for MetaError {
    fn from(e: &'static str) -> Self {
        MetaError::SomeError(e)
    }
}