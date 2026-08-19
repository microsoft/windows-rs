# windows-reactor-next

`windows-reactor-next` is an unpublished experiment for a simpler declarative WinUI architecture.
It is developed beside `windows-reactor` and will not replace it until the representative slice
meets the architecture, correctness, compile-time, and runtime gates in
[`reactor-next.md`](../../reactor-next.md).

The initial crate contains only the private native-runtime seam and recording test support. The
public API will be added from a metadata-derived schema rather than temporary hand-written control
mappings.
