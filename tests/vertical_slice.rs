use wraithphone::{
    AndroidKeystoreConfig, AndroidKeystorePlanner, ArtifactStage, ClaimBundle, Error,
    KeyWrapPlanner, KeystoreAlias, ManifestRecord, PlanStage, PlatformOperationStage,
    ProofArtifactMetadata, ProofPlan, VerificationStatus, WitnessDispatchPlan,
    WitnessDispatchStage, WitnessEnvelope,
};

fn example_claim() -> ClaimBundle {
    ClaimBundle::new(
        "device-root-7",
        "temperature-observation",
        1_750_000_000_123,
        br#"{"celsius":21.5}"#.to_vec(),
    )
    .expect("fixture is valid")
}

#[test]
fn canonical_digest_matches_stable_vector() {
    let claim = example_claim();
    let reconstructed = example_claim();

    assert_eq!(
        claim.to_canonical_bytes().unwrap(),
        reconstructed.to_canonical_bytes().unwrap()
    );
    assert_eq!(claim.digest().unwrap(), reconstructed.digest().unwrap());
    assert_eq!(
        claim.digest().unwrap().to_hex(),
        "a0945df5ec3822c8eec53ffa40f2c9727bbdf09920cd57f41eef1de7be345178"
    );

    let different_boundaries = ClaimBundle::new(
        "device-root-7temperature-",
        "observation",
        1_750_000_000_123,
        br#"{"celsius":21.5}"#.to_vec(),
    )
    .unwrap();
    assert_ne!(
        claim.digest().unwrap(),
        different_boundaries.digest().unwrap()
    );
}

#[test]
fn constructors_reject_invalid_metadata() {
    assert_eq!(
        ClaimBundle::new("   ", "temperature", 1, vec![1]).unwrap_err(),
        Error::InvalidField {
            field: "root_id",
            reason: "must not be empty or whitespace",
        }
    );
    assert_eq!(
        ClaimBundle::new("root", "temperature", 0, vec![1]).unwrap_err(),
        Error::InvalidField {
            field: "observed_at_unix_ms",
            reason: "must be greater than zero",
        }
    );
    assert_eq!(
        ClaimBundle::new("root", "temperature", 1, Vec::new()).unwrap_err(),
        Error::InvalidField {
            field: "payload",
            reason: "must not be empty",
        }
    );

    let claim = example_claim();
    assert!(ManifestRecord::from_bundle("", &claim, 1).is_err());
    assert!(ManifestRecord::from_bundle("manifest-1", &claim, 0).is_err());
}

#[test]
fn manifest_detects_a_different_claim() {
    let claim = example_claim();
    let manifest = ManifestRecord::from_bundle("manifest-1", &claim, 1_750_000_000_456).unwrap();
    manifest.check_bundle(&claim).unwrap();

    let changed_claim = ClaimBundle::new(
        claim.root_id(),
        claim.claim_type(),
        claim.observed_at_unix_ms(),
        br#"{"celsius":22.0}"#.to_vec(),
    )
    .unwrap();
    assert_eq!(
        manifest.check_bundle(&changed_claim),
        Err(Error::ManifestMismatch {
            field: "bundle_digest",
        })
    );
}

#[test]
fn integration_outputs_remain_explicitly_planned_and_unverified() {
    let claim = example_claim();
    let manifest = ManifestRecord::from_bundle("manifest-1", &claim, 1_750_000_000_456).unwrap();

    let proof_plan =
        ProofPlan::for_noir("proof-plan-1", &manifest, "temperature_claim", "./noir").unwrap();
    assert_eq!(proof_plan.stage(), PlanStage::Planned);
    assert_eq!(proof_plan.bundle_digest(), claim.digest().unwrap());
    assert!(ProofPlan::for_noir("bad-plan", &manifest, "temperature_claim", " ").is_err());

    let artifact = ProofArtifactMetadata::planned("artifact-1", &proof_plan).unwrap();
    assert_eq!(artifact.stage(), ArtifactStage::Planned);
    assert_eq!(
        artifact.verification_status(),
        VerificationStatus::NotVerified
    );
    assert_eq!(artifact.locator(), None);

    let produced = artifact
        .record_external_output("proofs/artifact-1")
        .unwrap();
    assert_eq!(produced.stage(), ArtifactStage::Produced);
    assert_eq!(
        produced.verification_status(),
        VerificationStatus::NotVerified
    );

    let key_planner =
        AndroidKeystorePlanner::new(AndroidKeystoreConfig::new("io.kinhold.wraith").unwrap());
    let key_plan = key_planner
        .plan_key_wrap(KeystoreAlias::new("claim-root").unwrap())
        .unwrap();
    assert_eq!(key_plan.stage(), PlatformOperationStage::Planned);

    let envelope = WitnessEnvelope::from_manifest(&manifest);
    let dispatch = WitnessDispatchPlan::new("witness://operator-1", envelope).unwrap();
    assert_eq!(dispatch.stage(), WitnessDispatchStage::Planned);

    let dispatch_json = serde_json::to_string(&dispatch).unwrap();
    assert!(!dispatch_json.contains("celsius"));
    assert!(!dispatch_json.contains("21.5"));
}
