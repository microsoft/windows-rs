# Virtual task editor

This sample qualifies `windows-reactor-next` virtualization with application behavior rather than
static labels. It includes:

- keyed row components with controlled `TextBox` and `ToggleSwitch` values;
- front insertion, removal, move-to-end, reversal, and a 1,000-item reset;
- selection distributed through `Context<bool>`;
- validation and queued typed focus;
- keyed row effects and cleanup on recycle;
- background loading with scope-owned cancellation;
- conditional row content;
- same-key payload updates and key-changing source resets.

Run it with:

```powershell
cargo run -p sample_reactor_next_virtual
```

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
