use anyhow::{bail, Result};
use reqwest::header::HeaderMap;
use reqwest::Method;

use crate::APIError;

#[allow(clippy::module_name_repetitions)]
pub async fn api_get_transation_details(base_url: &str, headers: HeaderMap, tx_hash: &str) -> Result<()> {
	let client = match reqwest::Client::builder().build() {
		Ok(client) => client,
		Err(e) => bail!(APIError::ClientError(e.to_string())),
	};

	let endpoint = format!("{base_url}apiusage/v2/txn?searchParams={tx_hash}");

	let response = match client.request(Method::GET, &endpoint).headers(headers).send().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	let response = match response.text().await {
		Ok(response) => response,
		Err(e) => bail!(APIError::ReqwestError(e)),
	};

	println!("response: {response}");

	Ok(())
}
