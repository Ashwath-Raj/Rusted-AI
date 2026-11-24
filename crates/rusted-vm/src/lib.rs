//! M0 scaffolding for `rusted-vm`.
//!
//! **Target milestone:** M3 — VM and isolation.
//! **Owns:** QEMU process configuration, Unix-domain QMP client, guest health and communication.
//! **Must not own:** Model prompts or task planning.

use rusted_types::ScaffoldDescriptor;
use thiserror::Error;

/// M0 scaffolding metadata for `rusted-vm`.
pub const SCAFFOLD: ScaffoldDescriptor = ScaffoldDescriptor {
    crate_name: "rusted-vm",
    target_milestone: "M3",
    responsibility: "QEMU/KVM lifecycle, Unix-domain QMP client, and guest communication",
};

/// Errors produced by the VM subsystem.
#[derive(Debug, Error)]
pub enum VmError {
    /// VM management is not yet implemented in M0.
    #[error("rusted-vm is M0 scaffolding; full implementation begins in M3")]
    NotImplemented,
}

/// Returns the M0 scaffolding descriptor for `rusted-vm`.
#[must_use]
pub const fn scaffold_info() -> ScaffoldDescriptor {
    SCAFFOLD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_identifies_m3_target() {
        let info = scaffold_info();
        assert_eq!(info.crate_name, "rusted-vm");
        assert_eq!(info.target_milestone, "M3");
    }
}
