//! M0 scaffolding for `rusted-workspace`.
//!
//! **Target milestone:** M5 — Workspace broker and Control Room.
//! **Owns:** Host workspace grants, snapshot staging, manifests, diff and approved copy-back.
//! **Must not own:** Arbitrary host path access.

use rusted_types::ScaffoldDescriptor;
use thiserror::Error;

/// M0 scaffolding metadata for `rusted-workspace`.
pub const SCAFFOLD: ScaffoldDescriptor = ScaffoldDescriptor {
    crate_name: "rusted-workspace",
    target_milestone: "M5",
    responsibility: "Task-scoped host workspace staging, diffing, and approved copy-back",
};

/// Errors produced by the workspace broker subsystem.
#[derive(Debug, Error)]
pub enum WorkspaceError {
    /// Workspace broker is not yet implemented in M0.
    #[error("rusted-workspace is M0 scaffolding; full implementation begins in M5")]
    NotImplemented,
}

/// Returns the M0 scaffolding descriptor for `rusted-workspace`.
#[must_use]
pub const fn scaffold_info() -> ScaffoldDescriptor {
    SCAFFOLD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_identifies_m5_target() {
        let info = scaffold_info();
        assert_eq!(info.crate_name, "rusted-workspace");
        assert_eq!(info.target_milestone, "M5");
    }
}
