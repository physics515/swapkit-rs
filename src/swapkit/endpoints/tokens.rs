use anyhow::Result;
use chrono::Utc;

use crate::Swapkit;
use crate::{api_get_cached_prices, api_get_currencies_with_details, api_get_token_pair_exchange_rate, api_get_token_providers, CachedPrice, CachedPricesParameters, CurrenciesWithDetails, ExchangeRate, Provider, TokenIdentifier};

impl Swapkit {
	/// Retrieve a list of all the currencies the API supports with details.
	///
	/// # Returns JSON Equivalent
	/// ```json
	/// [
	///     {
	///         "name": "JOE",
	///         "ticker": "JOE",
	///         "address": "0x6e84a6216eA6dACC71eE8E6b0a5B7322EEbC0fDd",
	///         "protocol": "AVAX",
	///         "blockchain": "avalanche",
	///         "chainId": 43114,
	///         "apiIdentifier": "LOGO",
	///         "decimals": 18,
	///         "fullName": "AVAX.JOE-0X6E84A6216EA6DACC71EE8E6B0A5B7322EEBC0FDD",
	///         "extraIdName": "",
	///         "image": "https://raw.githubusercontent.com/traderjoe-xyz/joe-tokenlists/main/logos/0x6e84a6216eA6dACC71eE8E6b0a5B7322EEbC0fDd/logo.png",
	///         "enabled": true,
	///         "addressUrl": "https://snowtrace.io/address/",
	///         "transactionUrl": "https://snowtrace.io/tx/",
	///         "payinConfirmation": 1,
	///         "averageBlockTime": 3000,
	///         "notifications": {}
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
	/// let currencies_with_details = swapkit.get_currencies_with_details().await.unwrap();
	///
	/// assert_ne!(currencies_with_details.get_currencies().len(), 0);
	/// # });
	/// ```
	///
	/// # Errors
	/// todo
	pub async fn get_currencies_with_details(&mut self) -> Result<CurrenciesWithDetails> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_currencies_with_details(self.get_config().get_base_url(), self.get_headers()).await
	}

	/// Retrieve the exchange rate between two tokens.
	///
	/// # Arguments
	/// Provider: The provider to use for the exchange rate.
	/// Buy Asset: The asset to buy.
	/// Sell Asset: The asset to sell.
	///
	/// # Returns JSON Equivalent
	/// ```json
	/// {
	///     "price": "1040560352527"
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
	/// let exchange_rate = swapkit.get_token_pair_exchange_rate("THORCHAIN", "DOGE.DOGE", "THOR.RUNE").await.unwrap();
	///
	/// assert_ne!(exchange_rate.get_price(), &Decimal::ZERO);
	/// # });
	/// ```
	///
	/// # Errors
	/// todo
	pub async fn get_token_pair_exchange_rate(&mut self, provider: &str, buy_asset: &str, sell_asset: &str) -> Result<ExchangeRate> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_token_pair_exchange_rate(self.get_config().get_base_url(), self.get_headers(), provider, buy_asset, sell_asset).await
	}

	/// Retrieve the cached prices for a list of tokens.
	///
	/// # Arguments
	/// Tokens: The list of tokens to retrieve the cached prices for.
	/// Metadata: Whether to include metadata in the response.
	/// Lookup: Whether to include lookup in the response.
	/// Sparkline: Whether to include sparkline in the response.
	///
	/// # Returns JSON Equivalent
	/// ```json
	/// [
	///     {
	///        "identifier": "BTC.BTC",
	///         "provider": "thorchain",
	///        "cg": {
	///             "id": "bitcoin",
	///             "name": "Bitcoin",
	///             "market_cap": 538744544621,
	///             "total_volume": 10969486163,
	///             "price_change_24h_usd": -280.7722306713,
	///             "price_change_percentage_24h_usd": -1.00657,
	///             "sparkline_in_7d": "[28089.09709191385,27855.474964663084,27927.67807075597,27862.451794300196,27401.389272685337,27548.852018252153,27615.06488509527,27435.934692025516,27562.09703912351,27598.39792470794,27602.136947882696,27598.9993500583,27571.437611819903,27665.63327626977,27553.297682750952,27602.018171651824,27563.595143671093,27625.437113184413,27558.0389366372,27461.551847933046,27569.889017420348,27368.015011005587,27298.401139637797,27395.034111122666,27435.513880295017,27309.802923311734,27265.62463094056,27382.985451629556,27392.25698000947,27317.95783922375,27426.281146181223,27320.072509867994,27346.85930262205,27414.617414216456,27432.019190020048,27388.27831172733,27400.17389979379,27417.772319131862,27447.69073131563,27534.856488004025,27603.635460136415,27523.122837058436,27575.341017285835,27592.909263075417,27477.46254030785,27369.56539790677,27429.897057694783,27535.337250467284,27521.509194542694,27567.34444638266,27617.79707466413,27675.18226365197,27695.855655305724,27749.3591623358,27792.1112772493,27876.32045700953,27732.499011630913,27699.322189011964,27660.43776930838,27680.70811109574,27660.999881505486,27664.97998586457,27590.036109010794,27667.90317363952,27688.253128131677,27721.67093310776,27725.682886029477,27726.762917492902,27994.89730864139,28002.696815579708,27589.94070738397,27441.098459102155,27480.642145540383,27508.41169867955,27467.996840385844,27456.093199426483,27457.408171499657,27467.11281497509,27435.874615135104,27449.182553534614,27517.47659313017,27558.748584098343,27562.639686375685,27534.994803406487,27503.43709227742,27494.18273652981,27620.121334693493,27621.813080590975,27702.884509536205,27683.873490852977,27701.91202188015,27243.358912167776,27456.300949264096,27662.24505724077,28013.47513717081,27892.269316717717,27981.479515547853,27917.380788407478,27985.7686619,27985.153411120693,28141.091408717115,28015.369152584746,27958.196437350485,27935.45095504778,27929.06137059663,27919.728618535293,27928.21340921988,27933.38550086624,27916.173694762296,27918.823019136453,27905.662730230513,28019.428794289277,27955.69272098428,27952.4164201191,27951.995889644913,27975.65933688445,27954.276783733738,27962.347106863614,27954.76428701786,27965.11535213417,27963.657703192323,27902.221666935628,27876.74105849813,27933.844027691975,27959.75997497363,27974.7571037059,27977.543490700005,27991.261470727324,28018.263813642363,28031.083435007025,28088.344154516606,27941.92298575307,27950.417711989663,27910.307222168412,27922.817290402683,27918.183840966572,27920.63263879015,27838.539286459865,27831.66156449875,27834.03341170247,27945.573583737674,27938.916346723378,27907.122197042805,27925.396851235255,27890.631604658025,27876.307273823193,27902.96253787604,27924.562097939088,27976.4021826868,27919.889110632826,27948.10365174851,27841.982877887764,27984.53805572639,27956.367599384863,27940.268226458327,27945.412331761243,27871.811818805272,27921.657949143835,27843.34062675791,27752.810045457743,27556.74411716421,27542.403193736372,27502.801112830097,27477.190872892766,27489.23855019311,27486.740612263195,27453.06304817622,27354.09084225846]",
	///             "timestamp": 1696883306189
	///        },
	///        "price_usd": 62744.82080390614,
	///        "timestamp": 1710352724180
	///    }
	///]
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
	/// let cached_prices = swapkit.get_cached_prices(vec!["BTC.BTC".to_string()], None, None, None).await.unwrap();
	///
	/// assert_ne!(cached_prices.len(), 0);
	/// # });
	/// ```
	///
	/// # Errors
	/// todo
	pub async fn get_cached_prices(&mut self, tokens: Vec<String>, metadata: Option<bool>, lookup: Option<bool>, sparkline: Option<bool>) -> Result<Vec<CachedPrice>> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		let tokens: Vec<TokenIdentifier> = tokens.iter().map(|token| TokenIdentifier { identifier: token.clone() }).collect();

		let parameters = CachedPricesParameters { tokens, metadata, lookup, sparkline };

		self.set_last_call(Utc::now());
		api_get_cached_prices(self.get_config().get_base_url(), self.get_headers(), parameters).await
	}

	/// Retrieve a list of all the token providers the API supports.
	///
	/// # Returns JSON Equivalent
	/// ```json
	/// [
	///     {
	///         "provider": "Woofi",
	///         "version": {
	///             "major": 1,
	///             "minor": 0,
	///             "patch" 0             
	///         },
	///         "nbTokens": 4,
	///         "logo": "https://static.thorswap.net/token-list/images/eth.woo-0x4691937a7508860f876c9c0a2a617e7d9e945d4b.png"
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
	/// let token_providers = swapkit.get_token_providers().await.unwrap();
	///
	/// assert_ne!(token_providers.len(), 0);
	/// # });
	/// ```
	///
	/// # Errors
	/// todo
	pub async fn get_token_providers(&mut self) -> Result<Vec<Provider>> {
		// Wait for rate limit timer
		self.sleep_until_ok_to_call().await;

		self.set_last_call(Utc::now());
		api_get_token_providers(self.get_config().get_base_url(), self.get_headers()).await
	}
}
