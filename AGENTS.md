# Lipsum

## Project Overview

`lipsum` is a Rust library for generating pseudo-Latin "lorem ipsum" placeholder
text. It uses a Markov chain of order two, trained on Cicero's _De finibus
bonorum et malorum_, to produce text that looks realistic but is actually
random.

The library is designed to be lightweight, fast, and compatible with WebAssembly
(Wasm). It balances randomness with platform constraints:

- **Non-Wasm (Linux, macOS, Windows):** Uses `std::time::SystemTime` to seed a
  thread-local random number generator (RNG). Subsequent calls produce different
  text.
- **Wasm (`wasm32-unknown-unknown`):** Uses a constant seed to prevent panics
  (as Wasm often lacks system time/entropy without JS bindings). However, it
  maintains state within a run, so subsequent calls still produce different text
  sequence segments.

## Building and Running

- **Build:** `cargo build`
- **Test (Standard):** `cargo test` (Runs unit tests, documentation tests, and
  version synchronization checks)
- **Test (Wasm):** `wasm-pack test --node` (Runs Wasm-specific integration tests
  using Node.js)
- **Benchmark:** `cargo bench` (Requires nightly Rust due to usage of the `test`
  feature)
- **Format:** `dprint fmt` (Auto-formats code and documentation)

## Development Conventions

- **Formatting:** The project uses `dprint` for formatting. Run `dprint fmt`
  before committing changes.
- **Platform Compatibility:** The crate must compile and run on
  `wasm32-unknown-unknown` without requiring users to manually enable complex
  feature flags (like `getrandom/js`).
- **Version Synchronization:** The crate uses `version-sync` in
  `tests/version-numbers.rs`. Ensure `README.md` and `src/lib.rs`
  (`html_root_url`) versions are updated when bumping `Cargo.toml`.
- **Randomness:**
  - The internal `MarkovChain` does not own an RNG but takes one as an argument
    (`iter(rng, ...)`).
  - Top-level functions (`lipsum`, `lipsum_words`) use a thread-local
    `DEFAULT_RNG`.
  - Use `lipsum_from_seed` or `lipsum_words_from_seed` for deterministic outputs
    in tests.
- **Dependencies:** Keep dependencies minimal. `rand` is used with
  `default-features = false` to avoid pulling in `libc` or `getrandom`'s system
  dependencies unnecessarily.
- **API Design:** The public API surface is small (`lipsum*` functions). The
  `MarkovChain` struct is exposed for advanced usage (custom training data) but
  its implementation details (like the `Words` iterator) are hidden behind
  `impl Iterator`.
- **Testing:**
  - Unit tests in `src/lib.rs` use fixed seeds for reproducibility.
  - Wasm tests in `src/lib.rs` (guarded by `cfg(target_arch = "wasm32")`) verify
    behavior in a browser-like environment.
