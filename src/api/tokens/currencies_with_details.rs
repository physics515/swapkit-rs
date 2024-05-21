use anyhow::{bail, Result};
use reqwest::header::HeaderMap;
use reqwest::Method;

use crate::{APIError, CurrenciesWithDetails};

#[allow(clippy::module_name_repetitions)]
pub async fn api_get_currencies_with_details(base_url: &str, headers: HeaderMap) -> Result<CurrenciesWithDetails> {
	let client = match reqwest::Client::builder().build() {
		Ok(client) => client,
		Err(e) => bail!(APIError::ClientError(e.to_string())),
	};

	let endpoint = format!("{base_url}tokenlist/utils/currencies/details");

	let response = match client.request(Method::GET, &endpoint).headers(headers).send().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let response = match response.text().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let currencies_with_details: CurrenciesWithDetails = match serde_json::from_str(&response) {
		Ok(currencies_with_details) => currencies_with_details,
		Err(e) => bail!(APIError::SerdeError(e)),
	};

	Ok(currencies_with_details)
}
