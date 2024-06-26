use anyhow::{bail, Result};
use reqwest::header::HeaderMap;
use reqwest::Method;

use crate::{APIError, LendingAsset};

#[allow(clippy::module_name_repetitions)]
pub async fn api_get_available_lending_assets(base_url: &str, headers: HeaderMap) -> Result<Vec<LendingAsset>> {
	let client = match reqwest::Client::builder().build() {
		Ok(client) => client,
		Err(e) => bail!(APIError::ClientError(e.to_string())),
	};

	let endpoint = format!("{base_url}aggregator/lending/assets");

	let response = match client.request(Method::GET, &endpoint).headers(headers).send().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let response = match response.text().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let response: Vec<LendingAsset> = match serde_json::from_str(&response) {
		Ok(response) => response,
		Err(e) => bail!(APIError::SerdeError{error: e, attempt: response}),
	};

	Ok(response)
}
