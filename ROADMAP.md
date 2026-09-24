# swapkit-rs ROADMAP

The single source of truth for work on this crate. Every item is a `[ ]`/`[x]` checkbox, in phase
order. Tick `[x]` only when the item genuinely shipped. Add discovered work as new `[ ]` items in
the phase it belongs to. No status tables, no run log — git history, the PR and the GitHub release
are the record.

Bootstrapped 2026-09-24 from a survey of the code at `1c908ec` plus live probes of the upstream API.
Every probe result below is dated and names how it was taken; re-probe before trusting one.

---

## Phase 0 — Unblock build, tests and release

Nothing downstream is trustworthy until this phase is clear.

- [x] **Establish whether master builds on the current nightly.** It does. Verified 2026-09-24 on
      `nightly-1.100.0 (6bb1652a0 2026-09-22)`: `cargo build --locked` exited 0 with zero errors, and
      `cargo clippy --locked --all-targets` exited 0. The `#![feature(const_trait_impl)]` gate still
      compiles, so nothing is on fire — the nightly gate is a *consumer* problem, not a build blocker.
- [x] **Drop the `#![feature(const_trait_impl)]` gate if nothing needs it.** Done 2026-09-24. The
      gate was vestigial: `src/` contains only ordinary inherent `pub const fn` getters and one free
      `pub const fn` in `src/utils.rs`, and no `impl const Trait for` anywhere. With the gate removed,
      `cargo +stable build --locked` succeeded on `rustc 1.98.0 (88d9e12ae 2026-08-18)`. **The crate
      now builds on stable Rust.** Never reintroduce a `#![feature(...)]`; nightly is for `rustfmt`
      only.
- [ ] **Work around the host's nightly-only `rustflags`.** `~/.cargo/config.toml` sets
      `rustflags = ["-Z", "threads=8"]`, which makes *any* `cargo +stable` invocation on this host
      fail with "the option `Z` is only accepted on the nightly compiler" before it reaches the
      crate. Stable checks must clear it (`RUSTFLAGS="" cargo +stable build --locked`). CI does not
      inherit that config, so this is a host quirk, not a crate defect — but it must not be mistaken
      for a stable-build regression.
- [x] **Make the test surface runnable without secrets.** Done 2026-09-24 for the integration test.
      `src/test_utils/mod.rs` now exposes `try_test_swapkit() -> Option<Swapkit>` plus a
      `skip_without_credentials!` macro; `cargo test --locked --lib` on a credential-less checkout
      prints `NOT RUN (no credentials): SWAPKIT_REFERER and SWAPKIT_X_API_KEY unset or empty` and
      exits 0 instead of panicking. **Note what this does and does not prove:** the test *executes*,
      but exercises zero endpoints. Real offline coverage is Phase 3.
- [x] **Make the ~17 doctests runnable without secrets too.** Done 2026-09-24. All 17 `​```rust`
      fences are now `​```rust,no_run`, so `cargo test --locked --doc` **compiles** every example
      without a network call or a credential: 17 passed, 0 failed, on a credential-less host. The
      change immediately caught a doc example that had never compiled — the
      `get_request_a_swap_quote` example was missing `use swapkit_rs::RequestASwapQuoteParams` and
      called a `Quote::get_quote()` that does not exist (the accessor is `get_quote_id()`).
- [ ] **Keep the examples executing somewhere.** `no_run` proves an example compiles, not that it
      works. Once the live suite is behind an env guard (Phase 3), consider a credentialed CI job
      that drops `no_run` — or accept compile-checking as the contract and say so in the docs.
- [x] **Add `/scratch` to `.gitignore`** (it previously listed only `/target` and `.env`), so routine
      scratch output can never be staged. Landed 2026-09-24.
