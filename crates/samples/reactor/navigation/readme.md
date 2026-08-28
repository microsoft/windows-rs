# Navigation and multi-window workspace

This sample qualifies navigation and window isolation by opening a secondary workspace from the
primary at runtime. A `SplitView` hosts the navigation pane and current page through generated
typed slots. Each window owns its page, editor text, focus reference, background work, and
component queue. The windows share a small application model that broadcasts theme changes and
window-lifecycle notifications.

Run it with:

```powershell
cargo run -p sample_reactor_navigation
```

Try this sequence:

1. Choose **Open secondary window**, then open the editor in each window and enter different text.
2. Move between Home and Editor and confirm each window retains its own text.
   The window title follows the current page.
3. Change the shared theme and confirm both windows update.
4. Start background work in the secondary window, then choose **Close this window**.
5. Confirm the primary window remains responsive and reports the secondary closure.

`ComponentContext::open_window` stages an independent root with the current component publication.
After publication, the host registers a pending open and mounts that root in a new Pump. The
opener's scope does not own the new Pump. Close requests use the same transactional boundary and
are applied only after publication. Each workspace declares its native title from component state.
Declarative size configuration remains deferred rather than being hidden in the sample with direct
WinUI calls.
