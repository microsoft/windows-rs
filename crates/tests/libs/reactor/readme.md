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
Scale transition coverage also requires animations. It observes centered intermediate rendering,
including interrupted transitions, clearing scale, resizing, and removal.
WinUI hides an element immediately when its scale changes to zero; the tests check growth from zero
and shrinkage to a nonzero factor, not animated collapse to zero. To reproduce the native limitation,
run the `scale` fixture and invoke `full`, `transition`, then `zero`. The same sequence with a plain
WinUI Border and a three-second `Vector3Transition` shows the same immediate disappearance.
Cases are serialized and run in separate fixture processes with bounded timeouts. Failures report
the child process's stdout/stderr log directory; successful cases remove their logs.

`build.rs` generates native bindings and deploys the Windows App Runtime and WebView2 projection
DLL. The first build needs network access for the pinned packages. Canvas uses WARP rendering.

## Leak and cast checks

With `RUSTFLAGS=--cfg windows_cast_diagnostics`, each fixture process asserts that no
`#[implement]` objects or delegates remain alive after the app exits and that it made no avoidable
casts: a cast to the same interface or from a WinRT interface to `IInspectable`, which need no
`QueryInterface` call. CI runs the suite this way.

```text
$env:RUSTFLAGS = "--cfg windows_cast_diagnostics"
cargo test -p test-reactor
```

## WebView callbacks

`webview-callbacks` exercises the `windows-webview` Reactor bridge with page-to-host messages,
script completion, and window teardown with a pending script. Non-`Send` callback captures must
be invoked and destroyed on the UI thread. The fixture closes itself and does not inject input.

## Collection interop

`collection-contracts` runs without opening a window or injecting input. It checks boxed-string
agility, a WinUI Border's free-threaded marshaler and canonical identity after MTA resolution, and
collection adaptation in ComboBox, AutoSuggestBox, and ItemsSourceView. Instrumented sources
distinguish direct indexed access from eager enumeration. Read-only vectors reject mutations with
`E_NOTIMPL`; ComboBox selection is checked by index and object.

`collection-live-mutable`, `collection-live-read-only`, `collection-live-shared`, and
`collection-live-trusted` compare marshaled mutable vectors, instrumented read-only vectors, and both
shared read-only storage modes in ComboBox, AutoSuggestBox, BreadcrumbBar, and RadioButtons.
UI Automation exercises selection and invocation across source replacement and clearing.
Instrumented sources must receive indexed
reads and no mutation attempts, including during teardown. These cases do not measure performance,
verify pixels, or test final native destruction on another thread.

## Scroll layout

`scroll-layout` checks far-edge anchoring on both axes during content growth and shrinkage,
scrolling away from the edge, disabling anchoring with `NaN`, and clearing the declared ratios.
It does not register anchor candidates or test row preservation inside the content or during
animated scrolling. `scroll-orientation` uses a stretched square image to observe measurement
constraints and the restored native default; content orientation does not cap explicitly sized
children.

## Button padding

`button-padding` compares a 24-DIP FontIcon in a 40-DIP Subtle button against an unclipped reference.
It checks padding at mount, uniform and asymmetric updates, zero padding, and restoration of the
native default when omitted. Rendered icon bounds, button identity, and click delivery are observed.

## Context snapshots

`context` checks provider updates through cached intermediate components, equal-value suppression,
shadowing, removal, and element identity. Its consumer wraps `ViewContext` with one borrow lifetime.
`context-virtual` checks two contexts in retained and newly realized rows, a row-local override,
provider removal and reapplication, and row-local messages reading the latest snapshot.

## Background cancellation

`callback-lifetime` checks UI-owned captures retained by native delegates and a COM implementation.
Calls from another thread must return `RPC_E_WRONG_THREAD` without touching UI state. Final release
on a worker queues capture destruction on the owner thread. Shutdown cancels retained captures;
later owner-thread calls return `RO_E_CLOSED`, and worker release must not destroy them again.
The fixture exercises the shared adapter with a real dispatcher, not off-thread calls to WinUI's
element factory or the native inner object of a composed application.

