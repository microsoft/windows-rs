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

Or build and run all three and print a comparison table:

```pwsh
crates/samples/lang_perf/run.ps1 -Iterations 10000000
```

## Sample results

Release builds, 10,000,000 iterations, milliseconds (lower is better). Absolute
numbers are machine-dependent; the relative shape is the point.

| Metric | C#/WinRT | C++/WinRT |  Rust |
|--------|---------:|----------:|------:|
| Create |    10052 |       522 |   456 |
| Int32  |       55 |        25 |    19 |
| String |     1714 |       404 |   223 |
| Object |     1576 |       270 |   268 |
| Cast   |    26147 |       440 |   453 |

C++/WinRT and Rust are both zero-overhead projections that compile down to direct
vtable calls, so they sit far below C#/WinRT and stay within noise of each other. Rust
leads on `Create`, `Int32`, and `Object`, and pulls clearly ahead on `String` by
building the `HSTRING` once up front; `Cast` is effectively a wash that trades the lead
run to run.

C#/WinRT pays the cost of the managed runtime — runtime-callable-wrapper allocation,
garbage collection, and per-call interop thunks — which dominates `Create` and `Cast`
where wrappers are allocated. Scalar `Int32` traffic, by contrast, is nearly free.

### A note on Native AOT

The C# consumer runs on the JIT runtime, which is what these numbers report. Publishing
it with [Native AOT](https://learn.microsoft.com/dotnet/core/deploying/native-aot/)
(`PublishAot`) optimizes *startup* time, but this benchmark measures steady-state ABI
cost — where AOT does not help and is markedly worse for the allocation-heavy loops.
With a fresh runtime-callable wrapper created every iteration, `Create` and especially
`Cast` degrade super-linearly under AOT's garbage collector (`Cast` is roughly 5× slower
at 1,000,000 iterations and worse beyond), while JIT stays roughly linear. The
non-allocating `String` and `Object` loops are actually a touch faster under AOT. JIT is
reported here as the representative C# result.
