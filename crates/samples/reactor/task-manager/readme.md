# Task Manager

A Reactor version of Task Manager with a navigation shell, a virtualized process tree, selectable
columns, filtering, sorting, stable selection, a per-process details page, and ending processes.
It runs without elevation, debug privileges, or process-memory reads.
Process sampling uses `NtQuerySystemInformation`, so identity and counters do not require opening
each process. Creation and CPU times depend on the native `SYSTEM_PROCESS_INFORMATION` layout,
including fields reserved in the SDK declaration.

```powershell
cargo run -p reactor-task-manager --release
cargo run -p reactor-task-manager --release -- --demo
```

The build deploys a self-contained Windows App Runtime. An interactive Windows desktop is required.
The demo has 60 deterministic snapshots with changing metrics, process exits, and PID reuse. It
does not enumerate real processes.

The **Processes** page supports full-row selection; double-click a row, or select it and choose
**Details**, to inspect the process. Tab and Enter/Space activate rows and controls. Use the
chevrons to expand or collapse branches. **Columns** adds or removes PID, parent PID, session,
priority, CPU, threads, handles, memory, and pool usage columns, all from the same system snapshot.
Click a column heading to sort, then click again to reverse direction. Identity columns initially
sort ascending and resource columns descending; unavailable values stay last. Hiding the sorted
column sorts by name. Drag the divider at the right edge of a heading to resize a column; the table
scrolls horizontally when the window is narrower than its columns. Tree mode sorts siblings. Turn
off **Process tree** for a global ranking. Filtering retains ancestors of matching processes and
expands their paths. Ctrl+Space pauses and resumes updates, as shown in the title bar; pausing stops
scheduling new snapshots, but an in-flight snapshot can still complete.

Right-click a row to select it and show **Details**, **End task**, and **End process tree**, or
select a row and choose **End task** above the table. Both ask for confirmation. Windows has no
process tree, so, like .NET's `Process.Kill(entireProcessTree: true)`, ending a tree is best effort:
descendants are captured from parent PIDs and creation times when the dialog opens, then each
process is ended before its descendants, without waiting for it to exit. A failure does not stop the
rest of the tree. A tree that includes Task Manager itself is refused. Each process is opened with
`PROCESS_TERMINATE` and its creation time checked first, so a reused PID is never ended. Failures,
such as access denied for elevated or protected processes, are listed in a closable error bar.
Ending processes is available only in live mode.

The **Details** page opens the selected process with `PROCESS_QUERY_LIMITED_INFORMATION` and checks
its creation time before every query, so a reused PID is never described as the original process.
It shows the image path and version resources, user, elevation, architecture, start time, kernel
and user CPU, priority, memory, I/O, and handle and GDI/USER object counts. Counters refresh with
each snapshot. Processes that deny access, such as protected system processes when not elevated,
show only their snapshot values.

The **Performance** page charts aggregate CPU and physical memory with `windows-canvas`. CPU uses
the language-neutral PDH `Processor Information(_Total)\% Processor Time` counter, not a sum of
process rows or frequency-adjusted utilization. Memory shows total physical memory minus available
memory. Counter errors are shown as unavailable, and missing observations leave gaps in the charts.

Use the **Graph** button above the CPU chart to choose a view.
**Overall utilization** shows the aggregate counter and **Logical processors** shows a grid
covering all active logical processors. Labels use PDH's `Nnode:number` identity: the NUMA node and
processor number within that node, not physical core numbers or processor-affinity groups. All
plots share the same canvas, device, and swap chain. Resizing changes the grid layout, not processor
identity. The chart's accessibility help text includes each processor's current value or error.

Memory labels follow Task Manager's MB/GB convention, using powers of 1024.

History covers the last 60 recorded seconds, retaining the latest observation per second. CPU
collection runs at most once per second even when process refresh is faster. Pausing freezes the
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
cargo run -p reactor-task-manager --release -- --record processes.jsonl --frames 60 --exit
cargo run -p reactor-task-manager --release -- --replay processes.jsonl --exit
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
cargo test -p reactor-task-manager
cargo clippy -p reactor-task-manager --all-targets -- -D warnings
```

The tests cover CPU normalization, PID reuse, tree filtering, unavailable identities, replay
compatibility, NUMA-qualified processor identities, bounded history, stable identity for processes
that cannot be opened, native process-time decoding, sampling cancellation, details and termination
identity checks, process-tree descendants, version resources, and drawing aggregate and logical
graphs with gaps on a shared WARP surface.
