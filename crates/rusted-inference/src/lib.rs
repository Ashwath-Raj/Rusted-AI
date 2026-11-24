//! M0 scaffolding for `rusted-inference`.
//!
//! **Target milestone:** M1 — Local intelligence runtime.
//! **Owns:** Embedded `mistral.rs` adapter, Gemma 4 model/vision projector loading,
//! prompt assembly, image input, and generation configuration.
//! **Must not own:** Tool authorization or direct tool execution.

use rusted_types::ScaffoldDescriptor;
use thiserror::Error;

/// M0 scaffolding metadata for `rusted-inference`.
pub const SCAFFOLD: ScaffoldDescriptor = ScaffoldDescriptor {
    crate_name: "rusted-inference",
    target_milestone: "M1",
    responsibility: "Embedded mistral.rs adapter for Gemma 4 text and vision inference",
};

/// Errors produced by the inference subsystem.
#[derive(Debug, Error)]
pub enum InferenceError {
    /// Inference runtime is not yet implemented in M0.
    #[error("rusted-inference is M0 scaffolding; full implementation begins in M1")]
    NotImplemented,
}

/// Returns the M0 scaffolding descriptor for `rusted-inference`.
#[must_use]
pub const fn scaffold_info() -> ScaffoldDescriptor {
    SCAFFOLD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_identifies_m1_target() {
        let info = scaffold_info();
        assert_eq!(info.crate_name, "rusted-inference");
        assert_eq!(info.target_milestone, "M1");
    }
}
