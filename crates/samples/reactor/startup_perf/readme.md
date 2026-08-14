# Windows Reactor startup tracing sample

This blank app demonstrates TraceLogging events across the Windows Reactor startup lifecycle. It
renders one static `TextBlock` in a 1000x1000 window.

The app uses the ETW team's `tracelogging` crate to emit TraceLogging events from the
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

The build script stages `Microsoft.WindowsAppRuntime.Bootstrap.dll` beside the executable.

## MSIX package

Run these commands from the repository root:

1. Install Windows App Development CLI 0.5.0 or later, or upgrade an existing installation:

    ```powershell
    # First installation:
    winget install Microsoft.WinAppCli --source winget

    # Existing installation:
    winget upgrade Microsoft.WinAppCli --source winget

    winapp --version
    ```

2. Build the app and prepare the package layout:

    ```powershell
    cargo build -p reactor_startup_perf --release --quiet
    $work = "target\reactor-startup-msix"
    $layout = "$work\layout"
    Remove-Item $layout -Recurse -Force -ErrorAction Ignore
    New-Item $layout -ItemType Directory | Out-Null
    Copy-Item `
        target\release\BlankWindowsReactor.exe, `
        target\release\microsoft.windowsappruntime.bootstrap.dll `
        $layout
    $output = "$work\BlankWindowsReactor_x64.msix"
    $manifest = "crates\samples\reactor\startup_perf\package\Package.appxmanifest"
    ```

3. Create an unsigned package:

    ```powershell
    winapp pack $layout --manifest $manifest --output $output --skip-pri
    ```

    To sign without a timestamp, use one `winapp pack` call:

    ```powershell
    $certificate = "C:\certificates\package.pfx"
    $securePassword = Read-Host "Certificate password" -AsSecureString
    $certificatePassword = [Net.NetworkCredential]::new("", $securePassword).Password
    winapp pack $layout `
        --manifest $manifest `
        --output $output `
        --skip-pri `
        --cert $certificate `
        --cert-password $certificatePassword
    ```

    `winapp pack` cannot timestamp a signature. For timestamped signing, package first and then
    sign:

    ```powershell
    $certificate = "C:\certificates\package.pfx"
    $timestampUrl = "https://timestamp.example.com"
    $securePassword = Read-Host "Certificate password" -AsSecureString
    $certificatePassword = [Net.NetworkCredential]::new("", $securePassword).Password
    winapp pack $layout --manifest $manifest --output $output --skip-pri
    winapp sign $output `
        $certificate `
        --password $certificatePassword `
        --timestamp $timestampUrl
    ```

The checked-in manifest uses version `1.0.0.0` and the Microsoft corporate publisher. Update its
`Identity` before packaging if either value needs to change. The publisher must exactly match the
signing certificate subject.

WinApp CLI accepts local and UNC certificate paths, but it passes the path directly to `signtool`;
copy the PFX locally first if the signing environment does not allow `signtool` to read the share.
Certificate passwords are command-line arguments because WinApp CLI does not provide a secure input
or environment-variable option.

The package remains framework-dependent. Install `Microsoft.WindowsAppRuntime.2.msix` on the target
machine before launching it.
