use anyhow::{bail, Result};
use reqwest::header::HeaderMap;
use reqwest::Method;

use crate::{APIError, SupportedProviders};

#[allow(clippy::module_name_repetitions)]
pub async fn api_get_supported_providers(base_url: &str, headers: HeaderMap) -> Result<SupportedProviders> {
	let client = match reqwest::Client::builder().build() {
		Ok(client) => client,
		Err(e) => bail!(APIError::ClientError(e.to_string())),
	};

	let endpoint = format!("{base_url}aggregator/providers/supportedProviders");

	let response = match client.request(Method::GET, &endpoint).headers(headers).send().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let response = match response.text().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let supported_providers: SupportedProviders = match serde_json::from_str(&response) {
		Ok(supported_providers) => supported_providers,
		Err(e) => bail!(APIError::SerdeError(e)),
	};

	Ok(supported_providers)
}
