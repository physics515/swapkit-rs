use chrono::Duration;
use chrono::{DateTime, Utc};
pub use config::*;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::APIError;

mod config;
mod endpoints;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Swapkit {
	config: Configuration,
	last_call: DateTime<Utc>,
}

impl Swapkit {
	#[must_use]
	pub fn new(config: Configuration) -> Self {
		Self { config, last_call: Utc::now() }
	}

	/// Builds the header map that every request carries.
	///
	/// # Errors
	/// Returns [`APIError::InvalidHeaderValue`] when the configured referer, API key or referrer
	/// contains a byte that is not legal in an HTTP header value — anything outside visible ASCII
	/// plus space and tab. A stray newline in a `.env` file is the usual cause.
	fn get_headers(&self) -> Result<HeaderMap, APIError> {
		let mut headers = HeaderMap::new();

		for (name, value) in [("Referer", self.config.get_referer()), ("X-API-KEY", self.config.get_x_api_key()), ("referrer", self.config.get_referrer())] {
			let value = HeaderValue::from_str(value).map_err(|source| APIError::InvalidHeaderValue { header: name, source })?;
			headers.insert(name, value);
		}

		Ok(headers)
	}

	#[must_use]
	pub const fn get_config(&self) -> &Configuration {
		&self.config
	}

	pub fn set_config(&mut self, config: Configuration) {
		self.config = config;
	}

	#[must_use]
	pub const fn get_last_call(&self) -> &DateTime<Utc> {
		&self.last_call
	}

	pub const fn set_last_call(&mut self, last_call: DateTime<Utc>) {
		self.last_call = last_call;
	}

	/// Returns a future timestamp of when it is ok to call the Swapkit API again
	fn ok_to_call_at(&self) -> DateTime<Utc> {
		let rate_limit: i64 = i64::try_from(self.config.get_rate_limit_ms()).unwrap_or(1000);
		let rate_limit = Duration::try_milliseconds(rate_limit).unwrap_or_else(Duration::zero);
		self.last_call.checked_add_signed(rate_limit).unwrap_or_else(Utc::now)
	}

	/// Sleeps until it is ok to call the Swapkit API again
	async fn sleep_until_ok_to_call(&mut self) {
		let now = Utc::now();
		let ok_to_call_at = self.ok_to_call_at();
		if now < ok_to_call_at {
			let sleep_duration = ok_to_call_at - now;
			tokio::time::sleep(sleep_duration.to_std().unwrap()).await;
		}
		self.set_last_call(Utc::now());
	}
}

#[cfg(test)]
mod tests {
	//! Live API tests.
	//!
	//! **Every test below calls the real `SwapKit` API with a real key.** Without
	//! `SWAPKIT_REFERER` and `SWAPKIT_X_API_KEY` each one prints why it did not run and returns —
	//! it never fails for a missing credential, because a red test here would be indistinguishable
	//! from "the network was down". Deterministic coverage lives in `tests/deserialization.rs`,
	//! which needs neither.
	//!
	//! Each test builds its own [`Swapkit`], and the 1 req/s limiter is per client, so run the live
	//! suite with `cargo test --lib -- --test-threads=1` to keep the aggregate request rate inside
	//! what upstream throttles to.

	use crate::{skip_without_credentials, RequestABorrowQuoteParams, RequestARepayQuoteParams, RequestASwapQuoteParams};

	#[tokio::test]
	async fn supported_chains() {
		skip_without_credentials!(swapkit);
		let supported_chains = swapkit.get_supported_chains().await.unwrap();
		assert_ne!(supported_chains.get_chains().len(), 0);
	}

	#[tokio::test]
	async fn chains_with_details() {
		skip_without_credentials!(swapkit);
		let chains_with_details = swapkit.get_chains_with_details().await.unwrap();
		assert_ne!(chains_with_details.get_chains().len(), 0);
	}

	#[tokio::test]
	async fn gas_prices() {
		skip_without_credentials!(swapkit);
		let gas_prices = swapkit.get_gas_prices().await.unwrap();
		// Deliberately not an exact count: the number of chains is upstream's business, and
		// asserting it made this test fail whenever a chain was added or removed.
		assert_ne!(gas_prices.get_gas_prices().len(), 0);
	}

	#[tokio::test]
	async fn available_assets_for_pool() {
		skip_without_credentials!(swapkit);
		swapkit.get_available_assets_for_pool("BTC.BTC").await.unwrap();
	}

	#[tokio::test]
	async fn available_lending_assets() {
		skip_without_credentials!(swapkit);
		let lending_assets = swapkit.get_available_lending_assets().await.unwrap();
		assert_ne!(lending_assets.len(), 0);
	}

	#[tokio::test]
	async fn loans() {
		skip_without_credentials!(swapkit);
		let loan = swapkit.get_loans("bc1qzafz3f0h90u7n9j862uupaf5hpeydmhvpnzwzz", "BTC.BTC").await.unwrap();
		assert_eq!(loan.get_asset(), "BTC.BTC");
	}

	#[tokio::test]
	async fn supported_providers() {
		skip_without_credentials!(swapkit);
		let supported_providers = swapkit.get_supported_providers().await.unwrap();
		assert_ne!(supported_providers.get_providers().len(), 0);
	}

