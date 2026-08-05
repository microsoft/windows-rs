# WinUI projection benchmark

This benchmark compares the public WinUI projection paths in windows-rs and CsWinRT. Both
consumers use the same pinned Windows App SDK metadata and equivalent windows-reactor
`IApplicationOverrides.OnLaunched` hosting. The host is outside the timed control operations.

Run from PowerShell:

```powershell
.\crates\samples\test_winui_bench\run.ps1
```

The script builds both consumers and reports medians from three runs by default. Each run uses two
fresh processes per consumer: a headless process for projection and construction metrics, then a
visible process for the sustained rendering track. Launch order rotates between runs.

The sustained window remains visible for `-SustainedSeconds` (three seconds by default), then for
`-SettleMs` after its result is captured. The runner terminates that process after retaining a
valid result if WinUI shutdown stalls. `-Headless` also makes the sustained process headless,
which is useful for call-cost checks but normally produces no rendering callbacks.

Use `-SustainedPercent` to change the dirty share and `-SustainedChurn` to detach and reattach
retained tail cells on every tick. `-BenchmarkBudgetMs` increases the timeout allowance for large
synchronous iteration counts. `-Runs`, `-Iterations`, `-CreateIterations`, `-TreeIterations`,
`-TreeSize`, and `-StressIterations` control the other sample counts.

| Metric | Definition |
| --- | --- |
| Main | Runner launch through managed or native process entry. |
| Host start | Runner launch through bootstrap and argument parsing. |
| Startup | Runner launch through the application launch callback. |
| Window | Build the configured `StackPanel`; visible mode activates the window. |
| Working set | Process working set after the timed operations. |
| Create | Create a `TextBlock` and set its text. |
| Update | Alternate one existing `TextBlock.Text` value. |
| Cast | View a `TextBlock` as `UIElement` through the public projection. |
| Tree | Build and retain a `StackPanel` containing `TextBlock` children. |
| Batch update | Change `Text` on every retained child, reported per child. |
| Churn | Clear and reattach every retained child, reported per child. |
| Event | Register and revoke `Button.Click`. |
| Boolean | Alternate `Button.IsEnabled`. |
| Teardown | Tear down a retained tree; CsWinRT includes forced finalization. |
| Stress build | Build a scrollable 70x70 Canvas with 4,900 `TextBlock` controls. |
| Stress update | Update a deterministic 0%, 10%, 50%, or 100% share. |
| Stress working set | Working set while the stress tree is attached. |
| Sustained update | UI-thread timer updates `Text` and `Foreground`. |
| Sustained churn | Retained tail cells detached and reattached on each tick. |
| Sustained rendering | `CompositionTarget.Rendering` callbacks per second. |
| Sustained working set | Working set after layout and repeated visible updates. |
| Language bytes | Rust allocator or managed GC-heap bytes during the operation. |

Each consumer builds and releases a one-cell stress fixture before measuring the full stress
build. This prepares control construction and activation factories without including warmup in
the reported allocation or time.

The cast operations are idiomatic rather than equivalent. windows-rs performs `QueryInterface`
and returns a pointer wrapper. CsWinRT uses projected managed inheritance and reuses the existing
wrapper, so its value is a reference conversion rather than a COM cast.

Allocation counts are allocator-specific and are not directly comparable across Rust and .NET.
The Rust counter excludes Win32 heap allocations such as HSTRING buffers; the managed counter
excludes native WinUI and COM allocations. Working set includes each consumer's runtime and
projection deployment surface.

The tree path hoists collection acquisition outside the append loop and keeps all trees alive
until timing ends. Batch update measures property changes across retained children. Churn clears
the child collection and appends the same children again. Rust releases deterministically, while
CsWinRT clears managed references and forces collection and finalization for teardown.

The stress phase uses the same 4,900-cell scale and dirty percentages as `test_reactor_perf`, but
does not run a declarative reconciler. The headless process updates the same deterministic index
stream to isolate projection calls, allocation, and native control construction. Zero percent
still updates one cell.

The sustained process attaches an equivalent tree to a visible window. A 33 ms UI-thread timer
updates text and alternates two retained foreground brushes. Optional churn removes and appends a
configured number of tail cells. `CompositionTarget.Rendering` provides a common callback-rate
signal, not compositor presentation telemetry.

## CsWinRT 3 preview

CsWinRT 3 cannot currently be added as another WinUI consumer. Windows App SDK 2.3.1 ships
`Microsoft.WinUI.dll` against the CsWinRT 2 `IWinRTObject` ABI, and the preview does not provide
all system ABI marshallers needed by a private projection. Add another column when a compatible
WinUI projection is available.
