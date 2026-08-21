# Navigation and multi-window workspace

This sample qualifies navigation and window isolation with two startup windows. Each window owns
its page, editor text, focus reference, background work, and component queue. The windows share a
small application model that broadcasts theme changes and window-lifecycle notifications.

Run it with:

```powershell
cargo run -p sample_reactor_next_navigation
```

Try this sequence:

1. Open the editor in each window and enter different text.
2. Move between Home and Editor and confirm each window retains its own text.
3. Change the shared theme and confirm both windows update.
4. Start background work in the secondary window, then choose **Close this window**.
5. Confirm the primary window remains responsive and reports the secondary closure.

The sample uses `App::run_windows`, which creates all windows at startup. Component close requests
are token-bound, staged with the current component publication, and applied only after it succeeds.
The current host does not yet expose runtime window creation or declarative title and size
configuration. Those APIs remain part of this qualification gate rather than being hidden in the
sample with direct WinUI calls.
