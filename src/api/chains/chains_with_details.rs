use anyhow::{bail, Result};
use reqwest::header::HeaderMap;
use reqwest::Method;

use crate::{APIError, ChainsWithDetails};

#[allow(clippy::module_name_repetitions)]
pub async fn api_get_chains_with_details(base_url: &str, headers: HeaderMap) -> Result<ChainsWithDetails> {
	let client = match reqwest::Client::builder().build() {
		Ok(client) => client,
		Err(e) => bail!(APIError::ClientError(e.to_string())),
	};

	let endpoint = format!("{base_url}tokenlist/utils/chains/details");

	let response = match client.request(Method::GET, &endpoint).headers(headers).send().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let response = match response.text().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let chains_with_details: ChainsWithDetails = match serde_json::from_str(&response) {
		Ok(chains_with_details) => chains_with_details,
		Err(e) => bail!(APIError::SerdeError{error: e, attempt: response}),
	};

	Ok(chains_with_details)
}
