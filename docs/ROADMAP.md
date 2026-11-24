# RUSTED AI — Development Roadmap

**Baseline:** 2026-11-24  
**Planning rule:** Milestones are dependency-ordered. Do not start a later milestone by silently bypassing an earlier milestone's exit criteria.

## Milestone map

| Milestone | Name | Depends on | Target outcome |
|---|---|---|---|
| M0 | Rust workspace foundation | None | Reproducible Rust workspace, CI, config, tracing, docs governance. |
| M1 | Local intelligence runtime | M0 | Exact Gemma 4 model loads through embedded mistral.rs for text and images. |
| M2 | Agent core and policy | M0, M1 | Typed plans, task state machine, deterministic policy, tool-call validation. |
| M3 | VM and isolation | M0, M2 | QEMU/KVM Ubuntu guest lifecycle and tested host/guest boundary. |
| M4 | Computer tool runtime | M1, M2, M3 | Screenshot, keyboard, mouse, shell, filesystem, and browser-through-GUI tools. |
| M5 | Workspace broker and Control Room | M2, M3, M4 | Task-scoped file staging/copy-back and local Leptos/Axum supervision UI. |
| M6 | Reliability and security validation | M1–M5 | Recovery, audit, performance, permission, isolation, and end-to-end test gates. |

## M0 — Rust workspace foundation

**Deliverables**
- Cargo workspace with the crate boundaries in `SYSTEM_ARCHITECTURE.md`.
- Formatting, lint, test, and CI baseline.
- Configuration loading with safe defaults and validation.
- `tracing` initialization and structured error conventions.
- SQLite schema/migration baseline may be scaffolded, but task semantics belong to M2/M5.

**Exit criteria**
- `cargo fmt --all -- --check` passes.
- `cargo clippy --workspace --all-targets -- -D warnings` passes.
- `cargo test --workspace` passes.
- CI runs the same checks on a clean checkout.
- No application code in Python, JavaScript, TypeScript, or Node.js.

## M1 — Local intelligence runtime

**Deliverables**
- Embedded mistral.rs Rust SDK adapter.
- Exact Gemma 4 Q4_K_M GGUF and matching BF16 mmproj loader.
- Configured CUDA build for supported NVIDIA hardware, with a documented CPU build profile for unsupported hardware.
- Text and image input paths; model outputs returned as typed application values.

**Exit criteria**
- Model and projector load from local disk with network disabled after assets are provisioned.
- A text prompt returns a valid response.
- A test screenshot is accepted and described by the same model instance.
- Tool-call-shaped output can be parsed and schema-validated; malformed output is rejected safely.
- VRAM, load time, time-to-first-token, decode rate, and image latency are recorded on target hardware.
- Exact artifact compatibility is marked `Verified` only after the test run is recorded.

## M2 — Agent core and policy

**Deliverables**
- Versioned task/plan/tool request/result data types.
- Explicit task state machine.
- Deterministic Rust policy and permission engine.
- Tool registry with typed request validation, deadlines, cancellation, and audit events.
- Bounded retry rules.

**Exit criteria**
- State transition tests cover legal and illegal transitions.
- Denied actions cannot reach tool adapters.
- Malformed model requests are rejected without execution.
- Non-idempotent actions are not automatically retried.
- Each task has a traceable event history.

## M3 — VM and isolation

**Deliverables**
- QEMU/KVM manager using a Unix-domain QMP socket.
- Versioned Ubuntu Desktop guest image build instructions.
- Start, stop, reset, snapshot, and health-check operations.
- Private guest management network and SSH key lifecycle.
- Default network-denied guest profile and separately approved outbound profile.

**Exit criteria**
- Guest can be started, queried, reset, and stopped through QMP.
- QMP is not exposed on a TCP listener.
- No host home directory is mounted into the guest.
- Guest egress is denied in default mode; approved network mode is tested separately.
- Host/guest management access is authenticated and scoped.

## M4 — Computer tool runtime

**Deliverables**
- Guest screenshot capture adapter.
- QMP keyboard/mouse input adapter.
- Guest shell and filesystem adapters.
- Browser interaction through the guest's visible browser UI for the initial release.
- Observe-act-observe orchestration hooks.

**Exit criteria**
- Each tool passes schema, permission, timeout, and integration tests.
- A GUI task can capture a screenshot, act, capture again, and compare the observed outcome.
- Shell execution is guest-only.
- Tool results include exit/status information and are not interpreted as task completion without verification.

## M5 — Workspace broker and Control Room

**Deliverables**
- Task-scoped workspace snapshot transfer into the guest.
- Changed-file manifest and diff generation.
- Human-reviewed copy-back/commit path.
- Leptos UI served by Axum on loopback only.
- Local task history, event log, screenshots, approvals, and result view.

**Exit criteria**
- A task cannot access host paths outside its explicit workspace grant.
- Guest changes are not copied back without the broker policy.
- Control Room refuses non-loopback binding in the initial release.
- User can submit, pause/cancel, approve/reject, and inspect a task.

## M6 — Reliability and security validation

**Deliverables**
- Failure recovery and cancellation tests.
- Threat-model validation suite.
- Performance baseline and regression tracking.
- End-to-end representative workflows.
- Release readiness checklist and operator documentation.

**Exit criteria**
- All critical and high security tests pass.
- End-to-end tasks verify final outcomes, not only tool-call results.
- Recovery is bounded and does not repeat irreversible actions.
- Benchmarks include the hardware, model revision, runtime version, and configuration.
- Remaining limitations are documented before release.

## Change control

The milestone order is fixed unless an accepted ADR changes a dependency. Scope may be split into smaller issues, but acceptance criteria must not be weakened merely to mark a milestone complete. Dates are intentionally omitted until capacity and hardware measurements are known.
