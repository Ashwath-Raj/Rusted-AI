# RUSTED AI — Testing Strategy

**Purpose:** Define the evidence required before features are called implemented or verified.

## 1. Test levels

| Level | What it covers | Environment |
|---|---|---|
| Unit | Pure state transitions, schema validation, policy evaluation, path normalization, retry classification. | Fast local tests; no VM/model required. |
| Component | Inference adapter, SQLite repository, QMP client, workspace manifest, tool adapters. | Local fixtures or disposable service/VM. |
| Integration | Policy-to-tool dispatch, task persistence, guest command execution, screenshot round-trip, workspace staging/copy-back. | Local host and disposable Ubuntu guest. |
| Model smoke | Exact model load, text output, image input, structured tool proposal. | Target model files and recorded hardware. |
| End-to-end | User instruction through plan, approval, execution, observation, verification, and report. | Full local stack and disposable guest. |
| Security | Denial paths, host/guest boundary, prompt injection, path traversal, network controls, output limits. | Isolated test environment. |
| Performance | Load time, inference latency, memory, screenshot time, VM boot, tool latency. | Named, recorded hardware/software profile. |

## 2. Required test principles

- Test behavior at the boundary where it matters. Do not mock away the policy engine in a security test.
- Prefer deterministic fixtures for state machine and policy tests.
- Do not use model-generated output as the only expected test oracle. Use fixed expected schemas, visible screenshot markers, file hashes, or known final state.
- Use disposable VM overlays for integration tests. Never run destructive tests against a user's daily VM or host workspace.
- Tests that need a model must declare that requirement and be skippable only in local developer workflows; CI must still run all non-model tests.
- Record model revision, projector hash, mistral.rs version, CUDA features, driver, GPU, and sampling profile for model tests.
- Every flaky test must have an issue and failure analysis. Do not hide flakes through unbounded retries.

## 3. Minimum test gates by milestone

### M0

Formatting, Clippy with warnings denied, workspace tests, clean CI build.

### M1

Exact artifact load; text smoke; image smoke; malformed tool-call rejection; offline reload; memory/latency report.

### M2

Allowed/denied state transitions; invalid schema rejection; policy denial prevents dispatch; approval lifecycle; bounded retries; cancellation.

### M3

QMP lifecycle; no TCP QMP; guest management authentication; default egress denial; snapshot/reset; no shared host mount.

### M4

Guest-only shell; scoped filesystem; keyboard/mouse input; screenshot after action; output and timeout limits; browser UI interaction in guest.

### M5

Workspace staging and path traversal defense; safe diff/copy-back approval; local API binding; CSRF/Origin checks; persistence across restart.

### M6

Threat-model test suite; representative end-to-end workflows; recovery from guest/app failure; performance baseline; release checklist.

## 4. Evidence record

A test result used to mark a feature `Verified` must record:

- Requirement ID(s).
- Commit SHA.
- Test name and command.
- Pass/fail result.
- OS, kernel, hardware, and relevant dependency/runtime versions.
- Model and projector revisions/hashes for model tests.
- Any skipped tests or known limitations.

## 5. Benchmark protocol

- Separate cold-start/load time from warm inference.
- Report prompt processing and token generation separately.
- Measure image preprocessing and visual response separately from text-only inference.
- Record peak VRAM and system RAM.
- Use fixed prompts/images and a fixed sampling profile.
- Run enough repetitions to report median and spread; do not report a single best run as typical performance.
- Compare like-for-like hardware, runtime version, model revision, context length, and image resolution.
