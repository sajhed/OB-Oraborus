# Troubleshooting

## Desktop backend unavailable

Run the desktop shell rather than the browser-only Vite page:

```powershell
npm run tauri:dev
```

## Ollama not connected

Check that Ollama is installed, the service is running, and the configured endpoint is `http://127.0.0.1:11434`. Verify with `ollama list`. OB does not start or install Ollama without consent.

## Gemini requires configuration

Store a valid Gemini credential inside Settings and enable the provider. Credentials entered in environment files are intentionally not read by the frontend.

## Browser runner requires configuration

Install Node dependencies, then install Playwright Chromium after consent:

```powershell
npm install
npm run browser:install
```

Corporate endpoint security can block child browser processes. Use an approved browser policy rather than weakening security controls.

## PDF extraction unavailable

Install a trusted Windows build of Poppler and ensure `pdftotext.exe` is on `PATH`.

## OCR unavailable

Install Tesseract and the needed language data. Common language codes are `eng`, `ben`, and `hin`, subject to the installed packs.

## Voice engine unavailable

Open Settings → Voice. OB reports only detected engines. Install and configure a model path for Piper, Coqui-compatible, or Kokoro-compatible engines as required.

## File search requires configuration

Enable filesystem access and add one or more approved roots. OB intentionally starts with an empty root list.

## GPU, battery, or temperature unavailable

Windows and hardware vendors expose different counters. OB shows unsupported sensors as unavailable. It never substitutes estimated values.

## Rust build fails

Confirm the MSVC Rust toolchain and Visual Studio Desktop C++ workload:

```powershell
rustup default stable-msvc
rustup target add x86_64-pc-windows-msvc
rustc --version
cargo --version
```

Then delete `src-tauri/target` and rerun the build script.

## Installer not signed

The build creates an unsigned local installer unless a Windows code-signing identity is configured in protected release infrastructure. Do not distribute unsigned binaries as trusted production updates.
