# ADR-0001: Rust-Only First-Party Application

- **Status:** Accepted
- **Date:** 2026-11-24

## Context

RUSTED AI is an autonomous system with a large tool-execution and security surface. A consistent implementation language supports shared types, explicit interfaces, and a single application toolchain.

## Decision

**All first-party application logic, including orchestration, policy, inference integration, VM integration, tools, persistence integration, API, and UI, shall be written in Rust.**

Bash may be used for developer/CI/install scripts. External infrastructure such as QEMU, Ubuntu, model weights, and upstream libraries is not first-party application code. Generated HTML/CSS/WASM assets are acceptable when produced from Rust UI source; handwritten JavaScript/TypeScript application logic is not.

## Alternatives considered

- Python agent backend.
- Node.js/TypeScript application.
- Mixed-language application.

## Consequences

- Rust is the only first-party application language.
- No Python or Node-based agent framework is introduced.
- Every external integration is wrapped behind a Rust interface.
- Build and test workflows use Cargo and the Rust toolchain.

## Change rule

A language exception requires a new ADR that identifies the exact subsystem, why Rust is insufficient, the security/maintenance impact, and how the exception remains isolated. Project-owner approval is mandatory.
