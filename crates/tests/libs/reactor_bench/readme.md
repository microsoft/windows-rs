# windows-reactor benchmarks

This crate measures the Rust-side `windows-reactor` planner and component frontend with
`RecordingRuntime`. It does not include WinUI control creation, layout, rendering, or COM calls.
Dedicated positional array and tuple rows include view construction so frontend allocation changes
are not hidden by prebuilt planner inputs. Separate component rows measure idle and effect-bearing
retained memory, effect mount cost, and isolated effect updates.

Run the benchmark in release mode:

```powershell
cargo run -p test_reactor_bench --bin test_reactor_bench `
    --release --quiet -- --iters 500 --reps 12
```

On a branch with full Git history, compare blocking allocation and retained-memory metrics with its
merge base:

```powershell
.\crates\tests\libs\reactor_bench\compare.ps1 -BaseRef origin/master
```

The comparison fails when allocation count increases or bytes per operation or retained component
memory increase by more than 10%. Timing is displayed for diagnosis but is not a hosted-runner gate.
`mount_shutdown` includes cloning the input `View` and constructing `RecordingRuntime` inside each
timed iteration. Run timing commands with `--release`.

## Profiling

The workspace `profiling` profile keeps release optimizations and emits symbols. Build the headless
benchmark with it, capture CPU sampling with Windows Performance Recorder, and inspect the ETL in
Windows Performance Analyzer:

```powershell
cargo build -p test_reactor_bench --bin test_reactor_bench --profile profiling
wpr.exe -start CPU -filemode
.\target\profiling\test_reactor_bench.exe --iters 500 --reps 12
wpr.exe -stop .\target\reactor-cpu.etl
```

Use the Heap profile when allocation call stacks are needed:

```powershell
wpr.exe -start Heap -filemode
.\target\profiling\test_reactor_bench.exe --iters 500 --reps 12
wpr.exe -stop .\target\reactor-heap.etl
```

WPR may require an elevated terminal. The benchmark's counting allocator remains the regression
metric for allocation count, allocated bytes, and retained bytes; ETW profiles explain where those
costs originate. Use `reactor-live-grid` with the same profile when WinUI, COM, layout, and process
memory need to be included.

Benchmark output starts with `reactor-benchmark-format: 1`. The comparison requires this marker in
both revisions. A merge base without the marker predates the final benchmark architecture, so the
script warns and exits successfully instead of comparing unrelated rows and formats.

## Bounds

Use these bounds until integrated samples provide a better workload:

- isolated component message p99 < 500 us at 16,384 scopes without reconciling unrelated subtrees;
- clean and source-only compile time <= the incumbent;
- retained component memory <= 4 KiB per scope at 16,384 scopes;
- 512-row broad reconciliation < 1 ms and 4,096-row broad reconciliation < 8 ms;
- 10,000-item virtual source update < 2 ms and 32-row realize/recycle < 100 us;
- reference-heavy mount adds < 50% time and < 10% transient bytes over the same unreferenced
  controls;
- thin release binary <= the incumbent, while each generated control tranche continues recording
  its incremental binary growth.

## Virtual editor application gate

The virtual editor sample now owns a release-mode `RecordingRuntime` driver so the benchmark uses
the same controlled inputs, parent-owned durable task model, contexts, effects, focus references,
background completion path, and virtual rows as the application. Run it with:

```powershell
cargo run -p sample_reactor_virtual --bin reactor-virtual-perf `
    --features perf --release -- --samples 500
```

The mixed cycle performs a background completion, a selection-changing parent update, and a complete
32-row recycle/realize batch. `process_realizations` has a 32-request work budget, so the driver
drains both the recycle and realization turns before stopping the timer.

These are Rust planning, publication, effect, and `RecordingRuntime` command-application times.
They exclude WinUI control work, layout, rendering, and presentation. Profile before changing
architecture if Rust planning approaches 4 ms or sustained p95 frame time exceeds 16.7 ms. Start
with repeated key/view collection, avoidable subtree reconciliation, copy-on-write mutation
granularity, unchanged child/property cloning, and component boundaries.

## Live grid and churn benchmark

`reactor-live-grid` measures a live WinUI tree with a seeded 70x70 stock grid. Every update
changes the configured percentage of stock prices. `--churn-count` alternately removes and restores
that many trailing cells, so `0` measures property updates without native control churn.

Run an unattended ten-second update workload:

```powershell
cargo run -p test_reactor_bench --bin reactor-live-grid `
    --release --quiet -- --headless --percent 10 --duration 10 --churn-count 0
```

Run the same workload while removing and restoring 400 cells per update:

```powershell
cargo run -p test_reactor_bench --bin reactor-live-grid `
    --release --quiet -- --headless --percent 10 --duration 10 --churn-count 400
```

Without `--headless`, click `Start` to begin the fixed-duration run. Live mode always creates a
WinUI window because native control creation, property application, and destruction are part of the
measurement.

The process writes one JSON object to standard output. It contains the run configuration, update
count, Rust allocator bytes, process CPU time, average and peak working set and private bytes, and
average and p95 host-dispatch and native-apply times in microseconds. `cpu_core_percent` treats
100% as one logical core. Host dispatch includes component reconciliation and command publication;
native apply is the command-application portion. The object has this shape and is compacted to one
line:

```json
{
  "benchmark": "reactor-live-grid",
  "headless": true,
  "dirty_percent": 10.000,
  "churn_count": 400,
  "duration_ms": 1000.000,
  "updates": 30,
  "rust_alloc_bytes": 0,
  "rust_alloc_bytes_per_update": 0.000,
  "cpu_time_ms": 0.000,
  "cpu_core_percent": 0.000,
  "working_set_avg_bytes": 0,
  "working_set_peak_bytes": 0,
  "private_avg_bytes": 0,
  "private_peak_bytes": 0,
  "host_dispatch_samples": 0,
  "host_dispatch_avg_us": 0.000,
  "host_dispatch_p95_us": 0.000,
  "native_apply_samples": 0,
  "native_apply_avg_us": 0.000,
  "native_apply_p95_us": 0.000
}
```

The Reactor workflow runs both grid workloads plus the virtual editor's recording and live
workloads on scheduled and manually dispatched runs. It publishes the results in the job summary
and the `reactor-performance` artifact. These live timing and memory results are advisory; pull
requests use the merge-base `RecordingRuntime` allocation and retained-memory gate above.
