# Language projection overhead

A refreshed take on <https://github.com/kennykerr/lang-perf> that measures the raw
overhead each WinRT *language projection* adds on top of the same underlying ABI. It
compares the latest [C++/WinRT](https://github.com/microsoft/cppwinrt),
[C#/WinRT](https://github.com/microsoft/CsWinRT), and
[windows-rs](https://github.com/microsoft/windows-rs).

## How it works

The callee is a tiny **no-op WinRT component** (`LangPerf.Class`). Every method ignores
its arguments and returns a fixed, known value — the setters discard their input, the
getters return `0`, an empty string, or a reference to an already-live object, and the
`Next` method always fails with a fixed `E_BOUNDS` `HRESULT` — so the component stores no
state and does no real work per call. That leaves a tight loop dominated by the
projection glue — activation, parameter marshaling, reference counting, error
propagation, and `QueryInterface` — which is exactly what we want to compare.

### One metadata, a component per language

A natural objection to "Rust calls Rust" is that it might somehow be an unfair
advantage. It is not — the WinRT ABI is a hard vtable boundary, so there is no inlining
across it and no language can see into the component it calls. To make that concrete, the
component is authored *twice*, in Rust and in C++/WinRT, so a consumer can call an
implementation written in its own language:

- [`component`](component) — the **Rust** component. [`src/lang.rdl`](component/src/lang.rdl)
  describes the API in RDL; [`build.rs`](component/build.rs) runs
  [`windows-rdl`](../../libs/rdl) to produce [`lang.winmd`](component/lang.winmd) and
  [`windows-bindgen`](../../libs/bindgen) for the implementation bindings;
  [`src/lib.rs`](component/src/lib.rs) implements the no-op class with `#[implement]`.
- [`component_cpp`](component_cpp) — the **C++/WinRT** component. Its
  [`build.rs`](component_cpp/build.rs) runs [`cppwinrt`](https://github.com/microsoft/cppwinrt)
  over the *same* `lang.winmd` and compiles [`src/component.cpp`](component_cpp/src/component.cpp)
  with [`cc`](https://crates.io/crates/cc), mirroring the [`robot`](../robot) sample.

Both components are generated from the single hand-authored `lang.winmd`, so every
consumer projects byte-for-byte identical metadata — the only thing that changes is the
implementation language behind the ABI. The Rust and C++ consumers depend (in Cargo) on
their own component, so a bare `cargo run -p lang_perf_rust` or `-p lang_perf_cpp` builds
and uses the matching one.

To *prove* which implementation answered, the class has a `Lang()` method that returns a
language tag (`"Rust"`, `"C++"`). Every consumer calls it once at startup and prints, for
example, `# C++ consumer -> C++ component`.

No registration is required: every projection falls back to
`LoadLibrary("LangPerf.dll")` when activating an unregistered class, and the
exe-adjacent DLL wins the loader search order. Each component therefore builds to a
distinct cdylib (`langperf_rust.dll`, `langperf_cpp.dll`) and each consumer copies its
own DLL to `LangPerf.dll` next to the executable at startup. The distinct names avoid a
shared-output collision between the two cdylibs and keep `cargo run` honest.

The **C# consumer calls the Rust component**: a drop-in same-metadata C# component is not
feasible without registration — see [the note below](#a-note-on-a-c-component). It still
calls `Lang()`, so its output honestly reports `# C# consumer -> Rust component`.

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
(or set `LANG_PERF_ITER`) for a real measurement. Each consumer depends on its own
component and stages it as `LangPerf.dll` next to the executable, so a plain `cargo run`
just works:

```pwsh
cargo run --release -p lang_perf_rust -- --iterations 10000000
cargo run --release -p lang_perf_cpp  -- --iterations 10000000
```

Each consumer defaults to its own-language component but accepts `--component rust|cpp` to
call the other implementation instead. This needs both component DLLs in the release
directory, so build `lang_perf_cpp` (or run the matrix script below) first:

```pwsh
cargo build --release -p lang_perf_cpp
cargo run --release -p lang_perf_rust -- --component cpp --iterations 10000000
```

The C# benchmark is built by `dotnet` rather than cargo and calls the Rust component. Build
that component, stage it as `LangPerf.dll` (the name activation probes for) on `PATH`, then
run. It targets **.NET 10** (the current release) and **C#/WinRT (CsWinRT) 2.2.0**, the
latest *stable* projection — see [the note below](#a-note-on-cswinrt-30) for why it does not
use the 3.0 preview. The .NET 10 SDK is required:

```pwsh
cargo build --release -p lang_perf_component
Copy-Item target/release/langperf_rust.dll target/release/LangPerf.dll
$env:PATH = "$PWD/target/release;$env:PATH"
dotnet run -c Release --project crates/samples/lang_perf/csharp -- --iterations 10000000
```

Or build and run the full **matrix** — every consumer calling every component — and print
the table:

```pwsh
crates/samples/lang_perf/run.ps1 -Iterations 10000000
```

Add `-IncludeAot` to also publish and measure a C# Native AOT build (this needs the
Visual Studio C++ toolchain for the AOT linker):

```pwsh
crates/samples/lang_perf/run.ps1 -Iterations 10000000 -IncludeAot
```

## Sample results

Release builds, 10,000,000 iterations, milliseconds (lower is better). In this table every
consumer calls the **Rust** component; the [matrix below](#does-the-components-language-matter-the-matrix)
confirms the component's language makes no difference except on `Error`. Each consumer
issues the identical sequence of ABI calls and passes each value the natural, idiomatic
way for its language — including the string argument, written as `h!("value")` in Rust,
`L"value"` in C++, and `"value"` in C#. Because the component ignores its inputs and
returns fixed values, the numbers are dominated by projection/ABI cost rather than any
work in the component. Absolute numbers are machine-dependent; the relative shape is the
point.

| Metric | C#/JIT | C#/AOT |    C++ | Rust |
|--------|-------:|-------:|-------:|-----:|
| Create |   9963 |  17288 |    507 |  442 |
| Int32  |     64 |     90 |     28 |   20 |
| String |    245 |    221 |     32 |   21 |
| Object |   1127 |   1358 |    135 |  133 |
| Cast   |   1337 |   2549 |    281 |  271 |
| Error  |  14543 |  15542 | 144601 |   53 |

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

### Does the component's language matter? The matrix

To check whether "Rust calls Rust" is somehow an unfair advantage, each consumer was
pointed at *both* components in turn (`run.ps1` runs the full grid; the `Lang()` header
confirms which implementation answered). One run at 1,000,000 iterations, consumer→component:

| Metric | C#→Rust | C#→C++ | C++→Rust | C++→C++ | Rust→Rust | Rust→C++ |
|--------|--------:|-------:|---------:|--------:|----------:|---------:|
| Create |    1087 |   1149 |       51 |      63 |        45 |       57 |
| Int32  |       5 |      5 |        2 |       3 |         2 |        3 |
| String |     110 |    106 |        3 |       3 |         2 |        3 |
| Object |     164 |    145 |       13 |      13 |        13 |       15 |
| Cast   |     228 |    238 |       28 |      27 |        26 |       27 |
| Error  |    1485 |  16699 |    14165 |   20760 |         5 |    15454 |

For every loop except `Error`, swapping the component's language changes nothing: each
consumer posts the same numbers whether it calls the Rust or the C++ component (`C++→Rust`
vs `C++→C++`, `Rust→Rust` vs `Rust→C++` are within noise). That is the whole point — the
ABI is a hard vtable boundary with no cross-language inlining, so the callee's language is
invisible to the caller. There is no "same-language" advantage to erase, and Rust's lead
over C#/WinRT is the projection, not the fact that it happened to be calling Rust.

`Error` is the exception, and it is illuminating. The cost is WinRT error *origination* —
building an `IRestrictedErrorInfo` via `RoOriginateLanguageException` — and the matrix
shows it is incurred by whichever endpoint is **C++/WinRT**, on *either* side of the call:

- **`Rust→Rust` (`5 ms`) is the only free combo.** The Rust component returns the bare
  failed `HRESULT` and the Rust consumer receives it as a `Result::Err` value — nobody
  originates, nobody throws.
- **The C++ *component* originates on the way out.** Holding the consumer fixed and
  pointing it at the C++ component adds ~15 µs/call: `C#→Rust` `1485` → `C#→C++` `16699`,
  and `Rust→Rust` `5` → `Rust→C++` `15454`. For C# the cost is cleanly additive — its own
  ~1.5 µs managed throw plus the component's ~15 µs origination.
- **A C++/WinRT *consumer* originates on the way in.** `C++→Rust` is `14165 ms` even though
  the Rust component originates nothing: handed a bare failed `HRESULT`, C++/WinRT's
  `throw_hresult` builds the full `hresult_error` itself, so the ~14 µs reappears on the
  catch side.
- When **both** ends are C++/WinRT (`C++→C++`, `20760 ms`) the work does not simply double —
  origination happens once and the consumer reuses the error info the component attached —
  but it is the most expensive combo, paying for the component's origination *and* the
  consumer materializing a thrown `hresult_error`.

The lesson: the instant either endpoint is C++/WinRT you pay for rich error origination,
whether the callee eagerly enriches the failure or the caller reconstructs it on receipt.
C# pays only its managed throw on top of whatever the component did, and Rust pays neither
on either side — so routine failures (an iterator hitting `E_BOUNDS`) stay essentially free
end to end only when both ends are Rust.

### Error propagation

`Error` is worth dwelling on, because the projections diverge by orders of magnitude even
before origination enters the picture. Rust projects a failed `HRESULT` as a `Result::Err`
— an ordinary returned value — so the loop stays as cheap as the pure-ABI loops (`53 ms` at
10M, ~5 ns per call). C#/WinRT projects the same failure as a *thrown* managed exception
(`14543 ms`, ~1.5 µs per call), almost entirely the throw itself: C#'s `ThrowExceptionForHR`
does *not* eagerly originate restricted error info, and .NET only materializes a stack trace
if one is read. That ~270× gap between a return and a bare throw is the headline — exceptions
are genuinely expensive, so a projection that returns errors wins big whenever failures are
routine, such as an iterator reaching its end with `E_BOUNDS`.

On top of the throw sits WinRT error *origination*, the ~14–20 µs cost dissected in the
[matrix](#does-the-components-language-matter-the-matrix) above: any C++/WinRT endpoint
builds a full `IRestrictedErrorInfo` via `RoOriginateLanguageException`, whether it is the
component enriching the failure on the way out or a C++/WinRT consumer reconstructing it on
receipt. This is why the headline table's `C++` column reaches `144601 ms` (~14 µs per call)
even calling the originating-free Rust component — the C++ *consumer* originates on receipt.
Rust originates on neither side.

The C#/AOT build pays about the same as JIT here (`15542 ms`, ~1.6 µs per call): Native AOT
changes startup, not the cost of throwing, so the exception machinery dominates either way.

### A note on Native AOT

The `C#/AOT` column publishes the same C# program with
[Native AOT](https://learn.microsoft.com/dotnet/core/deploying/native-aot/)
(`PublishAot`). Native AOT optimizes *startup* time, not steady-state ABI throughput, so
it does not help this benchmark: at 10,000,000 iterations it is slower than JIT on every
loop except `String`, where its leaner string marshaling is slightly faster. The `Cast`
loop is the worst case — each `QueryInterface`/wrapper lookup goes through AOT's interop
layer and garbage collector — but every loop stays linear and tractable, including `Error`,
which AOT now completes at roughly the same cost as JIT. JIT is the representative C#
result.

### A note on a C# component

Rust and C++ each call a component written in their own language; C# does not, because a
drop-in C# component that shares this exact metadata cannot be activated without
registration. CsWinRT *authoring* (`CsWinRTComponent`) was the obvious route, but it has
two blockers:

- **Name collision with the host.** Manifest-free activation requires the native host
  (`WinRT.Host.dll`) to be renamed to `LangPerf.dll`. CsWinRT authoring also requires the
  managed assembly name to match the type's root namespace, forcing *it* to be
  `LangPerf.dll` too — the host and the implementation cannot both own that one filename.
- **Incompatible metadata.** Authoring regenerates its own winmd with a *different*
  default interface. Reading both winmds with [`windows-metadata`](../../libs/metadata)
  shows the hand-authored default interface is `LangPerf.IClass`
  (`25901a4a-7a56-5621-97ca-51c51587322b`), while CsWinRT emits `LangPerf.IClassClass`
  (`8212d01d-bcc1-59bd-acbe-11084aaf3a8a`) — a different IID and slot layout, so it is not
  ABI-compatible with the shared `lang.winmd` the other consumers project.

A C#→C# in-process call via a project reference would also sidestep WinRT activation
entirely, so it would not measure projection overhead in the first place. The C# consumer
therefore calls the Rust component and labels it honestly via `Lang()`. This does not
weaken the comparison: the projection cost being measured is on the *caller* side, and the
component does no real work regardless of who wrote it.

### A note on CsWinRT 3.0

The C# numbers use **CsWinRT 2.2.0**, the latest *stable* projection. CsWinRT 3.0 — a
ground-up rewrite of the interop layer for .NET 10, currently in preview — does not work
with this component: it activates the class fine, but the first call through a projected
member access-violates (`0xC0000005`) inside its own marshaling layer. The component's ABI
is not at fault — C++/WinRT, Rust, CsWinRT 2.x, and even raw function-pointer calls from
.NET 10 all invoke the exact same vtable correctly, and the projection computes the right
IID and slot layout. The sample therefore pins 2.2.0 until the 3.0 projection can call into
the component; the comparison above is unaffected, since both project the identical
`lang.winmd`.
