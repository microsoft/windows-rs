# WinUI projection benchmark

This benchmark compares the public WinUI projection paths in windows-rs, windows-csharp, and
CsWinRT. All three consumers use the same pinned Windows App SDK metadata and equivalent
windows-reactor `IApplicationOverrides.OnLaunched` hosting. The host is outside the timed control
operations.

Run from PowerShell:

```powershell
.\crates\samples\test_winui_bench\run.ps1
```

The script builds all consumers and reports medians from three runs by default. Each run uses two
fresh processes per consumer: a headless process for projection and construction metrics, then a
visible process for the sustained rendering track. Splitting the tracks prevents the large
synchronous update sweep from delaying the first layout of the sustained tree. Launch order
rotates between runs.

The sustained window remains visible for `-SustainedSeconds` (three seconds by default), then for
`-SettleMs` after its result is captured. The runner terminates that exact process after retaining a
valid result if WinUI shutdown stalls. `-Headless` also makes the sustained process headless, which
is useful for call-cost checks but normally produces no rendering callbacks. Use
`-SustainedPercent` to change its dirty share and `-SustainedChurn` to detach and reattach that many
retained tail cells on every tick. `-BenchmarkBudgetMs` increases the timeout allowance for
unusually large synchronous iteration counts. Use `-Runs`, `-Iterations`, `-CreateIterations`,
`-TreeIterations`, `-TreeSize`, and `-StressIterations` to change the other sample counts.

| Metric | Definition |
| :-- | :-- |
| Main | Runner launch through managed/native process entry |
| Host start | Runner launch through bootstrap and argument parsing, immediately before the host |
| Startup | Runner launch through the application launch callback |
| Window | Build the configured `StackPanel` tree; visible mode activates the window |
| Working set | Process working set after the timed operations |
| Create | Create and set text; Rust/C# release in-loop, while CsWinRT defers to GC |
| Update | Alternate one existing `TextBlock.Text` value |
| Cast | Use each projection's public operation for viewing a `TextBlock` as `UIElement` |
| Tree | Build and retain a `StackPanel` containing the configured number of `TextBlock` children |
| Batch update | Change `Text` on every retained child; reported per child |
| Churn | Clear and reattach every retained child; reported per child |
| Event | Use each projection's public registration/revocation path for `Button.Click` |
| Boolean | Alternate `Button.IsEnabled`, exercising WinRT Boolean conversion |
| Teardown | Tear down a retained tree; CsWinRT includes forced GC/finalization |
| Stress build | Build a scrollable 70x70 Canvas with 4,900 fixed-size `TextBlock` controls |
| Stress update | Update a deterministic 0%, 10%, 50%, or 100% share per render |
| Stress working set | Process working set while the 4,900-control tree is attached to the window |
| Sustained update | UI-thread timer updates `Text` and `Foreground` on a deterministic dirty share |
| Sustained churn | Retained tail cells detached and reattached on each sustained update tick |
| Sustained rendering | `CompositionTarget.Rendering` callback count and callbacks per second |
| Sustained working set | Process working set after layout and repeated visible updates |
| Language bytes | Rust global-allocator or managed GC-heap bytes during the operation |

Each consumer builds and releases a one-cell stress fixture before measuring the full stress build.
This prepares the construction path and activation factories without including the warmup in the
reported allocation or time.

The view-cast operations are intentionally idiomatic rather than forced to the same implementation.
windows-rs performs `QueryInterface` and returns a pointer wrapper. windows-csharp performs
`QueryInterface` and creates an escapable owner. CsWinRT uses projected managed
inheritance and reuses the existing wrapper, so its value is a reference conversion rather than a
COM cast. These values describe each public projection shape but are not equivalent operations.

Language allocation counts are allocator-specific and are not directly comparable across Rust and
.NET. The Rust counter excludes Win32 heap allocations such as `HSTRING` buffers; the managed
counters exclude native WinUI and COM allocations. Working set includes each consumer's runtime and
projection deployment surface. CsWinRT loads the full managed Windows App SDK projection, while
windows-csharp loads only the selected generated slice.

windows-csharp uses one callback-confined borrowed view around the update loop. CsWinRT uses its
ordinary property setter and windows-rs uses its generated `SetText` method.

The tree path hoists collection acquisition outside the append loop and keeps all trees alive until
timing ends, so teardown is outside the measurement. windows-rs and windows-csharp explicitly QI
each `TextBlock` to `UIElement`; CsWinRT uses managed inheritance.

The retained-tree paths keep each child wrapper available after mounting. Batch update measures
steady property updates across the tree. Churn clears the child collection and appends the
same retained children again. Teardown is intentionally lifecycle-specific: Rust and
windows-csharp release deterministically, while CsWinRT clears managed references and forces
collection and finalization.

The stress phase uses the same 4,900-cell scale and dirty percentages as `test_reactor_perf`, but
does not run a declarative reconciler. Each consumer builds a bounded 70x70 Canvas with 64x18 cells,
explicit red and green brushes, and a ScrollViewer. The headless process updates the same
deterministic index stream to isolate projection calls, managed/Rust allocation, and native control
construction. Zero percent follows `test_reactor_perf` and still updates one cell.

The fresh sustained process attaches an equivalent 4,900-cell tree to a visible window. A 33 ms
UI-thread timer updates text and alternates the two retained foreground brushes on 10% of the cells
by default. Optional churn removes the configured number of tail cells and appends the same
retained controls after the property updates. `CompositionTarget.Rendering` supplies a common
callback-frequency signal while layout and rendering occur. This callback is not proof that a
compositor frame reached the display; the metric is rendering callbacks per second, not
presentation telemetry.

The event paths are idiomatic rather than equivalent. windows-rs creates a closure-backed delegate
and `EventRevoker` for each registration. windows-csharp and CsWinRT reuse one delegate; the former
uses raw add/remove tokens while the latter uses `+=`/`-=`.

The synchronous tables remain projection and UI-thread tree-construction measurements. The
sustained table adds layout, repeated visible updates, and rendering callback frequency, but still
does not run a declarative reconciler or measure compositor presentation.

## CsWinRT 3 preview

CsWinRT 3 cannot currently be added as a fourth WinUI consumer. The preview is selected with
`net10.0-windows10.0.26100.1`,
`WindowsSdkPackageVersion=10.0.26100.85-preview`, and
`Microsoft.Windows.CsWinRT=3.0.0-preview.260319.2`, but Windows App SDK 2.3.1 ships
`Microsoft.WinUI.dll` against the CsWinRT 2 `IWinRTObject` ABI. Generating a private projection
from the pinned WinUI metadata also reaches system ABI marshallers that the preview does not
provide. The CsWinRT team states that WinUI support requires a CsWinRT 3 projection from the WinUI
team. Add the fourth column when that projection is available.
