//! M0 scaffolding for `rusted-tools`.
//!
//! **Target milestone:** M4 — Computer tool runtime.
//! **Owns:** Typed tool registry and adapters for guest shell/filesystem/GUI/browser actions.
//! **Must not own:** Unrestricted host filesystem or host shell access.

use rusted_types::ScaffoldDescriptor;
use thiserror::Error;

/// M0 scaffolding metadata for `rusted-tools`.
pub const SCAFFOLD: ScaffoldDescriptor = ScaffoldDescriptor {
    crate_name: "rusted-tools",
    target_milestone: "M4",
    responsibility: "Typed guest tool registry and shell/filesystem/GUI/browser adapters",
};

/// Errors produced by the tool runtime subsystem.
#[derive(Debug, Error)]
pub enum ToolError {
    /// Tool runtime is not yet implemented in M0.
    #[error("rusted-tools is M0 scaffolding; full implementation begins in M4")]
    NotImplemented,
}

/// Architectural crate dependencies wired for `rusted-tools`.
pub const DEPENDENCY_SCAFFOLDS: &[ScaffoldDescriptor] =
    &[rusted_vm::SCAFFOLD, rusted_workspace::SCAFFOLD];

/// Returns the M0 scaffolding descriptor for `rusted-tools`.
#[must_use]
pub const fn scaffold_info() -> ScaffoldDescriptor {
    SCAFFOLD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_identifies_m4_target_and_dependencies() {
        let info = scaffold_info();
        assert_eq!(info.crate_name, "rusted-tools");
        assert_eq!(info.target_milestone, "M4");
        assert_eq!(DEPENDENCY_SCAFFOLDS.len(), 2);
    }
}
