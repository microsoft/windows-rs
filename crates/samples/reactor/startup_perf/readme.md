# Windows Reactor startup tracing sample

This blank app demonstrates TraceLogging events across the Windows Reactor startup lifecycle. It
renders one static `TextBlock` in a 1000x1000 window.

The app uses `windows-tracing` to emit TraceLogging events from the
`BenchmarkSyntheticApps` provider:

- Provider GUID: `FD80D616-E92B-4B2B-9BED-131ADA36A8FD`
- Keyword: `0x0000400000000000`
- App name: `blank_windows_reactor`
- Process name: `BlankWindowsReactor.exe`

| Event | Location |
| --- | --- |
| `wWinMainEntry` | Before Windows App SDK bootstrap |
| `XamlAppLoaded` | First render-function entry |
| `WindowLoaded` | First post-commit effect |
| `FirstRender` | First `CompositionTarget::Rendering` callback after commit |
| `FirstIdle` | Low-priority dispatcher callback after `FirstRender` |
| `ProcessStop` | Immediately before process exit after the final window closes |

Every event includes `AppName`, `Seq`, and `Pid` fields.

Run the framework-dependent app with:

```powershell
cargo run -p reactor_startup_perf --release
```

The build script stages `Microsoft.WindowsAppRuntime.Bootstrap.dll` and `resources.pri` beside the
executable.

## MSIX package

`package.ps1` builds the release executable and creates an x64 MSIX containing the executable,
bootstrap DLL, PRI file, manifest, and generated placeholder logos. The publisher must match the
certificate that will sign the package.

Create an unsigned package for inspection:

```powershell
.\package.ps1 -Publisher "CN=Publisher"
```

Sign with a PFX:

```powershell
$password = Read-Host "Certificate password" -AsSecureString
.\package.ps1 `
    -Publisher "CN=Publisher" `
    -CertificatePath C:\certificates\package.pfx `
    -CertificatePassword $password
```

The output defaults to `target\reactor-startup-msix\BlankWindowsReactor_x64.msix`. The package
remains framework-dependent; install `Microsoft.WindowsAppRuntime.2.msix` on the target machine
before launching it. Certificate paths may be local or UNC paths; network PFX files are copied to a
temporary local file for `signtool` and removed after signing.
