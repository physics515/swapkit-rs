use anyhow::{bail, Result};
use reqwest::header::HeaderMap;
use reqwest::Method;

use crate::{APIError, BorrowQuote, RequestABorrowQuoteParams};

#[allow(clippy::module_name_repetitions)]
pub async fn api_get_request_a_borrow_quote(base_url: &str, headers: HeaderMap, parameters: RequestABorrowQuoteParams) -> Result<BorrowQuote> {
	let client = match reqwest::Client::builder().build() {
		Ok(client) => client,
		Err(e) => bail!(APIError::ClientError(e.to_string())),
	};

	let endpoint = format!("{base_url}aggregator/lending/borrow");

	let params = vec![("assetIn", parameters.asset_in), ("assetOut", parameters.asset_out), ("slippage", parameters.slippage), ("amount", parameters.amount), ("senderAddress", parameters.sender_address), ("recipientAddress", parameters.recipient_address)];

	let endpoint = match reqwest::Url::parse_with_params(&endpoint, &params) {
		Ok(endpoint) => endpoint.to_string(),
		Err(e) => bail!(APIError::UrlParsingError(e)),
	};

	let response = match client.request(Method::GET, &endpoint).headers(headers).send().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let response = match response.text().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let Ok(response) = serde_json::from_str::<BorrowQuote>(&response) else {
		let res: serde_json::Value = match serde_json::from_str(&response) {
			Ok(res) => res,
			Err(e) => bail!(APIError::SerdeError{error: e, attempt: response}),
		};
		let Some(error) = res.get("message") else { bail!(APIError::ClientError("No error message in response".to_string())) };
		bail!(APIError::ClientError(error.to_string()))
	};

	Ok(response)
}
