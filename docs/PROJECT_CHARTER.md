# RUSTED AI — Project Charter

**Status:** Approved project direction  
**Last reviewed:** 2026-11-24

## 1. Purpose

RUSTED AI is a local-first autonomous computer-operation platform. It is intended to turn a multimodal language model into an operator that can understand a user's task, perceive a desktop, plan actions, use controlled computer tools, and verify the resulting state.

The product is a computer operator, not simply a chatbot, a prompt wrapper, or a collection of fixed macros.

## 2. Product outcome

A user gives RUSTED AI a natural-language objective. The system should be able to:

1. Interpret the objective and identify constraints.
2. Create a bounded, inspectable plan.
3. Observe the guest computer, including screenshots.
4. Propose structured tool actions.
5. Validate and authorize actions in Rust.
6. Execute permitted actions inside an isolated Ubuntu guest.
7. Observe the effects and recover from expected failures.
8. Verify the outcome and report completed, partial, failed, and pending work honestly.

## 3. Target users and operating environment

- Primary user: an individual operating a self-hosted Linux workstation.
- Host target: Ubuntu Linux on x86-64.
- Execution target: Ubuntu Desktop guest managed by QEMU/KVM.
- Primary interaction: local Control Room plus the agent-operated guest desktop.
- Inference mode: local model files and local inference; no required cloud inference service.

## 4. In scope for the initial product

- One local multimodal Gemma 4 model for reasoning and visual perception.
- Rust-native orchestration and deterministic policy/decision logic.
- Typed shell, filesystem, screenshot, keyboard, mouse, and guest application tools.
- QEMU/KVM guest lifecycle and controlled guest communication.
- Task-scoped workspace staging and reviewed copy-back to host folders.
- Local Control Room for task submission, logs, observations, and approvals.
- Local task state, execution records, and verification evidence.

## 5. Out of scope for the initial product

- Cloud-hosted inference or a cloud agent control plane.
- Ollama integration or an Ollama-compatible dependency.
- Multiple neural models, model committees, or a separate neural System One classifier.
- Python, JavaScript, TypeScript, or Node.js first-party application logic.
- Unrestricted host desktop control or direct host shell access by the model.
- Live mounting of the user's home directory into the guest.
- Public/LAN-accessible Control Room service.
- Autonomous financial transactions, account creation, messaging, purchases, or other irreversible external side effects without explicit user approval.
- A general-purpose multi-tenant agent platform.

## 6. Fixed product constraints

The binding decisions are recorded in ADRs. At a glance:

- First-party application code is Rust.
- One Gemma 4 12B QAT Uncensored model plus its vision projector is the selected model target.
- mistral.rs is the selected embedded Rust inference runtime; Ollama is excluded.
- System One is deterministic Rust policy/state logic.
- QEMU/KVM with an Ubuntu Desktop guest is the execution boundary.
- The Control Room uses Leptos + Axum, served on loopback only.
- Local persistence uses SQLite through SQLx.
- The host workspace is accessed through a Rust broker and task-scoped copies, not a live shared folder.

See `docs/decisions/` for the binding rationale and change process.

## 7. Product principles

1. **Local control:** model execution and task data remain under the user's control.
2. **Separation of authority:** model output is a proposal, never permission to execute.
3. **Isolation:** computer operations target the guest VM, not the host.
4. **Least privilege:** tools receive only the capabilities needed for the task.
5. **Observability:** actions, outputs, failures, approvals, and verification evidence are recorded.
6. **Truthful completion:** completion requires evidence of the requested outcome.
7. **Recoverability:** task execution must have explicit states, deadlines, and bounded retry rules.
8. **Reproducibility:** dependencies, model artifacts, runtime parameters, and VM configuration are versioned or recorded.

## 8. Definition of a successful task

A task is successful only when all required acceptance conditions are satisfied, the final state has been observed, and the result is recorded. If only some conditions are satisfied, report partial completion. If verification is unavailable, report the task as unverified rather than successful.

## 9. Project governance

- `REQUIREMENTS.md` defines what the system must do.
- `SYSTEM_ARCHITECTURE.md` defines component responsibilities and boundaries.
- `AGENT_LIFECYCLE.md` defines execution state transitions.
- `THREAT_MODEL.md` defines permission and isolation rules.
- ADRs define accepted technical decisions.
- `PROJECT_STATUS.md` records evidence-backed implementation state.
- `ROADMAP.md` defines milestone order and exit criteria.

Any change to a fixed product constraint requires a new ADR and project-owner approval.
