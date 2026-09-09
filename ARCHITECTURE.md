# OB Oraborus Architecture

## System boundary

```text
User
  ↓
Command and Context Layer
  ↓
OB Prime → Task DAG → Scoped Agents
  ↓                  ↓
Model Router      Security Gate
  ↓                  ↓
AI Providers      Tool Registry
  ↓                  ↓
Local or Cloud    Windows / Files / Browser / Services
  ↓                  ↓
Response          Observation → Verification
  └──────────────→ Audit + Approved Memory
```

No agent receives ambient unrestricted authority. Every agent declaration lists tools, risk classes, timeouts, retry policy, schemas, and verification strategy. A tool invocation is valid only when its registry declaration, agent scope, privacy switch, autonomy policy, and Security Gate agree.

## Runtime components

### Desktop shell

Tauri 2 owns the Windows process, WebView, system tray, global shortcut, startup plugin, and IPC boundary. The production window is a desktop application, not a remotely hosted site. Tauri’s capability file grants only the frontend permissions required for shortcut, notification, and startup controls.

### Frontend

React and TypeScript render a three-region operating interface:

- Left: navigation and privacy/connectivity state
- Center: ORA Core, active workspace, conversation, system panels, Agent Town, and World Explorer
- Right: process inspector for request, plan, agents, tools, permissions, result, verification, and errors
- Bottom: universal command surface

Zustand holds view, conversation, command, and current backend snapshot state. Values such as CPU, provider health, task history, and agent activity are received from the Rust backend. Empty states explicitly disclose when no verified data exists.

### Application state

`AppState` owns cloneable service handles:

- SQLite connection behind a mutex
- Windows credential vault
- audit writer
- Security Gate
- provider manager and model router
- agent and tool registries
- memory, research, GitHub, public API, and automation engines
- cancellation token tree
- application data and bundled-resource paths

Stop All cancels the active token tree and disables enabled automations. Child browser and terminal processes use kill-on-drop behavior and bounded timeouts.

## AI provider contract

Each adapter implements:

- connect
- connection test
- model listing
- chat
- streaming chat
- vision
- embeddings
- tool calling
- cancellation hook
- health check

Provider methods return typed errors. Secrets are retrieved at request time from Windows Credential Manager and are never included in provider status objects or logs.

### Ollama

Uses `/api/tags`, `/api/chat`, and `/api/embed`. Installed model names are converted into conservative capability declarations. Model download is not automatic and must be separately approved.

### Gemini

Uses the Generative Language API with `x-goog-api-key`, standard generation, server-sent streaming, inline multimodal data, embeddings, and function declarations. The key does not appear in a URL or frontend state.

### OpenAI-compatible

Supports model listing, chat completions, streaming, multimodal message content, embeddings, and tool calls against a configurable endpoint. Loopback endpoints are treated as local; remote endpoints remain subject to network and Local Only policy.

## Model routing

The Oraborus Model Router derives required capabilities from the request and attachments, then applies policy:

1. Local Only or private request: local providers first
2. Vision: vision-capable model
3. Code: coding-capable model
4. Tool intent: tool-call-capable model
5. General conversation: configured healthy model
6. Finite fallback chain
7. Explicit failure when no compatible model is available

The selected provider, model, required capabilities, reason, and fallback chain are returned to diagnostics.

## Agent engine

The registry contains thirty purpose-specific agents. The planner emits an acyclic graph with stable step identifiers and explicit dependencies. Verification depends on all relevant prior steps. Default concurrency is ten and is bounded to ten in the current release.

Run states are persisted in `agent_runs`; Agent Town reads only active persisted runs and otherwise shows an honest idle state.

## Tool engine

Every tool declaration includes input and output schemas, risk, timeout, agent access, implementation module, default state, and verification method. Implementations are modular Rust functions or a bounded child process. Registered tools without an available host dependency return configuration or availability errors.

## Security Gate

Risk classes:

| Class | Meaning | Default behavior |
|---|---|---|
| READ_ONLY | Observation without mutation | Allowed when the capability is enabled |
| SAFE | Low-risk reversible action | Allowed at Level 1 |
| SENSITIVE | Private context or meaningful mutation | Approval or scoped policy |
| DESTRUCTIVE | Data loss or difficult rollback | Expiring approval |
| CRITICAL | Credentials, messages, money, security, dangerous commands | Expiring approval every time |

Approval records preserve redacted payloads, task relation, expiry, and decision. A resolved or expired approval cannot be reused.

## Memory

Memory is divided into conversation, long-term, preference, behavioral rule, episodic, semantic, and task layers. Storage requires an explicit user action and rejects credential and payment-like patterns. Optional embeddings are stored with provider, model, and dimension metadata. Retrieval combines cosine similarity with recency; when an embedding model is not configured, the UI discloses lexical and recency fallback.

## Data layer

SQLite runs in WAL mode with foreign keys and a five-second busy timeout. Migration `0001_init.sql` creates all required tables and indexes. Future changes add numbered migrations and never edit an already released migration.

High-level domains:

- identity and settings
- providers and models
- conversations and messages
- memories and embeddings
- documents and chunks
- agents, tools, tasks, and runs
- automations and runs
- permissions, approvals, and audit records
- API adapters and integrations
- notifications and system snapshots
- research evidence
- knowledge entities and relations
- plugins

## Browser automation

A separate Node process runs Playwright with a persistent profile. The Rust boundary validates URL schemes, action count, timeout, and privacy state. The runner prefers selectors, checks reviewed text when supplied, disables automatic downloads, and returns final URL, title, timestamp, action count, screenshot path, or downloaded path. Authentication stays inside the browser profile.

## Files and terminal

Filesystem tools canonicalize every source and destination against approved roots, reject implicit overwrite, avoid symlink traversal, and verify changes using existence, size, and SHA-256. PowerShell receives commands through standard input, uses a restricted launch profile, captures bounded output, enforces timeout and cancellation, and classifies dangerous patterns before execution.

## Vision and voice

Screen capture is on demand and requires both a privacy switch and approval. Evidence includes dimensions, timestamp, path, and SHA-256. OCR calls an installed Tesseract engine. Vision models receive only the approved image.

Voice adapters detect Piper, Windows SAPI, Coqui-compatible engines, Kokoro-compatible engines, eSpeak NG, Whisper, faster-whisper, and Vosk-compatible executables. Language support is model-specific. Wake-word and microphone loops stay off until the user selects and enables an installed engine.

## Research and public APIs

The direct research reader accepts only HTTP(S), applies size and timeout limits, strips non-content markup, hashes source content, and preserves canonical URL and retrieval time. General discovery requires a configured search connector. Public API entries are data-driven and begin disabled except public GitHub reads; health checks update real states and back off through scheduler policy.

## Background operation and recovery

The tray exposes Open, Quick Command, Pause, Resume, Stop All, Privacy Mode, Settings, and Exit. SQLite persists task and run state. On restart, unresolved dangerous actions are never replayed. Interrupted tasks remain inspectable and require a fresh decision before mutation.

## Extension boundary

A plugin manifest declares identity, version, author, entry path, capabilities, permissions, dependencies, security requirements, and optional signature. Absolute paths and traversal are rejected. Production policy should require signed packages and an isolated host process before enabling third-party code.
