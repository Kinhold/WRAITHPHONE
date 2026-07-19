use serde::Serialize;

use crate::crypto::validate_text;
use crate::Result;

/// Stage of a platform operation that has not yet touched platform APIs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformOperationStage {
    Planned,
}

/// Configuration metadata for a future Android Keystore adapter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AndroidKeystoreConfig {
    namespace: String,
}

impl AndroidKeystoreConfig {
    pub fn new(namespace: impl Into<String>) -> Result<Self> {
        let namespace = namespace.into();
        validate_text("namespace", &namespace)?;
        Ok(Self { namespace })
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }
}

/// Validated alias for a key expected to live in a platform keystore.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct KeystoreAlias(String);

impl KeystoreAlias {
    pub fn new(alias: impl Into<String>) -> Result<Self> {
        let alias = alias.into();
        validate_text("keystore_alias", &alias)?;
        Ok(Self(alias))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Metadata describing a requested key-wrap operation.
///
/// It contains no key material and does not report that wrapping occurred.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct KeyWrapPlan {
    backend: &'static str,
    namespace: String,
    alias: KeystoreAlias,
    stage: PlatformOperationStage,
}

impl KeyWrapPlan {
    pub fn backend(&self) -> &str {
        self.backend
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn alias(&self) -> &KeystoreAlias {
        &self.alias
    }

    pub fn stage(&self) -> PlatformOperationStage {
        self.stage
    }
}

/// Planner interface. Implementations only describe work for an external
/// platform adapter.
pub trait KeyWrapPlanner {
    fn plan_key_wrap(&self, alias: KeystoreAlias) -> Result<KeyWrapPlan>;
}

/// Planner for a future Android Keystore integration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AndroidKeystorePlanner {
    config: AndroidKeystoreConfig,
}

impl AndroidKeystorePlanner {
    pub fn new(config: AndroidKeystoreConfig) -> Self {
        Self { config }
    }
}

impl KeyWrapPlanner for AndroidKeystorePlanner {
    fn plan_key_wrap(&self, alias: KeystoreAlias) -> Result<KeyWrapPlan> {
        Ok(KeyWrapPlan {
            backend: "android_keystore",
            namespace: self.config.namespace.clone(),
            alias,
            stage: PlatformOperationStage::Planned,
        })
    }
}
