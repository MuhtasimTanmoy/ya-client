//! Error definitions and mappings
use awc::error::{JsonPayloadError, PayloadError, SendRequestError};
use awc::http::StatusCode;

use ya_client_model::ErrorMessage;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("AWC error requesting {url}: {msg}")]
    SendRequestError { msg: String, url: String },
    #[error("AWC timeout requesting {url}: {msg}")]
    TimeoutError { msg: String, url: String },
    #[error("AWC payload error: {0}")]
    PayloadError(PayloadError),
    #[error("AWC JSON payload error: {0}")]
    JsonPayloadError(JsonPayloadError),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::error::Error),
    #[error("request for {url} resulted in HTTP status code: {code}: {msg}")]
    HttpStatusCode {
        code: StatusCode,
        url: String,
        msg: String,
    },
    #[error("serde JSON error: {0}")]
    SerdeJsonError(serde_json::Error),
    #[error("invalid address: {0}")]
    InvalidAddress(std::convert::Infallible),
    #[error("invalid header: {0}")]
    InvalidHeaderName(#[from] awc::http::header::InvalidHeaderName),
    #[error("invalid header: {0}")]
    InvalidHeaderValue(#[from] awc::http::header::InvalidHeaderValue),
    #[error("invalid UTF8 string: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("invalid UTF8 string: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("Url parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error(transparent)]
    ApiErrorMessage(#[from] ErrorMessage),
    #[error("Internal ya-client error: {0}")]
    InternalError(String),
    #[error("Event stream error: {0}")]
    EventStreamError(String),
}

impl From<SendRequestError> for Error {
    fn from(e: SendRequestError) -> Self {
        (e, "".into()).into()
    }
}

impl From<(SendRequestError, String)> for Error {
    fn from((e, url): (SendRequestError, String)) -> Self {
        let msg = e.to_string();
        match e {
            SendRequestError::Timeout => Error::TimeoutError { msg, url },
            _ => Error::SendRequestError { msg, url },
        }
    }
}

impl From<PayloadError> for Error {
    fn from(e: PayloadError) -> Self {
        Error::PayloadError(e)
    }
}

impl From<JsonPayloadError> for Error {
    fn from(e: JsonPayloadError) -> Self {
        Error::JsonPayloadError(e)
    }
}

impl<E: std::fmt::Display> From<(StatusCode, String, Result<ErrorMessage, E>)> for Error {
    fn from((code, url, err_msg): (StatusCode, String, Result<ErrorMessage, E>)) -> Self {
        let msg = err_msg
            .map(|e| e.message.unwrap_or_default())
            .unwrap_or_else(|e| format!("error parsing error msg: {}", e));
        if code == StatusCode::REQUEST_TIMEOUT {
            Error::TimeoutError { msg, url }
        } else {
            Error::HttpStatusCode { code, url, msg }
        }
    }
}
