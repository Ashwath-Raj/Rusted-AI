# ADR-0006: Task-Scoped Workspace Broker; No Live Host Folder Mount

- **Status:** Accepted
- **Date:** 2026-11-24

## Context

The agent must work with local development projects, but a live host-folder mount would give guest processes a direct path to modify host files and would weaken the intended boundary.

## Decision

**RUSTED AI will not live-mount the user's home directory or project folders into the guest. A Rust host workspace broker stages task-scoped copies into the guest and validates/approves changes before copy-back.**

## Alternatives considered

- Live virtiofs/shared-folder mount.
- Unrestricted SSH/SFTP access to host directories.
- No local project access.

## Consequences

- User grants a specific canonical workspace path and read/write mode.
- The broker copies a bounded snapshot to a task workspace and records hashes.
- The guest works only on the staged copy.
- Returned files are validated and converted into a diff/manifest.
- Copy-back is mediated by policy and requires approval according to risk.
- Symlink escape, path traversal, unsafe file types, and size limits are enforced.
- Large workspaces may require selective staging; this is an explicit user-visible limitation.

## Change rule

Introducing live shared folders or broader host access requires a new ADR and a demonstrated threat-model review. It must not be added as a convenience shortcut.
