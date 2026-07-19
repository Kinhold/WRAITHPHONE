use serde::Serialize;

use crate::crypto::validate_text;
use crate::{ClaimBundle, ClaimDigest, Error, Result};

/// Proposed schema for a future SQLite manifest adapter.
///
/// This crate currently models records only; it does not open or migrate a
/// database.
pub const SQLITE_SCHEMA_V1: &str = "CREATE TABLE IF NOT EXISTS manifests (
    manifest_id TEXT PRIMARY KEY NOT NULL,
    bundle_digest TEXT NOT NULL,
    root_id TEXT NOT NULL,
    claim_type TEXT NOT NULL,
    observed_at_unix_ms INTEGER NOT NULL,
    recorded_at_unix_ms INTEGER NOT NULL
);";

/// Index record linking a claim's public metadata to its content digest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ManifestRecord {
    manifest_id: String,
    bundle_digest: ClaimDigest,
    root_id: String,
    claim_type: String,
    observed_at_unix_ms: u64,
    recorded_at_unix_ms: u64,
}

impl ManifestRecord {
    /// Creates a manifest from a validated claim bundle.
    pub fn from_bundle(
        manifest_id: impl Into<String>,
        bundle: &ClaimBundle,
        recorded_at_unix_ms: u64,
    ) -> Result<Self> {
        let manifest_id = manifest_id.into();
        validate_text("manifest_id", &manifest_id)?;
        if recorded_at_unix_ms == 0 {
            return Err(Error::InvalidField {
                field: "recorded_at_unix_ms",
                reason: "must be greater than zero",
            });
        }

        Ok(Self {
            manifest_id,
            bundle_digest: bundle.digest()?,
            root_id: bundle.root_id().to_owned(),
            claim_type: bundle.claim_type().to_owned(),
            observed_at_unix_ms: bundle.observed_at_unix_ms(),
            recorded_at_unix_ms,
        })
    }

    /// Checks content and copied metadata against a claim bundle.
    ///
    /// This is an integrity consistency check, not cryptographic proof
    /// verification or source authentication.
    pub fn check_bundle(&self, bundle: &ClaimBundle) -> Result<()> {
        if self.bundle_digest != bundle.digest()? {
            return Err(Error::ManifestMismatch {
                field: "bundle_digest",
            });
        }
        if self.root_id != bundle.root_id() {
            return Err(Error::ManifestMismatch { field: "root_id" });
        }
        if self.claim_type != bundle.claim_type() {
            return Err(Error::ManifestMismatch {
                field: "claim_type",
            });
        }
        if self.observed_at_unix_ms != bundle.observed_at_unix_ms() {
            return Err(Error::ManifestMismatch {
                field: "observed_at_unix_ms",
            });
        }
        Ok(())
    }

    pub fn manifest_id(&self) -> &str {
        &self.manifest_id
    }

    pub fn bundle_digest(&self) -> ClaimDigest {
        self.bundle_digest
    }

    pub fn root_id(&self) -> &str {
        &self.root_id
    }

    pub fn claim_type(&self) -> &str {
        &self.claim_type
    }

    pub fn observed_at_unix_ms(&self) -> u64 {
        self.observed_at_unix_ms
    }

    pub fn recorded_at_unix_ms(&self) -> u64 {
        self.recorded_at_unix_ms
    }
}
