# test_bench

A side-by-side WinRT projection benchmark. Four consumers call the identical Rust WinRT
component, so every per-call delta is pure projection cost.

| Consumer | Column | Projection |
| --- | --- | --- |
| `rust` | windows-rs | the generated windows-rs projection |
| `csharp` | windows-csharp | raw `IDisposable` pointer owners |
| `cpp` | cppwinrt | the header-only cppwinrt projection |
| `cswinrt2` | cswinrt 2 | the conventional C#/WinRT projection (RCW-backed managed classes), latest released stable 2.x |

A fifth project, `cswinrt`, is a focused CsWinRT 3.0 preview probe rather than a benchmark column.
The preview package builds on plain `net10.0`, but it does not contain the `WinRT.Interop` assembly
that its delegate marshaller loads. The probe therefore fails at the first event subscription and
its Cargo test is ignored with that reason until a complete preview ships.

The component (`component` -> `bench_component.dll`) is a real WinRT component: RDL -> winmd ->
windows-bindgen -> `#[implement]`, activated registration-free via `DllGetActivationFactory`. It
exposes one activatable class, `Bench.Widget`, with scalar, string, metadata `Object`, nullable
reference, async, event, vector, map, and view operations. A non-default `INonDefault` interface
provides the QueryInterface fixture, and `LiveCount` reports native ownership balance.

## Running

```powershell
# Full run (10,000,000 iterations, median of three runs by default)
crates/samples/test_bench/run.ps1

# Quick run
crates/samples/test_bench/run.ps1 -Iterations 100000 -Runs 1
```

`run.ps1` builds the component first (alone, so the winmd writer never races a consumer build
script that reads the same winmd), builds the four active consumers, stages `bench_component.dll`
as `Bench.dll` beside each binary, runs each consumer three times, and prints the median
throughput and memory values plus the leak table. The lower native result and lowest C# result are
bold.

The windows-csharp runtime policy is:

- one raw interface pointer per owner;
- no per-call atomic lease;
- no agility probe or apartment/context token;
- no synchronization between calls and `Dispose`;
- no finalizer recovery;
- activation modules and factories are cached without synchronization.

The caller must not race a call with `Dispose`, must dispose non-agile objects in their originating
apartment, and must dispose every owner. Adding `.synchronized()` to the builder opts into atomic
call/dispose coordination, apartment checks, context-aware release, and finalizer recovery.

Each consumer also has a `cargo test` that runs the same path with a tiny iteration count, so the
matrix is exercised end to end in CI wherever the .NET SDK and MSVC toolchain are present (the
tests skip cleanly when they are absent).

### Generated COM WinRT probe

The windows-csharp executable accepts `--generated-com` to compare its direct owner with .NET's
`[GeneratedComInterface]` and `StrategyBasedComWrappers` over the same `Bench.Widget`. The probe
models IInspectable as a generated base interface, verifies slots 3-5 and `IWidget` slots 6+, and
uses a custom HSTRING marshaller.

After `run.ps1` builds and stages the component:

```powershell
crates/samples/test_bench/csharp/bin/x64/Release/net10.0/test_bench_cs.exe `
    --iterations 10000000 --generated-com
