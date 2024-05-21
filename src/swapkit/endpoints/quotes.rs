use anyhow::Result;
use chrono::Utc;

use crate::{api_get_request_a_borrow_quote, api_get_request_a_repay_quote, api_get_request_a_swap_quote, RequestABorrowQuoteParams, RequestARepayQuoteParams, RequestASwapQuoteParams};
use crate::{BorrowQuote, Quote, RepayQuote, Swapkit};

impl Swapkit {
	/// Retrieve a quote for a swap.
	///
	/// # Returns JSON Equivalent
	/// ```json
	/// {
	///     "quoteId": "477981ae-eed8-4162-885b-3af7495fb8fb",
	///     "routes": [
	///         QuoteRoute
	///     ],
	///     "sellAssetAmount": "1"
	/// }
	/// ```
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
	/// let quote = swapkit.get_request_a_swap_quote(RequestASwapQuoteParams {
	///     sell_asset: "BTC.BTC".to_string(),
	///     buy_asset: "ETH.AAVE-0X7FC66500C84A76AD7E9C93437BFC5AC33E2DDAE9".to_string(),
	///     sell_amount: "1".to_string(),
	///     sender_address: "bc1q7cyrfmck2ffu2ud3rn5l5a8yv6f0chkp0zpemf".to_string(),
	///     recipient_address: "0x2bD63111C794B29809f5F7d85aD2Ba67DB7C5CA5".to_string(),
	///     affiliate_address: None,
	///     affiliate_basis_points: None,
	///     is_affiliate_fee_flat: None,
	///     slippage: None,
	/// }).await.unwrap();
	///
	/// assert_ne!(quote.get_quote().len(), 0);
	/// # });
	/// ```
	///
	/// # Errors
	/// todo
	pub async fn get_request_a_swap_quote(&mut self, parameters: RequestASwapQuoteParams) -> Result<Quote> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_request_a_swap_quote(self.get_config().get_base_url(), self.get_headers(), parameters).await
	}

	/// Retrieve a quote for a borrow.
	///
	/// # Returns JSON Equivalent
	/// ```json
	/// {
	///     "inboundAddress": "bc1qpqkqxgln9qwahw76nyp3v9fqyc7zy252yhv2kc",
	///     "inboundConfirmationBlocks": 1,
	///     "inboundConfirmationSeconds": 600,
	///     "outboundDelayBlocks": 58,
	///     "outboundDelaySeconds": 348,
	///     "fees": HashMap<String, Vec<QuoteFee>>,
	///     "expiry": 1715973758,
	///     "warning": "Do not cache this response. Do not send funds after the expiry.",
	///     "notes": "First output should be to inbound_address, second output should be change back to self, third output should be OP_RETURN, limited to 80 bytes. Do not send below the dust threshold. Do not use exotic spend scripts, locks or address formats (P2WSH with Bech32 address format preferred).",
	///     "dustThreshold": "10000",
	///     "memo": "$+:BTC.BTC:bc1q7cyrfmck2ffu2ud3rn5l5a8yv6f0chkp0zpemf:1:t:100",
	///     "expectedAmountOut": "0.49226113",
	///     "expectedCollaterizationRatio": "20000",
	///     "expectedCollateralDeposited": "0.98814537",
	///     "expectedDebtIssued": "32909.635",
	///     "recommendedMinAmountIn": "0.00025588",
	///     "interestRate": 0,
	///     "streamingSwapBlocks": 4,
	///     "streamingSwapSeconds": 24,
	///     "totalOpenLoanSeconds": 972,
	///     "complete": true,
	///     "expectedOutput": "0.49226113",
	///     "expectedOutputMaxSlippage": "0.48981206965174129353",
	///     "expectedOutputUSD": "30886.8363905783448903382",
	///     "expectedOutputMaxSlippageUSD": "30733.1705378889003882495021603092742",
	///     "timeEstimates": QuoteTimeEstimates,
	///     "swaps": Vec<Vec<QuoteSwap>>,
	///     "route": BorrowQuoteRoute,
	///     "amountIn": "1",
	///     "amountOut": "0.49226113",
	///     "amountOutMin": "0.48981206965174129353",
	///     "targetAddress": "bc1qpqkqxgln9qwahw76nyp3v9fqyc7zy252yhv2kc",
	///     "recipientAddress": "bc1q7cyrfmck2ffu2ud3rn5l5a8yv6f0chkp0zpemf",
	///     "assetIn": "BTC.BTC",
	///     "assetOut": "BTC.BTC",
	///     "estimatedTime": 1008,
	///     "streamingSwap": StreamingSwap,
	/// }
	/// ```
	///
	/// # Example
	///
	/// ```rust
	/// use swapkit_rs::Swapkit;
	/// use dotenv;
	/// use swapkit_rs::Configuration;
	/// use swapkit_rs::RequestABorrowQuoteParams;
	///
	/// # tokio_test::block_on(async {
	/// let swapkit_config = Configuration::new(None, dotenv::var("SWAPKIT_REFERER").unwrap().as_str(), dotenv::var("SWAPKIT_X_API_KEY").unwrap().as_str());
	/// let mut swapkit = Swapkit::new(swapkit_config);
	///
	/// let quote = swapkit.get_request_a_borrow_quote(RequestABorrowQuoteParams {
	///     asset_in: "BTC.BTC".to_string(),
	///     asset_out: "BTC.BTC".to_string(),
	///     slippage: "0.5".to_string(),
	///     amount: "1".to_string(),
	///     sender_address: "bc1q7cyrfmck2ffu2ud3rn5l5a8yv6f0chkp0zpemf".to_string(),
	///     recipient_address: "bc1q7cyrfmck2ffu2ud3rn5l5a8yv6f0chkp0zpemf".to_string(),
	/// }).await.unwrap();
	///
	/// assert_eq!(quote.get_asset_in(), "BTC.BTC".to_string());
	/// # });
	/// ```
	///
	/// # Errors
	/// todo
	pub async fn get_request_a_borrow_quote(&mut self, parameters: RequestABorrowQuoteParams) -> Result<BorrowQuote> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_request_a_borrow_quote(self.get_config().get_base_url(), self.get_headers(), parameters).await
	}

	/// Retrieve a quote for a repay.
	///
	/// # Returns JSON Equivalent
	/// ```json
	/// {
	///     "inboundAddress": "bc1qpqkqxgln9qwahw76nyp3v9fqyc7zy252yhv2kc",
	///     "inboundConfirmationBlocks": 1,
	///     "inboundConfirmationSeconds": 600,
	///     "outboundDelayBlocks": 0,
	///     "outboundDelaySeconds": 0,
	///     "fees": Hashmap<String, Vec<QuoteFee>>
	///     "expiry": 1715977820,
	///     "warning": "Do not cache this response. Do not send funds after the expiry.",
	///     "notes": "First output should be to inbound_address, second output should be change back to self, third output should be OP_RETURN, limited to 80 bytes. Do not send below the dust threshold. Do not use exotic spend scripts, locks or address formats (P2WSH with Bech32 address format preferred).",
	///     "dustThreshold": "10000",
	///     "recommendedMinAmountIn": "0.00031104",
	///     "memo": "$-:BTC.BTC:bc1qzafz3f0h90u7n9j862uupaf5hpeydmhvpnzwzz:1",
	///     "expectedAmountOut": "0.00000000",
	///     "expectedAmountIn": "0.00013192",
	///     "expectedCollateralWithdrawn": "0.00000000",
	///     "expectedDebtRepaid": "8.82417067",
	///     "streamingSwapBlocks": 2,
	///     "streamingSwapSeconds": 12,
	///     "totalRepaySeconds": 612,
	///     "collateralCurrent": "0.06489696",
	///     "repayAssetAmount": "0.00019534",
	///     "repayAssetAmountUSD": "12.25657330",
	///     "timeEstimates": QuoteTimeEstimates,
	///     "streamingSwap": StreamingSwap
	/// }
	/// ```
	///
	/// # Example
	///
	/// ```rust
	/// use swapkit_rs::Swapkit;
	/// use dotenv;
	/// use swapkit_rs::Configuration;
	/// use swapkit_rs::RequestARepayQuoteParams;
	/// use rust_decimal::Decimal;
	///
	/// # tokio_test::block_on(async {
	/// let swapkit_config = Configuration::new(None, dotenv::var("SWAPKIT_REFERER").unwrap().as_str(), dotenv::var("SWAPKIT_X_API_KEY").unwrap().as_str());
	/// let mut swapkit = Swapkit::new(swapkit_config);
	///
	/// let quote = swapkit.get_request_a_repay_quote(RequestARepayQuoteParams {
	///     repay_asset: "BTC.BTC".to_string(),
	///     collateral_asset: "BTC.BTC".to_string(),
	///     amount_percentage: "0.5".to_string(),
	///     sender_address: "bc1qzafz3f0h90u7n9j862uupaf5hpeydmhvpnzwzz".to_string(),
	///     collateral_address: "bc1qzafz3f0h90u7n9j862uupaf5hpeydmhvpnzwzz".to_string(),
	///     affiliate_basis_points: "".to_string(),
	///     affiliate_address: "".to_string(),
	/// }).await.unwrap();
	///
	/// assert_ne!(quote.get_inbound_confirmation_blocks(), Decimal::ZERO);
	/// # });
	/// ```   
	///
	/// # Errors
	/// todo
	pub async fn get_request_a_repay_quote(&mut self, parameters: RequestARepayQuoteParams) -> Result<RepayQuote> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_request_a_repay_quote(self.get_config().get_base_url(), self.get_headers(), parameters).await
	}
}
