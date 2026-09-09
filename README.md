# OB Oraborus

> **Intelligence, Inside Your World.**

OB Oraborus is a local-first AI operating layer for Windows 10/11. It combines provider-agnostic AI, scoped agents, verified tools, persistent memory, browser and file automation, voice and vision adapters, system telemetry, and a permission-first desktop interface.

OB is engineered around one loop:

**THINK → ACT → OBSERVE → VERIFY → REMEMBER**

## What is in this repository

- Tauri 2 desktop shell and Rust backend
- React, TypeScript, Vite, and Zustand interface
- SQLite database with migrations for conversations, memories, agents, tools, tasks, automations, permissions, audit records, API adapters, system snapshots, research sources, and a knowledge graph
- Ollama, Gemini, and OpenAI-compatible provider adapters
- Capability-aware model routing with a local/private preference
- Thirty scoped agents and fifty-eight registered tools
- Security Gate with five risk levels, expiring approvals, path scoping, command analysis, and secret redaction
- Local semantic memory storage with optional provider embeddings, cosine similarity, metadata, and recency ranking
- Playwright browser runner using selectors and explicit verification evidence
- Real host telemetry through `sysinfo`; unavailable sensors remain unavailable
- Local document extraction for text, source, DOCX, and PDF when Poppler is installed
- Tesseract OCR and configurable local voice-engine adapters
- Windows tray, pause, privacy mode, Stop All, and global command shortcut
- English, Bangla, and Hindi interface dictionaries
- Windows NSIS build workflow and portable executable packaging script

## Honest capability behavior

OB does not synthesize connection success, agent work, telemetry, citations, browser actions, or file outcomes. A missing dependency or account is reported as **NOT CONNECTED**, **UNAVAILABLE**, or **REQUIRES CONFIGURATION**. Market execution, message sending, camera access, microphone capture, screen capture, and destructive tools are disabled until explicitly enabled and authorized.

The source tree is buildable on a Windows x64 development machine. The supplied build script creates `OB-Oraborus-Setup.exe` and copies the unsigned portable executable to `OB-Oraborus-Portable.exe`. Production distribution still requires your organization’s Windows code-signing certificate and release infrastructure.

## Quick start on Windows

Requirements:

- Windows 10 or 11 x64
- Node.js 20+
- Rust stable with the MSVC target
- Microsoft Visual Studio Build Tools with Desktop C++ workload
- WebView2 Runtime

```powershell
Set-ExecutionPolicy -Scope Process Bypass
./scripts/bootstrap-windows.ps1
npm run tauri:dev
```

Configure an AI provider inside **Settings → AI**:

- **Ollama**: install and start Ollama; no cloud credential is needed.
- **Gemini**: enter your API key in the credential dialog. It is written to Windows Credential Manager, not SQLite or browser storage.
- **OpenAI-compatible**: configure an endpoint and, when required, store its credential through the same secure dialog.

## Production build

```powershell
./scripts/build-windows.ps1
```

Outputs:

```text
artifacts/OB-Oraborus-Setup.exe
artifacts/OB-Oraborus-Portable.exe
artifacts/BUILD_REPORT.json
```

The portable executable uses the same application data directory unless a future signed portable policy is configured. Secure credential storage, startup registration, and browser runtime discovery can differ between machines; see `docs/PORTABLE_MODE.md`.

## Browser engine

Playwright is installed as a workspace dependency. Install its Chromium runtime only after consent:

```powershell
npm run browser:install
```

OB never attempts to defeat login protection, CAPTCHA, encryption, or site restrictions.

## Safe defaults

- Autonomy Level 1
- Cloud AI optional
- Microphone off
- Camera off
- Screen capture off
- Approved folders empty
- Message auto-send off
- Financial execution off
- Destructive and critical actions require expiring approval
- No hidden telemetry

## Source map

```text
src/                    React desktop interface
src-tauri/              Rust application, commands, security, data, and adapters
agents/                 Thirty versioned agent declarations
tools/                  Tool registry and verification contracts
browser/                Isolated Playwright runner
integrations/           Public API registry seed
plugins/                Extension manifest example
scripts/                Validation and Windows build automation
docs/                   Operational and design documentation
BUILD_STATE.json        Machine-readable delivery state
```

## Validation

```powershell
npm run validate
npm run lint
npm test
cargo test --manifest-path src-tauri/Cargo.toml
```

`BUILD_STATE.json` is updated by the project validator and distinguishes source checks from native Windows compilation.

## Security and privacy

Read `SECURITY.md` and `PRIVACY.md` before enabling background operation, external connectors, screen access, or high-risk tools. External webpages and documents are always treated as untrusted data, never as policy instructions.

## License

MIT for this repository. Third-party components retain their own licenses; see `THIRD_PARTY_NOTICES.md`.
