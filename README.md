# WRAITHPHONE

WRAITHPHONE is an experimental Rust package for modeling a phone-native claim
pipeline. Its long-term direction is to connect private observations to
selectively disclosed proofs, but the current crate does **not** generate or
verify zero-knowledge proofs.

## What works today

The crate implements one small metadata path:

1. validate and deterministically serialize a `ClaimBundle`;
2. calculate a domain-separated BLAKE3 content digest;
3. create a `ManifestRecord` tied to that digest;
4. stage typed Noir proof and artifact metadata; and
5. stage platform key-wrap and witness-dispatch metadata.

The digest provides deterministic content identification only. It is not a
signature, proof of origin, proof of observation, encryption mechanism, or
privacy guarantee. Plans are records of intended work; creating one does not
execute Nargo, call an Android keystore, or send data.

```rust
use wraithphone::{
    ArtifactStage, ClaimBundle, ManifestRecord, ProofArtifactMetadata, ProofPlan,
    VerificationStatus,
};

let claim = ClaimBundle::new(
    "device-root-1",
    "location-observation",
    1_750_000_000_000,
    br#"{"region":"example"}"#.to_vec(),
)?;
let manifest = ManifestRecord::from_bundle("manifest-1", &claim, 1_750_000_000_100)?;
let plan = ProofPlan::for_noir("plan-1", &manifest, "location_claim", "./noir")?;
let artifact = ProofArtifactMetadata::planned("artifact-1", &plan)?;

assert_eq!(artifact.stage(), ArtifactStage::Planned);
assert_eq!(
    artifact.verification_status(),
    VerificationStatus::NotVerified
);
# Ok::<(), wraithphone::Error>(())
```

## Integration still required

Before this can make security or ZK claims, an integration must supply:

- a Hardware Trust Engine (HTE) boundary and threat model;
- device-backed key generation, wrapping, attestation validation, rotation, and
  deletion;
- a versioned Noir circuit plus an explicit mapping from canonical claim fields
  to private/public circuit inputs;
- witness generation that preserves intended disclosure boundaries;
- Nargo/Noir proof generation and verifier execution;
- artifact authenticity, verification-result recording, and replay/domain
  separation rules;
- durable manifest storage, migrations, transport, and recovery behavior; and
- independent security review and production mobile hardening.

## Packaging intent

If this graduates under `kinhold.io`, it should become a governed phone-native
proof lane layered on top of the HTE. Until those adapters and controls exist,
this package should be treated as deterministic data modeling and integration
planning—not a secure proof system.
