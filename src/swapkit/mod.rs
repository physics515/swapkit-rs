use chrono::Duration;
use chrono::{DateTime, Utc};
pub use config::*;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

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

	fn get_headers(&self) -> HeaderMap {
		let mut headers = HeaderMap::new();
		headers.insert("Referer", HeaderValue::from_str(self.config.get_referer()).unwrap());
		headers.insert("X-API-KEY", HeaderValue::from_str(self.config.get_x_api_key()).unwrap());
		headers.insert("referrer", HeaderValue::from_str(self.config.get_referrer()).unwrap());
		headers
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

	pub fn set_last_call(&mut self, last_call: DateTime<Utc>) {
		self.last_call = last_call;
	}

	/// Returns a future timestamp of when it is ok to call the Swapkit API again
	fn ok_to_call_at(&self) -> DateTime<Utc> {
		let rate_limit: i64 = i64::try_from(self.config.get_rate_limit_ms()).map_or(1000, |rate_limit| rate_limit);
		let rate_limit = Duration::try_milliseconds(rate_limit).map_or_else(Duration::zero, |rate_limit| rate_limit);
		self.last_call.checked_add_signed(rate_limit).map_or_else(Utc::now, |res| res)
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
	use serde_json::json;

	use crate::test_utils::get_test_swapkit;
	use crate::{RequestABorrowQuoteParams, RequestARepayQuoteParams, RequestASwapQuoteParams};

	#[tokio::test]
	async fn test_all_endpoints() {
		let mut swapkit = get_test_swapkit();

  		// chains
		let supported_chains = swapkit.get_supported_chains().await.unwrap();
		println!("{}", json!(supported_chains));
		assert_ne!(supported_chains.get_chains().len(), 0);

		let chains_with_details = swapkit.get_chains_with_details().await.unwrap();
		println!("{}", json!(chains_with_details));
		assert_ne!(chains_with_details.get_chains().len(), 0);

		// gas
		let gas_prices = swapkit.get_gas_prices().await.unwrap();
		println!("{}", json!(gas_prices).to_string());
		assert_eq!(gas_prices.get_gas_prices().len(), 10);

		// lending
		let _ = swapkit.get_available_assets_for_pool("BTC.BTC").await.unwrap();

		let lending_assets = swapkit.get_available_lending_assets().await.unwrap();
		println!("{}", json!(lending_assets).to_string());
		assert!(!lending_assets.is_empty());

		let loan = swapkit.get_loans("bc1qzafz3f0h90u7n9j862uupaf5hpeydmhvpnzwzz", "BTC.BTC").await.unwrap();
		println!("{}", json!(loan).to_string());
		assert_eq!(loan.get_asset(), "BTC.BTC".to_string());

		// providers
		let supported_providers = swapkit.get_supported_providers().await.unwrap();
		println!("{}", json!(supported_providers));
		assert_ne!(supported_providers.get_providers().len(), 0);

		// quotes
		let request_a_quote_params = RequestASwapQuoteParams {
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
		let quote = swapkit.get_request_a_swap_quote(request_a_quote_params).await.unwrap();
		println!("{}", json!(quote).to_string());

		let request_a_quote_params = RequestABorrowQuoteParams {
			asset_in: "BTC.BTC".to_string(),
			asset_out: "BTC.BTC".to_string(),
			slippage: "0.5".to_string(),
			amount: "1".to_string(),
			sender_address: "bc1q7cyrfmck2ffu2ud3rn5l5a8yv6f0chkp0zpemf".to_string(),
			recipient_address: "bc1q7cyrfmck2ffu2ud3rn5l5a8yv6f0chkp0zpemf".to_string(),
		};
		let borrow_quote = swapkit.get_request_a_borrow_quote(request_a_quote_params).await.unwrap();
		println!("{}", json!(borrow_quote).to_string());

		let request_a_repay_quote_params = RequestARepayQuoteParams {
			repay_asset: "BTC.BTC".to_string(),
			collateral_asset: "BTC.BTC".to_string(),
			amount_percentage: "0.5".to_string(),
			sender_address: "bc1qzafz3f0h90u7n9j862uupaf5hpeydmhvpnzwzz".to_string(),
			collateral_address: "bc1qzafz3f0h90u7n9j862uupaf5hpeydmhvpnzwzz".to_string(),
			affiliate_basis_points: "".to_string(),
			affiliate_address: "".to_string(),
		};
		let repay_quote = swapkit.get_request_a_repay_quote(request_a_repay_quote_params).await.unwrap();
		println!("{}", json!(repay_quote).to_string());

		// resource worker
		let minimum_amount_to_send_with_details = swapkit.get_minimum_amount_to_send_with_details("BTC.BTC", "ETH.ETH").await.unwrap();
		println!("{}", json!(minimum_amount_to_send_with_details).to_string());
		assert_eq!(minimum_amount_to_send_with_details.get_asset(), "BTC.BTC");

 		let gas_history = swapkit.get_gas_history("bitcoin").await.unwrap();
		println!("{}", json!(gas_history).to_string());
		assert_eq!(gas_history.get_chain_id(), "bitcoin");

		let gas_prices = swapkit.get_gas_rates().await.unwrap();
		println!("{}", json!(gas_prices).to_string());
		assert_ne!(gas_prices.len(), 0);

		// tokens
		let currencies_with_details = swapkit.get_currencies_with_details().await.unwrap();
		println!("{}", json!(currencies_with_details).to_string());
		assert_eq!(currencies_with_details.get_currencies().len(), 1286);

		let exchange_rate = swapkit.get_token_pair_exchange_rate("THORCHAIN", "DOGE.DOGE", "THOR.RUNE").await.unwrap();
		println!("{}", json!(exchange_rate).to_string());
		assert_ne!(exchange_rate.get_price(), &rust_decimal::Decimal::ZERO);

		let cached_prices = swapkit.get_cached_prices(vec!["ETH.unshETH-0x0Ae38f7E10A43B5b2fB064B42a2f4514cbA909ef".to_string(), "BSC.DOT-0X7083609FCE4D1D8DC0C979AAB8C869EA2C873402".to_string(), "BTC.BTC".to_string(), "ETH.ARB-0XB50721BCF8D664C30412CFBC6CF7A15145234AD1".to_string(), "AVAX.EURC-0xC891EB4cbdEFf6e073e859e987815Ed1505c2ACD".to_string()], Some(true), Some(true), Some(true)).await.unwrap();
		println!("{}", json!(cached_prices).to_string());
		assert_eq!(cached_prices.len(), 5);

 		let providers = swapkit.get_token_providers().await.unwrap();
		println!("{}", json!(providers).to_string());
		assert_ne!(providers.len(), 0);
	}
}
