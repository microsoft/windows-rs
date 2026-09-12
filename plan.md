# Tray icon and application lifetime plan

This file tracks temporary implementation and validation work. Delete it before opening the final
pull request.

## Decisions

- `windows-trayicon` remains a focused Shell integration crate.
- Its dependency direction remains `windows-trayicon -> windows-window -> windows-core`.
- It does not depend on `windows`, `windows-sys`, or `windows-reactor`.
- The first release stays small: icon, tooltip, activation, context-menu request, geometry, update,
  Shell restart recovery, and RAII cleanup.
- Icon files use Rust paths and preserve Windows paths that are not UTF-8. Other icon sources can
  be added later without removing the file-path API.
- Initial registration errors return from `build`; later Shell restoration failures are reported
  as `TrayIconEvent::Unavailable`.
- Shell callbacks are converted to posted messages before invoking application handlers. This
  avoids Shell-call reentrancy and wakes message loops blocked in `GetMessageW`.
- Popup controls, raw input, notification balloons, promotion policy, and legacy callback details
  are out of scope until a concrete application needs them.
- Reactor integration is application-lifetime work, not tray-icon implementation.
- The standalone crate can be reviewed separately from Reactor lifetime changes, but it must pass
  the standalone gates below first.

## Gate 1: standalone API review

- [x] Add the publishable `windows-trayicon` crate and generated private bindings.
- [x] Add hidden top-level callback-window support to `windows-window`.
- [x] Avoid changing a host application's process DPI policy.
- [x] Use `NIM_ADD`, `NIM_MODIFY`, `NIM_DELETE`, and `NOTIFYICON_VERSION_4`.
- [x] Decode mouse, keyboard, and context-menu activation into semantic events.
- [x] Query placement with `Shell_NotifyIconGetRect` without taskbar-position assumptions.
- [x] Recover after `TaskbarCreated` and avoid duplicate-registration state.
- [x] Keep owned `HICON` and callback `HWND` lifetimes safe.
- [x] Make the event enum non-exhaustive.
- [x] Review the initial icon-source API before publication.
- [x] Confirm whether runtime restoration failure needs an event, an error hook, or both.
- [x] Confirm naming: crate name, `TrayIconEvent`, `Activate`, `ContextMenu`, and `Unavailable`.
- [x] Decide whether the first release should own a minimal native popup menu or only report the
      context-menu request.

Gate: no unresolved public API concern that would require a breaking change immediately after
publication.

## Gate 2: automated validation

- [x] Unit-test tooltip validation and version-4 callback decoding.
- [x] Test hidden callback-window creation and visibility.
- [x] Add an ignored live Shell test for registration and geometry.
- [x] Run the live Shell test successfully on Windows.
- [x] Run targeted tests, rustfmt, clippy with warnings denied, rustdoc, and package creation.
- [x] Regenerate bindings and generated workflows without an unexpected diff.
- [x] Add focused tests for clearing and replacing a tooltip.
- [x] Add focused tests for replacing an icon while registered.
- [x] Add a controllable test seam for registration failure and restart recovery if it can remain
      private and small.
- [x] Add ignored UI Automation coverage for a real native popup menu and menu-item invocation.

Gate: deterministic behavior is covered without requiring Explorer for the normal test suite.

## Gate 3: interactive Windows validation

Extend the existing sample or add a temporary manual harness. Do not turn the library into a menu
or application framework to satisfy this gate.

- [x] Repeat the first manual pass with console logging and persistent activation counters. The
      original clear-tooltip harness made successful events indistinguishable from failures.
- [x] Classify any apparent duplicate activation by timing separate mouse, keyboard, and context
      actions. The first logged pass included repeated user attempts and could not prove a Shell
      duplicate.
- [x] Verify mouse activation fires once for a single click.
- [x] Verify keyboard selection fires `Activate`.
- [x] Verify right-click context-menu requests report usable screen coordinates.
- [x] Verify the standard tooltip appears and can be changed and cleared.
- [x] Verify replacing the icon succeeds live and restores the previous owned icon after a
      simulated update failure.
- [x] Verify `rect` works for the live icon before and after Explorer recreates notification-area
      placement.
- [x] Verify the icon survives an Explorer restart and remains interactive.
- [x] Verify dropping `TrayIcon` removes the icon immediately.
- [x] Verify the sample exits without a ghost icon.
- [x] Verify the native menu opens adjacent to the icon on the same monitor with mixed-DPI
      displays.
- [x] Check mixed-monitor DPI popup placement without changing the host process DPI policy.
- [x] Record Windows 11 as the initial tested baseline; Windows 10 was not available locally.
- [x] Record any reproducible duplicate-click behavior before adding debounce logic. Timed
      single-click validation produced one activation, so no debounce is included.

Gate: the core user-visible behavior works on a real taskbar without relying on undocumented
taskbar placement or registry state.

## Gate 4: generic host integration

