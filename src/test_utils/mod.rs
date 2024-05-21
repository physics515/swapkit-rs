use dotenv;

use crate::Configuration;
use crate::Swapkit;

pub fn get_test_swapkit() -> Swapkit {
	dotenv::dotenv().ok();

	let swapkit_config = Configuration::new(None, dotenv::var("SWAPKIT_REFERER").unwrap().as_str(), dotenv::var("SWAPKIT_X_API_KEY").unwrap().as_str());

	Swapkit::new(swapkit_config)
}
