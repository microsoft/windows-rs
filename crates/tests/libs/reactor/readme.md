# Reactor tests

`test-reactor` exercises Reactor's public API in real WinUI applications. Fixtures live in `src/`;
UI Automation tests live in `tests/ui/`. Keep deterministic unit tests in `windows-reactor` and
performance benchmarks separate.

| Benchmark | Purpose |
| --- | --- |
| [`test-reactor-bench`](../reactor_bench/readme.md) | Headless update, allocation, and memory measurements |
| [`test_reactor_perf`](../reactor_perf/readme.md) | Live WinUI workload for the Rust/C# comparison harness |

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
cargo test -p test-reactor --test ui -- --ignored
```