- [x] **Add CI.** Done 2026-09-24: `.github/workflows/ci.yml`. A `build` matrix over **stable and
      nightly** runs `cargo build --locked --all-targets`, `cargo clippy --locked --all-targets`,
      `cargo test --locked --lib` and `cargo test --locked --doc`; a separate `fmt` job runs
      `cargo +nightly fmt --all --check`. Every step is offline: the lib test skips with
      "NOT RUN (no credentials)" and all doctests are `no_run`, so nothing calls the SwapKit API and
      no secret is needed. Clippy is **not** gated on `-D warnings` while the inherited backlog
      stands (lib 16, lib test 33); it must still exit 0.
- [ ] **Gate clippy on `-D warnings` once the backlog is zero.** The CI step deliberately does not,
      so the backlog cannot be mistaken for a green tree.
- [ ] **Clear the 22 open Dependabot alerts on master** (6 high, 11 moderate, 5 low as of
      2026-09-24 — `gh api repos/physics515/swapkit-rs/dependabot/alerts`). Every one is transitive
      through a `Cargo.lock` pinned in 2024. **`openssl` alone accounts for 11 of them, 5 of those
      high** (GHSA-xp3w-r5p5-63rr, GHSA-pqf5-4pqq-29f5, GHSA-hppc-g8h3-xhp3, GHSA-ghm9-cr32-g9qj,
      GHSA-8c75-8mhr-p7r9), and it is only present because `reqwest` defaults to `native-tls`. The
      rest are `rustls-webpki` (×4, one high), `ring`, `idna`, `bytes`, `time`, `tokio`, `rand` and
      `serde_with`. A plain `cargo update` clears most; switching `reqwest` to
      `default-features = false` + `rustls-tls` removes the whole `openssl` chain and the C
      dependency with it, which also serves the 100%-Rust principle. Verify the build after either.

- [x] **Tighten the dependency requirements in `Cargo.toml`.** Done 2026-09-24. The surviving 0.x
      requirements now name their real minor series — `reqwest = "0.12"`, `chrono = "0.4"`,
      `dotenv = "0.15"` (dev), `tokio-test = "0.4"` (dev) — so a breaking 0.x release can no longer
      land silently in a consumer's build. `rand` and `serde_urlencoded` were removed outright.
- [x] **Remove the four unused dependencies.** Done 2026-09-24. `rand`, `serde_with`, `serde-aux`
      and `serde_urlencoded` were all declared with **zero** references in `src/`; removing them
      dropped 16 crates from `Cargo.lock` (`darling`, `darling_core`, `darling_macro`, `deranged`,
      `hex`, `ident_case`, `indexmap`, `num-conv`, `powerfmt`, `serde-aux`, `serde_with`,
      `serde_with_macros`, `strsim`, `time`, `time-core`, `time-macros`) and cleared every
      `clippy::cargo` unused-dependency warning. `url` was re-checked and **kept** — `src/types/errors.rs:2`
      uses `url::ParseError` in the `APIError` enum.
- [x] **Move `dotenv` to `dev-dependencies`.** Done 2026-09-24. All 39 references are in doc
      examples and `src/test_utils/mod.rs`; nothing in the library itself uses it, so consumers no
      longer inherit it.
- [ ] **Stop using `dotenv` even as a dev-dependency.** It has been unmaintained since 2019
      (RUSTSEC-2021-0141). `dotenvy` is the maintained fork, but the cleaner move is to drop the
      crate from the *doc examples* entirely and read `std::env::var` there, so the published docs
      stop advertising an unmaintained crate to consumers who copy them.
