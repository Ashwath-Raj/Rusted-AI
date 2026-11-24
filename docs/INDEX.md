# Documentation Index and Source-of-Truth Rules

**Project:** RUSTED AI  
**Baseline date:** 2026-11-24  
**Audience:** Human contributors and AI coding agents

This index explains where information belongs. The goal is one canonical home for each fact, not repeated copies that drift apart.

## Read in this order

| Order | Document | Canonical responsibility |
|---:|---|---|
| 1 | [`PROJECT_CHARTER.md`](PROJECT_CHARTER.md) | Why the project exists, scope, and fixed product constraints. |
| 2 | [`PROJECT_STATUS.md`](PROJECT_STATUS.md) | What is actually implemented and evidenced now. |
| 3 | [`ROADMAP.md`](ROADMAP.md) | Milestone order, dependencies, and exit criteria. |
| 4 | [`REQUIREMENTS.md`](REQUIREMENTS.md) | Testable functional and non-functional requirements. |
| 5 | [`architecture/SYSTEM_ARCHITECTURE.md`](architecture/SYSTEM_ARCHITECTURE.md) | Components, ownership, interfaces, and trust boundaries. |
| 6 | [`architecture/AGENT_LIFECYCLE.md`](architecture/AGENT_LIFECYCLE.md) | Task states, transitions, and failure handling. |
| 7 | [`security/THREAT_MODEL.md`](security/THREAT_MODEL.md) | Threats, permissions, isolation, and security invariants. |
| 8 | [`research/MODEL_COMPATIBILITY.md`](research/MODEL_COMPATIBILITY.md) | Exact model artifact, chosen runtime, compatibility evidence, and smoke tests. |
| 9 | [`engineering/RUST_ENGINEERING_STANDARD.md`](engineering/RUST_ENGINEERING_STANDARD.md) | Rust workspace and implementation conventions. |
| 10 | [`engineering/TESTING_STRATEGY.md`](engineering/TESTING_STRATEGY.md) | Test levels, gates, and evidence required for verification. |

## Architecture decisions

ADRs record *why* fixed decisions were made. They do not replace the detailed architecture documents.

| ADR | Decision | Status |
|---|---|---|
| [ADR-0001](decisions/ADR-0001-RUST-ONLY-APPLICATION.md) | First-party application code is Rust. | Accepted |
| [ADR-0002](decisions/ADR-0002-SINGLE-GEMMA4-MODEL.md) | One Gemma 4 model handles reasoning and vision. | Accepted |
| [ADR-0003](decisions/ADR-0003-MISTRALRS-NO-OLLAMA.md) | Embedded mistral.rs is the inference runtime; Ollama is excluded. | Accepted |
| [ADR-0004](decisions/ADR-0004-QEMU-ISOLATED-GUEST.md) | QEMU/KVM Ubuntu guest is the execution boundary. | Accepted |
| [ADR-0005](decisions/ADR-0005-LEPTOS-AXUM-CONTROL-ROOM.md) | Leptos + Axum local-only Control Room. | Accepted |
| [ADR-0006](decisions/ADR-0006-WORKSPACE-BROKER.md) | No live host-folder mount; task-scoped workspace broker. | Accepted |
| [ADR-0007](decisions/ADR-0007-SQLITE-PERSISTENCE.md) | SQLite/SQLx for local persistent state. | Accepted |

## Single-source-of-truth rules

- **Product scope** belongs in `PROJECT_CHARTER.md`. Do not restate it in every document.
- **Requirements** belong in `REQUIREMENTS.md`; architecture documents explain the design that satisfies them.
- **Current progress** belongs only in `PROJECT_STATUS.md`. A planned feature is not implemented; an implemented feature is not verified until evidence exists.
- **Milestone sequence** belongs in `ROADMAP.md`. Milestones link to requirement IDs instead of copying requirement prose.
- **Component behavior and interfaces** belong in `SYSTEM_ARCHITECTURE.md`.
- **Task-state transitions** belong in `AGENT_LIFECYCLE.md`.
- **Security rules** belong in `THREAT_MODEL.md`.
- **Model/runtime compatibility evidence** belongs in `MODEL_COMPATIBILITY.md`.
- **The rationale for a major choice** belongs in an ADR. The operational detail remains in the relevant architecture document.
- `README.md` is a concise public overview, not the authoritative engineering specification.
- `AGENTS.md` gives coding agents the mandatory reading order and behavior; it points to the documents rather than duplicating all their contents.

## Status vocabulary

Use exactly these values for subsystem status:

- `Not Started` — no implementation exists.
- `In Progress` — implementation work has begun but acceptance criteria are incomplete.
- `Implemented` — code exists for the stated scope, but verification is incomplete.
- `Verified` — required tests and acceptance evidence passed on a recorded environment/commit.
- `Blocked` — work cannot proceed because a named dependency or decision is unresolved.
- `Deprecated` — intentionally retired; include the replacement.

Do not use percentages unless they are computed from a defined task list. Prefer requirement IDs, passing tests, and commit hashes over subjective progress estimates.

## Changing a fixed decision

Do not edit an accepted ADR in place to make a decision disappear. Create a new ADR that explicitly supersedes it, document migration consequences, and obtain project-owner approval before implementation. See the ADR format at the end of this index.

### ADR minimum fields

- Number and title.
- Status: Proposed, Accepted, Rejected, or Superseded.
- Context and problem.
- Decision stated in one unambiguous sentence.
- Alternatives considered.
- Consequences and constraints.
- Evidence/links, where the decision relies on external compatibility claims.
- Supersedes/superseded-by links, when applicable.
