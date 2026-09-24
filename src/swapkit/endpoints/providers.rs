use anyhow::Result;
use chrono::Utc;

use crate::Swapkit;
use crate::{api_get_supported_providers, SupportedProviders};

impl Swapkit {
	/// Retrieve a list of all the providers the API supports.
	///
	/// # Returns JSON Equivalent
	/// ```json
	/// [
	///  "Thorchain"...
	/// ]
	/// ```
	///
	/// # Example
	///
	/// ```rust,no_run
	/// use swapkit_rs::Swapkit;
	/// use dotenv;
	/// use swapkit_rs::Configuration;
	///
	/// # tokio_test::block_on(async {
	/// let swapkit_config = Configuration::new(None, dotenv::var("SWAPKIT_REFERER").unwrap().as_str(), dotenv::var("SWAPKIT_X_API_KEY").unwrap().as_str());
	/// let mut swapkit = Swapkit::new(swapkit_config);
	///
	/// let supported_providers = swapkit.get_supported_providers().await.unwrap();
	///
	/// assert_ne!(supported_providers.get_providers().len(), 0);
	/// # });
	/// ```
	///
	/// # Errors
	/// * [`APIError::InvalidHeaderValue`](crate::APIError::InvalidHeaderValue) — the configured
	///   referer, API key or referrer is not a legal HTTP header value.
	/// * [`APIError::ClientError`](crate::APIError::ClientError) — the `reqwest` client could not be
	///   built.
	/// * [`APIError::ReqwestError`](crate::APIError::ReqwestError) — the request failed, or the
	///   response body could not be read.
	/// * [`APIError::SerdeError`](crate::APIError::SerdeError) — the response body did not match the
	///   modelled type. The variant carries the raw body, so a shape change upstream is
	///   diagnosable from the error alone.
	pub async fn get_supported_providers(&mut self) -> Result<SupportedProviders> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_supported_providers(self.get_config().get_base_url(), self.get_headers()?).await
	}
}
