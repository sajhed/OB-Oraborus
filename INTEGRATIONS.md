# Integrations

## AI providers

| Provider | Transport | Secret storage | Default |
|---|---|---|---|
| Ollama | Loopback HTTP | None | Enabled, health-dependent |
| Gemini | HTTPS | Windows Credential Manager | Disabled until configured |
| OpenAI-compatible | HTTP(S) | Windows Credential Manager when needed | Disabled until configured |

Model capabilities are conservative and model-specific. Selecting a provider does not invent vision, embeddings, or tool-calling support.

## Browser

Playwright runs in a separate Node process with its own profile and Chromium installation. Install the browser runtime with user consent. Browser actions respect authentication boundaries and website controls.

## GitHub

Public read requests use the official REST API and its unauthenticated rate limit. Optional credentials are stored in Windows Credential Manager. Writes are critical and need explicit approval plus read-back verification. Push is never automatic.

## Public API registry

The seed registry contains actual adapter metadata for Open-Meteo, REST Countries, WorldTimeAPI, GitHub, Open Library, and Frankfurter. Only GitHub public reads begin enabled. Health status starts unknown and changes only after a real check. Dead or rate-limited services can be disabled without code changes.

## Voice and OCR

Detected local options:

- Piper
- Windows SAPI
- Coqui-compatible CLI
- Kokoro-compatible CLI
- eSpeak NG
- Whisper
- faster-whisper
- Vosk-compatible CLI
- Tesseract OCR

Availability and language coverage depend on installed executables and models. OB does not download large models silently.

## Documents

Built-in UTF-8 extraction covers text, Markdown, JSON, CSV, source code, and configuration formats. DOCX uses local Open XML parsing. PDF uses a local Poppler `pdftotext` installation. Unsupported formats remain unavailable until an approved parser adapter is installed.

## Messaging and calendars

The tool contracts are present, but account connectors are not bundled with credentials. Production adapters must use official APIs, explicit OAuth or equivalent authorization, rate limits, delivery identifiers, and revocation. Personal-account browser scraping is not the primary architecture.

## Market Lab

Market Lab is disabled by default. The current repository provides risk and tool contracts, not a live broker credential or enabled order channel. Paper analysis can be added through validated market-data adapters. Real-money execution requires a separately reviewed broker plugin and hard limits.

## Remote control

No unauthenticated remote command endpoint exists. A future remote module must implement encrypted pairing, device identity, authorization scopes, revocation, replay protection, and audit records before it can be enabled.
