use anyhow::{bail, Result};
use reqwest::header::HeaderMap;
use reqwest::Method;

use crate::{APIError, Loan};

#[allow(clippy::module_name_repetitions)]
pub async fn api_get_loans(base_url: &str, headers: HeaderMap, address: &str, asset: &str) -> Result<Loan> {
	let client = match reqwest::Client::builder().build() {
		Ok(client) => client,
		Err(e) => bail!(APIError::ClientError(e.to_string())),
	};

	let endpoint = format!("{base_url}aggregator/lending/loans?address={address}&asset={asset}");

	let response = match client.request(Method::GET, &endpoint).headers(headers).send().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let response = match response.text().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let loan: Loan = match serde_json::from_str(&response) {
		Ok(loan) => loan,
		Err(e) => bail!(APIError::SerdeError{error: e, attempt: response}),
	};

	Ok(loan)
}
