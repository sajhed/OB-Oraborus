# Third-Party Notices

This repository uses open-source packages through Cargo and npm. Each package retains its own license. Before distribution, generate and review an exact dependency inventory from the resolved lockfiles.

Primary libraries include Tauri, React, Vite, TypeScript, Zustand, Lucide, Tokio, Reqwest, Rusqlite, Serde, Sysinfo, Playwright, and their transitive dependencies. Optional host programs such as Ollama, Tesseract, Poppler, Piper, Whisper, Vosk, Coqui-compatible engines, Kokoro-compatible engines, and eSpeak NG are not redistributed by this repository.

Recommended release checks:

```powershell
cargo install cargo-about
cargo about generate about.hbs > artifacts/rust-licenses.html
npm licenses --json > artifacts/npm-licenses.json
```

Review all resolved licenses, notices, model licenses, and redistribution terms before shipping a signed binary.
