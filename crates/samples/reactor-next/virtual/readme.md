# Virtual task editor

This sample qualifies `windows-reactor-next` virtualization with application behavior rather than
static labels. It includes:

- keyed row components with controlled `TextBox` and `ToggleSwitch` values;
- front insertion, removal, move-to-end, reversal, and a 1,000-item reset;
- direct selection props for each keyed row;
- validation and queued typed focus;
- keyed row effects and cleanup on recycle;
- background loading with scope-owned cancellation;
- conditional row content;
- same-key payload updates and key-changing source resets.

Run it with:

```powershell
cargo run -p sample_reactor_next_virtual
```

The same component model has a feature-gated `RecordingRuntime` performance driver. Run it in
release mode:

```powershell
cargo run -p sample_reactor_next_virtual --bin reactor-next-virtual-perf `
    --features perf --release -- --samples 500
```

The driver measures local editing, broad and redundant parent messages, an unchanged root-component
memo hit, a forced value-equal root recomposition, 32-row recycle/realize batches, background
completion, and a mixed virtual cycle. It reports allocator traffic and median, p95, and p99
Rust-side turn time. It also measures value-equal declaration scaling at 1,000, 10,000, and 100,000
tasks. It does not include WinUI control work, layout, rendering, or presentation.

The live driver adds `CompositionTarget::Rendering` frame intervals and host/native phase timing:

```powershell
cargo run -p sample_reactor_next_virtual --bin reactor-next-virtual-live-perf `
    --features perf --release -- --samples 300
cargo run -p sample_reactor_next_virtual --bin reactor-next-virtual-live-perf `
    --features perf --release -- --baseline --samples 300
```

The active run moves by 32 virtual indices per frame, alternates selection, edits a
controlled row every six frames, and delivers a background completion every 30 frames. The
baseline opens the same 1,000-item editor without scheduling those actions. Each command writes a
report next to the executable. After the measured frames, the driver also verifies that queued
recycle and source-reset work left no retired WinUI shell tokens. A machine that emits no
composition frames writes an explicit failure report and exits unsuccessfully after 20 seconds.

The durable task title belongs to `TaskEditor`, not the realized row component. A row component is
retained across key-stable payload updates, but native virtualization or a source reset may recycle
it. Non-empty controlled drafts update that model immediately; a blank validation draft remains
row-local and falls back to the last valid model value after recycling. The recording-runtime test
enters edit mode, checks queued focus, edits a task, explicitly recycles it, reverses the source,
and verifies that its title survives each re-realization with one cleanup per retired row effect.

The buttons provide a deterministic manual stress sequence:

1. Select a row, enter edit mode, and change its title.
2. Reverse or move the source and confirm the title remains in the task model.
3. Add and remove rows at the front.
4. Load another 100 rows in the background.
5. Reset to 1,000 rows and repeat edits while scrolling.
