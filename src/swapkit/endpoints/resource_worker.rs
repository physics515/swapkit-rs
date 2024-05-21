use anyhow::Result;
use chrono::Utc;

use crate::Swapkit;
use crate::{api_get_gas_history, api_get_gas_rates, api_get_minimum_amount_to_send_with_details, GasHistory, GasPrice, MinimumAmountToSendWithDetails};

impl Swapkit {
	/// Retrieve the minimum swap amount with details.
	///
	/// # Arguments
	/// From: The asset to send.
	/// To: The asset to receive.
	///
	/// # Returns JSON Equivalent
	/// ```json
	/// {
	///     "result": "0.00077264",
	///     "asset": "BTC.BTC",
	///     "fromInboundAddress": InboundAddress,
	///     "toInboundAddress": InboundAddress,
	///     "toPool": Pool,
	///     "fromPool": Pool
	/// }
	/// ```
	///
	/// # Example
	///
	/// ```rust
	/// use swapkit_rs::Swapkit;
	/// use dotenv;
	/// use swapkit_rs::Configuration;
	/// use rust_decimal::Decimal;
	///
	/// # tokio_test::block_on(async {
	/// let swapkit_config = Configuration::new(None, dotenv::var("SWAPKIT_REFERER").unwrap().as_str(), dotenv::var("SWAPKIT_X_API_KEY").unwrap().as_str());
	/// let mut swapkit = Swapkit::new(swapkit_config);
	///
	/// let minimum_amount_to_send_with_details = swapkit.get_minimum_amount_to_send_with_details("BTC.BTC", "ETH.ETH").await.unwrap();
	///
	/// assert_ne!(minimum_amount_to_send_with_details.get_result(), &Decimal::ZERO);
	/// # });
	/// ```
	///
	/// # Errors
	/// todo
	pub async fn get_minimum_amount_to_send_with_details(&mut self, from: &str, to: &str) -> Result<MinimumAmountToSendWithDetails> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_minimum_amount_to_send_with_details(self.get_config().get_base_url(), self.get_headers(), from, to).await
	}

	/// Retrieve the gas history for a given chain.
	///
	/// # Arguments
	/// Chain ID: The chain ID to retrieve gas history for.
	///
	/// # Returns JSON Equivalent
	/// ```json
	/// {
	///     "lastTimestamp": 1688396693338,
	///     "chainId": "43114",
	///     "unitName": "wei",
	///     "history": [
	///         {
	///             "value": 25,
	///             "timestamp": 1688394892940
	///         }
	///     ],
	///     "average24h": 25.01171800476548,
	///     "average7d": 25.46623886101168
	/// }
	/// ```
	///
	/// # Example
	///
	/// ```rust
	/// use swapkit_rs::Swapkit;
	/// use dotenv;
	/// use swapkit_rs::Configuration;
	/// use rust_decimal::Decimal;
	///
	/// # tokio_test::block_on(async {
	/// let swapkit_config = Configuration::new(None, dotenv::var("SWAPKIT_REFERER").unwrap().as_str(), dotenv::var("SWAPKIT_X_API_KEY").unwrap().as_str());
	/// let mut swapkit = Swapkit::new(swapkit_config);
	///
	/// let gas_history = swapkit.get_gas_history("43114").await.unwrap();
	///
	/// assert_ne!(gas_history.get_last_timestamp(), &Decimal::ZERO);
	/// # });
	/// ```
	///
	/// # Errors
	/// todo
	pub async fn get_gas_history(&mut self, chain_id: &str) -> Result<GasHistory> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_gas_history(self.get_config().get_base_url(), self.get_headers(), chain_id).await
	}

	/// Retrieve the current gas rates.
	///
	/// # Returns JSON Equivalent
	/// ```json
	/// [
	///     {
	///         "asset": "THOR.RUNE",
	///         "units": "tor",
	///         "gas": 2000000,
	///         "chainId": "thorchain-mainnet-v1",
	///         "gasAsset": 0.02
	///     }
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
	/// let gas_rates = swapkit.get_gas_rates().await.unwrap();
	///
	/// assert_ne!(gas_rates.len(), 0);
	/// # });
	/// ```
	///
	/// # Errors
	/// todo
	pub async fn get_gas_rates(&mut self) -> Result<Vec<GasPrice>> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_gas_rates(self.get_config().get_base_url(), self.get_headers()).await
	}
}
