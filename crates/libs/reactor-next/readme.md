# Windows Reactor Next

This unpublished crate is the experimental successor to `windows-reactor`. It is being developed
side by side with the current crate until its architecture, correctness, compile time, and runtime
performance are proven.

See [`reactor-next.md`](../../../reactor-next.md) for the current plan and gates.

The current slice generates `TextBlock`, `Button`, `StackPanel`, and `TextBox` from WinUI metadata
plus a small curation schema. The private WinUI backend now mounts the first three controls,
applies properties and keyed structure, queues button events, and rerenders hook state. The
recording runtime remains the failure-injection and randomized-test backend.
