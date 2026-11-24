# RUSTED AI — Project Status

**Last updated:** 2026-11-24  
**Status source:** Repository inspection plus this documentation baseline

## Overall status

- **Current phase:** M0 — Rust workspace foundation (`Implemented`, awaiting commit and clean-checkout CI run before marking `Verified`)
- **Project health:** 11-crate Rust workspace, validated configuration, structured `tracing`, typed errors, `Cargo.lock`, `.gitignore`, and CI workflow implemented locally
- **Current release:** None
- **Latest verified application commit:** None recorded (working tree built on base commit `e57e173`)

## Subsystem status

| Subsystem | Status | Evidence / next proof |
|---|---|---|
| Product scope and fixed decisions | Implemented | Charter and accepted ADRs in this documentation baseline. |
| Rust workspace | Implemented | 11-crate workspace on `rustc 1.100.0 (0ef1bc386 2026-11-09)`; `cargo fmt --all -- --check`, `cargo check --workspace --locked`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked` (22 passed, 0 failed), and `cargo run -p rusted-app --locked -- --check-config` all passed locally on uncommitted working tree. Next proof: commit and pass `.github/workflows/ci.yml` on clean checkout. |
| mistral.rs integration | Not Started | M0 scaffolding only (`rusted-inference`); load exact model and projector from local files in M1. |
| Gemma 4 text inference | Not Started | Repeatable local text smoke test in M1. |
| Gemma 4 vision inference | Not Started | Screenshot/image input smoke test in M1. |
| Agent orchestrator | Not Started | M0 scaffolding only (`rusted-agent`); unit-tested task state machine and plan execution in M2. |
| Deterministic policy engine | Not Started | M0 scaffolding only (`rusted-policy`); permission/denial integration tests in M2. |
| QEMU/KVM manager | Not Started | M0 scaffolding only (`rusted-vm`); start/stop/reset disposable guest through QMP in M3. |
| Guest shell and filesystem tools | Not Started | M0 scaffolding only (`rusted-tools`); guest-only command/file tests in M4. |
| Screenshot/keyboard/mouse tools | Not Started | M0 scaffolding only (`rusted-tools`); observe-act-observe GUI integration test in M4. |
| Workspace broker | Not Started | M0 scaffolding only (`rusted-workspace`); staging, diff, approval, and copy-back tests in M5. |
| Leptos + Axum Control Room | Not Started | M0 scaffolding only (`rusted-api`, `rusted-ui`); owner UX intake and loopback task submission/monitoring test in M5. |
| SQLite persistence and audit log | Not Started | M0 scaffolding only (`rusted-storage`); restart-safe task history tests in M2/M5. |
| Security hardening | Not Started | Threat-model acceptance test suite in M6. |

## Active work

M0 Rust workspace foundation implemented locally on top of `e57e173` (uncommitted, awaiting owner review and clean-checkout CI execution).

**M0 local verification evidence:**
- **Base commit SHA:** `e57e173` (plus uncommitted M0 changes)
- **Rust toolchain:** `rustc 1.100.0 (0ef1bc386 2026-11-09)` / `cargo 1.100.0 (3acc79c0c 2026-11-05)` (`rust-toolchain.toml`)
- **Commands executed (all exit code 0):**
  - `cargo fmt --all -- --check`
  - `cargo check --workspace --locked`
  - `cargo clippy --workspace --all-targets --locked -- -D warnings`
  - `cargo test --workspace --locked` (22 unit tests passed, 0 failed, 0 ignored)
  - `cargo run -p rusted-app --locked -- --check-config`
- **Known limitations:**
  - Changes are uncommitted per review workflow; GitHub Actions CI (`.github/workflows/ci.yml`) has not yet run on a clean checkout.
  - M1–M6 subsystem crates are M0 scaffolding (`ScaffoldDescriptor` + `*Error::NotImplemented`) and do not yet implement runtime inference, policy evaluation, VM control, tool execution, SQLite persistence, or Control Room UI.

## Blockers and risks

1. The exact Gemma 4 GGUF + BF16 vision projector pair must be smoke-tested through the pinned mistral.rs Rust SDK.
2. GPU memory use, context length, screenshot resolution, and inference latency need measurements on the target host hardware.
3. The guest image, QMP configuration, screenshot capture method, and workspace transfer path need integration tests.

These are validation tasks, not invitations to choose a different architecture informally.

## Next milestone

**M0 — Rust workspace foundation**

Exit criteria are defined in `docs/ROADMAP.md`. No subsystem should be marked `Verified` without a test result and a recorded commit/environment.

## Update rules

- Update this file in the same pull request that changes a subsystem's implementation status.
- Include commit SHA, test command, result, and environment for each `Verified` status.
- Do not use subjective progress percentages.
- Keep milestone descriptions in `ROADMAP.md`; keep requirements in `REQUIREMENTS.md`.
