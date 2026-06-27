# Language projection overhead

A refreshed take on <https://github.com/kennykerr/lang-perf> that measures the raw
overhead each WinRT *language projection* adds on top of the same underlying ABI. It
compares the latest [C++/WinRT](https://github.com/microsoft/cppwinrt),
[C#/WinRT](https://github.com/microsoft/CsWinRT), and
[windows-rs](https://github.com/microsoft/windows-rs) calling an identical component.

## How it works

The callee is a tiny **no-op WinRT component** (`LangPerf.Class`). Because it does
nothing, the time spent in a tight loop is dominated by the projection glue —
activation, parameter marshaling, reference counting, and `QueryInterface` — rather
than by any real work. That isolates exactly what we want to compare.

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

Each consumer runs five loops and prints `label: N ms`:

| Loop     | What it exercises                                             |
|----------|--------------------------------------------------------------|
| `Create` | `Class()` activation + release                               |
| `Int32`  | set + get an `Int32` property (scalar in/out)               |
| `String` | set + get a `String` property (HSTRING reference-counting)  |
| `Object` | set + get an `Object` property (`IInspectable` ref-counting) |
| `Cast`   | `NewObject()` then `QueryInterface` to a non-default interface |

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

Release builds, 10,000,000 iterations, milliseconds (lower is better). All four
consumers issue the identical sequence of ABI calls, with the string hoisted out of the
loop in every language so only set/get traffic is measured. Absolute numbers are
machine-dependent; the relative shape is the point.

| Metric | C#/JIT | C#/AOT |  C++ | Rust |
|--------|-------:|-------:|-----:|-----:|
| Create |   9747 |  22820 |  519 |  460 |
| Int32  |     70 |     91 |   25 |   19 |
| String |   1646 |   1588 |  223 |  219 |
| Object |   1560 |   2225 |  270 |  268 |
| Cast   |  24268 |      ∞ |  435 |  426 |

C++/WinRT and Rust are both zero-overhead projections that compile down to direct
vtable calls, so they sit far below C# and stay within noise of each other — Rust is
marginally ahead on most loops and `String` is a tie now that both pass a pre-built
string.

C#/WinRT pays the cost of the managed runtime — runtime-callable-wrapper allocation,
garbage collection, and per-call interop thunks — which dominates `Create` and `Cast`
where wrappers are allocated. Scalar `Int32` traffic, by contrast, is nearly free.

### A note on Native AOT

The `C#/AOT` column publishes the same C# program with
[Native AOT](https://learn.microsoft.com/dotnet/core/deploying/native-aot/)
(`PublishAot`). Native AOT optimizes *startup* time, not steady-state ABI throughput, so
it does not help this benchmark: at 10,000,000 iterations it is slower than JIT on every
loop except `String`. `Cast` is the extreme case — it creates a fresh runtime-callable
wrapper every iteration, which degrades super-linearly under AOT's garbage collector
(≈14 µs/iter at 1,000,000 iterations and still climbing, versus a flat ≈2.6 µs/iter for
JIT), so a full run is impractical and is shown as ∞. JIT is the representative C#
result.
