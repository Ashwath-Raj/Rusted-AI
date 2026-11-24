# Unsensored Agent
## Executive Summary & Technical Vision

### 1. Project Vision

**Unsensored Agent is a fully local, Rust-native autonomous AI computer-operation platform designed to transform a large language model from a conversational assistant into an autonomous digital operator.**

The system gives an AI model the ability to understand natural-language instructions, perceive a graphical desktop, reason about computer state, plan actions, interact with the operating system, execute tasks and verify their outcomes.

Unlike conventional AI assistants that primarily generate text or rely on cloud-hosted agent platforms, Unsensored Agent is designed around local inference, independently controlled execution infrastructure and isolated virtual-machine environments.

The fundamental objective is to build a personal AI operating environment in which the user retains control over the model, execution environment, permissions and data.

### 2. What We Are Building

Unsensored Agent is not another chatbot or an interface around an existing AI agent framework.

It is an autonomous computer-operation system.

A user can provide an instruction such as:

> Organize my development directory, identify duplicate files, inspect the project documentation, resolve routine configuration issues and prepare a summary of the changes.

The agent should be capable of:

1. **Understanding:** Interpret natural-language instructions and determine the intended outcome.
2. **Planning:** Decompose complex instructions into executable steps.
3. **Perception:** Inspect screenshots and understand the graphical environment.
4. **Tool selection:** Select appropriate filesystem, shell, browser or GUI operations.
5. **Execution:** Perform actions inside an isolated Ubuntu virtual machine.
6. **Observation:** Capture execution results and inspect the updated environment.
7. **Recovery:** Identify failures, adapt its plan and retry appropriate operations.
8. **Verification:** Confirm whether the requested outcome was actually achieved.
9. **Reporting:** Provide a transparent account of completed, unsuccessful and pending actions.

### 3. Core Architecture

The architecture follows a single-model, deterministic-execution approach.

**A. Intelligence Layer — Gemma 4**

The primary reasoning and perception engine is Gemma 4 12B QAT Uncensored, using its multimodal capabilities.

It is responsible for:
- Natural-language understanding.
- Multi-step reasoning and planning.
- Screenshot interpretation and visual perception.
- Tool selection and structured tool requests.
- Evaluating execution observations.
- Adapting plans when unexpected conditions occur.

One neural model handles both visual understanding and reasoning. No separate System One neural model is introduced.

**B. Agent Orchestration — Rust**

The orchestration layer manages the complete agent lifecycle.

Responsibilities include:
- Task planning and execution state.
- Tool registration and dispatch.
- Context and execution history.
- Agent state transitions.
- Retry and recovery mechanisms.
- Coordination between inference and execution.

**C. System One — Deterministic Rust Decision Engine**

Rather than introducing another neural model, fast operational decisions are implemented directly in Rust.

This layer handles:
- Tool-request validation.
- Permission checks.
- Risk classification.
- Execution routing.
- Retry conditions.
- Timeout handling.
- State validation.
- Human-approval requirements.

Gemma proposes actions; the Rust decision engine determines whether and how those actions may proceed.

**D. Tool Runtime — Rust**

The tool runtime provides controlled access to the computer environment.

Its capabilities include:

| Tool category | Responsibilities |
|---|---|
| Filesystem | Read, write, create, move, organize and inspect files |
| Shell | Execute terminal commands and capture results |
| Keyboard | Send keyboard events |
| Mouse | Click, move, scroll and interact with desktop elements |
| Browser | Navigate and interact with web interfaces |
| Screen capture | Capture desktop screenshots for visual reasoning |
| Application control | Launch, inspect and manage applications |

Every operation is exposed through typed interfaces and controlled execution policies.

**E. Isolation Infrastructure — QEMU / Ubuntu VM**

All potentially destructive computer interactions are intended to take place inside an isolated Ubuntu virtual machine.

The host Ubuntu installation serves as the management and control environment, while the guest VM serves as the agent's operational environment.

Communication and control will use:
- QEMU management interfaces, including QMP.
- SSH or guest communication mechanisms.
- VM display and screenshot capture.
- Controlled keyboard and mouse event injection.

The agent must not have unrestricted access to the host operating system.

**F. Control Room — Rust-Based User Interface**

The Control Room is the central supervision interface.

It provides:
- Natural-language task submission.
- Task execution monitoring.
- Current agent activity.
- Tool-call inspection.
- Execution logs.
- Screenshot and observation history.
- Approval and rejection controls.
- Task completion reports.
- Error and recovery visibility.

