# windows-reactor benchmarks

This crate measures the Rust-side `windows-reactor` planner and component frontend with
`RecordingRuntime`. It does not include WinUI control creation, layout, rendering, or COM calls.
Dedicated positional array and tuple rows include view construction so frontend allocation changes
are not hidden by prebuilt planner inputs. The virtual construction row likewise measures source
construction separately from reconciliation. Separate component rows measure idle and
effect-bearing retained memory, effect mount cost, and isolated effect updates.
Border rows measure construction and retained native-tree memory with zero, one, two, and four
properties, plus one and four active events. These rows keep property and event storage changes
visible.

Run the benchmark in release mode:

```powershell
cargo run -p test-reactor-bench --bin test-reactor-bench `
    --release --quiet -- --iters 500 --reps 12
```

Measure targeted updates through recursive Reactor2 component scopes:

```powershell
cargo run -p test-reactor-bench --bin reactor2-component-bench `
    --release --quiet -- --samples 5000
```

This benchmark builds balanced recursive component trees, targets the deepest leaf, and reports
retained bytes per scope, update latency, allocations, allocated bytes, and mutation count.
Recording-adapter batch validation is disabled during timed updates because it intentionally clones
the complete adapter state before applying a batch.

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
cargo build -p test-reactor-bench --bin test-reactor-bench --profile profiling
wpr.exe -start CPU -filemode
.\target\profiling\test-reactor-bench.exe --iters 500 --reps 12
wpr.exe -stop .\target\reactor-cpu.etl
```

Use the Heap profile when allocation call stacks are needed:

```powershell
wpr.exe -start Heap -filemode
.\target\profiling\test-reactor-bench.exe --iters 500 --reps 12
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
cargo run -p reactor-virtual --bin reactor-virtual-perf `
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
`--component-cells` wraps each cell in a component boundary while preserving the native control
tree, which isolates component input and publication costs.

Run an unattended ten-second update workload:

```powershell
cargo run -p test-reactor-bench --bin reactor-live-grid `
    --release --quiet -- --headless --percent 10 --duration 10 --churn-count 0
```

Measure the same native grid with one component per cell:

```powershell
cargo run -p test-reactor-bench --bin reactor-live-grid `
    --release --quiet -- --headless --component-cells --percent 10 --duration 10 --churn-count 0
```

Run the same workload while removing and restoring 400 cells per update:

```powershell
cargo run -p test-reactor-bench --bin reactor-live-grid `
    --release --quiet -- --headless --percent 10 --duration 10 --churn-count 400
```

Without `--headless`, click `Start` to begin the fixed-duration run. Live mode always creates a
WinUI window because native control creation, property application, and destruction are part of the
measurement.

The process writes one JSON object to standard output. It contains the run configuration, update
count, Rust allocation count and bytes, process CPU time, average and peak working set and private
bytes, and average and p95 host-dispatch and native-apply times in microseconds.
`cpu_core_percent` treats 100% as one logical core. Host dispatch includes component reconciliation
and command publication; native apply is the command-application portion. The object has this shape
and is compacted to one line:

```json
{
  "benchmark": "reactor-live-grid",
  "headless": true,
  "dirty_percent": 10.000,
  "churn_count": 400,
  "component_cells": false,
  "duration_ms": 1000.000,
  "updates": 30,
  "rust_allocations": 0,
  "rust_allocations_per_update": 0.000,
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

## Reactor2 live comparison

`reactor-live-compare` runs the current Reactor and experimental Reactor2 frontends in separate
processes. The default `grid` surface uses the same native `Grid` containing keyed `TextBlock`
children. The `list` surface renders a real `ListView` for larger container and virtualization
workloads. Both runs use the same logical data, deterministic update sequence, render-paced update
count, allocator, and process CPU and memory measurements.

Run both frontends with identical arguments:

```powershell
.\crates\tests\libs\reactor_bench\compare-live.ps1 `
    -Workload text -Count 512 -Updates 120
```

Open a visible 10,000-item ListView and run it for about ten seconds:

```powershell
cargo run -p test-reactor-bench --bin reactor-live-compare --release --quiet -- `
    --frontend reactor2 --surface list --workload text --count 10000 --updates 600
