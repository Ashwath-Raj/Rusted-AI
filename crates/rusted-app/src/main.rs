//! Binary entry point for RUSTED AI.

use anyhow::{Context, bail};
use std::env;
use std::path::PathBuf;
use tracing::info;

#[derive(Debug, Default)]
struct CliOptions {
    config_path: Option<PathBuf>,
    check_config: bool,
}

fn parse_cli_args() -> anyhow::Result<CliOptions> {
    let mut options = CliOptions {
        config_path: env::var_os("RUSTED_AI_CONFIG").map(PathBuf::from),
        check_config: false,
    };

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--config" => {
                let value = args.next().context("--config requires a file path")?;
                options.config_path = Some(PathBuf::from(value));
            }
            "--check-config" => {
                options.check_config = true;
            }
            other => {
                bail!("unsupported argument: {other}");
            }
        }
    }

    Ok(options)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let options = parse_cli_args()?;
    let runtime = rusted_app::startup(options.config_path.as_deref())?;

    if options.check_config {
        info!("configuration check passed; exiting cleanly");
        return Ok(());
    }

    info!(
        subsystems = runtime.scaffold_inventory().len(),
        "M0 runtime ready; awaiting shutdown signal (Ctrl-C)"
    );
    tokio::signal::ctrl_c()
        .await
        .context("failed waiting for shutdown signal")?;
    info!("shutdown signal received; exiting cleanly");

    Ok(())
}
