# Development Guide

## Prerequisites

Use Windows 10/11 x64 with Node.js 20+, Rust stable MSVC, Visual Studio Build Tools, WebView2, Git, and PowerShell 7 or Windows PowerShell 5.1.

## Setup

```powershell
./scripts/bootstrap-windows.ps1
npm run tauri:dev
```

The bootstrap script installs JavaScript dependencies, validates source registries, checks the Rust toolchain, and leaves optional Playwright Chromium installation for explicit consent.

## Commands

```powershell
npm run dev
npm run build
npm run lint
npm test
npm run validate
npm run tauri:dev
npm run tauri:build
cargo test --manifest-path src-tauri/Cargo.toml
```

## Adding a provider

1. Implement the full `AiProvider` trait.
2. Keep credential retrieval inside Rust.
3. Return conservative model capabilities.
4. Support cancellation and finite timeouts.
5. Add provider health and error tests.
6. Register the adapter in `ProviderManager`.
7. Document network and data behavior.

## Adding a tool

1. Add a registry declaration.
2. Define strict input and output types.
3. Choose the highest applicable risk class.
4. Implement bounded execution.
5. Implement a real postcondition check.
6. Add redacted audit details.
7. Add security and failure tests.

## Adding an agent

1. Use a stable lowercase identifier.
2. Give it a narrow purpose.
3. Reference existing registered tools.
4. Declare only needed risk classes.
5. Define schemas, timeout, retry, and verification.
6. Run the registry validator.

## Database migrations

Released migration files are immutable. Add a numbered migration, execute inside a transaction, record the version, and test both clean creation and upgrade from the previous release.

## UI standards

- No synthetic telemetry or progress percentages
- Clear empty, loading, unavailable, and error states
- Keyboard focus visible
- Minimum 44px touch target where practical
- Reduced-motion support
- WCAG AA contrast
- No secret values in DOM diagnostics
- Point-cloud visuals must remain secondary to information hierarchy

## Release process

1. Clean checkout on a protected Windows runner.
2. Install locked dependencies.
3. Run project validation.
4. Run TypeScript and unit tests.
5. Run Rust formatting, lint, and tests.
6. Build Tauri NSIS bundle.
7. Sign executable and installer.
8. Verify signature and hashes.
9. Archive build report and software bill of materials.
10. Publish through an authenticated release channel.
