# Portable Mode

`build-windows.ps1` copies the release executable as `OB-Oraborus-Portable.exe`. This is a convenience binary, not an isolated data container.

Limitations:

- Application data defaults to the normal Windows user application-data location.
- Windows Credential Manager remains machine/user scoped.
- Startup registration and tray behavior depend on local policy.
- WebView2 and optional browser, OCR, voice, and model runtimes must exist on the host.
- Moving the executable does not move memories, settings, or credentials.

A future fully self-contained portable profile must define encrypted local secret storage, explicit data-root selection, and safe migration before release.
