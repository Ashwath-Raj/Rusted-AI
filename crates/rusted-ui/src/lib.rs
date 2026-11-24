//! M0 scaffolding for `rusted-ui`.
//!
//! **Target milestone:** M5 — Workspace broker and Control Room (following owner UX intake).
//! **Owns:** Leptos components and user interactions.
//! **Must not own:** Inference/model loading or direct QMP/SSH operations.

use rusted_types::ScaffoldDescriptor;
use thiserror::Error;

/// M0 scaffolding metadata for `rusted-ui`.
pub const SCAFFOLD: ScaffoldDescriptor = ScaffoldDescriptor {
    crate_name: "rusted-ui",
    target_milestone: "M5",
    responsibility: "Leptos Control Room components and user supervision interactions",
};

/// Errors produced by the Control Room UI subsystem.
#[derive(Debug, Error)]
pub enum UiError {
    /// Control Room UI is not yet implemented in M0.
    #[error("rusted-ui is M0 scaffolding; owner UX intake and implementation begin in M5")]
    NotImplemented,
}

/// Returns the M0 scaffolding descriptor for `rusted-ui`.
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
        assert_eq!(info.crate_name, "rusted-ui");
        assert_eq!(info.target_milestone, "M5");
    }
}