- [x] **Record the clippy baseline and drive it down.** Baseline measured 2026-09-24 before any
      change: **lib 16, lib test 35, manifest 3-4** unused-dependency warnings, zero errors. As of
      the same day: **lib 7, lib test 7 (all duplicates of the lib's), manifest 0** — 51 down to 7
      distinct, with no `#![allow(...)]` added anywhere. What went: the manifest warnings with the
      unused dependencies; 13 `to_string_in_format_args` with the test `println!`s; 4
      `unnecessary_unwrap` rewritten as `if let Some(..)` in `request_a_swap_quote`; 2
      `missing_const_for_fn`; 3 `map_or_else`/`map_or` identity closures in `ok_to_call_at`.
- [ ] **The last 7 clippy warnings are all `struct_field_names`-family**, on `Asset`, `Pool`,
      `Provider`, `Quote`, `StreamingSwap` (×2) and `QuoteTimeEstimates` (`all fields have the same
      postfix: ms`). Every one would mean renaming a public field whose name mirrors the upstream
      JSON key. Decide deliberately: rename with `#[serde(rename)]` to keep the wire format, or
      leave them and accept that the backlog floor is 7. **Do not** silence them with an `allow`.
- [x] **Declare an MSRV** in `Cargo.toml`. Done 2026-09-24: `rust-version = "1.83"`, and `"1.83"`
      is now the first leg of the CI matrix so it cannot drift silently. Measured, not guessed —
      1.83.0 builds and runs the whole suite green (18 lib + 4 deserialization + 18 doctests), and
      **1.82.0 fails** with two `E0658 mutable references are not allowed in constant functions` on
      `Configuration::set_rate_limit_ms` and `Swapkit::set_last_call`: `const_mut_refs` stabilised
      in 1.83. Those two `const fn`s came from satisfying `clippy::missing_const_for_fn`, so the
      lint cost exactly one Rust release of MSRV — an acceptable trade, but worth knowing.
- [ ] **Re-probe the true MSRV floor if 1.83 ever costs a consumer.** Dropping the two `const fn`
      setters would very likely get the crate back to 1.82 or lower; nothing else in `src/` is
      obviously recent. Only worth doing if someone actually asks.

---

## Phase 1 — Upstream reality check (the existential question)

The crate is bindings for an API that appears to have moved out from under it. Resolve this before
investing in any endpoint-level work — it decides whether later phases are repairs or a rewrite.

- [ ] **Confirm whether the legacy THORSwap aggregator API still answers.** Re-probed 2026-09-24
      (second run, unauthenticated, `curl -o /dev/null -w '%{http_code}'`):
      `https://api.thorswap.net/` **404**, `/aggregator/chains` **404**,
      `/aggregator/tokens/quote` **404**, `/tokenlist/utils/providers` **404**,
      `/resource-worker/gasPrice/getAll` **404**. `https://dev-api.thorswap.net/aggregator/chains`
      answered **403**, not 404 — the dev host is at least still resolving and terminating TLS.
      A 404 without an API key remains suggestive, not proof: a 404 is the *wrong* status for an
      auth failure, which is what makes it damning, but only a probe **with real credentials**
      settles it. **Still `[ ]` — this needs the owner's key.** Everything downstream of it is
      already written up as if the host is gone, because the v3 surface below is confirmed live.
- [x] **Map the current SwapKit API surface.** Done 2026-09-24. The OpenAPI definition is served at
      **<https://api.swapkit.dev/docs/json>** (the Swagger UI at `/docs` loads it from `./json`;
      `/openapi.json`, `/swagger.json`, `/docs-json` and `/api-json` all 404, so this is the only
      URL that works). It self-describes as `SwapKit API v0.1.0`, lists servers
      `https://api.swapkit.dev`, `https://dev-api.swapkit.dev` and `http://localhost:8080`, and
      declares exactly one security scheme: **`apiKey` in the `x-api-key` header**. The full path
      list, saved from that definition:

      | method | path |
      | --- | --- |
      | GET | `/approve`, `/asset/providers`, `/balance`, `/gas`, `/gas/history`, `/providers`, `/providers/identifiers-mapping`, `/providers/status`, `/swapFrom`, `/swapTo`, `/tokens`, `/tokens/search`, `/tokens/whitelist/pools`, `/tokens/whitelist/tokens` |
      | GET | `/v3/limit/orders`, `/v3/limit/orders/{orderId}`, `/v3/limit/tokens` |
      | POST | `/chainflip/broker/channel`, `/price`, `/quote`, `/screen`, `/track`, `/v3/quote`, `/v3/swap` |
      | POST | `/v3/limit/build`, `/v3/limit/cancel/build`, `/v3/limit/cancel/submit`, `/v3/limit/quote`, `/v3/limit/submit` |

      The upstream SDK repo moved: `thorswap/SwapKit` now 301-redirects to
      **<https://github.com/swapkit/SwapKit>**, last pushed 2026-08-03 and not archived.
