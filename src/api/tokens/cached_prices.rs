use anyhow::{bail, Result};
use reqwest::header::HeaderMap;
use reqwest::Method;
use serde_json::json;

use crate::{APIError, CachedPrice, CachedPricesParameters};

#[allow(clippy::module_name_repetitions)]
pub async fn api_get_cached_prices(base_url: &str, mut headers: HeaderMap, parameters: CachedPricesParameters) -> Result<Vec<CachedPrice>> {
	let client = match reqwest::Client::builder().build() {
		Ok(client) => client,
		Err(e) => bail!(APIError::ClientError(e.to_string())),
	};

	let endpoint = format!("{base_url}tokenlist/cached-price");

	headers.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());

	let mut params = vec![("metadata", parameters.metadata.unwrap_or(false).to_string()), ("lookup", parameters.lookup.unwrap_or(false).to_string()), ("sparkline", parameters.sparkline.unwrap_or(false).to_string())];

	for token in parameters.tokens {
		params.push(("tokens", json!(token).to_string()));
	}

	let response = match client.request(Method::POST, &endpoint).headers(headers).form(&params).send().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let response: String = match response.text().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};


	let response: Vec<CachedPrice> = match serde_json::from_str(&response) {
		Ok(response) => response,
		Err(e) => bail!(APIError::SerdeError(e)),
	};

	Ok(response)
}
