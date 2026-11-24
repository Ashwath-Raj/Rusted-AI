## Summary

What changed and why?

## Traceability

- Requirement IDs:
- Milestone:
- Related issues:
- ADR created/updated: yes / no / not needed

## Implementation

- [ ] First-party application changes are Rust.
- [ ] Existing crate boundaries and accepted ADRs are respected.
- [ ] Model output is treated as untrusted input.
- [ ] No Ollama dependency or service integration was introduced.
- [ ] Host/guest and workspace permissions remain within policy.

## Verification

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test --workspace`
- [ ] Relevant component/integration/model/VM tests

Commands and results:

## Security and data handling

- [ ] No secrets, model weights, VM images, or personal data committed.
- [ ] Permission-denial and failure behavior tested where relevant.
- [ ] No new host access, network capability, or external side effect without review.

## Documentation and status

- [ ] Canonical documentation updated.
- [ ] `PROJECT_STATUS.md` updated only if evidence changed.
- [ ] New fixed decision recorded in an ADR.

## Limitations / follow-up
