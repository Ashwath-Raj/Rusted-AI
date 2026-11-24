# ADR-0003: Embedded mistral.rs Inference; Ollama Excluded

- **Status:** Accepted
- **Date:** 2026-11-24

## Context

RUSTED AI requires local multimodal inference, a Rust-native integration boundary, and no dependency on a separate local model server.

## Decision

**Use the mistral.rs Rust SDK embedded in the RUSTED AI process for the selected Gemma 4 GGUF and vision projector. Ollama is explicitly excluded from RUSTED AI.**

The runtime is selected because current upstream documentation lists Gemma 4 GGUF architecture/projector support and provides a Rust `GgufModelBuilder` integration path. The exact HauhauCS model/projector pair remains subject to the project's smoke tests.

## Alternatives considered

- Ollama service/API.
- A separate llama.cpp server process.
- An alternative Rust inference runtime.

## Consequences

- No Ollama dependency, endpoint, configuration, process launch, or documentation as a RUSTED AI component.
- No separate inference HTTP server in the initial architecture.
- `rusted-inference` owns mistral.rs-specific types and translates them into RUSTED AI domain types.
- Runtime version/commit is pinned; no floating branch in reproducible builds.
- CUDA/CPU backend is selected through an explicit build profile based on target hardware, not at random.
- Failure of the exact model smoke test must be diagnosed and reported; it does not silently authorize a runtime swap.

## Evidence

- https://docs.mistralrs.dev/reference/gguf-support/
- https://docs.mistralrs.dev/guides/models/run-gguf/
- https://docs.mistralrs.dev/reference/hardware-support/
- https://huggingface.co/HauhauCS/Gemma4-12B-QAT-Uncensored-HauhauCS-Balanced

## Change rule

Changing inference runtime requires a new ADR, exact-model multimodal tests, measured hardware results, and project-owner approval.
