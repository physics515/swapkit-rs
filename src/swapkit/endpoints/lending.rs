use anyhow::Result;
use chrono::Utc;

use crate::{api_get_available_assets_for_pool, api_get_available_lending_assets, api_get_loans};
use crate::{LendingAsset, Loan, Swapkit};

impl Swapkit {
	/// # Errors
	/// Endpoint not returning a response.
	pub async fn get_available_assets_for_pool(&mut self, asset: &str) -> Result<()> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_available_assets_for_pool(self.get_config().get_base_url(), self.get_headers(), asset).await
	}

	/// Retrieve a list of all the lending assets the API supports.
	///
	/// # Returns JSON Equivalent
	/// ```json
	/// [
	///     {
	///         "asset": "BTC.BTC",
	///         "assetDepthAssetAmount": "1103.58594672",
	///         "runeDepthAssetAmount": "10841279.17868466",
	///         "loanCr": "200",
	///         "loanCollateral": "1931.73698979",
	///         "lendingAvailable": true,
	///         "filledPercentage": "95.31",
	///         "derivedDepthPercentage": "9497",
	///         "ltvPercentage": "50.00"
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
	/// let lending_assets = swapkit.get_available_lending_assets().await.unwrap();
	///
	/// assert_ne!(lending_assets.len(), 0);
	/// # });
	/// ```
	///
	/// # Errors
	/// todo
	pub async fn get_available_lending_assets(&mut self) -> Result<Vec<LendingAsset>> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_available_lending_assets(self.get_config().get_base_url(), self.get_headers()).await
	}

	/// Retrieve a loan for a given address and asset.
	///
	/// # Returns JSON Equivalent
	///
	/// ```json
	/// {
	///     "asset": "BTC.BTC",
	///     "debtCurrent": "1764.0462",
	///     "debtIssued": "1764.0462",
	///     "debtRepaid": "0",
	///     "collateralCurrent": "0.06489696",
	///     "collateralDeposited": "0.06489696",
	///     "collateralWithdrawn": "0",
	///     "lastOpenHeight": 14876835,
	///     "lastRepayHeight": 0,
	///     "ltvPercentage": "39.53",
	///     "owner": "bc1qzafz3f0h90u7n9j862uupaf5hpeydmhvpnzwzz"    
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
	/// let loan = swapkit.get_loans("bc1qzafz3f0h90u7n9j862uupaf5hpeydmhvpnzwzz", "BTC.BTC").await.unwrap();
	///
	/// assert_eq!(loan.get_asset(), "BTC.BTC".to_string());
	/// # });
	/// ```
	///
	/// # Errors
	/// todo
	pub async fn get_loans(&mut self, address: &str, asset: &str) -> Result<Loan> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_loans(self.get_config().get_base_url(), self.get_headers(), address, asset).await
	}
}
