# Navigation and multi-window workspace

Each workspace owns its navigation state, count, and note. An `Application` owns the shared theme
and two declared window slots. Opening a slot presents its existing window or creates a new one
if it was closed.

Run it with:

```powershell
cargo run -p reactor-navigation
```

Try this sequence:

1. Choose **Open other workspace** and enter a different note in each window's Editor page.
2. Switch between Home and Editor. Each window retains its own note and count.
3. Change the theme. Both windows update without exchanging messages directly.
4. Close either window and reopen it from the other. The reopened workspace starts with fresh state.

Closing the last window exits the application.
