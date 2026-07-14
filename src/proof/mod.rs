use serde::Serialize;

use crate::crypto::validate_text;
use crate::{ClaimDigest, ManifestRecord, Result};

/// Proof system targeted by a plan. No backend is executed by this crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofBackend {
    Noir,
}

/// Current lifecycle stage of a proof plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanStage {
    Planned,
}

/// Metadata needed by a future Noir adapter to produce a witness and proof.
///
/// Constructing a plan does not invoke Noir, Nargo, or the hardware trust
/// engine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProofPlan {
    plan_id: String,
    manifest_id: String,
    bundle_digest: ClaimDigest,
    backend: ProofBackend,
    circuit: String,
    package_path: String,
    stage: PlanStage,
}

impl ProofPlan {
    /// Creates staged metadata for a future Noir invocation.
    pub fn for_noir(
        plan_id: impl Into<String>,
        manifest: &ManifestRecord,
        circuit: impl Into<String>,
        package_path: impl Into<String>,
    ) -> Result<Self> {
        let plan_id = plan_id.into();
        let circuit = circuit.into();
        let package_path = package_path.into();
        validate_text("plan_id", &plan_id)?;
        validate_text("circuit", &circuit)?;
        validate_text("package_path", &package_path)?;

        Ok(Self {
            plan_id,
            manifest_id: manifest.manifest_id().to_owned(),
            bundle_digest: manifest.bundle_digest(),
            backend: ProofBackend::Noir,
            circuit,
            package_path,
            stage: PlanStage::Planned,
        })
    }

    pub fn plan_id(&self) -> &str {
        &self.plan_id
    }

    pub fn manifest_id(&self) -> &str {
        &self.manifest_id
    }

    pub fn bundle_digest(&self) -> ClaimDigest {
        self.bundle_digest
    }

    pub fn backend(&self) -> ProofBackend {
        self.backend
    }

    pub fn circuit(&self) -> &str {
        &self.circuit
    }

    pub fn package_path(&self) -> &str {
        &self.package_path
    }

    pub fn stage(&self) -> PlanStage {
        self.stage
    }
}

/// Whether proof bytes have been produced outside this crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactStage {
    Planned,
    Produced,
}

/// Explicit verification state for an artifact record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    NotVerified,
}

/// Metadata about a planned or externally produced proof artifact.
///
/// Deliberately contains no proof bytes and never implies verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProofArtifactMetadata {
    artifact_id: String,
    plan_id: String,
    bundle_digest: ClaimDigest,
    backend: ProofBackend,
    stage: ArtifactStage,
    verification: VerificationStatus,
    locator: Option<String>,
}

impl ProofArtifactMetadata {
    /// Reserves metadata for an artifact that has not been produced.
    pub fn planned(artifact_id: impl Into<String>, plan: &ProofPlan) -> Result<Self> {
        let artifact_id = artifact_id.into();
        validate_text("artifact_id", &artifact_id)?;
        Ok(Self {
            artifact_id,
            plan_id: plan.plan_id.clone(),
            bundle_digest: plan.bundle_digest,
            backend: plan.backend,
            stage: ArtifactStage::Planned,
            verification: VerificationStatus::NotVerified,
            locator: None,
        })
    }

    /// Records that an external adapter produced an artifact at `locator`.
    ///
    /// This transition still records the artifact as unverified.
    pub fn record_external_output(mut self, locator: impl Into<String>) -> Result<Self> {
        let locator = locator.into();
        validate_text("locator", &locator)?;
        self.stage = ArtifactStage::Produced;
        self.verification = VerificationStatus::NotVerified;
        self.locator = Some(locator);
        Ok(self)
    }

    pub fn artifact_id(&self) -> &str {
        &self.artifact_id
    }

    pub fn plan_id(&self) -> &str {
        &self.plan_id
    }

    pub fn bundle_digest(&self) -> ClaimDigest {
        self.bundle_digest
    }

    pub fn backend(&self) -> ProofBackend {
        self.backend
    }

    pub fn stage(&self) -> ArtifactStage {
        self.stage
    }

    pub fn verification_status(&self) -> VerificationStatus {
        self.verification
    }

    pub fn locator(&self) -> Option<&str> {
        self.locator.as_deref()
    }
}
