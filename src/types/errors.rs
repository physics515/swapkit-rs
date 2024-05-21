use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug)]
pub enum APIError {
	#[error("Reqwest Error: {0}")]
	ReqwestError(#[from] reqwest::Error),

	#[error("URL Parsing Error: {0}")]
	UrlParsingError(#[from] ParseError),

	#[error("Serde Error: {0}")]
	SerdeError(#[from] serde_json::Error),

	#[error("Invalid Parameter: {0}")]
	InvalidParameter(String),

	#[error("Client Error: {0}")]
	ClientError(String),
}
