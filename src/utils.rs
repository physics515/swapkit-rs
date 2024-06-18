use rust_decimal::prelude::*;
use serde::{Deserialize, Deserializer};

#[allow(clippy::unnecessary_wraps)]
pub fn deserialize_string_option_from_number<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
	D: Deserializer<'de>,
{
	#[derive(Deserialize)]
	#[serde(untagged)]
	enum StringOrNumber {
		String(String),
		Number(i64),
		Float(f64),
		Null,
	}

	match StringOrNumber::deserialize(deserializer) {
		Ok(StringOrNumber::String(s)) => {
			if s.is_empty() {
				Ok(None)
			} else {
				Ok(Some(s))
			}
		}
		Ok(StringOrNumber::Number(i)) => Ok(Some(i.to_string())),
		Ok(StringOrNumber::Float(f)) => Ok(Some(f.to_string())),
		_ => Ok(None),
	}
}

#[allow(clippy::unnecessary_wraps)]
pub fn deserialize_rust_decimal_from_anything_option<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
where
	D: Deserializer<'de>,
{
	#[derive(Deserialize)]
	#[serde(untagged)]
	enum StringOrNumber {
		String(String),
		Number(i64),
		Float(f64),
		Null,
	}

	match StringOrNumber::deserialize(deserializer) {
		Ok(StringOrNumber::String(s)) => {
			if s.is_empty() {
				Ok(None)
			} else {
				match s.to_uppercase().as_str() {
					"NULL" | "INFINITY" | "INF" => Ok(None),
					_ => Decimal::from_str(&s).map_or_else(|_| Ok(None), |d| Ok(Some(d))),
				}
			}
		}
		Ok(StringOrNumber::Number(i)) => Ok(Some(Decimal::from(i))),
		Ok(StringOrNumber::Float(f)) => Decimal::from_f64(f).map_or_else(|| Ok(None), |d| Ok(Some(d))),
		_ => Ok(None),
	}
}

#[allow(clippy::unnecessary_wraps)]
pub fn deserialize_rust_decimal_from_anything_option_default() -> Option<Decimal> {
        None
}

#[allow(clippy::unnecessary_wraps)]
pub fn deserialize_rust_decimal_from_anything<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
	D: Deserializer<'de>,
{
	#[derive(Deserialize)]
	#[serde(untagged)]
	enum StringOrNumber {
		String(String),
		Number(i64),
		Float(f64),
		Null,
	}

	match StringOrNumber::deserialize(deserializer) {
		Ok(StringOrNumber::String(s)) => {
			if s.is_empty() {
				Ok(Decimal::ZERO)
			} else {
				match s.to_uppercase().as_str() {
					"NULL" | "INFINITY" | "INF" => Ok(Decimal::ZERO),
					_ => Decimal::from_str(&s).map_or_else(|_| Ok(Decimal::ZERO), Ok),
				}
			}
		}
		Ok(StringOrNumber::Number(i)) => Ok(Decimal::from(i)),
		Ok(StringOrNumber::Float(f)) => Decimal::from_f64(f).map_or_else(|| Ok(Decimal::ZERO), Ok),
		_ => Ok(Decimal::ZERO),
	}
}
