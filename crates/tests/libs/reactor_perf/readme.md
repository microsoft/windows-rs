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

[comparison]: https://github.com/microsoft/microsoft-ui-reactor/blob/main/.github/workflows/perf-compare.yml
