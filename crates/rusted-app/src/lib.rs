//! Application wiring, configuration, tracing initialization, and startup/shutdown
//! boundaries for RUSTED AI.

pub mod config;
pub mod logging;

use crate::config::AppConfig;
use rusted_types::ScaffoldDescriptor;
use std::path::Path;
use tracing::info;

/// Runtime handle produced by [`startup`], holding immutable validated configuration.
#[derive(Debug, Clone)]
pub struct AppRuntime {
    config: AppConfig,
}

impl AppRuntime {
    /// Returns the immutable validated application configuration.
    #[must_use]
    pub const fn config(&self) -> &AppConfig {
        &self.config
    }

    /// Returns the scaffolding descriptors for all wired subsystem crates.
    #[must_use]
    pub fn scaffold_inventory(&self) -> Vec<ScaffoldDescriptor> {
        vec![
            rusted_inference::scaffold_info(),
            rusted_policy::scaffold_info(),
            rusted_agent::scaffold_info(),
            rusted_tools::scaffold_info(),
            rusted_vm::scaffold_info(),
            rusted_workspace::scaffold_info(),
            rusted_storage::scaffold_info(),
            rusted_api::scaffold_info(),
            rusted_ui::scaffold_info(),
        ]
    }
}

/// Loads configuration, validates invariants, initializes `tracing`, and returns [`AppRuntime`].
pub fn startup(config_path: Option<&Path>) -> anyhow::Result<AppRuntime> {
    let config = AppConfig::load_or_default(config_path)?;
    logging::init_tracing(&config.logging)?;

    let runtime = AppRuntime { config };
    let scaffolds = runtime.scaffold_inventory();

    info!(
        schema_version = runtime.config().schema_version,
        bind_addr = %runtime.config().control_room.bind_addr,
        port = runtime.config().control_room.port,
        network_egress_enabled = runtime.config().vm.network_egress_enabled,
        scaffold_count = scaffolds.len(),
        "RUSTED AI M0 foundation initialized"
    );

    Ok(runtime)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn startup_wires_all_nine_subsystem_scaffolds() -> anyhow::Result<()> {
        let runtime = startup(None)?;
        let inventory = runtime.scaffold_inventory();
        assert_eq!(inventory.len(), 9);
        Ok(())
    }
}
