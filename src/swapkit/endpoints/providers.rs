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
	/// ```rust
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
	/// todo
	pub async fn get_supported_providers(&mut self) -> Result<SupportedProviders> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_supported_providers(self.get_config().get_base_url(), self.get_headers()).await
	}
}
