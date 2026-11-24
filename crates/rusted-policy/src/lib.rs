//! M0 scaffolding for `rusted-policy`.
//!
//! **Target milestone:** M2 — Agent core and policy.
//! **Owns:** Capability checks, risk tiers, approvals, and deterministic authorization.
//! **Must not own:** Model inference or shell execution.

use rusted_types::ScaffoldDescriptor;
use thiserror::Error;

/// M0 scaffolding metadata for `rusted-policy`.
pub const SCAFFOLD: ScaffoldDescriptor = ScaffoldDescriptor {
    crate_name: "rusted-policy",
    target_milestone: "M2",
    responsibility: "Deterministic Rust capability checks, risk tiers, and authorization",
};

/// Errors produced by the policy subsystem.
#[derive(Debug, Error)]
pub enum PolicyError {
    /// Policy evaluation is not yet implemented in M0.
    #[error("rusted-policy is M0 scaffolding; full implementation begins in M2")]
    NotImplemented,
}

/// Returns the M0 scaffolding descriptor for `rusted-policy`.
#[must_use]
pub const fn scaffold_info() -> ScaffoldDescriptor {
    SCAFFOLD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_identifies_m2_target() {
        let info = scaffold_info();
        assert_eq!(info.crate_name, "rusted-policy");
        assert_eq!(info.target_milestone, "M2");
    }
}
