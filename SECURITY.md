# Security Model

## Principles

1. Deny ambient authority.
2. Keep private work local when requested.
3. Treat webpages, documents, model output, and connector data as untrusted.
4. Separate model reasoning from tool authorization.
5. Require evidence before reporting success.
6. Keep secrets out of prompts, logs, memory, browser state, and crash reports.
7. Make Stop All available from the window and tray.

## Trust boundaries

- User input is an instruction candidate.
- Model output is a proposal, never authority.
- External content is data, never policy.
- Agent declarations constrain available tools.
- Tool schemas constrain inputs.
- Privacy switches constrain context acquisition.
- Security Gate and autonomy policy decide whether execution is allowed.
- Verification decides whether success may be reported.

## Prompt-injection defense

Retrieved text is wrapped as untrusted context and cannot alter system policy, grant permission, request credentials, or cause a tool call. Tool arguments are produced as structured data and checked independently. Requests that attempt to cross the content/instruction boundary are recorded as blocked security events.

## Command execution

PowerShell execution is off by default. Commands are previewed and classified. Patterns associated with disk formatting, boot changes, security configuration, recursive deletion, credential changes, arbitrary downloaded code, and destructive Git commands escalate to destructive or critical review. Time, output, working directory, and cancellation are bounded.

The pattern classifier is defense in depth, not a complete shell sandbox. Production enterprise deployments should combine it with Windows AppContainer, WDAC, application allowlists, and a dedicated low-privilege broker.

## Filesystem safety

- No default indexed roots
- Canonical approved roots
- No silent elevation
- No implicit overwrite
- New paths validated through their existing parent
- Copy verified by SHA-256
- Move verified by source absence and destination presence
- Deletion always destructive
- Sensitive system folders excluded from normal policies

## Secret handling

Provider and connector secrets use Windows Credential Manager through the Rust keyring adapter. The frontend sends a secret only over local Tauri IPC during the storage call. The secret is not returned. Audit serialization recursively redacts keys matching credential, secret, token, cookie, password, authorization, and API-key patterns.

Do not store secrets in `.env`, SQLite, automation definitions, memory, model prompts, issue reports, or release archives.

## Browser safety

- HTTP(S) only
- Selector-based actions
- Optional reviewed target text
- Automatic download acceptance disabled
- Explicit save path
- No CAPTCHA or login-control bypass
- Bounded extraction size
- Dedicated profile
- Final URL and title verification

## Screen, microphone, and camera

All begin off. Screen capture requires a visible state, privacy permission, and scoped approval. Microphone and wake-word loops require deliberate enablement. Camera support follows the same policy and is not silently activated.

## Messaging and finance

Sending a message is critical unless an explicitly configured low-risk automation policy applies. Drafting does not send. Official connector APIs are required.

Market Lab begins disabled and does not submit brokerage orders. Any future broker adapter must independently enforce symbol allowlist, trading hours, order validation, maximum position, maximum exposure, daily loss, trade count, cooldown, explicit activation, and a market kill switch. Broker confirmation is the only valid execution evidence.

## Logging

Audit entries contain timestamp, category, action, actor, risk, state, summary, and recursively redacted structured detail. Retention is configurable. Secret values, raw audio, raw screen frames, and document contents are not written to general logs.

## Release security

Production releases must be built on a protected Windows runner, use locked dependencies, generate a software bill of materials, sign the executable and installer, verify update metadata, and publish checksums through a separate trusted channel. The repository does not embed a signing identity.

## Security testing

The validator and Rust tests cover registry integrity, migration completeness, secret redaction, memory credential blocking, task graph cycles, path-policy structure, dangerous command escalation, old-name removal, and incomplete-code markers. Windows CI adds TypeScript, unit, Rust, and Tauri build checks.

Report vulnerabilities privately to the maintainer responsible for your deployment. Do not include credentials or private user data in a report.
