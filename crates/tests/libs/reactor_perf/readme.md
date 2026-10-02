# Reactor stress benchmark

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
cargo run -p test_reactor_perf --release -- --headless --percent 10 --duration 10
```

The report is written next to the executable as `StressPerf.Reactor.report.txt`. Add `--json` to
also write `StressPerf.Reactor.metrics.json`.

The in-app frame counter uses `CompositionTarget::Rendering`, matching the C# workload. As with the
C# harness, render completion is a better no-admin throughput proxy, and ETW presents are required
for a final user-visible frame-rate comparison.
