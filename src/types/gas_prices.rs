use serde::{Deserialize, Serialize};

use crate::GasPrice;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasPrices(Vec<GasPrice>);

impl GasPrices {
	#[must_use]
	pub const fn get_gas_prices(&self) -> &Vec<GasPrice> {
		&self.0
	}
}
