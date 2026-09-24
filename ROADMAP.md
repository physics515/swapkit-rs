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
- [ ] **Drop the `#![feature(const_trait_impl)]` gate if nothing needs it.** A grep of `src/` finds
      only ordinary inherent `pub const fn` getters (`src/types/chain_with_details.rs:80` and
      ~40 siblings) and one free `pub const fn` in `src/utils.rs:64` — all of which are stable Rust.
      No `impl const Trait for` appears anywhere. If the gate is vestigial, removing it makes the
      published crate buildable on stable, which is the single biggest consumer-facing win available.
      Verify with `cargo +stable build --locked`, not just nightly.
- [ ] **Make the test surface runnable without secrets.** `src/test_utils/mod.rs:9` does
      `dotenv::var("SWAPKIT_REFERER").unwrap()`, and there is no `.env` in a fresh clone, so the one
      integration test and every doctest panic before they reach any assertion. Replace the unwrap
      with an explicit skip-with-reason (or a `Result`-returning helper) so a credential-less run
      reports "not run", never a false failure.
- [x] **Add `/scratch` to `.gitignore`** (it previously listed only `/target` and `.env`), so routine
      scratch output can never be staged. Landed 2026-09-24.
- [ ] **Add CI.** The repo has no `.github/` at all, so a PR has no checks. A workflow running
      `cargo build`, `cargo clippy --all-targets` and `cargo +nightly fmt --check` on both stable and
      nightly — with the network-dependent tests excluded — would make "merge when green" mean
      something.
- [ ] **Tighten the dependency requirements in `Cargo.toml`.** `reqwest = "0"`, `chrono = "0"`,
      `rand = "0"`, `dotenv = "0"`, `serde_urlencoded = "0"` and `tokio-test = "0"` each resolve to
      "any 0.x", so a breaking 0.x release lands silently in a consumer's build. Pin the real minor
      series the code compiles against.
- [ ] **Remove the four unused dependencies.** `clippy::cargo` reports `rand`, `serde_with`,
      `serde-aux` and `dotenv` as unused, and a grep of `src/` confirms **zero** references to
      `rand`, `serde_with`, `serde_aux` or `serde_urlencoded` anywhere. Five declared dependencies
      that nothing uses is pure cost to every consumer. Drop them, and re-check `url` while you are
      there — it has exactly one reference.
- [ ] **Move `dotenv` to `dev-dependencies`.** All 34 references are in doc examples and
      `src/test_utils/mod.rs` — nothing in the library itself uses it — yet it is a *non-dev*
      dependency, so every consumer inherits it. It has also been unmaintained since 2019
      (RUSTSEC-2021-0141); `dotenvy` is the maintained fork if a replacement is wanted for dev use.
