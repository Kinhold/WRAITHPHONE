use serde::Serialize;

use crate::crypto::validate_text;
use crate::{ClaimDigest, ManifestRecord, Result};

/// Public metadata suitable for handing to a future witness transport.
///
/// Claim payload bytes are intentionally excluded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WitnessEnvelope {
    manifest_id: String,
    bundle_digest: ClaimDigest,
}

impl WitnessEnvelope {
    pub fn from_manifest(manifest: &ManifestRecord) -> Self {
        Self {
            manifest_id: manifest.manifest_id().to_owned(),
            bundle_digest: manifest.bundle_digest(),
        }
    }

    pub fn manifest_id(&self) -> &str {
        &self.manifest_id
    }

    pub fn bundle_digest(&self) -> ClaimDigest {
        self.bundle_digest
    }
}

/// Lifecycle stage of an unexecuted witness dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WitnessDispatchStage {
    Planned,
}

/// Dispatch metadata for a future transport adapter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WitnessDispatchPlan {
    destination: String,
    envelope: WitnessEnvelope,
    stage: WitnessDispatchStage,
}

impl WitnessDispatchPlan {
    pub fn new(destination: impl Into<String>, envelope: WitnessEnvelope) -> Result<Self> {
        let destination = destination.into();
        validate_text("destination", &destination)?;
        Ok(Self {
            destination,
            envelope,
            stage: WitnessDispatchStage::Planned,
        })
    }

    pub fn destination(&self) -> &str {
        &self.destination
    }

    pub fn envelope(&self) -> &WitnessEnvelope {
        &self.envelope
    }

    pub fn stage(&self) -> WitnessDispatchStage {
        self.stage
    }
}
