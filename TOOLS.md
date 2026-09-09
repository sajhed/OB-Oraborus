# Tool Registry

`tools/registry.json` defines fifty-eight tools spanning system telemetry, Windows control, files, terminal, browser, vision, voice, memory, documents, data, research, GitHub, security, tasks, knowledge, automation, notifications, messaging, calendar, media, public APIs, and hardware.

## Required declaration fields

- name and description
- input and output JSON schemas
- risk class
- timeout
- agent access
- verification method
- implementation module
- default enabled state

## Dispatch contract

```text
agent proposal
  → registry lookup
  → schema validation
  → privacy check
  → autonomy policy
  → Security Gate
  → approval when required
  → bounded execution
  → observation
  → tool-specific verification
  → redacted audit record
```

## Availability

A declared tool does not imply that its host dependency or external account is connected. Runtime health determines availability. Missing Tesseract, Poppler, Playwright Chromium, AI models, account authorization, or Windows sensor support produces an explicit configuration or availability result.

## Verification examples

- File copy: destination size and SHA-256 match source
- App launch: matching process appears
- Browser navigation: final URL and title
- Terminal: exit code and requested postcondition
- Screen capture: image dimensions, timestamp, and hash
- Memory write: stored identifier reads back
- API call: status, schema validation, source, and timestamp
- Message send: official connector delivery confirmation
- Brokerage order: broker-assigned order identifier and accepted state

Critical connector and market tools remain disabled in the default registry.
