# ADR-0002: One Gemma 4 Model for Reasoning and Vision

- **Status:** Accepted
- **Date:** 2026-11-24

## Context

The agent needs natural-language reasoning and visual understanding of the guest desktop. Adding separate neural models for perception, routing, and reasoning would increase memory use, orchestration complexity, and failure modes.

## Decision

**The initial RUSTED AI architecture uses one neural model: `HauhauCS/Gemma4-12B-QAT-Uncensored-HauhauCS-Balanced`, with its matching BF16 vision projector. The same model handles reasoning and visual perception.**

The Rust policy engine performs fast deterministic decisions. It is not a second neural System One model.

## Alternatives considered

- Separate vision model plus language model.
- A second small neural classifier/router.
- Cloud-hosted multimodal model.

## Consequences

- One model instance and one model artifact family are used for the initial release.
- The vision projector is required for screenshot/image input.
- Model memory and latency must be measured before choosing default screenshot resolution/context limits.
- The optional MTP drafter is excluded from the initial release.
- The exact fine-tune artifact must pass the tests in `MODEL_COMPATIBILITY.md` before M1 is marked verified.

## Change rule

A second model or model replacement requires a superseding ADR with comparative accuracy, latency, memory, licensing, and integration evidence. A model must never be changed merely because a coding agent prefers another model.
