use anyhow::{bail, Result};
use reqwest::header::HeaderMap;
use reqwest::Method;

use crate::{APIError, SupportedChains};

#[allow(clippy::module_name_repetitions)]
pub async fn api_get_supported_chains(base_url: &str, headers: HeaderMap) -> Result<SupportedChains> {
	let client = match reqwest::Client::builder().build() {
		Ok(client) => client,
		Err(e) => bail!(APIError::ClientError(e.to_string())),
	};

	let endpoint = format!("{base_url}aggregator/chains");

	let response = match client.request(Method::GET, &endpoint).headers(headers).send().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let response = match response.text().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let supported_chains: SupportedChains = match serde_json::from_str(&response) {
		Ok(supported_chains) => supported_chains,
		Err(e) => bail!(APIError::SerdeError(e)),
	};

	Ok(supported_chains)
}
