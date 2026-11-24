# ADR-0005: Leptos + Axum Local Control Room

- **Status:** Accepted
- **Date:** 2026-11-24

## Context

RUSTED AI needs a human supervision interface for task submission, approvals, logs, screenshots, and results. The interface must follow the Rust-only application constraint.

## Decision

**Build the Control Room with Leptos UI source and Axum as the local server/control API. Bind the service to loopback only.**

The UI and API are first-party Rust code. The initial product is single-user and local; it is not a LAN or public service.

## Alternatives considered

- JavaScript/TypeScript SPA.
- Electron/Tauri frontend with JavaScript application logic.
- Native immediate-mode desktop UI.

## Consequences

- UI components and application UI logic are written in Rust.
- Axum is a local control-plane/API boundary, not an inference server.
- Default bind address is `127.0.0.1`; no `0.0.0.0` binding.
- Validate Host and Origin, protect state-changing requests against CSRF, and use a per-session unguessable token.
- No wildcard CORS.
- UI/API must not call privileged OS or QMP operations directly; they submit authenticated task/control requests to the application layer.

## Change rule

Changing the UI framework or exposing a remote interface requires a new ADR and a security review.
