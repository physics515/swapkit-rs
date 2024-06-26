use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug)]
pub enum APIError {
	#[error("Reqwest Error: {0}")]
	ReqwestError(#[from] reqwest::Error),

	#[error("URL Parsing Error: {0}")]
	UrlParsingError(#[from] ParseError),

	#[error("Serde Error: {error} while attempting to {attempt}")]
	SerdeError {
                error: serde_json::Error,
                attempt: String,
        },

	#[error("Invalid Parameter: {0}")]
	InvalidParameter(String),

	#[error("Client Error: {0}")]
	ClientError(String),
}