- [x] Prove the icon works with the `windows-window` message loop.
- [x] Prove the icon works with another message-loop owner without calling `windows_window::run`.
- [x] Confirm that a tray-only process can explicitly decide when to quit.
- [x] Confirm that a visible `windows-window` and a tray icon can be created and dropped
      independently.
- [x] Confirm that a popup-menu owner can follow the `SetForegroundWindow` and light-dismiss rules
      without requiring more public API from `windows-trayicon`.

Gate: `windows-trayicon` is message-loop compatible but does not own application lifetime.

## Gate 5: Reactor application lifetime

This is a separate design and implementation track. A simple Reactor sample with an open window is
not enough because current last-window behavior exits the Reactor loop.

Proposed API:

```rust,ignore
App::run_with(|app: &AppContext| {
    app.open_window(View::component::<MainWindow>(()))?;

    let exit = app.proxy();
    let tray = TrayIcon::new("app.ico")
        .menu(Menu::new().item(1, "Exit"))
        .on_event(move |event| {
            if matches!(event, TrayIconEvent::MenuItem { id: 1 }) {
                exit.exit();
            }
        })
        .build()?;

    Ok(tray)
})
```

`run_with` retains the returned value on the UI thread until explicit application exit. This keeps
tray icons and other app-scoped resources alive without a public keep-alive counter.
`AppContext::open_window` creates Reactor windows while the loop is running.
`AppContext::proxy` returns a cloneable `AppProxy` whose dispatch and exit operations may be called
from other threads. Existing `run`, `run_windows`, and `run_component` retain
last-Reactor-window-exits behavior.

- [x] Specify one process-wide `App::run_with` lifetime that may temporarily have zero windows.
- [x] Specify explicit exit and a UI-thread application context.
- [x] Specify a cloneable cross-thread proxy for posting application work.
- [x] Decide that the startup return value owns app-scoped resources instead of adding a public
      keep-alive counter.
- [x] Preserve `run_component` as a convenience API with last-window-exits behavior.
- [x] Allow tray-first startup to create a Reactor window later.
- [x] Allow a Reactor window to close while the tray icon and process remain alive.
- [x] Allow the tray icon to disappear and be created again while Reactor windows remain alive.
- [x] Verify the nested-window self-test and WebView lifecycle continue to use the shared pump.
- [x] Add a tray-first integration sample that exercises both peer lifetime directions.
- [x] Activate the existing Reactor window when the tray receives another open request.
- [ ] Add minimize-to-tray policy only if Reactor gains a window close/minimize interception API.

Gate: windows, tray icons, pickers, WebViews, and background services are peers under application
lifetime rather than implicit owners of the process.

## Gate 6: deep review and release polish

- [x] Separate initial WinUI activation from explicit restore-and-foreground requests.
- [x] Keep HWND lookup best-effort for explicit foreground activation.
- [x] Bound Explorer recovery retries and test the retry state.
- [x] Avoid application callback reentrancy when posting a tray event fails.
- [x] Clean up partial Shell registration after add/version failures.
- [x] Propagate startup errors without returning an error through WinUI's `OnLaunched` callback.
- [x] Test startup failure, rejected startup windows, and explicit exit with an active window.
- [x] Run the self-test harness under explicit application lifetime in self-contained CI.
- [x] Give the tray sample and tests tray-specific icon assets.
- [x] Resolve and document the popup-coordinate contract for hosts with mixed DPI-awareness
      contexts.
- [ ] Recheck tray-only popup placement and scale after creating the hidden callback window under
      per-monitor-v2 DPI awareness.
- [ ] Run the complete framework-dependent and self-contained live suites on CI.
- [ ] Repeat the interactive tray, Explorer recovery, foreground, and explicit-exit checks after
      the final implementation changes.

Gate: deterministic tests and CI cover failure paths and both lifetime modes, with no unresolved
coordinate or shutdown behavior.

## Pull request gates

### Standalone tray-icon pull request

- [x] Gates 1 through 4 are complete.
- [x] The diff contains no speculative Reactor API.
- [x] Public docs state the initial scope and popup-menu responsibility.
- [x] Generated files and CI matrices are current.
- [ ] Remove this temporary `plan.md` if this is the final pull request for the work.

### Reactor lifetime pull request

- [x] Gate 5 has an agreed API design.
- [x] The implementation works with tray-first, window-first, and zero-window transitions.
- [x] Existing `run_component`, picker, WebView, and multi-window behavior remains compatible.
- [x] The integration sample exercises real lifetime transitions rather than only coexisting while
      a window is open.
- [ ] Remove this temporary `plan.md`.

## Useful commands

```text
cargo test -p windows-trayicon -p test_trayicon -p test_window --quiet
cargo test -p test_trayicon -- --ignored --nocapture
cargo check -p trayicon-basic --quiet
cargo clippy -p windows-trayicon -p windows-window -p test_trayicon -p test_window \
    -p trayicon-basic --all-targets
cargo doc -p windows-trayicon --no-deps
cargo package -p windows-trayicon --allow-dirty --no-verify
cargo run -p tool-bindings --quiet
cargo run -p tool-yml --quiet
```