### 4. Technology Direction

| Layer | Technology |
|---|---|
| Application language | Rust |
| Host environment | Ubuntu Linux |
| Virtualization | QEMU |
| Guest environment | Ubuntu Linux VM |
| Local intelligence | Gemma 4 12B QAT Uncensored |
| Inference integration | Rust-native inference runtime |
| Asynchronous execution | Tokio |
| Backend API | Axum |
| Serialization | Serde |
| Persistent storage | SQLite / SQLx |
| VM communication | QMP / SSH |
| GUI interaction | Rust-based input drivers |
| Browser automation | Rust-based browser-control integration |
| Logging and observability | tracing |
| User interface | Rust-based desktop/web UI |

The exact inference runtime and model-format compatibility must be validated before final selection.

**Language constraint:** All first-party application logic, orchestration, tools, backend and interface implementations must be written in Rust. No Python, JavaScript, TypeScript, Node.js or external agent frameworks are part of the application architecture.

External infrastructure such as QEMU, Ubuntu and the inference model itself remains independent of the Rust application code.

### 5. Execution Philosophy

The fundamental operating principle is:

**Perception → Reasoning → Validation → Execution → Observation → Verification**

For example, when instructed to open a browser and inspect a GitHub repository:

1. Gemma interprets the user's instruction.
2. The orchestrator creates an execution plan.
3. The Rust runtime captures the current VM desktop.
4. Gemma identifies the browser and determines the appropriate interaction.
5. The decision engine validates the requested operation.
6. The Rust input driver executes the necessary keyboard and mouse events.
7. The runtime captures the resulting screen.
8. Gemma evaluates the new state and determines the next action.
9. The process continues until the requested objective is achieved or an explicit failure is reported.

The system must operate through feedback rather than blindly executing predetermined automation scripts.

### 6. Security, Control and Observability

Autonomous execution introduces risks that ordinary conversational AI does not face.

Unsensored Agent therefore treats security and operational control as first-class architectural requirements.

Core principles include:

- VM-based execution isolation.
- Least-privilege tool access.
- Typed and validated tool interfaces.
- Explicit execution permissions.
- Human approval for sensitive or destructive operations.
- Execution timeouts and resource limits.
- Persistent action and error logs.
- Verification of consequential operations.
- Separation between model-generated requests and actual execution authority.

An uncensored model is not inherently a capable or safe autonomous agent. Its operational permissions must always be enforced independently by the Rust runtime.

### 7. What Makes Unsensored Agent Different?

The project combines several capabilities into one self-hosted environment:

**Local intelligence:** Model inference takes place under the user's control without depending on cloud-hosted inference.

**Multimodal perception:** The same reasoning model can interpret the graphical desktop and make decisions based on visual observations.

**Autonomous execution:** The agent can perform multi-step computer operations instead of merely suggesting commands.

**Rust-native infrastructure:** The orchestration, decision engine, tools, execution management and interface are built in Rust.

**Isolation:** Computer operations take place inside a dedicated virtualized environment.

**Transparency:** Users can inspect plans, tool calls, observations, errors and execution results.

**Human oversight:** The user retains authority over sensitive operations and system permissions.

### 8. End Goal

The long-term objective is a self-hosted AI execution environment capable of:

- Operating a complete Linux desktop.
- Managing files and directories.
- Executing terminal commands.
- Working with local development projects.
- Operating browsers and graphical applications.
- Automating repetitive workflows.
- Performing multi-step technical tasks.
- Recovering from execution failures.
- Maintaining context across tasks.
- Reporting verified outcomes.

### 9. Executive Positioning

**Unsensored Agent is a Rust-native, local-first autonomous AI computer-operation platform that transforms multimodal language models into operational agents capable of perceiving, reasoning about and interacting with real computer environments.**

By combining a single local multimodal reasoning model, a deterministic Rust decision engine, a controlled computer-interaction runtime and QEMU-based execution isolation, the project aims to deliver a self-hosted alternative to cloud-dependent autonomous computer-use platforms.

Its defining characteristic is the separation of intelligence, authorization and execution: the model reasons, Rust enforces control, and the isolated environment performs the work.

**The fundamental product is not another chatbot, an Ollama wrapper or a collection of automation scripts. It is a locally controlled AI computer operator, engineered entirely in Rust.**

---

*Project status clarification: This document describes the intended architecture and engineering direction. It does not imply that all components have already been implemented or validated.*
