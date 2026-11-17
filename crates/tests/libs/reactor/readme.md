# Reactor tests

`test-reactor` exercises Reactor's public API in real WinUI applications. Fixtures live in `src/`;
UI Automation tests live in `tests/ui/`. Keep deterministic unit tests in `windows-reactor` and
performance benchmarks separate. [`test_reactor_perf`](../reactor_perf/readme.md) is the live WinUI
workload for the Rust/C# comparison harness.

## Running

```text
cargo test -p test-reactor
cargo test -p test-reactor --test ui canvas:: -- --nocapture
```

Use Cargo name filters to select cases and `-- --nocapture` to see driver steps.

Live UI tests run on x64 and require an interactive desktop. Start them from a foreground terminal,
keep other windows from covering the fixtures, and leave the mouse and keyboard idle.
Active-scroll removal coverage requires Windows animation effects to be enabled. It removes the
view after observing an intermediate offset, checks one `Unavailable` completion per request, and
compares delegate counts across repeated removals when COM diagnostics are enabled.
CI enables animation effects on its disposable runners; the tests do not change desktop settings.
Cases are serialized and run in separate fixture processes with bounded timeouts. Failures report
the child process's stdout/stderr log directory; successful cases remove their logs.

`build.rs` generates native bindings and deploys the Windows App Runtime and WebView2 projection
DLL. The first build needs network access for the pinned packages. Canvas uses WARP rendering.

## Leak checks

With `RUSTFLAGS=--cfg windows_cast_diagnostics`, each fixture process asserts that no
`#[implement]` objects or delegates remain alive after the app exits. CI runs the suite this way.

```text
$env:RUSTFLAGS = "--cfg windows_cast_diagnostics"
cargo test -p test-reactor
```

## Scroll layout

`scroll-layout` checks far-edge anchoring on both axes during content growth and shrinkage,
scrolling away from the edge, disabling anchoring with `NaN`, and clearing the declared ratios.
It does not register anchor candidates or test row preservation inside the content or during
animated scrolling. `scroll-orientation` uses a stretched square image to observe measurement
constraints and the restored native default; content orientation does not cap explicitly sized
children.

## Submitted-frame input

`canvas-submitted-input` stages a two-row layout separately from drawing it. Its `submitted`
marker is updated from `Canvas::on_submitted`, not from the draw callback. New-frame clicks wait
for that marker; staged or failed drawing must leave old-frame hit testing intact. Click sequence
numbers prevent an old result from satisfying a later assertion. The fixture also checks callback
invalidation and removal. Its device-loss case returns an error from the draw callback; it does
not remove the native device or measure physical display.

`pointer-routing` checks queued delivery, routed bubbling, handled events, and mode replacement.
Headless pointer tests separately check synchronous capture and reentrant callback replacement.
Mode replacement must preserve queued occurrences and their original callbacks before any removal
or teardown; those later operations cancel pending occurrences.

`pointer-cursor` checks that a Border's declared cursor, its inherited ancestor cursor, and clearing
both apply while the pointer stays still. It compares the global cursor with shared system cursors.

```powershell
cargo test -p test-reactor --test ui -- `
    canvas_input_follows_submission_not_pending_drawing `
    pointer_delivery_modes_preserve_routing_and_queued_order
```

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