- [ ] **Write the per-method old→new mapping.** From the definition above, the shape of the
      migration is: `get_supported_chains` / `get_chains_with_details` have **no v3 equivalent** —
      there is no `/chains` path at all (probed: 404), so chain data now comes out of `/tokens`;
      `get_supported_providers` → `GET /providers` (plus `/providers/status` and
      `/providers/identifiers-mapping`, which are new); `get_currencies_with_details` →
      `GET /tokens` (`provider`, `category`) and the new `GET /tokens/search`; `get_cached_prices` →
      **`POST /price`** — note the method change from GET to POST; `get_gas_prices` / `get_gas_rates`
      → `GET /gas` (`chainId`, `timeFrame`); `get_gas_history` → `GET /gas/history`;
      `get_request_a_swap_quote` → **`POST /v3/quote`** with a JSON body
      (`sellAsset`, `buyAsset`, `sellAmount`, `providers`, …) rather than query parameters;
      `get_transaction_details` → `POST /track`. The lending endpoints
      (`get_available_lending_assets`, `get_loans`, `get_request_a_borrow_quote`,
      `get_request_a_repay_quote`, `get_available_assets_for_pool`) and
      `get_token_pair_exchange_rate` and `get_minimum_amount_to_send_with_details` have **no
      counterpart in the v3 definition** — decide per endpoint whether they are dropped or rebuilt.
      Verify each line against the definition before writing code; this paragraph is a reading of
      the path list, not a tested mapping.
- [ ] **Decide the migration strategy and record it here** before writing migration code: whether
      v3 becomes the default base URL with the legacy paths removed outright, or the two live side by
      side behind a config switch. The crate is 0.0.x, so a clean break is permitted — but it must be
      a deliberate, written decision, not an accident of whichever endpoint got touched first.
- [ ] **Re-derive the auth contract.** **Answered by the OpenAPI definition, pending a live
      check.** `https://api.swapkit.dev/docs/json` declares exactly one security scheme —
      `{"apiKey": {"type": "apiKey", "name": "x-api-key", "in": "header"}}` — applied globally.
      There is **no `Referer` and no `referrer`** anywhere in it. So the two referer headers this
      client sends, including the third-party `https://sk.thorswap.net` hardcoded at
      `src/swapkit/config.rs`, are almost certainly legacy. Confirm with a credentialed request
      before removing them, because dropping a header that *is* required turns every call into a
      401 — but plan on `Configuration::new` losing its `referer` argument, which is a breaking
      signature change worth doing in the same release as the v3 migration.

---

## Phase 2 — Correctness of the public surface

- [ ] **Audit every public type for missing accessors.** `Provider` and `ProviderVersion` were
      returned from `get_token_providers` as public types with **every field private and not one
      getter** — a consumer could deserialize them and then read nothing out of them. Getters were
      added 2026-09-24; the same audit has not been done for the other ~60 types in `src/types/`.

- [x] **`dev_base_url` is dead.** Resolved 2026-09-24 by shipping the capability `ec47449` intended,
      with less state: the unreadable private field is gone, and the two URLs are now public
      constants `DEFAULT_BASE_URL` and `DEV_BASE_URL` in `src/swapkit/config.rs`, selectable through
      the existing `Configuration::set_base_url`. **Breaking** in one narrow way: `Configuration`
      derives `Serialize`/`Deserialize`, so a persisted configuration from 0.0.11 carries a
      `dev_base_url` key that no longer maps to a field.