`background` retains a worker and local timer across child replacement, observing timer-payload
release before releasing the worker and checking its cancellation snapshot. `background-shutdown`
keeps both handles and the worker gate outside the host. It records timer-payload release during
the owner's effect cleanup, before final application timer cleanup, then releases the worker after
the app exits and waits for its pre-enqueue cancellation acknowledgement. Process exit alone is not
evidence of cancellation.

## Nested retirement

`retirement-nested` removes and remounts a five-component subtree while keeping a sibling alive.
`retirement-virtual` scrolls that subtree offscreen, removes its virtual key, and then restores it.
WinUI can recycle the first row during startup and can keep offscreen rows realized. The fixture
audits those startup lifetimes too; scrolling alone is not treated as a retirement boundary.

Both cases check timer-payload release before effect cleanup, parent-before-child cleanup,
reverse effect order, and child-before-parent component destruction. Timer handles stay in the
surviving fixture. Cleanup records capture release at that moment, not after handle destruction.
Retained senders are exercised after remount, with a later message marking completion of the check.
The surviving sibling must retain its native identity and have no cleanup or drop.

These tests do not impose sibling traversal order or cover host shutdown. They also do not observe
private ancestor-scope availability or native-reference availability during cleanup; changes to
those boundaries need their own evidence.

## TextBox appearance

`text-appearance` observes padding through native layout and corner radius through rendered pixels,
including updates and clearing back to native defaults without replacing the TextBox. A keyboard
shortcut changes these properties during editing; focus, text, and continued typing are checked.
A leading icon overlay checks pointer pass-through, interception, and default hit testing after
clearing the property. This does not test state-specific resource styling or resource-dictionary
updates.

## Window maximization

`window-maximizable` and `window-maximizable-frame` check the setting at mount, live changes,
and omission restoring the default for standard and WinUI title bars. Tests inspect native window
styles and synchronously send caption double-click messages to observe maximize/restore behavior,
including the disabled no-op. They preserve native content identity and resize/minimize flags.
This tests caption handling, not physical pointer routing or programmatic maximize requests.

## Element backdrops

`backdrop` observes a bounded native system backdrop over a red panel, with a clickable foreground
button. It checks initial Acrylic, replacement with Mica and Mica Alt, explicit `None` and omitted
backdrops, rounded and square corners, omitted corner radius, resizing, and repeated removal and
remounting. Pixels outside the effect bounds stay red. Foreground identity and pointer clicks are
preserved, and process-exit COM diagnostics check cleanup.

The test observes the presence and clipping of each material, not exact colors or blur strength.
Those depend on Windows settings and native fallback behavior. It does not promise that arbitrary
composition effects, shadows, or transforms apply to WinUI's backdrop surface.

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

WinUI can drop the first pointer move shortly after a window is shown
([microsoft/microsoft-ui-xaml#12753](https://github.com/microsoft/microsoft-ui-xaml/issues/12753)),
so `pointer-routing` and `pointer-cursor` repeat their first move until the fixture observes the
pointer entering.

```powershell
cargo test -p test-reactor --test ui -- `
    canvas_input_follows_submission_not_pending_drawing `
    pointer_delivery_modes_preserve_routing_and_queued_order
```

## Notification icon updates

The `tray` image- and tooltip-refusal cases remove only the fixture's Shell registration, leaving
its callback window alive, to force a real `NIM_MODIFY` failure. They check refusal diagnostics,
retry on an unchanged declaration, current callbacks, and removal with pending work.
The image case also changes the requested path while both image and tooltip updates are pending.
It loads the same image through distinct paths to exercise replacement, not pixel appearance.
Each case sends `TaskbarCreated` only to that callback window to restore the registration; it does not
restart Explorer or reproduce the cause of a post-resume Shell failure. Headless tests check the image
and tooltip payload retained on failure and applied on a successful retry.

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
