//! M0 scaffolding for `rusted-agent`.
//!
//! **Target milestone:** M2 — Agent core and policy.
//! **Owns:** Task lifecycle, planning loop, state machine, and recovery coordination.
//! **Must not own:** Raw QMP/SSH details or UI rendering.

use rusted_types::ScaffoldDescriptor;
use thiserror::Error;

/// M0 scaffolding metadata for `rusted-agent`.
pub const SCAFFOLD: ScaffoldDescriptor = ScaffoldDescriptor {
    crate_name: "rusted-agent",
    target_milestone: "M2",
    responsibility: "Task lifecycle, planning loop, state machine, and recovery coordination",
};

/// Errors produced by the agent orchestrator subsystem.
#[derive(Debug, Error)]
pub enum AgentError {
    /// Agent orchestrator is not yet implemented in M0.
    #[error("rusted-agent is M0 scaffolding; full implementation begins in M2")]
    NotImplemented,
}

/// Architectural crate dependencies wired for `rusted-agent`.
pub const DEPENDENCY_SCAFFOLDS: &[ScaffoldDescriptor] = &[
    rusted_inference::SCAFFOLD,
    rusted_policy::SCAFFOLD,
    rusted_storage::SCAFFOLD,
    rusted_tools::SCAFFOLD,
];

/// Returns the M0 scaffolding descriptor for `rusted-agent`.
#[must_use]
pub const fn scaffold_info() -> ScaffoldDescriptor {
    SCAFFOLD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_identifies_m2_target_and_dependencies() {
        let info = scaffold_info();
        assert_eq!(info.crate_name, "rusted-agent");
        assert_eq!(info.target_milestone, "M2");
        assert_eq!(DEPENDENCY_SCAFFOLDS.len(), 4);
    }
}
