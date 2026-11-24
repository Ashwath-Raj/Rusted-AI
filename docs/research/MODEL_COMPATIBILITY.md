# RUSTED AI — Model and Inference Compatibility

**Decision:** Gemma 4 12B QAT Uncensored through embedded mistral.rs  
**Last reviewed:** 2026-11-24  
**Evidence status:** Upstream family/projector support documented; exact fine-tune artifact must pass project smoke tests

## 1. Selected model artifact

Use this model repository and these two files for the initial target:

- Model repository: [`HauhauCS/Gemma4-12B-QAT-Uncensored-HauhauCS-Balanced`](https://huggingface.co/HauhauCS/Gemma4-12B-QAT-Uncensored-HauhauCS-Balanced)
- Main weights: `Gemma4-12B-QAT-Uncensored-HauhauCS-Balanced-Q4_K_M.gguf`
- Vision projector: `mmproj-Gemma4-12B-QAT-Uncensored-HauhauCS-Balanced-BF16.gguf`
- Optional MTP drafter: `mtp-gemma-4-12B-it.gguf` — **not used in the initial release**.

The model card lists the main Q4_K_M artifact at approximately 6.9 GiB / 7.38 GB and the BF16 projector at approximately 168 MiB / 175 MB. It documents image input through the separate projector. The repository currently identifies the Gemma license; review and accept the applicable license before downloading or redistributing artifacts.

Do not commit model weights, tokenizer assets, caches, or local model paths to Git. Store them under a configurable local model directory ignored by Git.

## 2. Selected inference runtime

**mistral.rs is the fixed inference runtime for the initial implementation.** Integrate it through its Rust SDK inside the RUSTED AI process. Do not start an Ollama process, call an Ollama API, or use a separate inference server as a substitute.

Why this runtime was selected:

- Its current GGUF compatibility guide lists Gemma 4 dense and MoE under the `gemma4` architecture.
- Its multimodal guide states that Gemma 4 uses a projector and supports image/video input; audio is available only when the model/projector artifacts include audio components.
- Its Rust GGUF example builds a model through `GgufModelBuilder` and supplies projector files through `with_mmproj_files`.
- It has documented NVIDIA CUDA support for supported Linux x86-64 GPUs, as well as CPU support.

This upstream documentation establishes model-family support, not proof that this exact community fine-tune has already been tested. Exact artifact validation is mandatory before marking M1 complete.

## 3. Runtime integration rules

- Use the mistral.rs Rust SDK in-process; no local REST inference daemon in the initial design.
- Load the exact GGUF and matching mmproj as a pair.
- Keep model loading and generation behind the `rusted-inference` crate interface.
- The rest of RUSTED AI must not depend on mistral.rs-specific request types.
- Pin the selected mistral.rs release or commit in Cargo metadata and `Cargo.lock`. Do not track a floating Git branch in a reproducible build.
- Use CUDA on supported NVIDIA hardware. Use the documented CPU build profile only when CUDA is unavailable or unsupported. Backend selection is a build/configuration decision recorded in diagnostics, never a random runtime choice.
- Do not add the MTP drafter in M1. Evaluate speculative decoding only after baseline correctness and performance are measured, through a new implementation issue and an ADR if architecture changes.
- The model may propose tool calls, but `rusted-policy` is the sole authorization authority.

## 4. Compatibility matrix

| Capability | Upstream evidence | Project verification required | Current project status |
|---|---|---|---|
| Gemma 4 GGUF architecture | mistral.rs lists `gemma4` for dense and MoE. | Load the exact HauhauCS GGUF. | Not Started |
| Vision projector | Gemma 4 GGUF requires a projector; multimodal loader supports projector files. | Load the exact BF16 mmproj with the exact main model. | Not Started |
| Text generation | Supported model family and GGUF load path are documented. | Local text prompt on target hardware. | Not Started |
| Screenshot/image input | Gemma 4 image support documented. | Send a captured Ubuntu guest screenshot and inspect output. | Not Started |
| Structured tool-call output | Runtime supports tool calling when the model/chat template supports it. | Verify actual model template/output; malformed requests must be rejected. | Not Started |
| CUDA acceleration | mistral.rs documents CUDA for supported NVIDIA hardware. | Build and run on the target GPU/driver/toolkit combination. | Not Started |
| Offline loading | mistral.rs documents local/offline loading when all required assets are present. | Disconnect network after provisioning and rerun text + image smoke tests. | Not Started |
| MTP speculative decoding | Model card provides an optional MTP file. | Not in initial release; no test gate for M1. | Deferred |

## 5. Mandatory M1 smoke-test protocol

Do not mark the runtime verified until all of these pass on the same pinned runtime build and exact model files:

1. **Artifact identity:** Record repository ID, revision/commit, filenames, sizes, and SHA-256 hashes for the model and projector.
2. **Load:** Load the model and projector without an architecture, tensor, tokenizer, or processor error.
3. **Text:** Run a fixed prompt and record completion status, load time, time-to-first-token, decode rate, and peak VRAM/RAM.
4. **Vision:** Pass a fixed screenshot/image and verify that the response correctly identifies predetermined visible content.
5. **Tool proposal:** Request a simple typed tool proposal and verify the parser produces the expected schema. Test malformed JSON, unknown tool names, extra fields, missing fields, and out-of-range values.
6. **Offline:** With all assets already local, disable network access and repeat text and vision tests.
7. **Regression:** Run the same test set after changing the runtime, model files, chat template, sampling parameters, or CUDA build.

Store results in a versioned benchmark report under `docs/research/` or a dedicated results directory. Never report an unrun test as passing.

## 6. Sampling and reproducibility

The model card recommends the following starting profile for this community model: temperature `0.6`, top-k `64`, top-p `0.9`, min-p `0.05`, repeat penalty `1.1`. Treat these as an initial, versioned profile to test—not a guarantee of deterministic output.

- Put all inference parameters in one named configuration profile.
- Record the profile ID with every task run.
- Do not randomize parameters per request.
- Policy decisions, authorization, retry limits, state transitions, and tool argument validation remain deterministic Rust logic.
- The application must never use model sampling randomness as an authorization mechanism.

## 7. Hardware profile

The minimum runtime capability depends on the selected mistral.rs backend and target device. For Linux x86-64 NVIDIA acceleration, confirm the device is within the current upstream supported compute-capability range and that the driver supports the chosen CUDA build. CPU remains a compatibility path, not a promise of acceptable latency.

Record:

- GPU name and compute capability.
- Driver-reported CUDA compatibility version.
- Build CUDA version and Cargo feature set.
- System RAM and VRAM.
- Model/projector hashes.
- Context length and screenshot resolution.
- Load time, peak memory, prompt processing time, time-to-first-token, and decode tokens/sec.

Do not infer that a model fits because its GGUF file fits on disk. Runtime memory also includes projector, weights, KV cache, activations, and allocator overhead.

## 8. Re-evaluation rule

Do not switch to another model or runtime because an experiment is inconvenient or a different library appears fashionable. First classify the failure: artifact, tokenizer/template, projector, runtime bug, CUDA build, memory, or unsupported feature. Fix within the accepted stack where possible. A change of model/runtime requires a new ADR with reproducible comparative results and project-owner approval.

## 9. Upstream references

- [Selected HauhauCS model card and files](https://huggingface.co/HauhauCS/Gemma4-12B-QAT-Uncensored-HauhauCS-Balanced)
- [mistral.rs GGUF compatibility](https://docs.mistralrs.dev/reference/gguf-support/)
- [mistral.rs: Run GGUF models](https://docs.mistralrs.dev/guides/models/run-gguf/)
- [mistral.rs: Supported models](https://docs.mistralrs.dev/reference/supported-models/)
- [mistral.rs: Hardware support](https://docs.mistralrs.dev/reference/hardware-support/)
- [mistral.rs: Rust SDK](https://docs.mistralrs.dev/guides/rust/)
