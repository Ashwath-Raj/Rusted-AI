//! M0 scaffolding for `rusted-storage`.
//!
//! **Target milestone:** M2/M5 — SQLite persistence and audit log.
//! **Owns:** SQLite schema, migrations, repositories, and audit persistence.
//! **Must not own:** Agent policy decisions.

use rusted_types::ScaffoldDescriptor;
use thiserror::Error;

/// M0 scaffolding metadata for `rusted-storage`.
pub const SCAFFOLD: ScaffoldDescriptor = ScaffoldDescriptor {
    crate_name: "rusted-storage",
    target_milestone: "M2/M5",
    responsibility: "SQLite schema, migrations, task repositories, and audit persistence",
};

/// Errors produced by the storage subsystem.
#[derive(Debug, Error)]
pub enum StorageError {
    /// Storage layer is not yet implemented in M0.
    #[error("rusted-storage is M0 scaffolding; full implementation begins in M2/M5")]
    NotImplemented,
}

/// Returns the M0 scaffolding descriptor for `rusted-storage`.
#[must_use]
pub const fn scaffold_info() -> ScaffoldDescriptor {
    SCAFFOLD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_identifies_m2_m5_target() {
        let info = scaffold_info();
        assert_eq!(info.crate_name, "rusted-storage");
        assert_eq!(info.target_milestone, "M2/M5");
    }
}
