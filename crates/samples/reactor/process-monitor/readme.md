# Process monitor

A read-only Reactor sample with a Task Manager-style navigation shell, a virtualized process tree,
CPU and memory columns, filtering, sorting, stable selection, and on-demand executable paths.
It uses documented Win32 APIs without elevation, debug privileges, process termination, or
process-memory reads.

```powershell
cargo run -p reactor-process-monitor --release
cargo run -p reactor-process-monitor --release -- --demo
```

The build deploys a self-contained Windows App Runtime. An interactive Windows desktop is required.
The demo has 60 deterministic snapshots with changing metrics, process exits, and PID reuse. It
does not enumerate real processes.

The **Processes** page supports full-row selection; choose **Show details** to load the selected
process's executable path. Tab and Enter/Space activate rows and controls. Use the chevrons to expand
or collapse branches. Click a column heading to sort, then click again to reverse direction.
Name/PID initially sort ascending and CPU/memory descending; unavailable values stay last. Tree mode
sorts siblings. Turn off **Process tree** for a global ranking. Filtering retains ancestors of
matching processes and expands their paths. Pause stops scheduling new snapshots; an in-flight
snapshot can still complete.

The **Performance** page charts aggregate CPU and physical memory with `windows-canvas`. CPU uses
the language-neutral PDH `Processor Information(_Total)\% Processor Time` counter, not a sum of
process rows or frequency-adjusted utilization. Memory shows total physical memory minus available
memory. Counter errors are shown as unavailable, and missing observations leave gaps in the charts.

Right-click the CPU chart, or focus it and press Shift+F10, then choose **Change graph to**.
**Overall utilization** shows the aggregate counter and **Logical processors** shows a grid
covering all active logical processors. Labels use PDH's `Nnode:number` identity: the NUMA node and
processor number within that node, not physical core numbers or processor-affinity groups. All
plots share the same canvas, device, and swap chain. Resizing changes the grid layout, not processor
identity. The chart's accessibility help text includes each processor's current value or error.

Memory labels follow Task Manager's MB/GB convention, using powers of 1024.

History covers the last 60 recorded seconds, retaining the latest observation per second. CPU
collection runs at most once per second even when process refresh is faster. Pause freezes the
display. Sampling continues across pages, and history, chart selection, process selection, sorting,
and filtering are retained. Returning to Processes recreates its native list and starts at the top.
CPU and Memory share one demand-driven drawing surface. Switching metrics reuses its GPU device;
leaving Performance unmounts the surface but keeps the bounded history. The CPU graph choice is
retained across metric and page changes. Logical histories have the same 60-second bound; missing
or failed counters produce gaps, not zeros. The grid updates when the observed topology changes.

Use `cargo run -p reactor-maxcpu --release` to exercise the graphs. Its workers are not pinned, so
busy work can move between logical processors. Stop the load before collecting performance numbers.

## Record and replay

```powershell
cargo run -p reactor-process-monitor --release -- --record processes.jsonl --frames 60 --exit
cargo run -p reactor-process-monitor --release -- --replay processes.jsonl --exit
```

Recording is opt-in. Files contain process names, PIDs, creation times, counters, and query errors,
plus system CPU and memory observations.
Treat live recordings as potentially sensitive local data. Executable paths are not recorded.
Use `--demo --record demo.jsonl` to create a synthetic recording instead.

New recordings use version 3 and include logical processor identities, values, and errors.
Version-2 recordings retain aggregate CPU and memory history but have no logical CPU history.
Version-1 recordings retain their process data but have no system history. Replay never fills
missing observations from the current machine.

Replay reads one JSON snapshot per line and uses the same model and UI as live sampling. CPU values
use recorded sampling intervals, not replay wall time. It does not query processes on the replay
machine or replay keyboard/mouse actions. Invalid input stops replay with an explicit error.

| Option | Behavior |
| --- | --- |
| `--demo` | Replay the built-in synthetic process workload |
| `--replay FILE` | Read a version-1, version-2, or version-3 JSON-lines recording |
| `--record FILE` | Write live or demo snapshots to a new file |
| `--interval-ms N` | Delay after each completed snapshot; defaults to 1,000 ms |
| `--frames N` | Stop after N snapshots |
| `--exit` | Close on completion; failures return a nonzero exit code |
| `--report FILE` | Write snapshot acquisition and update-to-effect timing to a new file |

Recording and report files must not already exist. Without `--exit`, a finished replay leaves its
last snapshot visible. Without `--frames`, live sampling continues until paused or the window closes.
Sampling and recording run on background workers with at most one snapshot request in flight.
Timing samples are retained only with `--report`, which requires a finite source or `--frames`.
Reports are written on completion, not when the window is closed early.

## Measurement boundaries

CPU is the change in kernel plus user CPU time divided by elapsed time and the machine's active
logical processor count, rounded to one decimal place. It is not Task Manager's frequency-adjusted
CPU utilization. A new process needs two samples. Working set includes shared resident pages;
private bytes measure private committed memory, not private resident memory.

Process identity combines PID and creation time. Parent links require a known, older parent
instance, avoiding links to reused PIDs. When identity or counters cannot be queried, values remain
unavailable rather than becoming zero. Selection of an unverified identity expires on the next
snapshot. A collection failure leaves the last snapshot visible with a stale-data error.

For repeatable UI investigations, replay the same file at the same interval and window/display
settings on a quiet machine. Reports include startup and measure snapshot handling through the
post-commit effect, excluding later layout, virtualization, and presentation. Reports also separate
model/projection time from the rest of the update-to-effect interval. Acquisition includes
sampling or replay parsing and optional recording. These reports do not establish frame rate,
input latency, or a speed advantage over Task Manager.

## Development

```powershell
cargo test -p reactor-process-monitor
cargo clippy -p reactor-process-monitor --all-targets -- -D warnings
```

The tests cover CPU normalization, PID reuse, tree filtering, unavailable identities, replay
compatibility, NUMA-qualified processor identities, bounded history, executable-path identity
checks, and drawing aggregate and logical graphs with gaps on a shared WARP surface.
