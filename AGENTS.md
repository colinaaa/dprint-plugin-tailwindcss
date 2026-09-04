# Repository Guidelines

## Project Structure & Module Organization

Core Rust code lives in `src/`. `format_text.rs` extracts and sorts class attributes using regex, `sort.rs` handles class string sorting with dedup and whitespace handling, `order.rs` contains the static Tailwind v4 class order map, `configuration/` resolves dprint options, and `wasm_plugin.rs` exposes the WebAssembly plugin. Unit tests sit beside their modules. Integration coverage starts in `tests/test.rs` and reads fixtures from `tests/specs/`. The npm wrapper, setup script, smoke test, and package metadata live under `deployment/npm/`; `deployment/schema.json` describes the published plugin configuration. Release helpers are in `scripts/`, and GitHub Actions workflows are in `.github/workflows/`.

## Build, Test, and Development Commands

- `cargo build` — compile the native debug library.
- `cargo test` — run Rust unit tests and fixture-based integration tests.
- `cargo test --release` — exercise the optimized build used by CI.
- `rustup target add wasm32-unknown-unknown` — install the plugin's WebAssembly target once.
- `cargo build --target wasm32-unknown-unknown --features wasm --release` — produce the publishable Wasm artifact.
- `cd deployment/npm && npm install && node setup.js && npm test` — copy the previously built artifact into the npm package and run its Node smoke test.

Use Rust 1.98.1 from `rust-toolchain.toml`. The repository's `dprint.json` is the formatting source of truth.

## Coding Style & Naming Conventions

Match the existing two-space indentation and keep changes focused. Use `snake_case` for Rust functions, modules, and tests; use `PascalCase` for structs and other types. JavaScript uses CommonJS and semicolons. Configuration keys exposed to users use camel case, such as `removeDuplicates`. Format touched files with dprint and inspect the resulting diff; Rust formatting delegates to `rustfmt`.

## Testing Guidelines

Add focused `#[test]` cases beside Rust logic. Name tests by behavior, for example `skips_class_inside_html_comment`. Add end-to-end sorting cases as descriptive `.txt` fixtures under `tests/specs/`, with input and `[expect]` sections. Run `cargo test`; changes to Wasm packaging also require the npm smoke test. No numeric coverage threshold is enforced, but new behavior and regressions should have tests.

## Commit & Pull Request Guidelines

Recent history favors concise, imperative subjects with prefixes such as `feat:`, `fix:`, `test:`, `ci:`, and `chore:`. Follow that pattern when practical. Pull requests should explain what changed and why, link relevant issues, and list the commands run. Include representative input and expected output for sorting changes. Keep release-version changes coordinated across Cargo, npm, and schema metadata.
