use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::deserialize_rust_decimal_from_anything_option;
use crate::deserialize_rust_decimal_from_anything_option_default;

use crate::Sparkline;

/*
[
	{
		"identifier": "BTC.BTC",
		 "provider": "thorchain",
		"cg": {
			 "id": "bitcoin",
			 "name": "Bitcoin",
			 "market_cap": 538744544621,
			 "total_volume": 10969486163,
			 "price_change_24h_usd": -280.7722306713,
			 "price_change_percentage_24h_usd": -1.00657,
			 "sparkline_in_7d": "[28089.09709191385,27855.474964663084,27927.67807075597,27862.451794300196,27401.389272685337,27548.852018252153,27615.06488509527,27435.934692025516,27562.09703912351,27598.39792470794,27602.136947882696,27598.9993500583,27571.437611819903,27665.63327626977,27553.297682750952,27602.018171651824,27563.595143671093,27625.437113184413,27558.0389366372,27461.551847933046,27569.889017420348,27368.015011005587,27298.401139637797,27395.034111122666,27435.513880295017,27309.802923311734,27265.62463094056,27382.985451629556,27392.25698000947,27317.95783922375,27426.281146181223,27320.072509867994,27346.85930262205,27414.617414216456,27432.019190020048,27388.27831172733,27400.17389979379,27417.772319131862,27447.69073131563,27534.856488004025,27603.635460136415,27523.122837058436,27575.341017285835,27592.909263075417,27477.46254030785,27369.56539790677,27429.897057694783,27535.337250467284,27521.509194542694,27567.34444638266,27617.79707466413,27675.18226365197,27695.855655305724,27749.3591623358,27792.1112772493,27876.32045700953,27732.499011630913,27699.322189011964,27660.43776930838,27680.70811109574,27660.999881505486,27664.97998586457,27590.036109010794,27667.90317363952,27688.253128131677,27721.67093310776,27725.682886029477,27726.762917492902,27994.89730864139,28002.696815579708,27589.94070738397,27441.098459102155,27480.642145540383,27508.41169867955,27467.996840385844,27456.093199426483,27457.408171499657,27467.11281497509,27435.874615135104,27449.182553534614,27517.47659313017,27558.748584098343,27562.639686375685,27534.994803406487,27503.43709227742,27494.18273652981,27620.121334693493,27621.813080590975,27702.884509536205,27683.873490852977,27701.91202188015,27243.358912167776,27456.300949264096,27662.24505724077,28013.47513717081,27892.269316717717,27981.479515547853,27917.380788407478,27985.7686619,27985.153411120693,28141.091408717115,28015.369152584746,27958.196437350485,27935.45095504778,27929.06137059663,27919.728618535293,27928.21340921988,27933.38550086624,27916.173694762296,27918.823019136453,27905.662730230513,28019.428794289277,27955.69272098428,27952.4164201191,27951.995889644913,27975.65933688445,27954.276783733738,27962.347106863614,27954.76428701786,27965.11535213417,27963.657703192323,27902.221666935628,27876.74105849813,27933.844027691975,27959.75997497363,27974.7571037059,27977.543490700005,27991.261470727324,28018.263813642363,28031.083435007025,28088.344154516606,27941.92298575307,27950.417711989663,27910.307222168412,27922.817290402683,27918.183840966572,27920.63263879015,27838.539286459865,27831.66156449875,27834.03341170247,27945.573583737674,27938.916346723378,27907.122197042805,27925.396851235255,27890.631604658025,27876.307273823193,27902.96253787604,27924.562097939088,27976.4021826868,27919.889110632826,27948.10365174851,27841.982877887764,27984.53805572639,27956.367599384863,27940.268226458327,27945.412331761243,27871.811818805272,27921.657949143835,27843.34062675791,27752.810045457743,27556.74411716421,27542.403193736372,27502.801112830097,27477.190872892766,27489.23855019311,27486.740612263195,27453.06304817622,27354.09084225846]",
			 "timestamp": 1696883306189
		},
		"price_usd": 62744.82080390614,
		"timestamp": 1710352724180
	}
]
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedPrice {
	identifier: String,
	provider: Option<String>,
	cg: CachedPriceCG,

	#[serde(rename = "price_usd", serialize_with = "rust_decimal::serde::str_option::serialize", deserialize_with = "deserialize_rust_decimal_from_anything_option", default = "deserialize_rust_decimal_from_anything_option_default")]
	price_usd: Option<Decimal>,

	#[serde(serialize_with = "rust_decimal::serde::str::serialize", deserialize_with = "rust_decimal::serde::float::deserialize")]
	timestamp: Decimal,
}

impl CachedPrice {
	#[must_use]
	pub const fn get_identifier(&self) -> &String {
		&self.identifier
	}

	#[must_use]
	pub const fn get_provider(&self) -> &Option<String> {
		&self.provider
	}

	#[must_use]
	pub const fn get_cg(&self) -> &CachedPriceCG {
		&self.cg
	}

	#[must_use]
	pub const fn get_price_usd(&self) -> Option<Decimal> {
		self.price_usd
	}

	#[must_use]
	pub const fn get_timestamp(&self) -> Decimal {
		self.timestamp
	}
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedPriceCG {
	id: Option<String>,
	name: Option<String>,

	#[serde(rename = "market_cap", serialize_with = "rust_decimal::serde::str_option::serialize", deserialize_with = "rust_decimal::serde::float_option::deserialize", default)]
	market_cap: Option<Decimal>,

	#[serde(rename = "total_volume", serialize_with = "rust_decimal::serde::str_option::serialize", deserialize_with = "rust_decimal::serde::float_option::deserialize", default)]
	total_volume: Option<Decimal>,

	#[serde(rename = "price_change_24h_usd", serialize_with = "rust_decimal::serde::str_option::serialize", deserialize_with = "rust_decimal::serde::float_option::deserialize", default)]
	price_change_24h_usd: Option<Decimal>,

	#[serde(rename = "price_change_percentage_24h_usd", serialize_with = "rust_decimal::serde::str_option::serialize", deserialize_with = "rust_decimal::serde::float_option::deserialize", default)]
	price_change_percentage_24h_usd: Option<Decimal>,

	#[serde(deserialize_with = "crate::Sparkline::deserialize", default)]
	sparkline_in_7d: Sparkline,

	#[serde(serialize_with = "rust_decimal::serde::str_option::serialize", deserialize_with = "rust_decimal::serde::float_option::deserialize", default)]
	timestamp: Option<Decimal>,
}

impl CachedPriceCG {
	#[must_use]
	pub const fn get_id(&self) -> &Option<String> {
		&self.id
	}

	#[must_use]
	pub const fn get_name(&self) -> &Option<String> {
		&self.name
	}

	#[must_use]
	pub const fn get_market_cap(&self) -> Option<Decimal> {
		self.market_cap
	}

	#[must_use]
	pub const fn get_total_volume(&self) -> Option<Decimal> {
		self.total_volume
	}

	#[must_use]
	pub const fn get_price_change_24h_usd(&self) -> Option<Decimal> {
		self.price_change_24h_usd
	}

	#[must_use]
	pub const fn get_price_change_percentage_24h_usd(&self) -> Option<Decimal> {
		self.price_change_percentage_24h_usd
	}

	#[must_use]
	pub const fn get_sparkline_in_7d(&self) -> &Sparkline {
		&self.sparkline_in_7d
	}

	#[must_use]
	pub const fn get_timestamp(&self) -> Option<Decimal> {
		self.timestamp
	}
}
