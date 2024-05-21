use anyhow::Result;
use chrono::Utc;

use crate::api_get_transation_details;
use crate::Swapkit;

impl Swapkit {
	/// # Errors
	/// API not responding
	pub async fn get_transation_details(&mut self, tx_hash: &str) -> Result<()> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_transation_details(self.get_config().get_base_url(), self.get_headers(), tx_hash).await
	}
}
