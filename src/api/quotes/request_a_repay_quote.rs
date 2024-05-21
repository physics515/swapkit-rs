use anyhow::{bail, Result};
use reqwest::header::HeaderMap;
use reqwest::Method;

use crate::{APIError, RepayQuote, RequestARepayQuoteParams};

#[allow(clippy::module_name_repetitions)]
pub async fn api_get_request_a_repay_quote(base_url: &str, headers: HeaderMap, parameters: RequestARepayQuoteParams) -> Result<RepayQuote> {
	let client = match reqwest::Client::builder().build() {
		Ok(client) => client,
		Err(e) => bail!(APIError::ClientError(e.to_string())),
	};

	let endpoint = format!("{base_url}aggregator/lending/repay");

	let params = vec![("repayAsset", parameters.repay_asset), ("collateralAsset", parameters.collateral_asset), ("amountPercentage", parameters.amount_percentage), ("senderAddress", parameters.sender_address), ("collateralAddress", parameters.collateral_address), ("affiliateBasisPoints", parameters.affiliate_basis_points), ("affiliateAddress", parameters.affiliate_address)];

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

	let response = match serde_json::from_str::<RepayQuote>(&response) {
		Ok(response) => response,
		Err(e) => bail!(APIError::SerdeError(e)),
	};

	Ok(response)
}
