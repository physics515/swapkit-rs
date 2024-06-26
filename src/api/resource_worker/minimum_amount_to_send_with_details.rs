use anyhow::{bail, Result};
use reqwest::header::HeaderMap;
use reqwest::Method;

use crate::{APIError, MinimumAmountToSendWithDetails};

#[allow(clippy::module_name_repetitions)]
pub async fn api_get_minimum_amount_to_send_with_details(base_url: &str, headers: HeaderMap, from: &str, to: &str) -> Result<MinimumAmountToSendWithDetails> {
	let client = match reqwest::Client::builder().build() {
		Ok(client) => client,
		Err(e) => bail!(APIError::ClientError(e.to_string())),
	};

	let endpoint = format!("{base_url}resource-worker/minAmount/detail?from={from}&to={to}");

	let response = match client.request(Method::GET, &endpoint).headers(headers).send().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let response = match response.text().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let minimum_amount_to_send_with_details: MinimumAmountToSendWithDetails = match serde_json::from_str(&response) {
		Ok(minimum_amount_to_send_with_details) => minimum_amount_to_send_with_details,
		Err(e) => bail!(APIError::SerdeError{error: e, attempt: response}),
	};

	Ok(minimum_amount_to_send_with_details)
}
