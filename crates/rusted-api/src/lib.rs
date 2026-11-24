//! M0 scaffolding for `rusted-api`.
//!
//! **Target milestone:** M5 — Workspace broker and Control Room.
//! **Owns:** Axum routes, request validation, loopback server, and UI/API boundary.
//! **Must not own:** Direct privileged OS execution.

use rusted_types::ScaffoldDescriptor;
use thiserror::Error;

/// M0 scaffolding metadata for `rusted-api`.
pub const SCAFFOLD: ScaffoldDescriptor = ScaffoldDescriptor {
    crate_name: "rusted-api",
    target_milestone: "M5",
    responsibility: "Loopback-only Axum control API and request validation",
};

/// Errors produced by the Control Room API subsystem.
#[derive(Debug, Error)]
pub enum ApiError {
    /// Control API is not yet implemented in M0.
    #[error("rusted-api is M0 scaffolding; full implementation begins in M5")]
    NotImplemented,
}

/// Architectural crate dependencies wired for `rusted-api`.
pub const DEPENDENCY_SCAFFOLDS: &[ScaffoldDescriptor] = &[rusted_agent::SCAFFOLD];

/// Returns the M0 scaffolding descriptor for `rusted-api`.
#[must_use]
pub const fn scaffold_info() -> ScaffoldDescriptor {
    SCAFFOLD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_identifies_m5_target_and_dependencies() {
        let info = scaffold_info();
        assert_eq!(info.crate_name, "rusted-api");
        assert_eq!(info.target_milestone, "M5");
        assert_eq!(DEPENDENCY_SCAFFOLDS.len(), 1);
    }
}
