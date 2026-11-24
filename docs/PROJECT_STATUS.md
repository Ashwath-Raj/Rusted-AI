# RUSTED AI — Project Status

**Last updated:** 2026-11-24  
**Status source:** Repository inspection plus this documentation baseline

## Overall status

- **Current phase:** M0 — Documentation and workspace foundation
- **Project health:** Architecture baseline established; application implementation not yet evidenced
- **Current release:** None
- **Latest verified application commit:** None recorded

At the repository inspection used to prepare this kit, the repository contained a README and no visible application source tree. This document must be updated after the documentation package is installed and the repository is audited. Do not infer implementation from architecture prose.

## Subsystem status

| Subsystem | Status | Evidence / next proof |
|---|---|---|
| Product scope and fixed decisions | Implemented | Charter and accepted ADRs in this documentation baseline. |
| Rust workspace | Not Started | Create workspace and pass `cargo check --workspace`. |
| mistral.rs integration | Not Started | Load exact model and projector from local files. |
| Gemma 4 text inference | Not Started | Repeatable local text smoke test. |
| Gemma 4 vision inference | Not Started | Screenshot/image input smoke test. |
| Agent orchestrator | Not Started | Unit-tested task state machine and plan execution. |
| Deterministic policy engine | Not Started | Permission/denial integration tests. |
| QEMU/KVM manager | Not Started | Start/stop/reset disposable guest through QMP. |
| Guest shell and filesystem tools | Not Started | Guest-only command/file tests. |
| Screenshot/keyboard/mouse tools | Not Started | Observe-act-observe GUI integration test. |
| Workspace broker | Not Started | Staging, diff, approval, and copy-back tests. |
| Leptos + Axum Control Room | Not Started | Loopback task submission and monitoring test. |
| SQLite persistence and audit log | Not Started | Restart-safe task history tests. |
| Security hardening | Not Started | Threat-model acceptance test suite. |

## Active work

No application implementation task is recorded at this baseline.

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
