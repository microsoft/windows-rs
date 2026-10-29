# Virtual task editor

This sample qualifies `windows-reactor` virtualization with application behavior rather than
static labels. It includes:

- keyed row components with controlled `TextBox` and `ToggleSwitch` values;
- front insertion, removal, move-to-end, reversal, and a 1,000-item reset;
- direct selection input for each keyed row;
- validation and queued typed focus;
- a keyed row effect that focuses the title when editing starts;
- background loading with scope-owned cancellation;
- conditional row content;
- same-key payload updates and key-changing source resets.

Run it with:

```powershell
cargo run -p reactor-virtual
```

The durable task title belongs to `TaskEditor`, not the realized row component. A row component is
retained across key-stable payload updates, but native virtualization or a source reset may recycle
it. Non-empty controlled drafts update that model immediately; a blank validation draft remains
row-local and falls back to the last valid model value after recycling.

The buttons provide a deterministic manual stress sequence:

1. Select a row, enter edit mode, and change its title.
2. Reverse or move the source and confirm the title remains in the task model.
3. Add and remove rows at the front.
4. Load another 100 rows in the background.
5. Reset to 1,000 rows and repeat edits while scrolling.