	#[tokio::test]
	async fn request_a_swap_quote() {
		skip_without_credentials!(swapkit);
		let parameters = RequestASwapQuoteParams {
			sell_asset: "BTC.BTC".to_string(),
			buy_asset: "ETH.AAVE-0X7FC66500C84A76AD7E9C93437BFC5AC33E2DDAE9".to_string(),
			sell_amount: "1".to_string(),
			sender_address: "bc1q7cyrfmck2ffu2ud3rn5l5a8yv6f0chkp0zpemf".to_string(),
			recipient_address: "0x2bD63111C794B29809f5F7d85aD2Ba67DB7C5CA5".to_string(),
			affiliate_address: None,
			affiliate_basis_points: None,
			is_affiliate_fee_flat: None,
			slippage: None,
		};
		let quote = swapkit.get_request_a_swap_quote(parameters).await.unwrap();
		assert_ne!(quote.get_quote_id().len(), 0);
	}

	#[tokio::test]
	async fn request_a_borrow_quote() {
		skip_without_credentials!(swapkit);
		let parameters = RequestABorrowQuoteParams {
			asset_in: "BTC.BTC".to_string(),
			asset_out: "BTC.BTC".to_string(),
			slippage: "0.5".to_string(),
			amount: "1".to_string(),
			sender_address: "bc1q7cyrfmck2ffu2ud3rn5l5a8yv6f0chkp0zpemf".to_string(),
			recipient_address: "bc1q7cyrfmck2ffu2ud3rn5l5a8yv6f0chkp0zpemf".to_string(),
		};
		swapkit.get_request_a_borrow_quote(parameters).await.unwrap();
	}

	#[tokio::test]
	async fn request_a_repay_quote() {
		skip_without_credentials!(swapkit);
		let parameters = RequestARepayQuoteParams {
			repay_asset: "BTC.BTC".to_string(),
			collateral_asset: "BTC.BTC".to_string(),
			amount_percentage: "0.5".to_string(),
			sender_address: "bc1qzafz3f0h90u7n9j862uupaf5hpeydmhvpnzwzz".to_string(),
			collateral_address: "bc1qzafz3f0h90u7n9j862uupaf5hpeydmhvpnzwzz".to_string(),
			affiliate_basis_points: String::new(),
			affiliate_address: String::new(),
		};
		swapkit.get_request_a_repay_quote(parameters).await.unwrap();
	}

	#[tokio::test]
	async fn minimum_amount_to_send_with_details() {
		skip_without_credentials!(swapkit);
		let minimum = swapkit.get_minimum_amount_to_send_with_details("BTC.BTC", "ETH.ETH").await.unwrap();
		assert_eq!(minimum.get_asset(), "BTC.BTC");
	}

	#[tokio::test]
	async fn gas_history() {
		skip_without_credentials!(swapkit);
		let gas_history = swapkit.get_gas_history("bitcoin").await.unwrap();
		assert_eq!(gas_history.get_chain_id(), "bitcoin");
	}

	#[tokio::test]
	async fn gas_rates() {
		skip_without_credentials!(swapkit);
		let gas_rates = swapkit.get_gas_rates().await.unwrap();
		assert_ne!(gas_rates.len(), 0);
	}

	#[tokio::test]
	async fn currencies_with_details() {
		skip_without_credentials!(swapkit);
		let currencies_with_details = swapkit.get_currencies_with_details().await.unwrap();
		// Deliberately not an exact count: the currency list is upstream's business.
		assert_ne!(currencies_with_details.get_currencies().len(), 0);
	}

	#[tokio::test]
	async fn token_pair_exchange_rate() {
		skip_without_credentials!(swapkit);
		let exchange_rate = swapkit.get_token_pair_exchange_rate("THORCHAIN", "DOGE.DOGE", "THOR.RUNE").await.unwrap();
		assert_ne!(exchange_rate.get_price(), &rust_decimal::Decimal::ZERO);
	}

	#[tokio::test]
	async fn cached_prices() {
		skip_without_credentials!(swapkit);
		let tokens = vec!["ETH.unshETH-0x0Ae38f7E10A43B5b2fB064B42a2f4514cbA909ef".to_string(), "BSC.DOT-0X7083609FCE4D1D8DC0C979AAB8C869EA2C873402".to_string(), "BTC.BTC".to_string(), "ETH.ARB-0XB50721BCF8D664C30412CFBC6CF7A15145234AD1".to_string(), "AVAX.EURC-0xC891EB4cbdEFf6e073e859e987815Ed1505c2ACD".to_string()];
		let requested = tokens.len();
		let cached_prices = swapkit.get_cached_prices(tokens, Some(true), Some(true), Some(true)).await.unwrap();
		assert_eq!(cached_prices.len(), requested);
	}

	#[tokio::test]
	async fn token_providers() {
		skip_without_credentials!(swapkit);
		let providers = swapkit.get_token_providers().await.unwrap();
		assert_ne!(providers.len(), 0);
	}

	#[tokio::test]
	async fn transaction_details() {
		skip_without_credentials!(swapkit);
		swapkit.get_transaction_details("0x0000000000000000000000000000000000000000000000000000000000000000").await.unwrap();
	}
}
