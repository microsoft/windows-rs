# Reactor tests

`test-reactor` is the home for the new Cargo-based Reactor tests. It lives alongside the existing
selftest, integration, and performance crates without depending on their harnesses or Reactor's
`test` feature. Add new cases here and migrate existing coverage individually. Remove an old case
only after its replacement proves the same behavior; retire old helpers when their consumers are
gone.

The crate uses the public Reactor API, direct Windows UI Automation, and the Rust standard library.
`build.rs` generates only the required native bindings into `OUT_DIR` using `windows-bindgen` and
stages the host's private Windows App Runtime using `windows-reactor-setup`.
There are no generated test cases, external runners, or dependencies on the `windows` crate.

## Running

```text
cargo test -p test-reactor
```

On x64 this runs the entire suite, including live WinUI cases and harness failure checks. It requires
an interactive desktop. Cargo deploys the runtime; no machine-wide runtime installation is needed.
The first build needs network access to download the pinned runtime packages. There are no ignored
tests or silent skips when foreground or focus is unavailable. Do not interact with other applications
while the suite runs. Standard Cargo name filters can select individual cases during development.

The crate also runs in the ordinary workspace test command. Other workspace packages can enable
Reactor's legacy `test` feature through Cargo feature unification; this fixture still uses only the
public application API and real WinUI. The package-only command covers the default-feature build.

The `ui` integration target is x64-only. The existing x64 workspace job runs the live suite; x86 and
ARM64 build the fixture and run `windows-reactor-setup`'s filesystem and multiprocess tests as part
of workspace testing. This is an explicit architecture boundary, not a runtime skip when focus
or foreground is unavailable.

## Layout

| Location | Purpose |
| --- | --- |
| `build.rs` | Native binding generation and self-contained runtime deployment |
| `src/lib.rs` | Small public-API applications and process-failure probes |
| `src/main.rs` | Entry point for the isolated WinUI host |
| `tests/ui/main.rs` | Named behavioral tests driven through UI Automation |
| `tests/ui/automation.rs` | Scoped lookup, native actions, condition waits, and diagnostics |
| `tests/ui/process.rs` | Child ownership, deadlines, shutdown, and harness failure checks |

The library contains the application fixtures. Cargo's integration-test harness supplies the
host executable's path without filesystem searches or a child mode in the test runner. Keeping the
host separate gives WinUI its own main thread and application lifetime. The binary's `test = false`
disables its empty unit-test harness, not the behavioral tests.

Each UI case launches the Cargo-built `test-reactor` executable in a fresh process. The suite
serializes desktop cases even with Cargo's default thread count. UI Automation runs on an MTA
worker, while the test thread watches the child and enforces a 30-second deadline, including blocked
UIA calls. Success requires completed assertions and a successful exit after closing the fixture's
own window. Failure cancels further driver work and terminates the owned child.

| Case | Assertion |
| --- | --- |
| `menu_callback_replacement_and_removal` | An open item's native identity survives callback replacement; invoking it reaches the new callback exactly once. A retired item's invocation does not change the displayed count during a 250 ms observation period. |
| `text_box_native_edit_and_controlled_reset` | UIA Value edits reach component state; declarative reset clears the native value; focused Unicode keyboard input reaches both state and the native TextBox. |
| `early_successful_exit_is_rejected` | Exiting with code zero before the driver finishes is a failure, not a passing test. |
| `child_panic_is_reported` | A child panic produces a failed exit and captured stdout and stderr. |
| `hung_child_is_terminated` | A ready but unresponsive child times out and is terminated. |
| `driver_timeout_cancels_worker` | The supervisor deadline cancels a cooperative driver and reaps its child without consuming the cleanup allowance. |
| `driver_timeout_bounds_blocked_worker_cleanup` | The supervisor reaps the child and returns after its cleanup allowance even when the driver remains blocked. |

The driver timeout regressions use the same supervisor as live UI cases, with short private
deadlines and synthetic workers rather than COM calls. Each case releases and joins its worker
before asserting the outcome. Live cases retain the 30-second deadline and five-second cleanup
allowance.

The driver uses exact automation IDs, or exact name and control type for menu items, scoped to the
child's windows. Repeated entries for the same native element count once; distinct matching elements
are an error. Waits inspect current UI state; they do not force layout or synthesize Reactor
callbacks. UIA invocation, Value edits, and keyboard input are separate operations. Keyboard injection
checks the foreground process, focused element, and modifier keys.

Cargo captures driver steps and assertion failures. On a condition timeout the driver also prints
the fixture's UI tree. Child stdout and stderr are written to separate files to avoid blocked pipes;
failures report the log directory under the system temporary directory. Successful cases remove
their logs. The parent deadline still applies if reading the UI tree stalls.

## App-process coverage

The pilot uses compiler instrumentation and the `llvm-tools` Rust toolchain component, without a
Cargo coverage plugin. In a fresh PowerShell session at the repository root:

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

The unique profile directory prevents earlier runs from contributing counts. Child processes
inherit the profile pattern, and `%p-%m` separates their output. Report against `test-reactor.exe`,
not just the integration-test driver: the native Reactor code runs in the fixture. The source filter
above shows the menu callback update and native text setter paths exercised by these cases. This
is a pilot path report, not an overall Reactor coverage percentage or a replacement for the core
coverage job. Close the PowerShell session afterwards to discard the instrumentation environment.
