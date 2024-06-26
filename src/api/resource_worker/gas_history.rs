use anyhow::{bail, Result};
use reqwest::header::HeaderMap;
use reqwest::Method;

use crate::{APIError, GasHistory};

#[allow(clippy::module_name_repetitions)]
pub async fn api_get_gas_history(base_url: &str, headers: HeaderMap, chain_id: &str) -> Result<GasHistory> {
	let client = match reqwest::Client::builder().build() {
		Ok(client) => client,
		Err(e) => bail!(APIError::ClientError(e.to_string())),
	};

	let endpoint = format!("{base_url}resource-worker/gasHistory/get?chainId={chain_id}");

	let response = match client.request(Method::GET, &endpoint).headers(headers).send().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let response = match response.text().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let gas_history: GasHistory = match serde_json::from_str(&response) {
		Ok(gas_history) => gas_history,
		Err(e) => bail!(APIError::SerdeError{error: e, attempt: response}),
	};

	Ok(gas_history)
}
