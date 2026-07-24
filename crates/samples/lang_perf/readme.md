# Language projection overhead

> A WinRT language projection benchmark for C++/WinRT, C#/WinRT, and windows-rs.

- [C++/WinRT](https://github.com/microsoft/cppwinrt)
- [C#/WinRT](https://github.com/microsoft/CsWinRT)
- [windows-rs](https://github.com/microsoft/windows-rs)

This sample measures the overhead each WinRT language projection adds above the same ABI. It calls a
small WinRT component from Rust, C++, and C#. The component returns fixed values and performs no
work per call. The loops therefore measure projection cost, not component logic.

## How it works

The component exposes `LangPerf.Class`. Its methods ignore their arguments. Its getters return `0`,
an empty string, or an existing object. Its `Next` method returns `E_BOUNDS`. The component stores
no per-call state.

The measured cost comes from the projection glue:

- Activation
- Parameter marshaling
- Reference counting
- Error propagation
- `QueryInterface`
- Delegate calls
- Collection iteration
- Boxing
- Async completion

### One metadata file

All consumers use the same `lang.winmd` metadata. The ABI is a vtable boundary, so no language can
inline through it.

The sample includes two component implementations:

- [`component`](component) is the Rust component. [`src/lang.rdl`](component/src/lang.rdl) describes
  the API in RDL. [`build.rs`](component/build.rs) runs [`windows-rdl`](../../libs/rdl) and
  [`windows-bindgen`](../../libs/bindgen). [`src/lib.rs`](component/src/lib.rs) implements the no-op
  class with `#[implement]`.
- [`component_cpp`](component_cpp) is the C++/WinRT component. Its
  [`build.rs`](component_cpp/build.rs) runs [`cppwinrt`](https://github.com/microsoft/cppwinrt) over
  the same metadata and builds [`src/component.cpp`](component_cpp/src/component.cpp) with
  [`cc`](https://crates.io/crates/cc).

Rust and C++ consumers default to the component written in their own language. Each consumer calls
`Lang()` at startup and prints the component language.

No registration is required. Activation probes `LangPerf.dll` next to the executable. Each component
builds with a distinct DLL name, and each consumer stages its selected DLL as `LangPerf.dll` before
activation.

The C# consumer calls the Rust component. It also calls `Lang()`, so the output reports
`# C# consumer -> Rust component`.

## What is measured

Each consumer runs these loops and prints `label: N ms`:

| Loop            | What it exercises                                             |
| :-------------- | ------------------------------------------------------------ |
| `Create`        | `Class()` activation + release                               |
| `Int32`         | set + get an `Int32` property (scalar in/out)               |
| `String`        | set + get a `String` property (HSTRING marshaling, in/out)  |
| `Object`        | set + get an `Object` property (`IInspectable` ref-counting) |
| `Cast`          | `ObjectProperty()` then `QueryInterface` to a non-default interface |
| `Event`         | raise a WinRT event once per iteration with one subscriber (delegate invoke) |
| `AddRemove`     | subscribe a handler and immediately unsubscribe each iteration (delegate marshaling + token bookkeeping) |
| `IterateVector` | iterate an `IVector<Int32>` idiomatically (`for x in &v`) - one element at a time across the ABI |
| `GetMany`       | bulk-copy the same vector into a buffer with a single `GetMany` ABI call |
| `Map`           | iterate an `IMap<String, Int32>` idiomatically (`for pair in &map`) - batched iterator, but per-pair `Value()` access |
| `Async`         | await a synchronously-completed `IAsyncOperation<Int32>` (delegate-backed completion) |
| `Reference`     | set + get an `IReference<Int32>` property - box in the consumer/unbox in the component, then box in the component/unbox in the consumer (boxing both ways) |
| `Error`         | a `Next()` call that always returns `E_BOUNDS` (error propagation) |

`Create` activates and releases an object each iteration. `Int32`, `String`, `Object`, and `Cast`
call an existing object. They measure scalar copies, string marshaling, reference counting, and
interface casts.

`Event` invokes one registered delegate each iteration. `AddRemove` creates, subscribes, and removes
one delegate each iteration.

`IterateVector`, `GetMany`, and `Map` measure WinRT collection access. `IterateVector` uses
idiomatic iteration. `GetMany` uses one bulk ABI call per buffer. `Map` uses a batched iterator,
then reads each pair value.

`Async` awaits an operation that is already complete. `Reference` measures boxing on both sides of
the ABI. `Error` measures conversion from a failed `HRESULT` to each language projection's error
form.

## Running

The loop count defaults to a small value so `cargo run` stays fast. Pass `--iterations`, or set
`LANG_PERF_ITER`, for a measurement run.

```pwsh
cargo run --release -p lang_perf_rust -- --iterations 10000000
cargo run --release -p lang_perf_cpp  -- --iterations 10000000
```

Each consumer accepts `--component rust|cpp` to call a selected implementation. Build
`lang_perf_cpp` first, or run the matrix script.

```pwsh
cargo build --release -p lang_perf_cpp
cargo run --release -p lang_perf_rust -- --component cpp --iterations 10000000
```

The C# benchmark uses `dotnet`. It calls the Rust component. Build that component, stage it as
`LangPerf.dll`, add it to `PATH`, and run. It targets .NET 10 and C#/WinRT 2.2.0. The .NET 10 SDK is
required.

```pwsh
cargo build --release -p lang_perf_component
Copy-Item target/release/langperf_rust.dll target/release/LangPerf.dll
$env:PATH = "$PWD/target/release;$env:PATH"
dotnet run -c Release --project crates/samples/lang_perf/csharp -- --iterations 10000000
```

Run the full matrix with this command:

```pwsh
crates/samples/lang_perf/run.ps1 -Iterations 10000000
```

Add `-IncludeAot` to also measure a C# Native AOT build. This requires the Visual Studio C++
toolchain for the AOT linker.

```pwsh
crates/samples/lang_perf/run.ps1 -Iterations 10000000 -IncludeAot
```

## Sample results

Release builds, 10,000,000 iterations, milliseconds. Lower is better. In this table, every consumer
calls the Rust component.

Each consumer issues the same ABI calls. Each passes values in its idiomatic form: `h!("value")` in
Rust, `L"value"` in C++, and `"value"` in C#. The component ignores inputs and returns fixed values.
The relative shape is the important result.

High iteration counts measure steady-state per-call throughput. They amortize startup and one-time
JIT cost. Per-iteration allocation and garbage collection remain part of the measurement.

| Metric        | C#/AOT | C#/JIT |    C++ | Rust |
| :------------ | -----: | -----: | -----: | ---: |
| Create        |  17288 |   9963 |    507 |  442 |
| Int32         |     90 |     64 |     28 |   20 |
| String        |    221 |    245 |     32 |   21 |
| Object        |   1358 |   1127 |    135 |  133 |
| Cast          |   2549 |   1337 |    281 |  271 |
| Event         |   1213 |    913 |    317 |  321 |
| AddRemove     |  82277 |  29048 |   1475 | 1512 |
| IterateVector |    383 |    673 |    127 |    4 |
| GetMany       |    191 |    329 |      2 |    6 |
| Map           |   2603 |   1813 |    955 |  775 |
| Async         | 479320 |  49902 |    994 |  452 |
| Reference     | 141045 |  25166 |   2154 |  703 |
| Error         |  15542 |  14543 | 144601 |   53 |

C++/WinRT and Rust compile most calls to direct vtable calls. Both stay far below C# on most loops.
Rust leads or ties C++ in these results. Its largest wins are collection iteration, boxed values,
async completion, and errors.

C#/WinRT pays managed interop cost on each call. This includes wrapper lookup, interop thunks, and
garbage collection pressure. Native AOT improves startup, but it does not improve this steady-state
ABI benchmark.

### Component language matrix

This matrix runs each consumer against both component implementations. It uses 1,000,000 iterations.
The headings show `consumer->component`.

| Metric        | C#->C++ | C#->Rust | C++->C++ | Rust->C++ | C++->Rust | Rust->Rust |
| :------------ | -----: | ------: | ------: | -------: | -------: | --------: |
| Create        |   1159 |    1217 |      62 |       59 |       51 |        47 |
| Int32         |      6 |       5 |       3 |        3 |        2 |         2 |
| String        |    108 |     106 |       3 |        3 |        3 |         2 |
| Object        |    162 |     154 |      13 |       14 |       13 |        13 |
| Cast          |    217 |     243 |      27 |       27 |       27 |        27 |
| Event         |    102 |     103 |      33 |       32 |       30 |        30 |
| AddRemove     |   2953 |    2932 |     133 |      138 |      139 |       144 |
| IterateVector |     48 |      51 |      13 |        1 |       12 |         1 |
| GetMany       |     23 |      24 |       0 |        0 |        0 |         0 |
| Map           |    385 |     242 |      95 |       84 |       78 |        74 |
| Async         |   3677 |    3794 |      95 |       94 |       46 |        46 |
| Reference     |   2660 |    2496 |     324 |      187 |      188 |        62 |
| Error         |  17996 |    1893 |   21181 |    15865 |    14346 |         5 |

Pure ABI loops do not change when the component language changes. The caller sees the same vtable
shape and metadata.

`Error`, `Async`, and `Reference` depend on endpoint work. Error origination runs in the endpoint
that creates or materializes rich error information. Async completion depends on the operation
object the component creates. Reference boxing happens once in the consumer and once in the
component.

### Error propagation

`Error` calls `Next()`, which always returns `E_BOUNDS`. Rust returns this as a `Result::Err` value.
C#/WinRT throws a managed exception. C++/WinRT throws an `hresult_error` and creates WinRT
restricted error information.

The Rust path is cheap because it does not throw and does not originate rich error information. This
matters when failures are routine, such as iterator end.

### Delegates and events

`Event` raises a WinRT event with one subscriber. `AddRemove` subscribes a fresh delegate and
removes it each iteration.

Both native components use thread-safe event storage. The Rust component uses
`windows_core::Event<T>`. The C++ component uses `winrt::event`. These loops therefore mostly expose
consumer cost.

C#/WinRT is much slower on `AddRemove`. Each subscribe wraps a managed delegate. Each unsubscribe
tears it down. Native projections pass a small COM delegate.

### Collections and iteration

`IterateVector` and `GetMany` walk the same component-owned `IVector<Int32>`. The component language
has little effect. The consumer projection controls the cost.

windows-rs uses a buffered iterator for `for x in &v`. It fills a small buffer with `GetMany` and
yields from that buffer. C++/WinRT range iteration reads one element per ABI call. This gives Rust a
large lead for idiomatic vector walks.

`Map` also uses a batched iterator. The iterator returns `IKeyValuePair` COM objects, so each
`Value()` call still crosses the ABI. Batching removes iterator step cost, but it cannot remove each
pair object's accessor cost.

### Async

`Async` awaits an `IAsyncOperation<Int32>` that is complete before the wait. Rust uses
`windows-future` and returns a ready operation. C++/WinRT uses a coroutine operation. The
component's operation object drives most of the native cost.

C#/WinRT pays managed await and interop cost. Native AOT is slower here because this benchmark
measures steady-state ABI throughput, not process startup.

### Boxed values

`Reference` sets and gets an `IReference<Int32>` property. Each iteration boxes once in the consumer
and once in the component.

The matrix cost is the sum of the two endpoint boxing costs:

| combo       | consumer box (set) | component box (get) | total |
| :---------- | -----------------: | ------------------: | ----: |
| `Rust->Rust` |               ~30  |                ~30  |  `62` |
| `Rust->C++`  |               ~30  |               ~155  | `187` |
| `C++->Rust`  |              ~155  |                ~30  | `188` |
| `C++->C++`   |              ~155  |               ~155  | `324` |

windows-rs boxes with `windows-reference`. C++/WinRT boxes through `PropertyValue::CreateInt32`. The
Rust path allocates a small in-process object. The C++ path calls a factory and creates a general
property value object.

### Benchmark structure

Each measured operation crosses a WinRT vtable boundary. Rust, C++, and C# cannot inline into the
component or fold the result.

The C# consumer also supports per-scenario methods with warmup and garbage collection between loops.
The results stay in the same range:

| Scenario | Single method | Per-scenario method + warmup + GC |
| -------- | ------------: | --------------------------------: |
| Create   |     ~10000 ms |                         ~10300 ms |
| Int32    |       ~49 ms  |                           ~39 ms  |
| String   |      ~1090 ms |                          ~980 ms  |
| Object   |      ~1170 ms |                         ~1090 ms  |
| Cast     |      ~1410 ms |                         ~1320 ms  |
| Error    | ~14000 ms     |                       ~15000 ms   |

The method structure does not explain the projection gap. The gap comes from projection overhead at
the ABI boundary.

### Native AOT

The `C#/AOT` column publishes the same C# program with [Native
AOT](https://learn.microsoft.com/dotnet/core/deploying/native-aot/) (`PublishAot`). Native AOT
improves process startup. This benchmark measures steady-state ABI calls, so AOT does not help these
results.

### C# component

Rust and C++ each include a component written in their own language. The C# consumer calls the Rust
component.

A C# component with this exact metadata does not fit the registration-free setup. The native host
and the managed assembly both need the same `LangPerf.dll` name. CsWinRT authoring also emits
different default-interface metadata.

This does not change the comparison. The benchmark measures caller projection cost, and the
component performs no per-call work.