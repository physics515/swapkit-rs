use crate::Configuration;
use crate::Swapkit;

/// The environment variables every live test needs.
const REFERER_VAR: &str = "SWAPKIT_REFERER";
const X_API_KEY_VAR: &str = "SWAPKIT_X_API_KEY";

/// Builds a [`Swapkit`] from the credentials in the environment, falling back to a `.env` file.
///
/// Returns `None` when either credential is missing or empty. **Every test in this crate talks to
/// the live `SwapKit` API with a real key**, so a checkout without credentials must report "not
/// run" rather than fail: a red test would otherwise mean "the network was down", "the key was
/// wrong" or "the code is broken" with no way to tell which.
///
/// Prefer [`crate::skip_without_credentials`] over calling this directly.
#[must_use]
pub fn try_test_swapkit() -> Option<Swapkit> {
	dotenv::dotenv().ok();

	let referer = non_empty_var(REFERER_VAR)?;
	let x_api_key = non_empty_var(X_API_KEY_VAR)?;

	Some(Swapkit::new(Configuration::new(None, &referer, &x_api_key)))
}

/// Reads an environment variable, treating an empty value as absent.
fn non_empty_var(name: &str) -> Option<String> {
	match dotenv::var(name) {
		Ok(value) if !value.trim().is_empty() => Some(value),
		_ => None,
	}
}

/// The reason a live test did not run, for printing to the test log.
#[must_use]
pub fn missing_credentials_reason() -> String {
	let missing: Vec<&str> = [REFERER_VAR, X_API_KEY_VAR].into_iter().filter(|name| non_empty_var(name).is_none()).collect();

	format!("NOT RUN (no credentials): {} unset or empty. This is not a failure — every test in this crate calls the live SwapKit API.", missing.join(" and "))
}

/// Binds a live [`Swapkit`] client, or returns from the test after printing why it did not run.
///
/// Used instead of a panic so that a credential-less `cargo test` reports "not run, no
/// credentials" rather than a false failure.
#[macro_export]
macro_rules! skip_without_credentials {
	($name:ident) => {
		let Some(mut $name) = $crate::test_utils::try_test_swapkit() else {
			println!("{}", $crate::test_utils::missing_credentials_reason());
			return;
		};
	};
}
