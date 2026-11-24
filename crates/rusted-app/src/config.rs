//! Explicit, validated application configuration for RUSTED AI.

use rusted_types::{CONTRACT_SCHEMA_VERSION, DomainError};
use serde::{Deserialize, Serialize};
use std::fs;
use std::net::{IpAddr, Ipv4Addr};
use std::path::{Path, PathBuf};
use tracing_subscriber::EnvFilter;

/// Top-level immutable runtime configuration loaded once at startup.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct AppConfig {
    /// Configuration contract schema version.
    pub schema_version: u32,
    /// Inference runtime and Gemma 4 model paths.
    pub inference: InferenceConfig,
    /// QEMU/KVM guest and QMP Unix socket settings.
    pub vm: VmConfig,
    /// Loopback-only Control Room server settings.
    pub control_room: ControlRoomConfig,
    /// Host workspace broker settings.
    pub workspace: WorkspaceConfig,
    /// Local SQLite and artifact storage settings.
    pub storage: StorageConfig,
    /// Bounded execution and resource limits.
    pub limits: ResourceLimitsConfig,
    /// Structured tracing configuration.
    pub logging: LoggingConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            schema_version: CONTRACT_SCHEMA_VERSION,
            inference: InferenceConfig::default(),
            vm: VmConfig::default(),
            control_room: ControlRoomConfig::default(),
            workspace: WorkspaceConfig::default(),
            storage: StorageConfig::default(),
            limits: ResourceLimitsConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

/// Inference asset configuration for the single Gemma 4 model and projector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct InferenceConfig {
    /// Local path to the Gemma 4 Q4_K_M GGUF model file.
    pub model_path: PathBuf,
    /// Local path to the matching BF16 vision projector file.
    pub mmproj_path: PathBuf,
    /// Maximum model context window in tokens.
    pub context_length: u32,
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            model_path: PathBuf::from("models/gemma4-12b-qat-uncensored.gguf"),
            mmproj_path: PathBuf::from("models/gemma4-12b-mmproj-bf16.gguf"),
            context_length: 8192,
        }
    }
}

/// QEMU/KVM guest VM configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct VmConfig {
    /// Path or binary name for the QEMU system emulator.
    pub qemu_binary: PathBuf,
    /// Path to the Ubuntu Desktop guest disk image or overlay.
    pub guest_image_path: PathBuf,
    /// Path to the local Unix-domain QMP socket (never TCP).
    pub qmp_socket_path: PathBuf,
    /// Outbound guest network access is disabled by default (FR-015).
    pub network_egress_enabled: bool,
    /// Guest RAM allocation in MiB.
    pub memory_mb: u32,
    /// Guest virtual CPU count.
    pub vcpus: u16,
}

impl Default for VmConfig {
    fn default() -> Self {
        Self {
            qemu_binary: PathBuf::from("qemu-system-x86_64"),
            guest_image_path: PathBuf::from("vm/ubuntu-desktop-guest.qcow2"),
            qmp_socket_path: PathBuf::from("state/qmp.sock"),
            network_egress_enabled: false,
            memory_mb: 8192,
            vcpus: 4,
        }
    }
}

/// Control Room HTTP/UI binding configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ControlRoomConfig {
    /// Bind address; must be loopback (`127.0.0.1` or `::1`) per ADR-0005 and NFR-005.
    pub bind_addr: IpAddr,
    /// Local TCP port for the Control Room server.
    pub port: u16,
}

impl Default for ControlRoomConfig {
    fn default() -> Self {
        Self {
            bind_addr: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port: 8484,
        }
    }
}

/// Host workspace broker configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct WorkspaceConfig {
    /// Directory used for task-scoped staged snapshots.
    pub staging_dir: PathBuf,
    /// Maximum allowed snapshot size in bytes.
    pub max_snapshot_bytes: u64,
    /// Explicitly granted host workspace root paths (empty by default).
    pub granted_paths: Vec<PathBuf>,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            staging_dir: PathBuf::from("state/workspace-staging"),
            max_snapshot_bytes: 256 * 1024 * 1024,
            granted_paths: Vec::new(),
        }
    }
}

/// Local SQLite and artifact storage configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct StorageConfig {
    /// Path to the local SQLite database file.
    pub database_path: PathBuf,
    /// Directory for large observation artifacts such as screenshots.
    pub artifact_dir: PathBuf,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            database_path: PathBuf::from("data/rusted-ai.sqlite3"),
            artifact_dir: PathBuf::from("artifacts"),
        }
    }
}

