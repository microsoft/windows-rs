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
