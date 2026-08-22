# Matched Reactor application benchmark

This benchmark builds the same bounded task application with `windows-reactor` and
`windows-reactor-next`. It measures Rust declaration, reconciliation, publication, and recording
runtime work. It does not include WinUI control creation, layout, rendering, or presentation.

The tree contains:

- an open inline SplitView with a four-label pane;
- a two-column Grid with heading, filter TextBox, status, and task rows;
- 32 keyed task-row Grids;
- one title TextBlock, ToggleSwitch, and selection-marker TextBlock per row.

Buttons are intentionally absent. `windows-reactor` represents button text as a string content
property, while `windows-reactor-next` represents it as an explicit child control. Excluding
buttons keeps the matched native control count and topology aligned.

Each measured operation constructs a fresh declaration and updates an already mounted root:

| Operation | State transition |
| --- | --- |
| Local edit | Alternate one task title |
| Selection | Alternate the selected task between ids 0 and 1 |
| Broad toggle | Flip every task's done state |
| Reverse keys | Reverse all 32 keyed task rows |
| Value equal | Rebuild an equal declaration |

Run the release driver with:

```powershell
cargo run -p test_reactor_matched_bench --release --quiet -- --samples 500
```

The driver reports median, p95, p99, allocator bytes, and allocations per operation. It also reports
separately sorted declaration and reconciliation/publication medians plus the bytes and allocations
owned by each phase. The phase medians are marginal distributions and are not expected to sum to
the total median. Retained bytes measure the model, frontend state, published tree, and recording
runtime after the initial mount. They are allocator deltas in one process, so use them as a relative
Rust comparison rather than a process working-set measurement.

## Live WinUI protocol

Two feature-isolated binaries mount the same model and tree through real WinUI. Keeping the
frontends in separate executables avoids linking two application hosts into one measured process.
Both binaries maximize the active window before 16 warmup updates and record the actual client
dimensions at the start and end of measurement. Each composition frame applies one operation from
the five-operation sequence. The next update does not begin until the dispatcher and compositor
return for another frame.

Run the live binaries separately:

```powershell
cargo run --release -p test_reactor_matched_bench --no-default-features `
    --features incumbent,live --bin reactor-matched-live-incumbent -- --samples 500
cargo run --release -p test_reactor_matched_bench --no-default-features `
    --features next,live --bin reactor-matched-live-next -- --samples 500
```

The live report includes Rust allocation calls and bytes, process working/private bytes, verified
client size, frame intervals and misses, and the timing phases exposed by each host. The incumbent
reports tree-build, reconcile, and effect timing. Next reports full host dispatch and native apply
timing. These phase boundaries differ, so compare whole-frame behavior and process measurements
before interpreting individual phase values.

The matching C# M15 benchmark lives in the `microsoft-ui-reactor` repository under
`tests\perf_bench\PerfBench.ControlModel`. Run its apphost executable, not `dotnet` with the DLL, so
its Per-Monitor-V2 manifest is active and client dimensions are physical pixels. M15 uses the same
warmup, operation sequence, frame cadence, settle period, and process measurements. Managed and
Rust allocation counters remain runtime-specific.