```

The supported workloads are `text`, `rotate`, `reverse`, and `churn`. `text` changes one child
property, `rotate` moves one keyed child from the front to the back, `reverse` exercises a dense
reorder, and `churn` alternately removes and restores `--churn-count` trailing children.

The Grid surface is the strict identical-native-tree comparison. Reactor uses visual list items
while Reactor2 uses keyed data items, so the ListView surface is an end-to-end scale and backend
policy comparison rather than an identical native representation.

The shared model stores text as `Rc<str>`. Reactor2 declarations can retain those values without
allocating a new string for every unchanged item, while ordinary `String` and `&str` builder inputs
remain supported.

Each run prints one JSON object containing throughput, Rust allocations, live Rust-byte growth,
process CPU, working-set and private-memory samples, and end-to-end update latency. Reactor also
reports its separately instrumented native-apply latency. Reactor2 update latency includes
declaration construction, direct retained-arena reconciliation, and native mutation application.

This is an acceptance gate rather than a demonstration benchmark. Do not expand Reactor2 based on
recording-backend results when this native comparison regresses. The benchmark exposed and then
verified fixes for two early regressions: allocating empty property and relation vectors for every
declaration object, and replacing a complete native collection for a one-item move. The
`reactor2-live-phases` JSON object separates declaration construction, runtime, and native-adapter
time and allocation so later optimizations remain attributable.

## Controlled TextBox input

`reactor-live-notepad` and `reactor2-live-notepad` inject real Unicode keyboard input into a
controlled TextBox and wait for each update to finish. Use `--single-line` for the Reactor run so
both frontends use the same plain TextBox configuration:

```powershell
cargo run -p test-reactor-bench --bin reactor-live-notepad --release --quiet -- `
    --warmup 100 --samples 1000 --text-size 0 --single-line
cargo run -p test-reactor-bench --bin reactor2-live-notepad --release --quiet -- `
    --warmup 100 --samples 1000 --text-size 0
```

Reactor2 reports callback-to-reconcile latency, emitted mutations, and native `SetText` calls.
Native observations enter the retained graph before reconciliation, so a controlled rerender of
the value received from `TextChanged` produces no mutation and no native setter call.

Representative matched runs produced:

| Initial text | Frontend | End-to-end median | End-to-end p95 | Allocations/input | Bytes/input |
| ---: | --- | ---: | ---: | ---: | ---: |
| 0 | Reactor | 4.70 ms | 6.99 ms | 16 | 3,433 |
| 0 | Reactor2 | 4.80 ms | 6.68 ms | 4 | 1,245 |
| 100,000 | Reactor | 40.90 ms | 43.56 ms | 16 | 401,500 |
| 100,000 | Reactor2 | 40.75 ms | 43.60 ms | 4 | 200,285 |

Most end-to-end time is inside WinUI input processing. Reactor2 callback-to-reconcile measured
about 1.0 us median for short text and 1.7 us for 100,000-byte text. Native events first enter a
revision-checked queue and wake one later UI-dispatch turn, so application callbacks never run
inside the native event handler. The large-text allocation result still scales at about twice the
text length because minimal bindings convert native `HSTRING` to `String` before Reactor2 copies it
into `Rc<str>`. The representation benchmark below measures whether avoiding that copy justifies
changing the internal or public string model.

`reactor2-string-bench` isolates that representation choice:

```powershell
cargo run -p test-reactor-bench --bin reactor2-string-bench --release --quiet -- `
    --iterations 1000 --text-size 100000
```

At 100,000 bytes, representative results were:

| Operation | Time | Rust allocations | Rust bytes |
| --- | ---: | ---: | ---: |
| `str -> Rc<str>` | 1.67 us | 1 | 100,016 |
| `str -> HSTRING` | 48.47 us | 0 | 0 |
| `Rc<str>` clone | 0.7 ns | 0 | 0 |
| `HSTRING` clone | 9.9 ns | 0 | 0 |
| `HSTRING -> String -> Rc<str>` | 139.15 us | 3 | 200,016 |
| `HSTRING -> Rc<String>` | 91.01 us | 3 | 100,040 |

The allocator cannot see storage allocated by the Windows string implementation, so zero Rust
bytes for `HSTRING` construction does not mean zero allocation. `HSTRING` is compact and cheap to
clone, but constructing it from UTF-8 and converting it back are much slower for large strings.
`Rc<String>` removes one content copy on native input but requires two allocations for ordinary
declaration construction and would introduce a second retained string representation.

Keep `Rc<str>` as the declaration, retained, and public callback representation. The avoidable
native-input copy is about 48 us at 100,000 bytes, less than 0.2% of the roughly 41 ms end-to-end
input path, and does not justify a dual UTF-8/UTF-16 model. Revisit this only if a future API can
retain an opaque native string end to end without forcing ordinary application state to use
`HSTRING`.
