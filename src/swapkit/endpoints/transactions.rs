use anyhow::Result;

use crate::api_get_transaction_details;
use crate::Swapkit;

impl Swapkit {
	/// Requests the details of a transaction by hash.
	///
	/// **This endpoint returns nothing useful and is not modelled.** It calls
	/// `{base_url}apiusage/v2/txn`, which — like every other `api.thorswap.net` path — answered
	/// HTTP 404 to an unauthenticated probe on 2026-09-24. The response body is discarded and the
	/// method returns `Ok(())` on any successful HTTP exchange. Treat it as a placeholder until the
	/// `SwapKit` v3 migration replaces it with `POST /track` (`https://api.swapkit.dev/docs/json`).
	///
	/// # Errors
	/// * [`APIError::InvalidHeaderValue`](crate::APIError::InvalidHeaderValue) — the configured
	///   referer, API key or referrer is not a legal HTTP header value.
	/// * [`APIError::ClientError`](crate::APIError::ClientError) — the `reqwest` client could not
	///   be built.
	/// * [`APIError::ReqwestError`](crate::APIError::ReqwestError) — the request failed, or the
	///   response body could not be read.
	pub async fn get_transaction_details(&mut self, tx_hash: &str) -> Result<()> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		api_get_transaction_details(self.get_config().get_base_url(), self.get_headers()?, tx_hash).await
	}
}
