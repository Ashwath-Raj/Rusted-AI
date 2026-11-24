//! Shared versioned domain types, identifiers, and error taxonomy for RUSTED AI.
//!
//! `rusted-types` is the lowest-level contract crate. It must not perform OS calls,
//! model loading, network I/O, or database access.

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Current schema version for serialized RUSTED AI domain contracts.
pub const CONTRACT_SCHEMA_VERSION: u32 = 1;

/// Deterministic error categories required by the RUSTED AI engineering standard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCategory {
    /// Action or access was denied by policy or validation boundary.
    Denied,
    /// Input, schema, or configuration failed validation.
    Invalid,
    /// Operation exceeded its configured deadline.
    Timeout,
    /// Temporary failure that may succeed if safely retried (for idempotent actions only).
    Transient,
    /// Non-recoverable failure for the current operation.
    Permanent,
    /// Operation was explicitly cancelled before completion.
    Cancelled,
    /// Side effect may have occurred; observation/reconciliation is required before retry.
    UnknownOutcome,
}

/// Shared domain error type for RUSTED AI libraries.
#[derive(Debug, Error)]
pub enum DomainError {
    /// Policy or capability check denied the operation.
    #[error("denied: {message}")]
    Denied {
        /// Human-readable reason for denial.
        message: String,
    },

    /// Input or configuration validation failed.
    #[error("invalid input: {message}")]
    Invalid {
        /// Human-readable validation failure detail.
        message: String,
    },

    /// Operation exceeded its deadline.
    #[error("operation timed out: {message}")]
    Timeout {
        /// Human-readable timeout context.
        message: String,
    },

    /// Transient operational error.
    #[error("transient failure: {message}")]
    Transient {
        /// Human-readable transient failure detail.
        message: String,
    },

    /// Permanent operational error.
    #[error("permanent failure: {message}")]
    Permanent {
        /// Human-readable permanent failure detail.
        message: String,
    },

    /// Explicit cancellation.
    #[error("cancelled: {message}")]
    Cancelled {
        /// Human-readable cancellation detail.
        message: String,
    },

    /// Outcome after a side effect is unknown and requires observation.
    #[error("unknown outcome: {message}")]
    UnknownOutcome {
        /// Human-readable reconciliation context.
        message: String,
    },
}

impl DomainError {
    /// Returns the deterministic [`ErrorCategory`] for this error.
    #[must_use]
    pub const fn category(&self) -> ErrorCategory {
        match self {
            Self::Denied { .. } => ErrorCategory::Denied,
            Self::Invalid { .. } => ErrorCategory::Invalid,
            Self::Timeout { .. } => ErrorCategory::Timeout,
            Self::Transient { .. } => ErrorCategory::Transient,
            Self::Permanent { .. } => ErrorCategory::Permanent,
            Self::Cancelled { .. } => ErrorCategory::Cancelled,
            Self::UnknownOutcome { .. } => ErrorCategory::UnknownOutcome,
        }
    }
}

/// Strongly typed, non-empty task identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct TaskId(String);

impl TaskId {
    /// Creates a new [`TaskId`], rejecting empty or whitespace-only values.
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let raw = value.into();
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err(DomainError::Invalid {
                message: "task_id must not be empty".to_string(),
            });
        }
        Ok(Self(trimmed.to_string()))
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for TaskId {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<TaskId> for String {
    fn from(value: TaskId) -> Self {
        value.0
    }
}

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Versioned envelope for externally serialized or persisted contracts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VersionedContract<T> {
    /// Schema version of the serialized contract.
    pub schema_version: u32,
    /// Contract payload.
    pub payload: T,
}

impl<T> VersionedContract<T> {
    /// Wraps a payload with [`CONTRACT_SCHEMA_VERSION`].
    #[must_use]
    pub const fn new(payload: T) -> Self {
        Self {
            schema_version: CONTRACT_SCHEMA_VERSION,
            payload,
        }
    }
}

/// Explicit scaffolding metadata for subsystem crates awaiting later milestones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldDescriptor {
    /// Crate name.
    pub crate_name: &'static str,
    /// Target milestone where full implementation begins.
    pub target_milestone: &'static str,
    /// Short description of the crate's owned responsibility.
    pub responsibility: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_id_rejects_empty_and_whitespace() {
        assert!(matches!(TaskId::new(""), Err(DomainError::Invalid { .. })));
        assert!(matches!(
            TaskId::new("   "),
            Err(DomainError::Invalid { .. })
        ));
    }

    #[test]
    fn task_id_trims_and_roundtrips_json() -> Result<(), Box<dyn std::error::Error>> {
        let id = TaskId::new(" task-001 ")?;
        assert_eq!(id.as_str(), "task-001");

        let contract = VersionedContract::new(id);
        let json = serde_json::to_string(&contract)?;
        assert!(json.contains("\"schema_version\":1"));

        let decoded: VersionedContract<TaskId> = serde_json::from_str(&json)?;
        assert_eq!(decoded.schema_version, CONTRACT_SCHEMA_VERSION);
        assert_eq!(decoded.payload.as_str(), "task-001");
        Ok(())
    }

    #[test]
    fn versioned_contract_rejects_unknown_fields() {
        let raw = r#"{"schema_version":1,"payload":"task-1","extra":true}"#;
        let parsed: Result<VersionedContract<TaskId>, _> = serde_json::from_str(raw);
        assert!(parsed.is_err());
    }

    #[test]
    fn domain_error_maps_all_categories() {
        let cases = [
            (
                DomainError::Denied {
                    message: "a".into(),
                },
                ErrorCategory::Denied,
            ),
            (
                DomainError::Invalid {
                    message: "b".into(),
                },
                ErrorCategory::Invalid,
            ),
            (
                DomainError::Timeout {
                    message: "c".into(),
                },
                ErrorCategory::Timeout,
            ),
            (
                DomainError::Transient {
                    message: "d".into(),
                },
                ErrorCategory::Transient,
            ),
            (
                DomainError::Permanent {
                    message: "e".into(),
                },
                ErrorCategory::Permanent,
            ),
            (
                DomainError::Cancelled {
                    message: "f".into(),
                },
                ErrorCategory::Cancelled,
            ),
            (
                DomainError::UnknownOutcome {
                    message: "g".into(),
                },
                ErrorCategory::UnknownOutcome,
            ),
        ];

        for (err, expected) in cases {
            assert_eq!(err.category(), expected);
        }
    }
}
