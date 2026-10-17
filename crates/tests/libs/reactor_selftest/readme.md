# Reactor native self-test

This executable runs the Reactor WinUI adapter inside the Reactor application host. Generated
coverage constructs and clears every schema property and event contract. Handwritten phases cover
controlled input, ordered native events, attachments, images, references, pointer injection,
retirement, TreeView content, ListView data, and virtualization.

```text
cargo run -p test-reactor-selftest --quiet
cargo run -p test-reactor-selftest --quiet -- --headless
```

A watchdog terminates the process with an error if WinUI hangs or the application fails to exit.
Application lifecycle, menu, Canvas, WebView, and window-state fixtures live in
[`test-reactor-integration`](../reactor_integration/readme.md). They use the public application
API without enabling Reactor's diagnostic `test` feature.

Asynchronous phases wait for native readiness rather than assuming a timer tick is sufficient.
The ContentDialog check requests opening before its owner window exists, waits for `Opened`, then
calls `Hide` and requires `Closed`.
Use assertions for failures in component callbacks. Calling `process::exit` there can run
application teardown while callback state is still borrowed.

CI also runs the self-test and native library tests in debug mode with
`RUSTFLAGS="-D warnings --cfg windows_cast_diagnostics"`. `check_casts.ps1` checks their captured
output for unexpected same-pointer `QueryInterface` calls. Its allowlist covers agility checks,
canonical COM identity, activation, and known conversions from `IInspectable`; these still require
queries even when the returned pointer is unchanged. Review new diagnostics before adding an
exception. A clean result covers only the paths exercised by those tests.

Run every default Reactor, Composition, and WebView sample long enough to catch startup failures:

```text
powershell -File crates\tests\libs\reactor_selftest\run_samples.ps1
```

The script builds each package independently, captures runtime stdout and stderr, requires the
process to remain alive for three seconds, and then closes or terminates that process. Use
`-RunSeconds`, `-Family`, or `-Package` to narrow or lengthen a run, and `-KeepLogs` to retain all
captured output.

Use `-Package` with `-ChangedSince <commit>` to run the named packages plus samples changed between
that commit and `HEAD`. Added files and deleted files within surviving packages count as changes;
removed packages are not run. `-List` prints the selection without building or launching anything.
The comparison commit must be available locally.

## CI

PRs and master pushes run the native self-test once, with cast diagnostics, and retain the native
regressions, public integration fixtures, deployment checks, isolated feature checks, and public
API snapshot. Workspace unit tests and generation checks belong to `test.yml` and `gen.yml`.
The two [quarantined live theme checks](#quarantined-live-theme-checks) run only on scheduled and
manually dispatched workflows.

The smoke gate runs Counter, Gallery, Workbench, the Reactor Composition host, standalone system
Composition, and Reactor WebView, plus changed or new samples. PRs compare against their base commit;
pushes compare against the previous branch tip, including every commit in the push. Each package
is built independently, and GUI processes run serially.

Scheduled and manually dispatched runs also enforce coverage floors, repeat the native self-test,
and launch every sample. The routine gate targets roughly ten minutes; the exhaustive runs can take
longer. CLI tools use pinned `cargo install` versions. The public API tool has a separate cache saved
immediately after installation, so a later test failure does not discard the compiled tool.

Run the sample selection checks without starting any windows:

```text
pwsh -File crates\tests\libs\reactor_selftest\test_samples.ps1
```

## Quarantined live theme checks

PR and master-push gates exclude the checks below. Scheduled and manually dispatched workflows
still run them, including both deployment modes for `window_theme`, and failures fail the job.
Deterministic theme/error-handling tests, window-observer tests, and other native tests remain
enabled in the routine gates. This is a temporary coverage gap, not a fix for either failure.

| Check | Unresolved failure |
| --- | --- |
| `window_theme` | Phase 11 observed Dark instead of Light in the installed-runtime run. |
| `system_menu_theme_updates_live_and_propagates_errors` | A flyout item reported Light instead of Dark while its anchor had the expected theme. |

Both failures occurred in [PR #5121's workflow run](https://github.com/microsoft/windows-rs/actions/runs/39045500488):
the first in attempt 1, the second in attempt 2. Both checks use 300 ms timer steps; a timing
cause has not been established. Re-enabling them requires investigating theme propagation with
bounded observations and useful phase diagnostics, not longer sleeps or retries.

Run the checks locally on an interactive desktop:

```text
cargo run -p test-reactor-integration --bin window_theme
cargo run -p test-reactor-integration --features self-contained --bin window_theme
cargo test -p windows-reactor --lib native::transient_menu::tests::system_menu_theme_updates_live_and_propagates_errors -- --exact --ignored --nocapture
```
