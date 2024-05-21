use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::quote_route::QuoteRoute;

/*
{
	"quoteId": "477981ae-eed8-4162-885b-3af7495fb8fb",
	"routes": Vec<QuoteRoute>,
	"sellAssetAmount": "1"
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
	#[serde(rename = "quoteId")]
	quote_id: String,

	#[serde(rename = "routes")]
	routes: Vec<QuoteRoute>,

	#[serde(rename = "sellAssetAmount", with = "rust_decimal::serde::str")]
	sell_asset_amount: Decimal,
}

impl Quote {
	#[must_use]
	pub const fn get_quote_id(&self) -> &String {
		&self.quote_id
	}

	#[must_use]
	pub const fn get_routes(&self) -> &Vec<QuoteRoute> {
		&self.routes
	}

	#[must_use]
	pub const fn get_sell_asset_amount(&self) -> Decimal {
		self.sell_asset_amount
	}
}
