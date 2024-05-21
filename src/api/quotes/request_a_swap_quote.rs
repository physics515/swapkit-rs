use anyhow::{bail, Result};
use reqwest::header::HeaderMap;
use reqwest::Method;

use crate::{APIError, Quote, RequestASwapQuoteParams};

#[allow(clippy::module_name_repetitions)]
pub async fn api_get_request_a_swap_quote(base_url: &str, headers: HeaderMap, parameters: RequestASwapQuoteParams) -> Result<Quote> {
	let client = match reqwest::Client::builder().build() {
		Ok(client) => client,
		Err(e) => bail!(APIError::ClientError(e.to_string())),
	};

	let endpoint = format!("{base_url}aggregator/tokens/quote");

	let mut params = vec![("sellAsset", parameters.sell_asset), ("buyAsset", parameters.buy_asset), ("sellAmount", parameters.sell_amount), ("senderAddress", parameters.sender_address), ("recipientAddress", parameters.recipient_address)];

	if parameters.affiliate_address.is_some() {
		params.push(("affiliateAddress", parameters.affiliate_address.unwrap()));
	}

	if parameters.affiliate_basis_points.is_some() {
		params.push(("affiliateBasisPoints", parameters.affiliate_basis_points.unwrap()));
	}

	if parameters.is_affiliate_fee_flat.is_some() {
		params.push(("isAffiliateFeeFlat", parameters.is_affiliate_fee_flat.unwrap().to_string()));
	}

	if parameters.slippage.is_some() {
		params.push(("slippage", parameters.slippage.unwrap()));
	}

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

	println!("{response}");

	let response: Quote = match serde_json::from_str(&response) {
		Ok(response) => response,
		Err(e) => bail!(APIError::SerdeError(e)),
	};

	Ok(response)
}
