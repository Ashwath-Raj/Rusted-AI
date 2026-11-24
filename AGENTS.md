# Instructions for AI Coding Agents — RUSTED AI

You are contributing to **RUSTED AI**. Follow these instructions before editing code.

## Read before acting

1. Read `docs/INDEX.md`.
2. Read `docs/PROJECT_STATUS.md` and the relevant milestone in `docs/ROADMAP.md`.
3. Read the requirement IDs that govern the task in `docs/REQUIREMENTS.md`.
4. Read the relevant architecture document and every applicable ADR in `docs/decisions/`.
5. If a task conflicts with an accepted ADR, stop and report the conflict. Do not silently choose another design.

## Fixed project decisions

These are binding unless a new ADR explicitly supersedes the relevant decision and the project owner approves it:

- Product name and repository identity: **RUSTED AI**.
- All first-party application logic is Rust. Do not add Python, JavaScript, TypeScript, Node.js, or a Python/JavaScript agent backend.
- One neural model only: `HauhauCS/Gemma4-12B-QAT-Uncensored-HauhauCS-Balanced`, with its matching vision projector. The same model performs reasoning and visual perception.
- The inference runtime is embedded **mistral.rs** through its Rust SDK. Do not use an inference HTTP server as the application runtime.
- **Ollama is excluded**. Do not add Ollama dependencies, APIs, configuration, process management, or documentation as a RUSTED AI component.
- System One is deterministic Rust policy/state logic, not a second neural model.
- Host OS: Ubuntu Linux. Agent operations run in an Ubuntu Desktop guest managed by QEMU/KVM.
- QMP is the VM control interface. Guest shell and task file transfer use controlled guest communication over a private management network.
- No live host-home/shared-folder mount. Host workspace access is mediated by a Rust workspace broker using task-scoped staging and reviewed copy-back.
- Guest network egress is disabled by default and enabled only through an explicit task capability/approval. No bridged networking or inbound port forwarding in the baseline design.
- Local Control Room: Leptos UI served by Axum, bound to loopback only. It is not a public or LAN service.
- Persistence: SQLite through SQLx. Async orchestration: Tokio. Serialization: Serde. Diagnostics: `tracing`. Domain errors: `thiserror`.

## Implementation rules

- Treat model output as untrusted data. Never execute free-form model text as a shell command.
- Tools must have typed request/response schemas, explicit capabilities, bounded arguments, deadlines, and structured results.
- The model may propose an action; only the Rust policy engine can authorize dispatch.
- Do not give the model direct host-shell, host-filesystem, QMP-socket, or unrestricted network access.
- Execute shell, GUI, filesystem, and browser actions inside the guest, subject to the workspace broker and permission policy.
- A task is not complete because a tool returned success. Verify the resulting state and record evidence.
- Do not automatically retry non-idempotent or destructive actions.
- Do not invent crate versions, APIs, command flags, model support, or benchmark results. Confirm against upstream documentation and compile/test the exact dependency versions.
- Pin dependency versions through `Cargo.toml` and commit `Cargo.lock` for applications.
- Keep secrets, model weights, VM images, caches, logs containing personal data, and generated artifacts out of Git.
- Do not add a new major dependency or subsystem without recording its purpose and evaluating whether an existing component already covers the need.

## Required workflow for each task

1. State the requirement ID(s) and milestone being implemented.
2. Inspect the existing code before proposing changes.
3. Make the smallest coherent change that satisfies the acceptance criteria.
4. Add or update tests at the correct level.
5. Run formatting, linting, tests, and relevant smoke checks.
6. Update `PROJECT_STATUS.md` only when implementation evidence changes.
7. Update the owning architecture/engineering document only when its documented behavior changes.
8. Add an ADR when an accepted architectural decision must change.
9. Report changed files, commands run, test results, unverified behavior, and known limitations.

## Conflict resolution

The precedence order is:

1. Approved ADRs in `docs/decisions/`.
2. `docs/REQUIREMENTS.md` for required behavior.
3. `docs/architecture/` and `docs/security/` for implementation boundaries.
4. `docs/ROADMAP.md` for sequence and dependencies.
5. `docs/PROJECT_STATUS.md` for current evidence only.
6. README and other summaries.

If two authoritative documents conflict, do not guess. Report the conflict and propose a specific ADR/document correction.
