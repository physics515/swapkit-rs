use serde::{Deserialize, Serialize};

/// The base URL [`Configuration::new`] starts from.
///
/// Every path this crate builds is appended to it, so it must end in a `/`.
///
/// **This host is not answering.** Every path probed on it returned HTTP 404 on 2026-09-24; the
/// current `SwapKit` API is a v3 surface at `https://api.swapkit.dev`. Use
/// [`Configuration::set_base_url`] to point the client somewhere that answers.
pub const DEFAULT_BASE_URL: &str = "https://api.thorswap.net/";

/// The legacy `THORSwap` development base URL.
///
/// Pass it to [`Configuration::set_base_url`] to target the dev instance:
///
/// ```rust
/// use swapkit_rs::{Configuration, DEV_BASE_URL};
///
/// let mut config = Configuration::new(None, "referer", "x-api-key");
/// config.set_base_url(DEV_BASE_URL.to_string());
/// assert_eq!(config.get_base_url(), DEV_BASE_URL);
/// ```
pub const DEV_BASE_URL: &str = "https://dev-api.thorswap.net/";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
	base_url: String,
	rate_limit_ms: u64,
	referer: String,
	referrer: String,
	x_api_key: String,
}

impl Configuration {
	#[must_use]
	pub fn new(rate_limit_ms: Option<u64>, referer: &str, x_api_key: &str) -> Self {
		let rate_limit_ms = rate_limit_ms.unwrap_or(1000);

		Self { base_url: DEFAULT_BASE_URL.to_string(), rate_limit_ms, referer: referer.to_string(), x_api_key: x_api_key.to_string(), referrer: "https://sk.thorswap.net".to_string() }
	}

	#[must_use]
	pub fn get_base_url(&self) -> &str {
		&self.base_url
	}

	#[must_use]
	pub const fn get_rate_limit_ms(&self) -> u64 {
		self.rate_limit_ms
	}

	#[must_use]
	pub fn get_referer(&self) -> &str {
		&self.referer
	}

	#[must_use]
	pub fn get_x_api_key(&self) -> &str {
		&self.x_api_key
	}

	#[must_use]
	pub fn get_referrer(&self) -> &str {
		&self.referrer
	}

	pub fn set_base_url(&mut self, base_url: String) {
		self.base_url = base_url;
	}

	pub const fn set_rate_limit_ms(&mut self, rate_limit_ms: u64) {
		self.rate_limit_ms = rate_limit_ms;
	}

	pub fn set_referer(&mut self, referer: String) {
		self.referer = referer;
	}

	pub fn set_x_api_key(&mut self, x_api_key: String) {
		self.x_api_key = x_api_key;
	}

	pub fn set_referrer(&mut self, referrer: String) {
		self.referrer = referrer;
	}
}