- [x] **`get_headers()` can panic on user input.** Fixed 2026-09-24. `get_headers` now returns
      `Result<HeaderMap, APIError>` and each of the three headers is built with
      `HeaderValue::from_str(..).map_err(..)?`, surfacing the new
      `APIError::InvalidHeaderValue { header, source }` variant instead of panicking. All 18 call
      sites in `src/swapkit/endpoints/` propagate it with `?`. A trailing newline in a `.env` API key
      used to abort the process; it now returns an error naming which header was bad.
- [x] **Every `# Errors` doc section says `todo`.** Done 2026-09-24. All 18 endpoint methods now
      list the exact `APIError` variants they can return, derived per method from the `bail!` sites
      in the `api_*` function each one calls, so the list is accurate rather than boilerplate — the
      three quote endpoints are the only ones that can return `UrlParsingError`, and the two
      unmodelled placeholders are the only ones that cannot return `SerdeError`.
- [ ] **`anyhow::Result` is the public return type of every endpoint method.** `anyhow` is an
      application error type; a library should expose something a consumer can match on. `APIError`
      (`src/types/errors.rs`) already exists as a `thiserror` enum — return it, and keep `anyhow`
      to `dev-dependencies` if it is wanted for tests.
- [x] **Split the monolithic integration test.** Done 2026-09-24: `test_all_endpoints` is now 18
      `#[tokio::test]`s, one per endpoint, each skipping with a reason when credentials are absent.
      A failure in one endpoint no longer hides the 17 after it. The live suite should be run with
      `--test-threads=1`, since the 1 req/s limiter is per client — noted in the module docs.
- [x] **Drop the brittle assertion** `assert_eq!(gas_prices.get_gas_prices().len(), 10)`. Done
      2026-09-24, along with its twin `assert_eq!(currencies_with_details.get_currencies().len(), 1286)`.
      Both are now non-empty checks. The one *kept* count is `cached_prices`, which asserts the
      response has an entry per requested token — that is this crate's contract, not upstream's
      inventory.
- [x] **`get_transation_details` is misspelled in the public API.** Renamed 2026-09-24 to
      `get_transaction_details`, matching what `README.md` and `src/lib.rs` always advertised. The
      internal `api_get_transation_details` was renamed alongside it. **Breaking** for anyone who
      called the misspelled name — 0.0.x permits it, and it is called out in the release notes.
- [x] **Fix the remaining endpoint names in the docs.** Done 2026-09-24. Both `README.md` and the
      crate docs in `src/lib.rs` now list the real method names, checked against
      `grep 'pub async fn' src/swapkit/endpoints/*.rs` — all 18 match. The `no_run` doctests are a
      standing guard against this drift returning.
- [x] **Fix the crate-doc typos** in `src/lib.rs`: the stray `9++` and "fully typeed" are both gone
      (2026-09-24), in `README.md` too.
- [x] **Re-check the two endpoints documented as broken.** Answered 2026-09-24: **they are gone,
      along with the entire host.** Every probed `api.thorswap.net` path returns HTTP 404 (see
      Phase 1), so `get_available_assets_for_pool` and `get_transaction_details` are not
      *specifically* broken — nothing on that base URL answers. `get_transaction_details` also
      discarded its response body entirely and `println!`'d it to stdout; the `println!` is removed
      (a library must not write to stdout) and the method now documents itself as an unmodelled
      placeholder.