```

The extra `Owner*`, `GeneratedCom*`, `GeneratedComLive`, and `GeneratedComLeak` rows are excluded
from the four-consumer table. The probe compares ordinary owner calls, the borrowed hot path,
default generated HRESULT translation, `PreserveSig`, wrapper allocation, and finalizer cleanup.
The standard generated wrapper is not `IDisposable`.

## Metrics

| Metric | What it measures |
| --- | --- |
| Create | activate a fresh `Widget` |
| Int32 | set + get the `Int32` property |
| String | set + get the `String` property (HSTRING conversion) |
| Add | call a method with two `Int32` arguments and a return |
| Cast | reach `INonDefault` with each projection's preferred short-lived cast and call `Value` |
| CastOwned | request an escapable projected interface owner and call `Value` |
| Interface | acquire `INonDefault` once, then call `Value` repeatedly |
| Object | set and get metadata `Object` (`IInspectable`) |
| Event | subscribe one handler and raise `Changed` through `Signal` (component calls back into the consumer) |
| AddRemove | subscribe and unsubscribe a `Changed` handler each iteration |
| Vector | read one element (`GetAt`) per iteration from an `IVector<int>` built once via `Items` |
| IterateVector | `foreach` over the whole `IVector<int>`, repeated a bounded number of passes |
| GetMany | copy a vector into caller-owned storage through each projection's public bulk-copy surface |
| Map | enumerate values from an `IMap<string,int>` through its iterable interface |
| Lookup | read one value (`Lookup`) per iteration by key from an `IMap<int,int>` built once via `Map` |
| VectorView | read one element (`GetAt`) per iteration from an `IVectorView<int>` built once via `ItemsView` |
| MapView | read one value (`Lookup`) per iteration by key from an `IMapView<int,int>` built once via `MapView` |
| Reference | box and unbox a nullable `Int32` through `IReference<int>` |
| Async | synchronously consume an already-completed `IAsyncOperation<int>` |
| Error | call a method that always returns a failing `HRESULT` and propagate the error (reduced count) |
| Memory | client-side allocated bytes per retained live object |
| Leak | component live-instance count above baseline after the run (0 is correct) |

The three interface metrics separate operations that were previously conflated. `Cast` uses the
preferred short-lived form: C++/WinRT and windows-rs use stack-value owners, windows-csharp calls a
generated runtime-class forwarder that performs QI + call + Release, and CsWinRT returns its
existing RCW and cached declared-interface pointer. Both C# loop bodies call `Value()` directly;
windows-csharp borrows the source once around the loop. It creates no temporary owner and caches
no target pointer. `CastOwned` forces windows-csharp to allocate an escapable owner;
C++/WinRT and windows-rs owners are stack values, while CsWinRT still preserves RCW identity.
`Interface` acquires the interface once before the loop and measures steady calls. The three
CsWinRT rows use cached interface state and do not contain a per-iteration QI; their different
times reflect the generated class, `As<T>()`, and interface-variable call shapes.

The timed C# bodies are matched wherever the projection surfaces permit: the same loop bounds,
inputs, result retention, collection sizes, and reduced error count. windows-csharp uses borrowed
views for callback-confined hot loops; CsWinRT uses its normal RCW surface. Deterministic
owners are disposed inside their timed iteration. CsWinRT has no equivalent disposal, so transient
RCWs are finalized outside the timer after affected metrics to prevent cleanup from contaminating
the next row. Memory and Leak report the lifecycle difference separately.

The Object metric uses the metadata `Object` shape from `component/src/bench.rdl`. The setter borrows an
`IInspectable`; the getter returns a `+1` pointer. windows-csharp wraps each result in one small
owner, while CsWinRT resolves it through its RCW machinery.

The Event and AddRemove metrics exercise a WinRT delegate in both directions. windows-csharp projects
a delegate as a reverse vtable - a native COM object whose `Invoke` slot is an `[UnmanagedCallersOnly]`
function backed by a `GCHandle` to the managed callback - so raising it allocates nothing, and it
projects an event as raw `Add{Event}(handler) -> long` / `Remove{Event}(long token)` accessors rather
than a C# `event`. Event lands beside Rust and C++/WinRT; AddRemove beats Rust (which allocates a
delegate box and revoker per iteration) and runs dramatically faster than CsWinRT 2.x, whose `+=` /
`-=` maintain a per-object `EventSource` table allocated and mutated on every add and remove.

The Vector metric measures a generic collection call across the ABI. `IWidget::Items` returns an
`IVector<int>` built once (1024 elements), and each iteration reads one element with `GetAt`.
windows-csharp projects `IVector<T>` as an owning generic class whose element IID is computed at
generation time. Its callback-confined borrowed view makes `GetAt` a direct vtable call with no
per-element allocation.

The IterateVector metric measures a full `foreach` over the collection, repeated over a bounded
number of passes. windows-csharp's `IVector<T>` exposes a `GetEnumerator` that returns a struct
enumerator batching through `GetMany` into a stack `[InlineArray]` buffer, so a pass makes one
vtable call per 64-element block and allocates nothing - the same batching windows-rs uses. It is
second only to the Rust column and runs an order of magnitude faster than both C++/WinRT (a
per-element iterator) and CsWinRT (an `IEnumerator<int>` RCW that boxes an enumerator per pass and
marshals each element).

The GetMany row requests the same final result - copy the whole vector into caller-owned storage -
through each public projection. windows-csharp, windows-rs, and C++/WinRT expose the WinRT
`GetMany` method directly. CsWinRT maps `IVector<T>` to `IList<T>`; its public `CopyTo` adapter
loops over `GetAt`, so that column measures the cost of the available projected bulk-copy surface
rather than one `GetMany` ABI call.

The Lookup metric measures a generic dictionary call across the ABI. `IWidget::Map` returns an
`IMap<int,int>` built once (1024 entries), and each iteration reads one value with `Lookup`.
windows-csharp projects `IMap<K,V>` as an owning generic class whose instantiation IID is computed
at generation time. Its borrowed view makes `Lookup` a direct vtable call with no per-lookup
allocation. CsWinRT projects `IMap<int,int>` as `IDictionary<int,int>`.

The Map metric enumerates `IMap<string,int>` through
`IIterable<IKeyValuePair<string,int>>`. windows-csharp computes the nested parameterized IID at
generation time and reads each pair's value without allocating a projected pair owner. The current
entry surface exposes `Value`; projecting `Key` remains collection breadth work.

Reference uses a temporary native COM box for nullable inputs and unboxes returned
`IReference<int>` pointers directly. Async returns an owning `IAsyncOperation<int>` with a C#
awaiter; the current awaiter polls `IAsyncInfo.Status`, while the benchmark operation is already
complete.

The VectorView and MapView metrics measure the read-only views `IVectorView<int>` and
`IMapView<int,int>`, returned by `IWidget::ItemsView`/`MapView`. windows-csharp emits them from the
same generic machinery as `IVector`/`IMap`, with generation-time IIDs and borrowed hot paths.
CsWinRT projects them as `IReadOnlyList<int>` and `IReadOnlyDictionary<int,int>` RCWs. The views
run faster than mutable collections because the component's mutable `IVector`/`IMap` guard calls
with a lock the immutable view skips.

The Error metric measures the error model. Rust surfaces a failing `HRESULT` as `Result` and
observes it with a branch (no unwind), so its error path costs about the same as its success path.
The other three raise and catch an exception, which is orders of magnitude more expensive, so the
loop runs a reduced iteration count. windows-csharp consumes thread error information, extracts
the restricted or regular description, and throws one `COMException` carrying the unchanged
HRESULT. It does not guess an exception type from a context-free HRESULT.

The Leak metric checks each projection balances its native `AddRef`/`Release`. Rust (`Drop`),
C++/WinRT (`com_ptr` destructor), and windows-csharp (`Dispose`) release deterministically at scope
exit. The CsWinRT column only returns to baseline after a forced `GC.Collect()` +
`WaitForPendingFinalizers()`, since an RCW holds its native reference until finalization.

To add a metric, emit a matching `Name: <ms> ms` line from every consumer and add its name to the
`$Metrics` array near the bottom of `run.ps1`.
