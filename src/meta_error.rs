
/*
*  MetaError is an enum contains all error for this lib
*/
#[derive(Debug)]
pub enum MetaError {
    RequestError(reqwest::Error),
    ParseError(serde_json::Error),
    SomeError(&'static str),
}

/*
* implement From for MetaError -> serde_json::Error
*/
impl From<serde_json::Error> for MetaError {
    fn from(e: serde_json::Error) -> Self {
        MetaError::ParseError(e)
    }
}

/*
* implement From for MetaError -> reqwest::Error
*/
impl From<reqwest::Error> for MetaError {
    fn from(e: reqwest::Error) -> Self {
        MetaError::RequestError(e)
    }
}

/*
* implement From for MetaError -> String / any error
*/
impl From<&'static str> for MetaError {
    fn from(e: &'static str) -> Self {
        MetaError::SomeError(e)
    }
}