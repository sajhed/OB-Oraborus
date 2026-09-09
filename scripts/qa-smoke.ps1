$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root
$Cases = @(
  "Open Chrome and verify its process",
  "Find PDF files in an approved folder",
  "Read system RAM from the host",
  "Store a non-secret preference through approval",
  "Reject a credential-like memory",
  "Stop all active cancellable work",
  "Show unavailable providers honestly",
  "Capture a screen only after privacy access and approval",
  "Read an explicit research URL with source and timestamp",
  "Keep Market Lab execution disabled"
)
python scripts/validate_project.py
npm test
cargo test --manifest-path src-tauri/Cargo.toml
$Cases | ForEach-Object { Write-Host "MANUAL: $_" -ForegroundColor Cyan }
