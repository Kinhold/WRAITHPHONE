# WRAITHPHONE

Phone-native Rust security core under the Kinhold `@kinhold/mobile` family.

> **Spine upgrade (2026-07-16):** The verified security core (crypto, AEAD SQLite, witness transport, platform simulators, Noir circuit trees, lifecycle engine — 28 tests) is documented in [`pack/20_WRAITHPHONE_SPINE.md`](../../pack/20_WRAITHPHONE_SPINE.md). Sync the upgraded source tree into this directory (and `Kinhold/WRAITHPHONE`) before treating the mirror as authoritative.

## Verified spine (summary)

- X25519 ECDH → HKDF → AES-256-GCM envelopes
- Argon2id PIN-to-KEK; `zeroize`; `#![forbid(unsafe_code)]`
- Encrypted SQLite manifests with AEAD-bound scope/ID
- Replay-protected witness transport + hash-chained audit
- Software / TEE-sim / StrongBox-sim backends
- Six Noir circuit trees; Rust bridge returns honest `ToolchainMissing` without fabricating proofs
- `src/lifecycle.rs` key state machine (Active → Rotated → Retired/Destroyed + Revoked)

## Not production yet

Android Keystore/StrongBox JNI, real Noir prove/verify CI, durable lifecycle persistence, Double Ratchet, independent crypto audit.

## kinhold.io

Provision lane: https://kinhold.io/packages/wraithphone
