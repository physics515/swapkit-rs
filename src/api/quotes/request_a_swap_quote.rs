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

	if let Some(affiliate_address) = parameters.affiliate_address {
		params.push(("affiliateAddress", affiliate_address));
	}

	if let Some(affiliate_basis_points) = parameters.affiliate_basis_points {
		params.push(("affiliateBasisPoints", affiliate_basis_points));
	}

	if let Some(is_affiliate_fee_flat) = parameters.is_affiliate_fee_flat {
		params.push(("isAffiliateFeeFlat", is_affiliate_fee_flat.to_string()));
	}

	if let Some(slippage) = parameters.slippage {
		params.push(("slippage", slippage));
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
		Err(e) => bail!(APIError::SerdeError { error: e, attempt: response }),
	};

	Ok(response)
}