/// Bounded execution and resource limits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ResourceLimitsConfig {
    /// Maximum number of concurrent tasks.
    pub max_concurrent_tasks: usize,
    /// Default deadline in seconds for tool invocations (NFR-007).
    pub default_tool_timeout_secs: u64,
    /// Maximum screenshot payload size in bytes.
    pub max_screenshot_bytes: usize,
}

impl Default for ResourceLimitsConfig {
    fn default() -> Self {
        Self {
            max_concurrent_tasks: 1,
            default_tool_timeout_secs: 60,
            max_screenshot_bytes: 16 * 1024 * 1024,
        }
    }
}

/// Output format for structured `tracing` logs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogFormat {
    /// Human-readable terminal output.
    Pretty,
    /// Machine-parseable JSON lines.
    Json,
}

/// Structured logging configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct LoggingConfig {
    /// `tracing_subscriber::EnvFilter` directive string.
    pub filter: String,
    /// Log formatting mode.
    pub format: LogFormat,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            filter: "info".to_string(),
            format: LogFormat::Pretty,
        }
    }
}

impl AppConfig {
    /// Parses and validates configuration from a TOML string.
    pub fn from_toml_str(raw: &str) -> Result<Self, DomainError> {
        let config: Self = toml::from_str(raw).map_err(|err| DomainError::Invalid {
            message: format!("failed to parse TOML configuration: {err}"),
        })?;
        config.validate()?;
        Ok(config)
    }

    /// Loads, parses, and validates configuration from a TOML file on disk.
    pub fn load_from_file(path: &Path) -> Result<Self, DomainError> {
        let raw = fs::read_to_string(path).map_err(|err| DomainError::Invalid {
            message: format!("failed to read config file {}: {err}", path.display()),
        })?;
        Self::from_toml_str(&raw)
    }

    /// Loads configuration from `path` if provided, or validates and returns [`AppConfig::default`].
    pub fn load_or_default(path: Option<&Path>) -> Result<Self, DomainError> {
        match path {
            Some(file_path) => Self::load_from_file(file_path),
            None => {
                let defaults = Self::default();
                defaults.validate()?;
                Ok(defaults)
            }
        }
    }

    /// Validates all security, boundary, and resource invariants on the configuration.
    pub fn validate(&self) -> Result<(), DomainError> {
        if self.schema_version != CONTRACT_SCHEMA_VERSION {
            return Err(DomainError::Invalid {
                message: format!(
                    "unsupported config schema_version {} (expected {CONTRACT_SCHEMA_VERSION})",
                    self.schema_version
                ),
            });
        }

        if !self.control_room.bind_addr.is_loopback() {
            return Err(DomainError::Denied {
                message: format!(
                    "control_room.bind_addr '{}' must be a loopback address (127.0.0.1 or ::1)",
                    self.control_room.bind_addr
                ),
            });
        }

        if self.control_room.port == 0 {
            return Err(DomainError::Invalid {
                message: "control_room.port must be non-zero".to_string(),
            });
        }

        validate_non_empty_path(&self.inference.model_path, "inference.model_path")?;
        validate_non_empty_path(&self.inference.mmproj_path, "inference.mmproj_path")?;
        if self.inference.context_length == 0 {
            return Err(DomainError::Invalid {
                message: "inference.context_length must be greater than zero".to_string(),
            });
        }

        validate_non_empty_path(&self.vm.qemu_binary, "vm.qemu_binary")?;
        validate_non_empty_path(&self.vm.guest_image_path, "vm.guest_image_path")?;
        validate_non_empty_path(&self.vm.qmp_socket_path, "vm.qmp_socket_path")?;

        let qmp_lower = self
            .vm
            .qmp_socket_path
            .to_string_lossy()
            .to_ascii_lowercase();
        if ["tcp:", "http:", "https:", "ws:", "wss:"]
            .iter()
            .any(|prefix| qmp_lower.starts_with(prefix))
        {
            return Err(DomainError::Denied {
                message: "vm.qmp_socket_path must be a local Unix socket path, not a network URI"
                    .to_string(),
            });
        }

        if self.vm.memory_mb < 512 {
            return Err(DomainError::Invalid {
                message: "vm.memory_mb must be at least 512 MiB".to_string(),
            });
        }
        if self.vm.vcpus == 0 {
            return Err(DomainError::Invalid {
                message: "vm.vcpus must be at least 1".to_string(),
            });
        }

        validate_non_empty_path(&self.workspace.staging_dir, "workspace.staging_dir")?;
        if self.workspace.max_snapshot_bytes == 0 {
            return Err(DomainError::Invalid {
                message: "workspace.max_snapshot_bytes must be greater than zero".to_string(),
            });
        }

        validate_non_empty_path(&self.storage.database_path, "storage.database_path")?;
        validate_non_empty_path(&self.storage.artifact_dir, "storage.artifact_dir")?;

        if self.limits.max_concurrent_tasks == 0 {
            return Err(DomainError::Invalid {
                message: "limits.max_concurrent_tasks must be at least 1".to_string(),
            });
        }
        if self.limits.default_tool_timeout_secs == 0 {
            return Err(DomainError::Invalid {
                message: "limits.default_tool_timeout_secs must be greater than zero".to_string(),
            });
        }
        if self.limits.max_screenshot_bytes < 1024 {
            return Err(DomainError::Invalid {
                message: "limits.max_screenshot_bytes must be at least 1024 bytes".to_string(),
            });
        }

        EnvFilter::try_new(&self.logging.filter).map_err(|err| DomainError::Invalid {
            message: format!("invalid logging.filter '{}': {err}", self.logging.filter),
        })?;

        Ok(())
    }
}

