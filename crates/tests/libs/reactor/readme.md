# Reactor tests

`test-reactor` exercises Reactor's public API in real WinUI applications. Fixtures live in `src/`;
UI Automation tests live in `tests/ui/`. Keep deterministic unit tests in `windows-reactor` and
performance benchmarks separate.

## Running

```text
cargo test -p test-reactor
cargo test -p test-reactor --test ui canvas:: -- --nocapture
```

Use Cargo name filters to select cases and `-- --nocapture` to see driver steps.

Live UI tests run on x64 and require an interactive desktop. Start them from a foreground terminal,
keep other windows from covering the fixtures, and leave the mouse and keyboard idle.
Cases are serialized and run in separate fixture processes with bounded timeouts. Failures report
the child process's stdout/stderr log directory; successful cases remove their logs.

`build.rs` generates native bindings and deploys the Windows App Runtime and WebView2 projection
DLL. The first build needs network access for the pinned packages. Canvas uses WARP rendering.

Run this crate alone to check default features; workspace builds can enable Reactor's `test`
feature through benchmark dependencies.

## Opt-in drag/drop coverage

Five drag/drop cases are ignored by default. They require a non-elevated terminal and an idle
interactive desktop. WinUI does not support this path in elevated processes
([microsoft/microsoft-ui-xaml#7690](https://github.com/microsoft/microsoft-ui-xaml/issues/7690));
GitHub-hosted Windows runners run elevated.

```text
cargo test -p test-reactor --test ui drop_policy -- --ignored
cargo test -p test-reactor --test ui drag_source_is_stopped_after_driver_failure -- --ignored
```

`cancelled_drag_can_be_restarted` has an unresolved intermittent UIA timeout (`0x80131505`).
To run all opt-in cases, including that regression:

```text
cargo test -p test-reactor --test ui -- --ignored
```

## App-process coverage

Measure the fixture executable, not just the UIA driver. With the `llvm-tools` Rust component
installed, run this in a fresh PowerShell session at the repository root:

```powershell
$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $true
$target = Join-Path $PWD "target\reactor-ui-coverage"
$profiles = Join-Path $target ("profiles-" + [Guid]::NewGuid())
New-Item -ItemType Directory $profiles | Out-Null
$env:RUSTFLAGS = "-C instrument-coverage"
$env:LLVM_PROFILE_FILE = Join-Path $profiles "%p-%m.profraw"
cargo test -p test-reactor --target-dir $target
$sysroot = rustc --print sysroot
$hostTriple = (rustc -vV | Select-String "^host: ").ToString().Substring(6)
$llvm = Join-Path $sysroot "lib\rustlib\$hostTriple\bin"
$raw = @(Get-ChildItem $profiles -Filter "*.profraw")
& "$llvm\llvm-profdata.exe" merge -sparse $raw.FullName -o "$profiles\ui.profdata"
& "$llvm\llvm-cov.exe" show "$target\debug\test-reactor.exe" `
    "-instr-profile=$profiles\ui.profdata" `
    '--include-filename-regex=native[\\/]+winui[\\/]+(hosting|events)\.rs$'
```

The unique directory isolates each run; `%p-%m` separates child profiles. Adjust the source filter
for the code being investigated. Keep generated code and test scaffolding separate from handwritten
product coverage. Close the PowerShell session afterwards to discard the instrumentation settings.
