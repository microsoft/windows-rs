# Reactor Rust/C# comparison

`test_reactor_perf` compares windows-reactor with the C# Reactor
`StressPerf.ReactorOptimized` workload:

- 70 columns x 70 rows = 4,900 `TextBlock` cells;
- deterministic `.NET Random(42)` data and update sequence;
- a 33 ms update timer;
- at least one randomly selected cell per tick, controlled by `--percent`;
- cached cell declarations rebuilt only for changed indices;
- snapshot-driven root component updates;
- phase, frame, allocation, and working-set reporting.

Run the live benchmark in release mode:

```powershell
cargo run -p test_reactor_perf --release -- --percent 10 --duration 10
```

This opens a WinUI window and requires an interactive desktop. The `--headless` option is accepted
for the comparison harness, but only changes the displayed label; it does not disable the window
or rendering. For non-UI measurements, use [`test-reactor-bench`](../reactor_bench/readme.md).

The report is written next to the executable as `StressPerf.Reactor.report.txt`. Add `--json` to
also write `StressPerf.Reactor.metrics.json`.

The [C# Reactor comparison workflow][comparison] builds this crate from a pinned windows-rs revision.
It replaces `build.rs` to deploy a self-contained runtime. Coordinate changes to the crate path,
package name, command-line options, or report format with that harness. This fixture is separate
from the functional suite in [`test-reactor`](../reactor/readme.md).

The in-app frame counter uses `CompositionTarget::Rendering`, matching the C# workload. As with the
C# harness, render completion is a better no-admin throughput proxy, and ETW presents are required
for a final user-visible frame-rate comparison.

## Task viewer workload

```powershell
cargo run -p test_reactor_perf --release -- --workload tasks --count 1000 --cycles 20
```

This separate workload leaves the grid comparison and its report format unchanged. It opens a
virtualized task list with a filter, detail editor, toolbar, and independent background-status
component. Each cycle replays selection, two local draft edits, status progress, filtering and
clearing, a stable-key row update, reversal and restoration, detail removal and remounting, then
another status update. One complete warmup cycle is excluded.

The replay sends component messages every 33 ms and waits for each commit before advancing. It does
not synthesize keyboard input, scroll, or measure input routing, IME behavior, focus preservation, or
save operations. The filter is script-controlled; do not interact with the window during a run.
Virtual rows are realized by WinUI, not by a fixed simulated viewport.

`TaskViewer.Reactor.report.txt` beside the executable groups measurements by action:

| Metric | Measurement boundary |
| --- | --- |
| Median/p95 commit time | Message dispatch through the target component's post-commit effect |
| Mean app time/bytes | Component model changes and declaration construction before that effect |
| Mean bytes/calls | Process-wide Rust allocator traffic within the commit interval |
| Mean view calls | Workspace/list/detail/status/row calls through the following timer tick |
| Rendering callback gaps | Intervals between `CompositionTarget::Rendering` notifications |
| Detail mounts | One initial mount and one remount per warmup/measured cycle |

Commit time includes dispatcher delay and native property application, but not subsequent layout,
virtual realization, or presentation. App time is not a full framework/native phase breakdown.
View counts include late virtual realization; its allocations are outside the commit interval.
Rust allocation counts exclude WinUI allocations and do not measure retained memory. Rendering
callbacks are not presentation timestamps; an idle UI can have long gaps without dropped frames.

Use the same window size, display scale, instrumentation, and quiet interactive desktop for
comparisons. Leave the window visible. Repeat runs at 100, 1,000, and 10,000 tasks to separate
collection-size effects from the realized viewport. The replay uses `--cycles`, not `--duration` or
`--percent`; `--json` remains grid-only. No speedup against direct WinUI or C# is implied: those
implementations do not yet have a matching task-viewer replay.

[comparison]: https://github.com/microsoft/microsoft-ui-reactor/blob/main/.github/workflows/perf-compare.yml
