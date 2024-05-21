use serde::{Deserialize, Serialize};

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

		Self {
			base_url: "https://dev-api.thorswap.net/".to_string(),
			rate_limit_ms,
			referer: referer.to_string(),
			x_api_key: x_api_key.to_string(),
			referrer: "https://sk.thorswap.net".to_string(),
		}
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
	pub const fn get_referer(&self) -> &str {
		&self.referer
	}

	#[must_use]
	pub const fn get_x_api_key(&self) -> &str {
		&self.x_api_key
	}

	#[must_use]
	pub const fn get_referrer(&self) -> &str {
		&self.referrer
	}

	pub fn set_base_url(&mut self, base_url: String) {
		self.base_url = base_url;
	}

	pub fn set_rate_limit_ms(&mut self, rate_limit_ms: u64) {
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
