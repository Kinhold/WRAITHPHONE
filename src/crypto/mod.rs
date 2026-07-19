use std::fmt;

use serde::{Serialize, Serializer};

use crate::{Error, Result};

const CLAIM_BUNDLE_DOMAIN: &[u8] = b"WRAITHPHONE\0CLAIM-BUNDLE\0V1";

/// Current version of the deterministic claim-bundle encoding.
pub const CLAIM_BUNDLE_FORMAT_VERSION: u16 = 1;

/// Claim material whose content can be deterministically serialized and hashed.
///
/// The digest produced from this type is only a content identifier. It does not
/// authenticate the observation, hide the payload, or constitute a proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClaimBundle {
    root_id: String,
    claim_type: String,
    observed_at_unix_ms: u64,
    payload: Vec<u8>,
}

impl ClaimBundle {
    /// Constructs a validated claim bundle.
    pub fn new(
        root_id: impl Into<String>,
        claim_type: impl Into<String>,
        observed_at_unix_ms: u64,
        payload: impl Into<Vec<u8>>,
    ) -> Result<Self> {
        let bundle = Self {
            root_id: root_id.into(),
            claim_type: claim_type.into(),
            observed_at_unix_ms,
            payload: payload.into(),
        };
        bundle.validate()?;
        Ok(bundle)
    }

    /// Stable binary encoding used as the input to [`Self::digest`].
    ///
    /// Fields are domain-separated and length-prefixed with big-endian `u32`
    /// lengths. This format is independent of Serde data-format choices.
    pub fn to_canonical_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;

        let mut encoded = Vec::with_capacity(
            CLAIM_BUNDLE_DOMAIN.len()
                + self.root_id.len()
                + self.claim_type.len()
                + self.payload.len()
                + 20,
        );
        encoded.extend_from_slice(CLAIM_BUNDLE_DOMAIN);
        append_field(&mut encoded, "root_id", self.root_id.as_bytes())?;
        append_field(&mut encoded, "claim_type", self.claim_type.as_bytes())?;
        encoded.extend_from_slice(&self.observed_at_unix_ms.to_be_bytes());
        append_field(&mut encoded, "payload", &self.payload)?;
        Ok(encoded)
    }

    /// Computes the BLAKE3 content digest of the canonical encoding.
    pub fn digest(&self) -> Result<ClaimDigest> {
        Ok(ClaimDigest(
            *blake3::hash(&self.to_canonical_bytes()?).as_bytes(),
        ))
    }

    pub fn format_version(&self) -> u16 {
        CLAIM_BUNDLE_FORMAT_VERSION
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

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    fn validate(&self) -> Result<()> {
        validate_text("root_id", &self.root_id)?;
        validate_text("claim_type", &self.claim_type)?;
        if self.observed_at_unix_ms == 0 {
            return Err(Error::InvalidField {
                field: "observed_at_unix_ms",
                reason: "must be greater than zero",
            });
        }
        if self.payload.is_empty() {
            return Err(Error::InvalidField {
                field: "payload",
                reason: "must not be empty",
            });
        }
        Ok(())
    }
}

/// A BLAKE3 content digest for a canonical [`ClaimBundle`].
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClaimDigest([u8; 32]);

impl ClaimDigest {
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn to_hex(self) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut output = String::with_capacity(64);
        for byte in self.0 {
            output.push(HEX[(byte >> 4) as usize] as char);
            output.push(HEX[(byte & 0x0f) as usize] as char);
        }
        output
    }
}

impl fmt::Display for ClaimDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_hex())
    }
}

impl fmt::Debug for ClaimDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("ClaimDigest")
            .field(&self.to_hex())
            .finish()
    }
}

impl Serialize for ClaimDigest {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_hex())
    }
}

pub(crate) fn validate_text(field: &'static str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(Error::InvalidField {
            field,
            reason: "must not be empty or whitespace",
        });
    }
    Ok(())
}

fn append_field(encoded: &mut Vec<u8>, field: &'static str, bytes: &[u8]) -> Result<()> {
    let length = u32::try_from(bytes.len()).map_err(|_| Error::FieldTooLong {
        field,
        length: bytes.len(),
    })?;
    encoded.extend_from_slice(&length.to_be_bytes());
    encoded.extend_from_slice(bytes);
    Ok(())
}
