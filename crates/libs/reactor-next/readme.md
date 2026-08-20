# Windows Reactor Next

This unpublished crate is the experimental successor to `windows-reactor`. It is being developed
side by side with the current crate until its architecture, correctness, compile time, and runtime
performance are proven.

See [`reactor-next.md`](../../../reactor-next.md) for the current plan and gates.

The current thin slice generates `TextBlock`, `Button`, `StackPanel`, and `TextBox` from WinUI
metadata plus a small curation schema. It includes clearable properties, callbacks, content, and
keyed children. The arena, reconciler, and WinUI backend remain private while their contracts are
proven with a recording runtime.
