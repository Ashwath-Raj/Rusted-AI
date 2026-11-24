# ADR-0004: QEMU/KVM Ubuntu Guest as the Execution Boundary

- **Status:** Accepted
- **Date:** 2026-11-24

## Context

Autonomous shell and GUI actions can modify or destroy data. The model must not operate directly on the user's host desktop or host shell.

## Decision

**All agent computer operations run inside a dedicated Ubuntu Desktop guest managed by QEMU/KVM.** The host application manages the guest but does not expose raw host execution tools to the model.

The baseline uses QMP through a local Unix-domain socket for VM management and supported input events. Guest management uses authenticated communication over a private network. Guest screenshots are captured through the QEMU display/monitor adapter owned by `rusted-vm`.

## Alternatives considered

- Direct host desktop automation.
- Container-only isolation.
- Remote cloud VM.

## Consequences

- Guest image, snapshot/overlay, resource limits, and network profile are explicit configuration.
- QMP socket is never exposed over TCP or passed to model/tool code.
- Guest runs as an unprivileged user for ordinary tasks.
- No host home-directory or credential-store mount.
- Guest egress is denied by default and enabled only via approved capability.
- The VM is a containment layer, not a guarantee against all host/guest escape vulnerabilities.

## Change rule

Changing the execution boundary requires a superseding ADR and an updated threat model with tested equivalence or stronger isolation.
