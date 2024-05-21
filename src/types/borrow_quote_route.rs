use serde::{Deserialize, Serialize};

use crate::BorrowQuoteRouteMeta;

/*
{
		"meta": {
				"thornodeMeta": {
						"inboundConfirmationSeconds": 600,
						"outboundDelaySeconds": 348,
						"expectedAmountOut": "0.49226113"
				}
		}
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorrowQuoteRoute {
	meta: BorrowQuoteRouteMeta,
}

impl BorrowQuoteRoute {
	#[must_use]
	pub const fn get_meta(&self) -> &BorrowQuoteRouteMeta {
		&self.meta
	}
}
