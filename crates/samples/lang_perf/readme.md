# Language projection overhead

A refreshed take on <https://github.com/kennykerr/lang-perf> that measures the raw
overhead each WinRT *language projection* adds on top of the same underlying ABI. It
compares the latest [C++/WinRT](https://github.com/microsoft/cppwinrt),
[C#/WinRT](https://github.com/microsoft/CsWinRT), and
[windows-rs](https://github.com/microsoft/windows-rs) calling an identical component.

## How it works

The callee is a tiny **no-op WinRT component** (`LangPerf.Class`). Every method ignores
its arguments and returns a fixed, known value — the setters discard their input, the
getters return `0`, an empty string, or a reference to an already-live object, and the
`Next` method always fails with a fixed `E_BOUNDS` `HRESULT` — so the component stores no
state and does no real work per call. That leaves a tight loop dominated by the
projection glue — activation, parameter marshaling, reference counting, error
propagation, and `QueryInterface` — which is exactly what we want to compare.

The component is authored entirely in Rust:

- [`component/src/lang.rdl`](component/src/lang.rdl) describes the API in RDL.
- [`component/build.rs`](component/build.rs) runs [`windows-rdl`](../../libs/rdl) to
  produce [`lang.winmd`](component/lang.winmd) and
  [`windows-bindgen`](../../libs/bindgen) to generate the implementation bindings.
- [`component/src/lib.rs`](component/src/lib.rs) implements the no-op class with
  `#[implement]` and exports `DllGetActivationFactory`.

The single `lang.winmd` then feeds all three consumers, so each language projects the
*same* metadata. This mirrors the [`robot`](../robot) interop sample, and doubles as a
showcase of consuming one Rust-authored component from three languages.

No registration is required: every projection falls back to
`LoadLibrary("LangPerf.dll")` when activating an unregistered class, so the component's
cdylib is simply named `langperf.dll` and co-located with the consumers.

## What is measured

Each consumer runs six loops and prints `label: N ms`:

| Loop     | What it exercises                                             |
|----------|--------------------------------------------------------------|
| `Create` | `Class()` activation + release                               |
| `Int32`  | set + get an `Int32` property (scalar in/out)               |
| `String` | set + get a `String` property (HSTRING marshaling, in/out)  |
| `Object` | set + get an `Object` property (`IInspectable` ref-counting) |
| `Cast`   | `ObjectProperty()` then `QueryInterface` to a non-default interface |
| `Error`  | a `Next()` call that always returns `E_BOUNDS` (error propagation) |

`Create` is the only loop that allocates: it activates and releases an object each
iteration. The next four are pure ABI traffic — scalar copies, string marshaling, an
`AddRef`/`Release` pair, and a `QueryInterface` — all on an already-live object, with no
allocation in the component. `Error` isolates the failure path: the component's `Next`
method does nothing but return the `E_BOUNDS` `HRESULT`, so the loop measures how each
projection turns an ABI error code into its idiomatic error type — a `Result::Err` in
Rust, a thrown `hresult_error` in C++/WinRT, and a thrown managed exception in C#/WinRT.

## Running

The loop count defaults to a tiny value so `cargo run` stays fast; pass `--iterations`
(or set `LANG_PERF_ITER`) for a real measurement. Build the component package first so
`langperf.dll` lands next to the executables:

```pwsh
cargo build --release -p lang_perf_component
cargo run --release -p lang_perf_rust -- --iterations 10000000
cargo run --release -p lang_perf_cpp  -- --iterations 10000000
```

The C# benchmark is built by `dotnet` rather than cargo; run it directly (with the
component's output directory on `PATH` so `langperf.dll` resolves):

```pwsh
$env:PATH = "$PWD/target/release;$env:PATH"
dotnet run -c Release --project crates/samples/lang_perf/csharp -- --iterations 10000000
```

Or build and run them all and print a comparison table:

```pwsh
crates/samples/lang_perf/run.ps1 -Iterations 10000000
```

Add `-IncludeAot` to also publish and measure a C# Native AOT build (this needs the
Visual Studio C++ toolchain for the AOT linker):

```pwsh
crates/samples/lang_perf/run.ps1 -Iterations 10000000 -IncludeAot
```

## Sample results

Release builds, 10,000,000 iterations, milliseconds (lower is better). Each consumer
issues the identical sequence of ABI calls and passes each value the natural, idiomatic
way for its language — including the string argument, written as `h!("value")` in Rust,
`L"value"` in C++, and `"value"` in C#. Because the component ignores its inputs and
returns fixed values, the numbers are dominated by projection/ABI cost rather than any
work in the component. Absolute numbers are machine-dependent; the relative shape is the
point.

| Metric | C#/JIT |  C#/AOT |    C++ | Rust |
|--------|-------:|--------:|-------:|-----:|
| Create |   9198 |   16622 |    501 |  443 |
| Int32  |     54 |      87 |     26 |   20 |
| String |   1049 |     223 |     32 |   21 |
| Object |   1448 |    2107 |    135 |  133 |
| Cast   |   1650 |    5325 |    281 |  271 |
| Error  |  22392 | crashed | 141663 |   53 |

For every loop except `Error`, C++/WinRT and Rust are both zero-overhead projections that
compile down to direct vtable calls, so they sit far below C# and stay within noise of
each other — Rust is marginally ahead on most loops and ties the rest. With the component
doing nothing, the pure-ABI loops (`Int32`, `String`, `Object`, `Cast`) cost tens to low
hundreds of milliseconds: a scalar copy, a fast-pass string marshal, an `AddRef`/`Release`
pair, and a `QueryInterface` are all essentially free. Only `Create` costs more, because
it genuinely activates and releases an object each iteration.

C#/WinRT pays for the managed runtime on every call — runtime-callable-wrapper lookups,
garbage collection, and per-call interop thunks — so even the pure-ABI loops run an order
of magnitude slower than C++/Rust, and the allocating `Create` loop dramatically so.

### Error propagation

`Error` is where the projections diverge by orders of magnitude, and there are two costs
stacked on top of each other. The first is the exception machinery itself. Rust projects a
failed `HRESULT` as a `Result::Err` — an ordinary returned value — so the loop stays as
cheap as the other pure-ABI loops (`53 ms`, ~5 ns per call). C#/WinRT projects the same
failure as a *thrown* managed exception and pays `22392 ms`, roughly `2 µs` per call — and
that is almost entirely the throw, because C#'s `ThrowExceptionForHR` does *not* eagerly
originate restricted error info and .NET only materializes a stack trace if one is read.
That ~400× gap between a return and a bare throw is the headline: exceptions are genuinely
expensive, and a projection that returns errors is dramatically cheaper whenever failures
are routine — an iterator reaching its end with `E_BOUNDS`, say.

C++/WinRT then piles a second cost on top of the throw. Its `141663 ms` is about `14 µs`
per call, an order of magnitude worse than C#'s already-expensive throw. A bare native
throw/catch on this machine is only ~1.5 µs, so the remaining ~12 µs is WinRT error
*origination*: `winrt::throw_hresult` builds a full `hresult_error`, which calls
`RoOriginateLanguageException` to create an `IRestrictedErrorInfo` and capture the error
context on every single throw. So C++/WinRT is slow for both reasons — it throws *and* it
eagerly enriches each failure — while C# pays only the throw and Rust pays neither.

The C#/AOT build can't run this loop at all: throwing the projected exception in a tight
loop terminates the process with a `StackOverflowException`, so its `Error` cell reads
`crashed`.

### A note on Native AOT

The `C#/AOT` column publishes the same C# program with
[Native AOT](https://learn.microsoft.com/dotnet/core/deploying/native-aot/)
(`PublishAot`). Native AOT optimizes *startup* time, not steady-state ABI throughput, so
it does not help this benchmark: at 10,000,000 iterations it is slower than JIT on every
loop except `String`, where its leaner string marshaling is several times faster. The
`Object` and `Cast` loops are the worst cases — each `QueryInterface`/wrapper lookup goes
through AOT's interop layer and garbage collector — but they remain linear and tractable.
The AOT build cannot complete the `Error` loop at all: throwing the projected exception in
a tight loop terminates the process with a `StackOverflowException`. JIT is the
representative C# result.