fn validate_non_empty_path(path: &Path, field_name: &str) -> Result<(), DomainError> {
    if path.as_os_str().is_empty() {
        return Err(DomainError::Invalid {
            message: format!("{field_name} must not be empty"),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_valid_and_secure() -> Result<(), Box<dyn std::error::Error>> {
        let cfg = AppConfig::load_or_default(None)?;
        assert!(cfg.control_room.bind_addr.is_loopback());
        assert!(!cfg.vm.network_egress_enabled);
        assert!(cfg.workspace.granted_paths.is_empty());
        Ok(())
    }

    #[test]
    fn rejects_non_loopback_bind_address() {
        let raw = r#"
[control_room]
bind_addr = "0.0.0.0"
"#;
        let err = AppConfig::from_toml_str(raw);
        assert!(matches!(err, Err(DomainError::Denied { .. })));
    }

    #[test]
    fn rejects_tcp_qmp_socket_path() {
        let raw = r#"
[vm]
qmp_socket_path = "tcp:127.0.0.1:4444"
"#;
        let err = AppConfig::from_toml_str(raw);
        assert!(matches!(err, Err(DomainError::Denied { .. })));
    }

    #[test]
    fn rejects_unknown_toml_fields() {
        let raw = r#"
unknown_field = true
"#;
        let err = AppConfig::from_toml_str(raw);
        assert!(matches!(err, Err(DomainError::Invalid { .. })));
    }

    #[test]
    fn rejects_invalid_log_filter() {
        let raw = r#"
[logging]
filter = "invalid[directive"
"#;
        let err = AppConfig::from_toml_str(raw);
        assert!(matches!(err, Err(DomainError::Invalid { .. })));
    }

    #[test]
    fn rejects_out_of_range_numeric_limits() {
        let cases = [
            "[control_room]\nport = 0\n",
            "[vm]\nmemory_mb = 256\n",
            "[vm]\nvcpus = 0\n",
            "[limits]\nmax_concurrent_tasks = 0\n",
            "[limits]\ndefault_tool_timeout_secs = 0\n",
            "[limits]\nmax_screenshot_bytes = 512\n",
        ];

        for raw in cases {
            let err = AppConfig::from_toml_str(raw);
            assert!(
                matches!(err, Err(DomainError::Invalid { .. })),
                "expected Invalid error for config:\n{raw}"
            );
        }
    }

    #[test]
    fn accepts_json_log_format_and_rejects_missing_file() -> Result<(), Box<dyn std::error::Error>>
    {
        let raw = r#"
[logging]
filter = "debug"
format = "json"
"#;
        let cfg = AppConfig::from_toml_str(raw)?;
        assert_eq!(cfg.logging.format, LogFormat::Json);
        assert_eq!(cfg.logging.filter, "debug");

        let missing = AppConfig::load_from_file(Path::new("/nonexistent/rusted-ai-config.toml"));
        assert!(matches!(missing, Err(DomainError::Invalid { .. })));
        Ok(())
    }
}