- [ ] **Record the clippy baseline and drive it down.** Verified 2026-09-24: 47 warnings, none of
      them errors — 16 from the lib (15 duplicates), 35 from the lib tests (clippy offers 23
      auto-fixes via `cargo clippy --fix --lib -p swapkit-rs --tests`), plus the manifest's
      unused-dependency warnings. The big clusters are `to_string_in_format_args` (×13, all in the
      test's `println!`s), `struct_field_names` (×7) and four `unnecessary_unwrap`s on
      `parameters.*` in the quote endpoints. These are cheap, verifiable-offline increments — good
      filler when a harder item stalls, but never a substitute for phase-order work.
- [ ] **Declare an MSRV** in `Cargo.toml` once the crate builds on stable, so the toolchain contract
      is explicit rather than implied.

---

## Phase 1 — Upstream reality check (the existential question)

The crate is bindings for an API that appears to have moved out from under it. Resolve this before
investing in any endpoint-level work — it decides whether later phases are repairs or a rewrite.

- [ ] **Confirm whether the legacy THORSwap aggregator API still answers.** `Configuration::new`
      hardcodes `https://api.thorswap.net/` (`src/swapkit/config.rs:19`) and every endpoint builds
      paths like `{base_url}aggregator/chains` (`src/api/chains/supported_chains.rs:14`). Unauthenticated
      probes on 2026-09-24 returned **HTTP 404** for `https://api.thorswap.net/`,
      `/aggregator/chains`, `/aggregator/tokens/quote` and `/tokenlist/utils/providers`. A 404
      without an API key is suggestive, not proof — re-probe **with real credentials** and record the
      result here before concluding the base URL is dead.
- [ ] **Map the current SwapKit API surface.** SwapKit's own docs now describe a v3 REST API at
      `api.swapkit.dev` with `/providers`, `/tokens`, `/v3/quote`, `/v3/swap`, `/track` and `/swapTo`
      (<https://docs.swapkit.dev/swapkit-api/introduction.md>). `https://api.swapkit.dev/providers`
      and `/tokens` answered **HTTP 401** on 2026-09-24 — live and auth-gated — and
      `https://api.swapkit.dev/docs` answered **200**, so the OpenAPI definition is readable. Pull
      that definition and write the per-method old→new mapping into this roadmap as its own items.
- [ ] **Decide the migration strategy and record it here** before writing migration code: whether
      v3 becomes the default base URL with the legacy paths removed outright, or the two live side by
      side behind a config switch. The crate is 0.0.x, so a clean break is permitted — but it must be
      a deliberate, written decision, not an accident of whichever endpoint got touched first.
- [ ] **Re-derive the auth contract.** The client sends `Referer`, `X-API-KEY` and a lowercase
      `referrer` header (`src/swapkit/mod.rs:23-26`), the last hardcoded to
      `https://sk.thorswap.net`. Confirm against the current docs what v3 actually requires; a
      hardcoded third-party referrer in a library is wrong if it is not required.

---

## Phase 2 — Correctness of the public surface

- [ ] **`dev_base_url` is dead.** It is set in `Configuration::new` (`src/swapkit/config.rs:20`) and
      never read — there is no getter, no setter and no call path that can select it, so the
      "add support for dev api" commit (`ec47449`) never actually shipped a user-visible capability.
      Either finish it (a way to choose the dev base, plumbed into the request path) or remove the
      field.
- [ ] **`get_headers()` can panic on user input.** `HeaderValue::from_str(...).unwrap()`
      (`src/swapkit/mod.rs:24-26`) panics on any API key or referer containing a non-visible-ASCII
      byte. A library should return the error, never panic on a caller's string.
- [ ] **Every `# Errors` doc section says `todo`.** They exist only to quiet
      `clippy::missing_errors_doc` and tell a consumer nothing. Document what each method actually
      returns on failure, at least for the endpoints that are known-good.
- [ ] **`anyhow::Result` is the public return type of every endpoint method.** `anyhow` is an
      application error type; a library should expose something a consumer can match on. `APIError`
      (`src/types/errors.rs`) already exists as a `thiserror` enum — return it, and keep `anyhow`
      to `dev-dependencies` if it is wanted for tests.
- [ ] **Split the monolithic integration test.** `test_all_endpoints`
      (`src/swapkit/mod.rs:74`) is the crate's only `#[test]`/`#[tokio::test]` and exercises every
      endpoint in one function, so the first failure hides every endpoint after it. One test per
      endpoint gives a real pass/fail count.
- [ ] **Drop the brittle assertion** `assert_eq!(gas_prices.get_gas_prices().len(), 10)`
      (`src/swapkit/mod.rs:~90`) — it fails whenever upstream adds or removes a chain, which is not a
      defect in this crate.
- [ ] **`get_transation_details` is misspelled in the public API** (`src/swapkit/endpoints/transactions.rs`)
      — the method is missing the `c` in "transaction", while `README.md` and `src/lib.rs` both
      advertise `get_transaction_details`. Renaming it is a breaking change, which 0.0.x permits;
      call it out loudly in the release notes.
- [ ] **Fix the remaining endpoint names in the docs.** `README.md` and the mirrored crate docs in
      `src/lib.rs` advertise `get_chains`, but the method is `get_supported_chains`. A consumer
      copying the README does not compile.
- [ ] **Fix the crate-doc typos** in `src/lib.rs`: the stray `9++` on the `get_chains` line and
      "fully typeed". They are the first thing a docs.rs visitor reads.
- [ ] **Re-check the two endpoints documented as broken.**
      `get_available_assets_for_pool` and `get_transaction_details` are both marked
      "***Errors (Thorswap endpoint does not return a value.)***" in the README. Establish whether
      they are broken upstream, broken here, or simply gone, and say so precisely.

---

## Phase 3 — Offline, deterministic tests

The whole point: today a green test run proves the network was up, the credentials were valid *and*
the code was right; a red one could be any of the three.

- [ ] **Capture JSON fixtures** for each endpoint's real response and add deserialization tests over
      them, so type drift is caught without a network call or a secret.
- [ ] **Add a fixture test per type that upstream has already broken once** — the git history shows
      repeated fixes to gas-price and cached-price deserialization (`584cfaa`, `25e0381`, `53dd10a`,
      `235b692`, `43f9b38`). Each of those is a shape that deserves a pinned fixture.
- [ ] **Keep the live tests, but behind a feature or an env guard**, so `cargo test` is meaningful
      offline and the network suite is opt-in.

---

## Phase 4 — Ergonomics and modernization

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
