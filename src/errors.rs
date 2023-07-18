use failure::Fail;
pub use failure::Error;

#[derive(Debug, Fail)]
pub enum ApiError {
    #[fail(display = "Invalid api_key")]
    InvalidApiKey(),

    #[fail(display = "Invalid api_key")]
    InvalidSecretApiKey(),

    #[fail(display = "Error: {}", _0)]
    GenericError(String),
}

impl From<reqwest::Error> for ApiError {
    fn from(req_err: reqwest::Error) -> Self {
        ApiError::GenericError(format!("{}", req_err))
    }
}

impl From<std::io::Error> for ApiError {
    fn from(io_err: std::io::Error) -> Self {
        ApiError::GenericError(format!("{}", io_err))
    }
}

impl From<walkdir::Error> for ApiError {
    fn from(io_err: walkdir::Error) -> Self {
        ApiError::GenericError(format!("{}", io_err))
    }
}

impl From<std::path::StripPrefixError> for ApiError {
    fn from(io_err: std::path::StripPrefixError) -> Self {
        ApiError::GenericError(format!("{}", io_err))
    }
}