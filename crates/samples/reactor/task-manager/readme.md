# Task Manager

A Reactor process viewer with a virtualized process tree, per-process details, and CPU and memory
charts. It runs without elevation, debug privileges, or process-memory reads.

```powershell
cargo run -p reactor-task-manager --release
cargo run -p reactor-task-manager --release -- --demo
```

The build deploys a self-contained Windows App Runtime. An interactive Windows desktop is required.
The demo has 60 deterministic snapshots with changing metrics, process exits, and PID reuse. It
does not enumerate real processes.

## Use the viewer

On **Processes**, double-click a row or choose **Details** to inspect it. Use **Columns** to choose
metrics, click headings to sort, and drag heading dividers to resize. Turn off **Process tree**
for a global ranking. Filtering retains ancestors of matching processes. Tab and Enter/Space
activate rows and controls; Ctrl+Space pauses or resumes updates. An in-flight snapshot can still
complete after pausing.

Right-click a row for **End task** or **End process tree**. Both require confirmation and are
available only in live mode. Tree termination is best effort: descendants are captured when the
dialog opens, and a failure does not stop the remaining attempts. A tree containing this viewer is
refused. Creation-time checks prevent ending a reused PID. Access-denied and other failures appear
in an error bar.

**Details** also checks process identity before querying. Protected or inaccessible processes show
only their snapshot values.

On **Performance**, use **Graph** to choose aggregate CPU or logical processors. Processor labels
identify NUMA nodes and logical processors, not physical cores. CPU uses PDH processor time, not
Windows Task Manager's frequency-adjusted utilization. Memory is total physical memory minus
available memory; MB/GB use powers of 1024. Missing or failed observations leave gaps, not zeros.

History covers the last 60 recorded seconds. Sampling continues across pages, and pausing freezes
the display. History, graph choice, selection, sorting, and filtering are retained across page
changes, but returning to Processes starts the list at the top.

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

Recordings use version 3. Replay also accepts version 2 without logical CPU history and version 1
without system history. It never fills missing observations from the current machine.

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
last snapshot visible. Without `--frames`, live sampling continues until paused or the window
closes. Timing samples are retained only with `--report`, which requires a finite source or
`--frames`. Reports are written on completion, not when the window is closed early.

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

Tests cover process identity, sampling, replay compatibility, and off-screen WARP graph rendering.
