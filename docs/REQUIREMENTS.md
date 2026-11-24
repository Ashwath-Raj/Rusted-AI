# RUSTED AI — Requirements

**Status:** Baseline requirements  
**Last reviewed:** 2026-11-24

Requirements are testable statements. Each item has a stable ID so issues, tests, pull requests, and milestones can refer to it without copying its text.

## 1. Functional requirements

| ID | Requirement | Acceptance evidence |
|---|---|---|
| FR-001 | The user shall be able to submit a natural-language task through the local Control Room. | UI integration test creates a task and persists its ID. |
| FR-002 | The orchestrator shall represent a task as an explicit plan with ordered steps, dependencies, and completion conditions. | Unit tests validate plan schema and invalid dependency rejection. |
| FR-003 | The system shall use the single selected Gemma 4 model for language reasoning and visual screenshot understanding. | Model smoke tests pass for text and image input with the same loaded model. |
| FR-004 | Model-proposed actions shall be represented as typed tool requests, not executed from free-form text. | Parser/schema tests reject malformed, unknown, or out-of-range tool requests. |
| FR-005 | Every tool call shall pass through the Rust policy engine before dispatch. | Integration test proves denied capability never reaches a tool adapter. |
| FR-006 | Initial guest tools shall include shell, filesystem, screenshot, keyboard, and mouse operations. | Per-tool integration tests run against a disposable guest VM. |
| FR-007 | Tool execution shall return a structured result containing status and relevant evidence. | Contract tests validate success, failure, timeout, and cancellation result forms. |
| FR-008 | The agent shall observe the guest after consequential GUI or filesystem actions and verify the intended state. | End-to-end tests assert state evidence, not merely tool-call success. |
| FR-009 | The system shall support human approval for actions designated by the permission policy. | Tests cover approval, rejection, timeout, and cancellation. |
| FR-010 | The VM manager shall start, stop, inspect, and reset the configured guest through QMP. | QMP integration tests against a disposable VM. |
| FR-011 | Host workspace files shall be transferred only through the workspace broker using task-scoped staging. | Tests prove out-of-scope paths cannot be read or written by a task. |
| FR-012 | The system shall record task state changes, tool calls, approvals, errors, and verification outcomes locally. | SQLite persistence tests and audit-record assertions. |
| FR-013 | The user shall be able to inspect task history and the reason for a failed or blocked task. | Control Room integration test displays status and diagnostic summary. |
| FR-014 | A task shall report completed, partial, failed, blocked, or unverified outcomes accurately. | End-to-end tests cover each terminal outcome. |
| FR-015 | The guest shall have no external network access by default. | Network isolation test confirms egress is denied in default profile. |
| FR-016 | A task that requires internet access shall request the network capability and follow the approval policy. | Tests prove no egress before approval and egress only under the approved profile. |

## 2. Non-functional requirements

| ID | Requirement | Acceptance evidence |
|---|---|---|
| NFR-001 | All first-party application logic shall be written in Rust. | Repository audit; no Python/JS/TS/Node application runtime. |
| NFR-002 | Inference shall run locally through the embedded mistral.rs Rust runtime. | Runtime integration test works with network access disabled after model assets are present. |
| NFR-003 | Ollama shall not be a runtime, build, or deployment dependency. | Dependency/configuration audit. |
| NFR-004 | The model shall have no direct authority over host shell, host filesystem, QMP, or unrestricted networking. | Security integration tests and code review. |
| NFR-005 | Control Room shall bind to loopback only in the initial release. | Socket binding and remote connection tests. |
| NFR-006 | VM management shall use a local Unix-domain QMP socket, not an exposed TCP management port. | VM launch configuration and socket audit. |
| NFR-007 | Every tool invocation shall have a deadline, bounded input, and structured error path. | Contract and timeout tests. |
| NFR-008 | Destructive/non-idempotent actions shall not be automatically retried. | Policy and recovery tests. |
| NFR-009 | Application dependencies and inference runtime versions shall be pinned for reproducible builds. | Committed Cargo.lock and CI build. |
| NFR-010 | Logs shall avoid storing secrets and shall distinguish model claims from verified evidence. | Logging tests and security review. |
| NFR-011 | Core state transitions and policy decisions shall be deterministic for the same input state and configuration. | Pure-function unit tests and replay tests. |
| NFR-012 | The system shall preserve an audit trail sufficient to reconstruct a task's decisions and tool outcomes. | Replay/audit integration test. |

## 3. Explicit non-goals

- Perfectly reliable autonomy on arbitrary websites or applications.
- Treating model confidence as an authorization signal.
- Host-level automation in the first release.
- Unreviewed commit of guest-generated file changes into the host workspace.
- Running several reasoning models to choose between their answers.
- Claiming a security guarantee solely because a VM is used.

## 4. Requirement changes

A requirement may be added, clarified, or retired through a reviewed issue/PR. Changes that contradict a fixed ADR require a new superseding ADR and project-owner approval. Tests should reference requirement IDs in names, descriptions, or test metadata where practical.
