use std::fmt;

/// Errors returned when constructing or checking WRAITHPHONE records and plans.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// A required field did not satisfy its documented constraints.
    InvalidField {
        field: &'static str,
        reason: &'static str,
    },
    /// A value cannot be represented by the versioned claim-bundle format.
    FieldTooLong { field: &'static str, length: usize },
    /// A manifest and claim bundle do not describe the same claim.
    ManifestMismatch { field: &'static str },
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidField { field, reason } => {
                write!(formatter, "invalid {field}: {reason}")
            }
            Self::FieldTooLong { field, length } => {
                write!(formatter, "{field} is too long to encode ({length} bytes)")
            }
            Self::ManifestMismatch { field } => {
                write!(
                    formatter,
                    "manifest does not match claim bundle field {field}"
                )
            }
        }
    }
}

impl std::error::Error for Error {}

/// Result type used by the public WRAITHPHONE API.
pub type Result<T> = std::result::Result<T, Error>;
