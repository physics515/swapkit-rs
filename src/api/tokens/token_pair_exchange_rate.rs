use anyhow::{bail, Result};
use reqwest::header::HeaderMap;
use reqwest::Method;

use crate::{APIError, ExchangeRate};

#[allow(clippy::module_name_repetitions)]
pub async fn api_get_token_pair_exchange_rate(base_url: &str, headers: HeaderMap, provider: &str, buy_asset: &str, sell_asset: &str) -> Result<ExchangeRate> {
	let client = match reqwest::Client::builder().build() {
		Ok(client) => client,
		Err(e) => bail!(APIError::ClientError(e.to_string())),
	};

	let endpoint = format!("{base_url}aggregator/tokens/quote/pairPrice?provider={provider}&buyAsset={buy_asset}&sellAsset={sell_asset}");

	let response = match client.request(Method::GET, &endpoint).headers(headers).send().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let response = match response.text().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let exchange_rate: ExchangeRate = match serde_json::from_str(&response) {
		Ok(exchange_rate) => exchange_rate,
		Err(e) => bail!(APIError::SerdeError{error: e, attempt: response}),
	};

	Ok(exchange_rate)
}
