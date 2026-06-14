use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyHierarchy {
    pub root_id: String,
}

pub struct KeyLifecycle;

impl KeyLifecycle {
    pub fn derive_claim_bundle(hierarchy: &KeyHierarchy) -> ClaimBundle {
        ClaimBundle {
            root_id: hierarchy.root_id.clone(),
            payload: "claim_material".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClaimBundle {
    pub root_id: String,
    pub payload: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lifecycle_derivation() {
        let hierarchy = KeyHierarchy { root_id: "test_root".to_string() };
        let bundle = KeyLifecycle::derive_claim_bundle(&hierarchy);
        assert_eq!(bundle.root_id, "test_root");
    }
}
