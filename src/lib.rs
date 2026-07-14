//! Honest, deterministic metadata primitives for a future phone-native claim
//! pipeline.
//!
//! The crate currently provides claim content hashing and typed plans. It does
//! not generate or verify zero-knowledge proofs, authenticate observations,
//! call mobile keystores, or dispatch witnesses.

pub mod crypto;
mod error;
pub mod platform;
pub mod proof;
pub mod storage;
pub mod witness;

pub use crypto::{ClaimBundle, ClaimDigest, CLAIM_BUNDLE_FORMAT_VERSION};
pub use error::{Error, Result};
pub use platform::{
    AndroidKeystoreConfig, AndroidKeystorePlanner, KeyWrapPlan, KeyWrapPlanner, KeystoreAlias,
    PlatformOperationStage,
};
pub use proof::{
    ArtifactStage, PlanStage, ProofArtifactMetadata, ProofBackend, ProofPlan, VerificationStatus,
};
pub use storage::ManifestRecord;
pub use witness::{WitnessDispatchPlan, WitnessDispatchStage, WitnessEnvelope};
