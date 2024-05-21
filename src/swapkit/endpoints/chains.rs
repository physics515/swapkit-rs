use anyhow::Result;
use chrono::Utc;

use crate::Swapkit;
use crate::{api_get_chains_with_details, api_get_supported_chains, ChainsWithDetails, SupportedChains};

impl Swapkit {
	/// Retrieve a list of all the chains the API supports.
	///
	/// # Returns JSON Equivalent
	/// ```json
	/// {
	///      "ETH": "1",
	///      "AVAX": "43114",
	///      "THOR": "thorchain-mainnet-v1",
	///      "BTC": "bitcoin",
	///      "LTC": "litecoin",
	///      "BNB": "Binance-Chain-Tigris",
	///      "BSC": "56",
	///      "BCH": "bitcoincash",
	///      "GAIA": "cosmoshub-4",
	///      "DOGE": "dogecoin"
	/// }
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
	/// let supported_chains = swapkit.get_supported_chains().await.unwrap();
	///
	/// assert_ne!(supported_chains.get_chains().len(), 0);
	/// # });
	/// ```
	///
	/// # Errors
	/// todo
	pub async fn get_supported_chains(&mut self) -> Result<SupportedChains> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_supported_chains(self.get_config().get_base_url(), self.get_headers()).await
	}

	/// Retrieve a list of all the chains the API supports with details.
	///
	/// # Returns JSON Equivalent
	/// ```json
	/// {
	///     "chain": "AVAX",
	///     "chainId": 43114,
	///     "displayName": "Avalanche",
	///     "symbol": "AVAX",
	///     "logo": "https://static.thorswap.net/token-list/images/avax.avax.png",
	///     "providers": [
	///         "THORCHAIN",
	///         "PANGOLIN"
	///     ],
	///     "status": "active",
	///     "category": "evm",
	///     "evm": true,
	///     "mainnet": true,
	///     "defaultDecimals": 18,
	///     "averageBlockTime": 3000,
	///     "confirmationsRequired": 1,
	///     "gasRate": null,
	///     "gasAsset": {
	///         "chain": "AVAX",
	///         "symbol": "AVAX",
	///         "identifier": "AVAX.AVAX",
	///         "decimals": 18
	///      },
	/// }
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
	/// let chains_with_details = swapkit.get_chains_with_details().await.unwrap();
	///
	/// assert_ne!(chains_with_details.get_chains().len(), 0);
	/// # });
	/// ```
	///
	/// # Errors
	/// todo
	pub async fn get_chains_with_details(&mut self) -> Result<ChainsWithDetails> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_chains_with_details(self.get_config().get_base_url(), self.get_headers()).await
	}
}
