use anyhow::{bail, Result};
use reqwest::header::HeaderMap;
use reqwest::Method;

use crate::{APIError, GasPrice};

#[allow(clippy::module_name_repetitions)]
pub async fn api_get_gas_rates(base_url: &str, headers: HeaderMap) -> Result<Vec<GasPrice>> {
	let client = match reqwest::Client::builder().build() {
		Ok(client) => client,
		Err(e) => bail!(APIError::ClientError(e.to_string())),
	};

	let endpoint = format!("{base_url}resource-worker/gasPrice/getAll");

	let response = match client.request(Method::GET, &endpoint).headers(headers).send().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let response = match response.text().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let gas_rates: Vec<GasPrice> = match serde_json::from_str(&response) {
		Ok(gas_rates) => gas_rates,
		Err(e) => bail!(APIError::SerdeError{error: e, attempt: response}),
	};

	Ok(gas_rates)
}
