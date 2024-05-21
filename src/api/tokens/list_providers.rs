use anyhow::{bail, Result};
use reqwest::header::HeaderMap;
use reqwest::Method;

use crate::{APIError, Provider};

#[allow(clippy::module_name_repetitions)]
pub async fn api_get_token_providers(base_url: &str, headers: HeaderMap) -> Result<Vec<Provider>> {
	let client = match reqwest::Client::builder().build() {
		Ok(client) => client,
		Err(e) => bail!(APIError::ClientError(e.to_string())),
	};

	let endpoint = format!("{base_url}tokenlist/providers");

	let response = match client.request(Method::GET, &endpoint).headers(headers).send().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let response = match response.text().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let providers: Vec<Provider> = match serde_json::from_str(&response) {
		Ok(providers) => providers,
		Err(e) => bail!(APIError::SerdeError(e)),
	};

	Ok(providers)
}
