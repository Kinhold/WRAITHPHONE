# AGENTS.md

## Cursor Cloud specific instructions

WRAITHPHONE is a single Rust **library crate** (`wraithphone`, edition 2021). There is no binary, server, or external service — the "application" is the library plus its `#[cfg(test)]` unit tests. Nothing listens on a port; SQLite/Noir-Nargo/Android-Keystore only exist as placeholder types and command-plan strings, so no databases or network services are needed.

Standard commands (already implied by `Cargo.toml`):
- Build: `cargo build`
- Test (this is the end-to-end suite for the scaffold): `cargo test`
- Lint: `cargo clippy --all-targets`
- Format check: `cargo fmt --check`

Non-obvious notes:
- The Rust toolchain (cargo/rustc/clippy/rustfmt 1.83) is preinstalled system-wide; no `rustup`/`nvm`-style version juggling is needed.
- `Cargo.lock` and `target/` are gitignored, so `cargo build` regenerating them will not show up in `git status`.
- Current `cargo clippy` emits a few warnings and `cargo fmt --check` reports formatting diffs on the existing scaffold code. These are pre-existing; do not "fix" them unless the task asks for it.
- `scripts/termux_bootstrap.sh` is an Android/Termux bootstrap stub (just echoes a message) and is irrelevant to the cloud environment.