- [ ] **Model a real return type for transaction tracking.** `get_transaction_details` returns
      `Result<()>` and drops the body. Its v3 replacement is `POST /track`
      (<https://api.swapkit.dev/docs/json>); model that response properly rather than keeping a
      method that can only ever say "the HTTP call did not error".

---

## Phase 3 — Offline, deterministic tests

The whole point: today a green test run proves the network was up, the credentials were valid *and*
the code was right; a red one could be any of the three.

- [ ] **Capture JSON fixtures** for each endpoint's real response and add deserialization tests over
      them, so type drift is caught without a network call or a secret. **First slice landed
      2026-09-24**, **second slice the same run** — `tests/deserialization.rs` now has **9 tests**
      over `tests/fixtures/{gas_prices,gas_prices_edge_cases,supported_chains,cached_prices,
      chains_with_details,gas_history,providers,supported_providers,exchange_rate}.json`, all
      offline and credential-free. Still `[ ]`: **`Quote` / `QuoteRoute` and
      `MinimumAmountToSendWithDetails` / `Loan` / `LendingAsset` have no fixture.** **Next slice:**
      `Quote`, which is by far the largest and most nested type in the crate and the one a swap
      consumer actually depends on.
- [x] **Add a fixture test per type that upstream has already broken once.** Done 2026-09-24 for the
      two the git history keeps repairing. `gas_prices_edge_cases.json` pins the whole contract of
      `deserialize_rust_decimal_from_anything` — number, numeric string, `null`, `""`, `"NULL"`,
      `"Infinity"`, `"inf"` — which is what `584cfaa`, `25e0381`, `53dd10a` and `235b692` each
      changed; `cached_prices.json` pins the sparse entry (no provider, empty `cg`, null price) from
      `43f9b38`.
- [ ] **Audit the remaining `f64`-mediated decimal paths.** The `Float` arm of the custom
      deserializers is fixed, but `market_cap`, `total_volume` and the other `CachedPriceCG` fields
      still use `rust_decimal::serde::float_option::deserialize`, which is `Decimal::from_f64` and
      therefore has the same ~15-significant-digit ceiling. They hold integers today, so nothing is
      visibly wrong — pin a fixture before deciding whether to change them.
- [ ] **Keep the live tests, but behind a feature or an env guard**, so `cargo test` is meaningful
      offline and the network suite is opt-in.

---

## Phase 4 — Ergonomics and modernization

- [ ] **The rate limiter stamps `last_call` before the request, not after.**
      `sleep_until_ok_to_call` sets `last_call = now` and *then* the request is made, so a request
      that takes 3 s leaves the next call free to fire immediately — the 1 req/s floor is on request
      *starts*, not on the gap between them. That may well be what upstream throttles on; decide
      deliberately and document it either way. Do not change it without a reason.

- [ ] **Reuse one `reqwest::Client`.** Every `api_*` function builds a brand-new client per call
      (`reqwest::Client::builder().build()`, e.g. `src/api/chains/supported_chains.rs:9`), which
      throws away connection pooling and TLS session reuse on every request. Build it once and hold
      it in `Swapkit`.
- [ ] **Stop requiring `&mut self` for reads.** Every endpoint method takes `&mut self` purely so
      the rate limiter can record `last_call`, which makes the client awkward to share. Move that
      state behind interior mutability so the endpoints take `&self`.
- [ ] **Consider edition 2024.** The crate is on edition 2021; the move is mechanical once the build
      is healthy, and it is a good forcing function for the MSRV item.
- [ ] **Add `# Panics` docs, or remove the panics.** `clippy::missing_panics_doc` fires on a
      documented function that can panic; the `get_headers()` item in Phase 2 is the same problem
      seen from the other side. Prefer removing the panic to documenting it.

---

## Locked principles

- **Fully typed is the point.** Never widen a field to `serde_json::Value` or `String` to dodge a
  parse failure — model the real shape and pin it with a fixture. `rust_decimal` is already load-
  bearing here (575 references). `serde-aux` and `serde_with` are *declared but unused* as of
  2026-09-24 and Phase 0 removes them; if a modeling task genuinely needs one, add it back
  deliberately rather than treating it as already available.
- **The rate limiter stays.** The 1 req/s default exists because the upstream API throttles; no new
  endpoint may bypass `sleep_until_ok_to_call()`.
- **The base URL stays configurable.** Never hardcode an instance into a call path.
- **This is a library.** Its consumers pay for every dependency and every breaking change. Keep the
  dependency set minimal and justified; prefer `dev-dependencies` for anything only tests need.
- **100% Rust, no FFI.**
