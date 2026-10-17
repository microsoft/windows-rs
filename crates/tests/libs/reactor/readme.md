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

On x64 this runs the automated suite, including live WinUI cases and harness failure checks. It requires
an interactive desktop. Cargo deploys the runtime; no machine-wide runtime installation is needed.
The first build needs network access to download the pinned runtime packages. Five native drag/drop
cases are explicitly ignored; see [Opt-in drag/drop coverage](#opt-in-dragdrop-coverage). Foreground
or focus failures do not silently skip tests. Do not interact with other applications while the
suite runs. Standard Cargo name filters can select individual cases during development.

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
| `tests/ui/menu.rs` | Shared button, dropdown, and menu-bar lifetime scenarios |
| `tests/ui/process.rs` | Child ownership, deadlines, shutdown, and harness failure checks |
| `tests/ui/pointer.rs` | Native mouse injection, owned button cleanup, and physical coordinates |
| `src/pointer.rs` | Property-only pointer capture and focus fixtures |
| `src/drag/` | Property-only drop target and owned native OLE text source |
| `tests/ui/drag.rs` | Opt-in drag negotiation, policy changes, and cancellation |

The library contains the application fixtures. Cargo's integration-test harness supplies the
host executable's path without filesystem searches or a child mode in the test runner. Keeping the
host separate gives WinUI its own main thread and application lifetime. The binary's `test = false`
disables its empty unit-test harness, not the behavioral tests.

Each UI case launches the Cargo-built `test-reactor` executable in a fresh process. The suite
serializes desktop cases even with Cargo's default thread count. UI Automation runs on an MTA
worker, while the test thread watches the child and enforces a 30-second deadline, including blocked
UIA calls. Success requires completed assertions and a successful exit after closing the fixture's
own window. Failure cancels further driver work and terminates the owned child.

Menu cases use the prefixes `menu_` (Button), `dropdown_` (DropDownButton), and `menu_bar_`
(MenuBarItem). Each prefix has the four shared scenarios listed below.

| Case | Assertion |
| --- | --- |
| `*_callback_replacement_and_removal` | An open item's native identity survives callback replacement; invoking it reaches the new callback exactly once. A retired item's invocation does not change the displayed count during a 250 ms observation period. |
| `*_content_replacement_retires_open_items` | Changing content closes the old popup and creates a new item on the same owner. The old item stays inert before and after opening the replacement. |
| `*_reattachment_does_not_revive_retired_items` | Removing and reattaching a menu on the same native owner creates new items without reviving retained providers. |
| `*_owner_replacement_retires_open_items` | Replacing the keyed owner closes its popup and changes native owner/item identity. A callback update on the replacement preserves its open item and reaches the current callback. |
| `text_box_native_edit_and_controlled_reset` | UIA Value edits reach component state; declarative reset clears the native value; focused Unicode keyboard input reaches both state and the native TextBox. |
| `keyed_reorder_moves_only_displaced_children` | Moving the last keyed TextBox to the front keeps every element's native identity and produces the new layout order. The focused TextBox that triggered the reorder is not displaced, so it keeps keyboard focus. |
| `early_successful_exit_is_rejected` | Exiting with code zero before the driver finishes is a failure, not a passing test. |
| `child_panic_is_reported` | A child panic produces a failed exit and captured stdout and stderr. |
| `hung_child_is_terminated` | A ready but unresponsive child times out and is terminated. |
| `driver_timeout_cancels_worker` | The supervisor deadline cancels a cooperative driver and reaps its child without consuming the cleanup allowance. |
| `driver_timeout_bounds_blocked_worker_cleanup` | The supervisor reaps the child and returns after its cleanup allowance even when the driver remains blocked. |
| `pointer_capture_policy_at_mount` / `pointer_capture_policy_added_to_existing_border` | With only the capture policy, movement outside reaches the pressed Border; after release it reaches the outside surface. |
| `pointer_focus_policy_at_mount` / `pointer_focus_policy_added_to_existing_border` | With only the release/focus policy, the Border receives direct native pointer focus after release, not on press. |
| `pointer_button_is_released_after_driver_failure` | A driver failure while pressing the mouse releases the injected button before the fixture is stopped. |
| `text_data_transfers_owned_unicode_memory` | The data source rejects unsupported formats and returns independent, nul-terminated UTF-16 allocations that outlive its COM object. This unit test runs normally. |

The driver timeout regressions use the same supervisor as live UI cases, with short private
deadlines and synthetic workers rather than COM calls. Each case releases and joins its worker
before asserting the outcome. Live cases retain the 30-second deadline and five-second cleanup
allowance.

The driver uses exact automation IDs, or exact name and control type for menu items, scoped to the
child's windows. Repeated entries for the same native element count once; distinct matching elements
are an error. Waits inspect current UI state; they do not force layout or synthesize Reactor
callbacks. UIA invocation, Value edits, and keyboard input are separate operations. Keyboard injection
checks the foreground process, focused element, and modifier keys.

Menu cases establish foreground and keyboard focus before using UIA Invoke to exercise WinUI's
menu actions, not synthetic Reactor events. They retain old invoke providers across content,
attachment, and owner changes, check popup closure including the submenu entry, and invoke retired
providers before and after opening a replacement.
Retired invocations must leave both the callback count and the replacement popup unchanged. Current
items must deliver the expected key and callback generation exactly once; callback counts are
observed for 250 ms to detect extra deliveries. All three surfaces use the same assertions and one
fixture with a typed surface input, not separate copies of the scenario code. The menu-bar cases
also exercise WinUI's item collection updates: duplicate visible entries fail even when the new
command is present. Command-bar and tray surfaces remain in the legacy integration fixture.

Pointer cases use `SendInput` with physical virtual-desktop coordinates derived from UIA anchor
bounds. The fixture's Border has no UIA peer; short text anchors locate its bare background without
clicking the text itself. Guards check foreground ownership, the window under button input, and
held buttons/modifiers. The supervisor owns button cleanup, so cancellation or a blocked UIA worker
cannot leave cleanup dependent on that worker returning. A case that finishes with an injected
button still pressed fails.

The policy fixtures install no press/release callbacks on the target. Capture and release are
observed through native movement routing, not a stored policy or capture flag. Focus is observed
through the target's direct native focus event; setting focus on the baseline button is a separate
driver action. Dynamic cases omit the policy entirely at mount, exercise that state, and then enable
it while preserving the UIA anchor's identity. They do not set the policy to false initially, since
that setter can itself activate a lazy native subscription.

Cargo captures driver steps and assertion failures. On a condition timeout the driver also prints
the fixture's UI tree. Child stdout and stderr are written to separate files to avoid blocked pipes;
failures report the log directory under the system temporary directory. Successful cases remove
their logs. The parent deadline still applies if reading the UI tree stalls.

## Opt-in drag/drop coverage

The native drag/drop cases are preserved from PR #5115, but are not part of the default automated
suite. WinUI does not support this drag/drop path in elevated processes
([microsoft/microsoft-ui-xaml#7690](https://github.com/microsoft/microsoft-ui-xaml/issues/7690)).
GitHub-hosted Windows runners run as administrator. The tests use `#[ignore = "..."]` so Cargo reports
them as ignored, not passed. Explicitly running them elevated fails before launching a fixture.
No runner-specific skip, token manipulation, or automatic elevation change is used.

Run the policy and failure-cleanup cases from a non-elevated terminal on an idle interactive desktop:

```text
cargo test -p test-reactor --test ui drop_policy -- --ignored
cargo test -p test-reactor --test ui drag_source_is_stopped_after_driver_failure -- --ignored
```

The cancellation case also has an unresolved intermittent UIA timeout (`0x80131505`) after
cancellation. Keeping the source window and its STA alive across drags has not eliminated that
failure. It remains an opt-in regression case, not evidence that cancellation is reliable:

```text
cargo test -p test-reactor --test ui cancelled_drag_can_be_restarted -- --ignored --nocapture
```

To run all five opt-in cases, including the unresolved cancellation regression:

```text
cargo test -p test-reactor --test ui -- --ignored
```

| Opt-in case | Assertion |
| --- | --- |
| `drop_policy_at_mount` / `drop_policy_added_to_existing_border` | A callback-free Border accepts native text drags through its policy, including activation after an initially rejected drop. |
| `drop_policy_changes_during_drag` | Source feedback follows Copy-to-Move policy changes while hovering; switching back before release produces a Copy drop that reads the text. |
| `cancelled_drag_can_be_restarted` | Three cancelled hovers preserve the source's native UIA identity; another drag then drops successfully. Subject to the unresolved timeout above. |
| `drag_source_is_stopped_after_driver_failure` | Failure during an accepted OLE hover releases the injected button and reaps the fixture process, including its source thread. |

The source is a native window on a dedicated OLE STA in the fixture process. It receives injected
input and runs `DoDragDrop`, leaving WinUI free to render policy changes and read dropped data.
It advertises only `CF_UNICODETEXT`. Assertions require source-reported hover operations, the final
OLE operation, and a text read, not just a Copy cursor. The target has no drag/drop callbacks that
could mask policy-only activation.

The source window and STA live for the fixture session. Each drag owns its data and feedback
objects. Normal shutdown joins the UIA driver before closing the fixture, which stops and joins the
source thread. The OLE callback checks cancellation and a 20-second deadline; the existing parent
supervisor still bounds blocked native calls. No UIA timeout is retried or treated as success.

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
