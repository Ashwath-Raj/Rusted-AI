# RUSTED AI — Threat Model and Security Policy

**Status:** Baseline security policy  
**Last reviewed:** 2026-11-24

## 1. Security objective

RUSTED AI must allow useful autonomous work inside a guest VM without giving model output unrestricted authority over the host. Security is enforced by Rust policy and execution boundaries, not by trusting the model's intentions or refusal behavior.

A VM reduces risk but is not a perfect security guarantee. QEMU configuration, guest networking, shared resources, credentials, and host services remain part of the attack surface.

## 2. Assets to protect

- Host operating system and user home directory.
- Host credentials, SSH keys, browser profiles, cookies, and tokens.
- User project files and source repositories.
- Local model weights, configuration, and inference prompts.
- VM images, snapshots, and guest state.
- Task history, screenshots, audit logs, and approval records.
- Control Room API and QMP management socket.
- Network access and any external service reached from the guest.

## 3. Trust boundaries

| Boundary | Trusted side | Untrusted side / input | Required control |
|---|---|---|---|
| Model → orchestrator | Typed Rust application | Model-generated text/tool proposal | Strict parse, schema validation, size limits. |
| Orchestrator → policy | Rust application state | Proposed operation and arguments | Re-derive capability/risk in Rust; never trust model-supplied risk. |
| Policy → tool adapter | Authorized request | Tool execution environment | Capability-scoped dispatch and deadlines. |
| Host → guest | Host VM manager and broker | Guest OS and guest processes | QEMU/KVM boundary, no host home mount, authenticated private channel. |
| Guest → host workspace | Workspace broker | Guest-generated files and manifest | Path allowlist, content/hash checks, diff review, approved copy-back. |
| Control Room → local API | Local user session | Browser-originated requests | Loopback bind, Host/Origin validation, CSRF protection, unguessable session token. |
| Guest → internet | Explicitly approved network capability | Remote sites and downloaded content | Egress denied by default; enable only for approved task. |

## 4. Threats and mitigations

| Threat | Example | Required mitigation |
|---|---|---|
| Prompt injection | Web page tells the model to exfiltrate a file. | Treat page text as untrusted data; no authority escalation; policy checks all tools. |
| Malformed tool output | Model emits invalid JSON or invented tool. | Typed schema, allowlisted tool registry, reject unknown/malformed requests. |
| Host compromise through tool | Model constructs a host shell command. | No host shell tool; shell adapter executes only inside guest. |
| Path traversal | Guest asks broker to read `../../.ssh`. | Canonicalize paths, resolve symlinks safely, enforce workspace root after resolution. |
| Malicious file return | Guest returns symlink, device, oversized archive, or crafted filename. | Broker accepts regular files only by default, rejects unsafe path types, size limits, hash manifest, quarantine before copy-back. |
| Unauthorized host modification | Guest changes a project file without review. | Task-scoped staging; copy-back creates a diff and requires policy/approval. |
| Network abuse | Guest scans LAN or sends data externally. | Network denied by default; no bridged mode; no inbound port forwarding; explicit egress capability. |
| QMP takeover | Tool attempts to send arbitrary monitor commands. | QMP socket only in VM manager; expose narrow Rust methods, not raw QMP to model/tools. |
| Control Room CSRF/DNS rebinding | Malicious page calls local control API. | Loopback-only bind, strict Host/Origin validation, CSRF token, no wildcard CORS, no permissive cookies. |
| Resource exhaustion | Huge output, endless loop, image flood, context abuse. | Per-tool deadlines, output limits, token/context caps, VM CPU/RAM/disk quotas, bounded queues. |
| Unreliable retry | Agent repeats a file deletion after timeout. | Idempotency classification; observe before retry; no automatic retries for non-idempotent actions. |
| Secret exposure | Tokens included in prompt or screenshot. | No secret mounts by default; redact logs; credential use requires a separate explicit capability and approval. |

## 5. Permission model

Permissions are assigned by Rust policy from the tool type, arguments, task grants, and current environment. The model cannot declare its own risk tier or request elevated privilege by changing JSON fields.

| Risk class | Examples | Default policy |
|---|---|---|
| R0 — Observe | Screenshot, list files inside granted task workspace, read guest process status. | Auto-allow within granted scope. |
| R1 — Reversible task change | Create/edit files inside task-scoped guest workspace. | Allow only when the task has an explicit write grant; preserve diff/backup evidence. |
| R2 — External side effect | Enable internet egress, install packages, download content, submit/upload data, access a credential. | Require explicit user approval for the operation/capability. |
| R3 — Critical/destructive | Delete data outside task workspace, change system/network/security settings, use privilege escalation, access host paths. | Deny by default. No host-level elevation. Any exceptional guest action requires explicit per-action approval and must remain within the guest. |

Approval grants are narrow: task ID, capability, target scope, action, and expiry are recorded. Approval for one operation is not blanket permission for future tasks.

## 6. Isolation policy

The initial VM profile is:

- QEMU/KVM Ubuntu Desktop guest.
- QMP exposed only through a host-owned Unix-domain socket with restricted filesystem permissions.
- SSH/guest management available only over a private host/guest management network using per-install or ephemeral keys.
- No host home directory, SSH agent, browser profile, credential store, or unrestricted clipboard passed through.
- No live shared-folder mount.
- Guest egress disabled by default. Internet access is a separately approved task capability.
- No bridged networking and no inbound guest port forwards in the initial design.
- VM disk uses a base image plus disposable task overlay/snapshot so guest state can be reset.
- Guest user is unprivileged; `sudo`/root actions are denied by default.

## 7. Workspace broker policy

The workspace broker is the only permitted bridge between host project folders and guest task workspaces.

1. User grants one canonical host directory and a read or read/write mode.
2. Broker resolves and validates the grant; paths escaping the root through `..` or symlinks are rejected.
3. Broker creates a task-scoped snapshot/staging directory and records file hashes.
4. Guest receives only the staged copy, not a live host mount.
5. On task completion, the broker receives a manifest and candidate changes.
6. Broker validates filenames, file types, sizes, paths, and hashes, then creates a diff.
7. Copy-back requires the policy-defined approval. Preserve an original backup or version-control diff where possible.
8. The task record stores the grant, manifest, approval, and copy-back result.

## 8. Operational security requirements

- Do not log passwords, access tokens, private keys, or full sensitive environment variables.
- Do not store model-generated commands in an executable queue without typed validation and policy authorization.
- Do not accept raw QMP commands from the model.
- Do not expose the Control Room or QMP over `0.0.0.0`.
- Keep model files and VM disk images outside the source repository.
- Record security-relevant denials and approvals in the local audit trail.
- Keep VM and runtime dependencies patched; record versions used in verified tests.

## 9. Security test gates

Before a release, test at least:

- Prompt-injection attempt requesting host file access.
- Invalid/unknown tool request.
- Path traversal and symlink escape through workspace broker.
- Guest egress denial and approved egress behavior.
- QMP socket is not reachable from guest/LAN.
- Local API rejects foreign Host/Origin and CSRF-less state-changing requests.
- Oversized command output and screenshot payload.
- Tool timeout and task cancellation.
- Non-idempotent action timeout does not trigger duplicate execution.
- Guest-generated unsafe file types are rejected at copy-back.
