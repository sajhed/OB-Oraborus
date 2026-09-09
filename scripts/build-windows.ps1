$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root
$Artifacts = Join-Path $Root "artifacts"
New-Item -ItemType Directory -Force -Path $Artifacts | Out-Null

if ($env:OS -ne "Windows_NT") { throw "The native installer must be built on Windows." }
foreach ($Command in @("node", "npm", "rustc", "cargo")) {
  if (-not (Get-Command $Command -ErrorAction SilentlyContinue)) { throw "$Command is required." }
}

if (Test-Path package-lock.json) { npm ci } else { npm install }
python scripts/validate_project.py
npm run lint
npm test
npm run build
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets
cargo test --manifest-path src-tauri/Cargo.toml
npm run tauri:build

$Setup = Get-ChildItem "src-tauri/target/release/bundle/nsis" -Filter "*.exe" | Sort-Object LastWriteTime -Descending | Select-Object -First 1
if (-not $Setup) { throw "Tauri completed without an NSIS executable." }
$Portable = Join-Path $Root "src-tauri/target/release/ob-oraborus.exe"
if (-not (Test-Path $Portable)) { throw "Release executable was not produced." }
Copy-Item $Setup.FullName (Join-Path $Artifacts "OB-Oraborus-Setup.exe") -Force
Copy-Item $Portable (Join-Path $Artifacts "OB-Oraborus-Portable.exe") -Force

$Report = [ordered]@{
  product = "OB Oraborus"
  version = (Get-Content package.json | ConvertFrom-Json).version
  builtAt = (Get-Date).ToUniversalTime().ToString("o")
  host = [System.Environment]::OSVersion.VersionString
  node = (node --version)
  rust = (rustc --version)
  cargo = (cargo --version)
  setupSha256 = (Get-FileHash (Join-Path $Artifacts "OB-Oraborus-Setup.exe") -Algorithm SHA256).Hash
  portableSha256 = (Get-FileHash (Join-Path $Artifacts "OB-Oraborus-Portable.exe") -Algorithm SHA256).Hash
  signed = $false
  tests = @("project validation", "TypeScript", "Vitest", "Vite build", "Rust format", "Clippy", "Rust tests", "Tauri NSIS")
}
$Report | ConvertTo-Json -Depth 5 | Set-Content (Join-Path $Artifacts "BUILD_REPORT.json") -Encoding utf8
python scripts/validate_project.py
Write-Host "Build complete: $Artifacts" -ForegroundColor Green
Write-Host "Sign both executables before public distribution." -ForegroundColor Yellow
