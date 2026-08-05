# test_bench

A side-by-side WinRT projection benchmark. Three consumers call the identical Rust WinRT
component, so every per-call delta is projection cost.

| Consumer | Column | Projection |
| --- | --- | --- |
| `rust` | windows-rs | The generated windows-rs projection. |
| `cpp` | cppwinrt | The header-only C++/WinRT projection. |
| `cswinrt2` | CsWinRT 2 | The stable C#/WinRT projection with RCW-backed managed classes. |

The `cswinrt` project is a focused CsWinRT 3.0 preview probe rather than a benchmark column. The
preview package builds on plain `net10.0`, but it does not contain the `WinRT.Interop` assembly
that its delegate marshaller loads. The probe fails at the first event subscription and its Cargo
test remains ignored until a complete preview ships.

The component (`component` -> `bench_component.dll`) is a real WinRT component: RDL -> winmd ->
windows-bindgen -> `#[implement]`, activated registration-free via `DllGetActivationFactory`. It
exposes one activatable class, `Bench.Widget`, with scalar, string, metadata `Object`, nullable
reference, async, event, vector, map, and view operations. A non-default `INonDefault` interface
provides the QueryInterface fixture, and `LiveCount` reports native ownership balance.

## Running

```powershell
# Full run (10,000,000 iterations, median of three runs by default)
crates/tests/bench/run.ps1

# Quick run
crates/tests/bench/run.ps1 -Iterations 100000 -Runs 1
```

`run.ps1` builds the component first so its metadata writer does not race a consumer build, builds
the three consumers, stages `bench_component.dll` as `Bench.dll`, and prints median throughput,
memory, and leak values. The lower native result and the CsWinRT result are bold.

Each consumer also has a `cargo test` that runs the same path with a small iteration count, so CI
exercises the matrix wherever the .NET SDK and MSVC toolchain are present.

## Results

Measured on August 5, 2026 with .NET SDK 10.0.302 and CsWinRT 2.3.1. Times are milliseconds for
10,000,000 operations (median of three runs); `Error` uses 1,000,000 operations. Lower is better.

| Metric | C++/WinRT | windows-rs | CsWinRT 2 |
| --- | ---: | ---: | ---: |
| Create | 624 | 567 | 11,075 |
| Int32 | 25 | 24 | 45 |
| String | 298 | 291 | 1,588 |
| Add | 14 | 17 | 25 |
| Cast | 153 | 146 | 26 |
| CastOwned | 151 | 145 | 149 |
| Interface | 14 | 17 | 64 |
| Object | 141 | 138 | 1,205 |
| Event | 219 | 218 | 929 |
| AddRemove | 353 | 781 | 24,526 |
| Vector | 119 | 121 | 254 |
| IterateVector | 1,287 | 129 | 4,516 |
| GetMany | 2 | 2 | 183 |
| Map | 722 | 579 | 17,012 |
| Lookup | 219 | 183 | 297 |
| VectorView | 19 | 24 | 139 |
| MapView | 152 | 167 | 246 |
| Reference | 2,323 | 766 | 24,049 |
| Async | 579 | 490 | 54,547 |
| Error | 21,263 | 7 | 2,858 |
| Memory (bytes/object) | 8 | 8 | 296 |
| Leak (live objects) | 0 | 0 | 0 |

## Metrics

| Metric | What it measures |
| --- | --- |
| Create | Activate a fresh `Widget`. |
| Int32 | Set and get the `Int32` property. |
| String | Set and get the `String` property, including HSTRING conversion. |
| Add | Call a method with two `Int32` arguments and a return value. |
| Cast | Reach `INonDefault` with each projection's preferred short-lived cast. |
| CastOwned | Request an owning projected interface and call `Value`. |
| Interface | Acquire `INonDefault` once, then call `Value` repeatedly. |
| Object | Set and get metadata `Object` (`IInspectable`). |
| Event | Subscribe one handler and raise `Changed` through `Signal`. |
| AddRemove | Subscribe and unsubscribe a `Changed` handler each iteration. |
| Vector | Read one element per iteration from an `IVector<int>`. |
| IterateVector | Iterate the whole `IVector<int>` for a bounded number of passes. |
| GetMany | Copy a vector into caller-owned storage through the public bulk-copy surface. |
| Map | Enumerate values from an `IMap<string, int>`. |
| Lookup | Read one value per iteration from an `IMap<int, int>`. |
| VectorView | Read one element per iteration from an `IVectorView<int>`. |
| MapView | Read one value per iteration from an `IMapView<int, int>`. |
| Reference | Box and unbox a nullable `Int32` through `IReference<int>`. |
| Async | Consume an already-completed `IAsyncOperation<int>`. |
| Error | Propagate a failing `HRESULT` with a reduced iteration count. |
| Memory | Client-side allocated bytes per retained live object. |
| Leak | Component live-instance count above baseline after the run. |

The three interface metrics separate distinct projection shapes. `Cast` uses the preferred
short-lived form, `CastOwned` requests an owning interface value, and `Interface` acquires the
interface once before the loop. C++/WinRT and windows-rs use stack-value pointer owners. CsWinRT
preserves RCW identity and lazily caches interface state.

The timed bodies use the same loop bounds, inputs, result retention, collection sizes, and reduced
error count wherever the public projection surfaces permit. CsWinRT finalization is forced outside
affected timers so cleanup does not contaminate the following metric.

The `GetMany` row requests the same final result through each public projection. windows-rs and
C++/WinRT expose the WinRT `GetMany` method directly. CsWinRT maps `IVector<T>` to `IList<T>`, so
its public `CopyTo` adapter loops over `GetAt`.

The `Error` row highlights projection error models. Rust observes a failed `HRESULT` as a `Result`
branch. C++/WinRT and CsWinRT throw and catch exceptions, so this row uses a reduced loop count.

The leak row checks native `AddRef`/`Release` balance. Rust and C++/WinRT release deterministically
at scope exit. CsWinRT returns to baseline after `GC.Collect()` and
`WaitForPendingFinalizers()` because an RCW retains its native reference until finalization.

To add a metric, emit a matching `Name: <ms> ms` line from every consumer and add its name to the
`$Metrics` array in `run.ps1`.
