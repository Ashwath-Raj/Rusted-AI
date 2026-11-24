# RUSTED AI — Rust Engineering Standard

**Applies to:** All first-party Rust crates  
**Status:** Initial standard

## 1. Workspace structure

Use the crate boundaries from `docs/architecture/SYSTEM_ARCHITECTURE.md`. Do not create a catch-all `utils` crate. Shared types belong in `rusted-types`; domain behavior belongs in the crate that owns the domain.

## 2. Language and dependencies

- Rust stable toolchain, pinned in `rust-toolchain.toml` once M0 selects the tested release.
- Cargo workspace with a committed `Cargo.lock`.
- Use Tokio for asynchronous I/O and task coordination.
- Use Serde for serialized contracts.
- Use thiserror for domain/library errors.
- Use anyhow only at executable or top-level orchestration boundaries where adding context is useful.
- Use tracing for structured logs and spans.
- Use SQLx + SQLite for persistence.
- Use Axum + Leptos for the local Control Room.
- Use mistral.rs Rust SDK for embedded inference.
- Every new dependency must have an owning crate, reason, maintenance/license check, and an alternative considered.

## 3. Async and concurrency

- Never block Tokio worker threads with long inference, filesystem, or VM operations. Use the runtime's supported blocking/async boundaries.
- Use bounded channels. Unbounded queues are prohibited for user- or model-controlled inputs.
- Propagate cancellation tokens and deadlines through task, step, and tool layers.
- Protect mutable shared state with the narrowest synchronization primitive that fits. Do not hold locks across inference, network, database, or VM awaits.
- Keep task execution concurrency bounded; do not run unlimited concurrent model generations or VM tools.

## 4. API and data contracts

- All model/tool/API inputs are untrusted, including values produced by the local model.
- Prefer enums and newtypes to loosely typed strings and numbers.
- Add a schema version to persisted or externally serialized contracts.
- Reject unknown tool names and invalid fields. Do not silently coerce dangerous values.
- Set explicit limits for strings, arrays, file sizes, screenshot sizes, and generated output.
- Do not expose internal structs as public API without a stability reason.

## 5. Error handling

- Return `Result`; do not `unwrap()`/`expect()` on runtime-controlled input or external I/O.
- Panics are defects, not normal error handling.
- Add context at system boundaries while preserving typed error categories.
- Distinguish denied, invalid, timeout, transient, permanent, cancelled, and unknown-outcome errors.
- Unknown outcome after a side effect must trigger observation/reconciliation, not blind retry.

## 6. Security rules

- Do not use shell string interpolation for model-controlled data.
- Do not pass arbitrary model text into a command interpreter.
- Build command arguments as explicit `OsString`/argument vectors for fixed guest tools.
- Canonicalize and scope filesystem operations; account for symlinks and path races.
- QMP is accessible only to `rusted-vm`; tools receive typed capabilities, not the raw socket.
- Host filesystem operations are restricted to `rusted-workspace` and granted roots.
- Secrets must not enter tracing fields, prompts, screenshots, or persisted tool outputs without an explicit design and approval.

## 7. Code quality gates

Required before merge:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Add targeted tests for each changed behavior. For runtime/model/VM work, include the relevant smoke or integration test; compile success alone is not sufficient.

## 8. Comments and documentation

- Comment the reason for non-obvious safety checks and invariants, not every line.
- Document public traits and types, especially tool interfaces and state transitions.
- When behavior changes, update the owning document identified by `docs/INDEX.md`.
- Do not duplicate full architecture explanations in module READMEs. Link to canonical docs.
