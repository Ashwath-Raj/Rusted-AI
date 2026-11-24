//! Structured `tracing` initialization for RUSTED AI.

use crate::config::{LogFormat, LoggingConfig};
use anyhow::Context;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// Initializes global structured logging from [`LoggingConfig`].
///
/// Uses `try_init` so repeated calls in integration tests are safe and non-panicking.
pub fn init_tracing(config: &LoggingConfig) -> anyhow::Result<()> {
    let filter = EnvFilter::try_new(&config.filter)
        .with_context(|| format!("invalid logging filter '{}'", config.filter))?;

    match config.format {
        LogFormat::Pretty => {
            let _ = tracing_subscriber::registry()
                .with(filter)
                .with(fmt::layer().with_target(true))
                .try_init();
        }
        LogFormat::Json => {
            let _ = tracing_subscriber::registry()
                .with(filter)
                .with(fmt::layer().json().with_target(true))
                .try_init();
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_tracing_succeeds_with_default_config() -> anyhow::Result<()> {
        let cfg = LoggingConfig::default();
        init_tracing(&cfg)?;
        Ok(())
    }
}
