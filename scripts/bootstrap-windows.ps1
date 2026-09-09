$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

function Require-Command([string]$Name, [string]$Guidance) {
  if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) { throw "$Name is required. $Guidance" }
}
Require-Command node "Install Node.js 20 or newer."
Require-Command npm "Install npm with Node.js."
Require-Command rustc "Install Rust stable MSVC from https://rustup.rs/."
Require-Command cargo "Install Rust stable MSVC from https://rustup.rs/."

$NodeMajor = [int]((node --version).TrimStart('v').Split('.')[0])
if ($NodeMajor -lt 20) { throw "Node.js 20 or newer is required." }

if (Test-Path package-lock.json) { npm ci } else { npm install }
python scripts/validate_project.py
Write-Host "Core dependencies are ready." -ForegroundColor Green
Write-Host "Playwright Chromium is optional and can be installed with: npm run browser:install" -ForegroundColor Cyan
Write-Host "Start OB with: npm run tauri:dev" -ForegroundColor Cyan
