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

Each consumer runs thirteen loops and prints `label: N ms`:

| Loop            | What it exercises                                             |
| :-------------- | ------------------------------------------------------------ |
| `Create`        | `Class()` activation + release                               |
| `Int32`         | set + get an `Int32` property (scalar in/out)               |
| `String`        | set + get a `String` property (HSTRING marshaling, in/out)  |
| `Object`        | set + get an `Object` property (`IInspectable` ref-counting) |
| `Cast`          | `ObjectProperty()` then `QueryInterface` to a non-default interface |
| `Event`         | raise a WinRT event once per iteration with one subscriber (delegate invoke) |
| `AddRemove`     | subscribe a handler and immediately unsubscribe each iteration (delegate marshaling + token bookkeeping) |
| `IterateVector` | iterate an `IVector<Int32>` idiomatically (`for x in &v`) — one element at a time across the ABI |
| `GetMany`       | bulk-copy the same vector into a buffer with a single `GetMany` ABI call |
| `Map`           | iterate an `IMap<String, Int32>` idiomatically (`for pair in &map`) — batched iterator, but per-pair `Value()` access |
| `Async`         | await a synchronously-completed `IAsyncOperation<Int32>` (delegate-backed completion) |
| `Reference`     | set + get an `IReference<Int32>` property — box in the consumer/unbox in the component, then box in the component/unbox in the consumer (boxing both ways) |
| `Error`         | a `Next()` call that always returns `E_BOUNDS` (error propagation) |

`Create` is the only loop that allocates: it activates and releases an object each
iteration. The next four are pure ABI traffic — scalar copies, string marshaling, an
`AddRef`/`Release` pair, and a `QueryInterface` — all on an already-live object, with no
allocation in the component. `Event` and `AddRemove` exercise WinRT **delegates and
events**: `Event` keeps one subscriber registered and invokes the delegate across the ABI
each iteration; `AddRemove` churns subscription, constructing a fresh WinRT delegate from a
language callback and handing it to the component's `add`/`remove` each time. Both components
back the event with equivalent thread-safe storage (windows-rs `Event<T>`, cppwinrt
`winrt::event`), so these loops end up consumer-driven; they get their own
[section](#delegates-and-events) because the managed projection's delegate cost is dramatic.
`IterateVector`, `GetMany`, and `Map` exercise
WinRT **collections**: the first two walk an `IVector<Int32>` the component owns —
`IterateVector` is the idiomatic per-element loop, `GetMany` bulk-copies in a single call —
while `Map` iterates an `IMap<String, Int32>`, whose iterator is batched the same way but still
pays a per-pair accessor crossing. All three are
dissected in [Collections and iteration](#collections-and-iteration). `Async` awaits a
synchronously-completed `IAsyncOperation<Int32>` and is dissected in
[Async](#async). `Reference` is an `IReference<Int32>` property exercised both ways each
iteration: the getter boxes an `Int32` in the component and unboxes it in the consumer, while the
setter boxes in the consumer and unboxes it in the component — so it stresses boxing in **both
directions** and depends on the component *and* the consumer. It is dissected in
[Boxed values](#boxed-values). `Error` isolates the failure path: the
component's `Next` method does nothing but return the `E_BOUNDS` `HRESULT`, so the loop
measures how each projection turns an ABI error code into its idiomatic error type — a
`Result::Err` in Rust, a thrown `hresult_error` in C++/WinRT, and a thrown managed
exception in C#/WinRT.

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
confirms the component's language makes no difference except on `Error`, `Async`, and
`Reference`. Each consumer
issues the identical sequence of ABI calls and passes each value the natural, idiomatic
way for its language — including the string argument, written as `h!("value")` in Rust,
`L"value"` in C++, and `"value"` in C#. Because the component ignores its inputs and
returns fixed values, the numbers are dominated by projection/ABI cost rather than any
work in the component. Absolute numbers are machine-dependent; the relative shape is the
point.

The high iteration counts deliberately measure **steady-state per-call throughput** — the
marginal cost of each ABI call in a hot loop — so one-time JIT and startup overhead is
amortized away (per-iteration GC and allocation, which scale with the work, are fully
counted). Short-lived workloads where process startup dominates are a different axis this
benchmark does not measure; that is the cost Native AOT targets, covered in
[A note on Native AOT](#a-note-on-native-aot).

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

For every loop except `Error`, C++/WinRT and Rust are both zero-overhead projections that
compile down to direct vtable calls, so they sit far below C#. Between them, Rust leads or
ties every loop: it edges C++ on the pure-ABI calls, ties it on the delegate loops (`Event`,
`AddRemove`) where both projections do equivalent work, and wins outright where it counts —
`IterateVector` (~10×), `Reference` (3×), `Async` (2.2×), and `Error` (2700×) — and it leads C# in
every category, often by orders of magnitude. With the component doing nothing, the pure-ABI loops
(`Int32`, `String`, `Object`, `Cast`) cost tens to low hundreds of milliseconds: a scalar
copy, a fast-pass string marshal, an `AddRef`/`Release` pair, and a `QueryInterface` are all
essentially free. `Create` costs more because it genuinely activates and releases an object
each iteration, and `Event`/`AddRemove` add delegate work — both dissected in
[Delegates and events](#delegates-and-events).

C#/WinRT pays for the managed runtime on every call — runtime-callable-wrapper lookups,
garbage collection, and per-call interop thunks — so even the pure-ABI loops run an order
of magnitude slower than C++/Rust, and the allocating `Create` loop dramatically so.

### Does the component's language matter? The matrix

To check whether "Rust calls Rust" is somehow an unfair advantage, each consumer was
pointed at *both* components in turn (`run.ps1` runs the full grid; the `Lang()` header
confirms which implementation answered). One run at 1,000,000 iterations, consumer→component:

| Metric        | C#→C++ | C#→Rust | C++→C++ | Rust→C++ | C++→Rust | Rust→Rust |
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

For every pure-ABI loop, swapping the component's language changes nothing: each
consumer posts the same numbers whether it calls the Rust or the C++ component (`C++→Rust`
vs `C++→C++`, `Rust→Rust` vs `Rust→C++` are within noise). That is the whole point — the
ABI is a hard vtable boundary with no cross-language inlining, so the callee's language is
invisible to the caller. There is no "same-language" advantage to erase, and Rust's lead
over C#/WinRT is the projection, not the fact that it happened to be calling Rust.

`Error`, `Async`, and `Reference` are the exceptions, because each does real work that depends on
an endpoint's language — originating an error, completing an async operation, or boxing a value. For
`Error` and `Async` that work lives *inside* the component, so the component's language shows through;
`Reference` is the subtler case, boxing on **both** sides of the call, so it depends on the consumer
*and* the component. `Async` is dissected in [Async](#async) and boxing in
[Boxed values](#boxed-values). The delegate loops (`Event`, `AddRemove`) used to differ here too,
but only because the Rust component once hand-rolled a lighter subscriber list; now that both
components use equivalent thread-safe event storage — windows-rs `Event<T>` and cppwinrt
`winrt::event` — they converge to within noise, as [Delegates and events](#delegates-and-events)
shows. `Error` is illuminating in its own right: on
top of each projection's own error cost sits WinRT error *origination* — building an
`IRestrictedErrorInfo` via `RoOriginateLanguageException` — and the matrix shows it is
incurred by whichever endpoint is **C++/WinRT**, on *either* side of the call:

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

### Delegates and events

`Event` raises a WinRT event with one subscriber; `AddRemove` subscribes a fresh delegate and
immediately unsubscribes it each iteration. Both touch a real data structure the component owns —
its subscriber list — so they *could* expose the component's implementation. Both components use
the idiomatic thread-safe storage for their language: windows-rs `Event<T>` and cppwinrt
`winrt::event`. Each takes a copy-on-write snapshot of its delegate array, agile-wraps non-agile
delegates, and raises against a snapshot so a raise never blocks an add or remove. Because the two
implementations are equivalent, these loops turn out **component-invisible**: `Event` costs
~30 ms/1M against either component for either native consumer (`Rust→Rust` `30` vs `Rust→C++` `32`;
`C++→Rust` `30` vs `C++→C++` `33`), and `AddRemove` lands at ~135–145 ms/1M across every native
combo. (An earlier revision hand-rolled a lighter, single-threaded `Vec` in the Rust component,
which made events look ~2.5× cheaper against Rust; switching to `Event<T>` removed that artifact and
the numbers converged — the thread-safety and cross-apartment agility `winrt::event` pays for are
real, and worth matching.)

What these loops *do* expose is the **consumer**. `AddRemove` constructs a fresh WinRT delegate from
a language callback and runs the component's `add`/`remove` every iteration. Natively that is cheap
(~135–145 ns/call: a delegate allocation plus a copy-on-write array edit). C#/WinRT is the outlier
by two orders of magnitude: each subscribe wraps the managed delegate in a runtime-callable wrapper
and each unsubscribe tears it down, which dominates regardless of component — `29,048 ms` at 10M
(~2.9 µs/call) on JIT, and Native AOT is *worse* at `82,277 ms` (~8.2 µs/call), since its leaner
steady-state interop does not offset the per-subscription wrapper churn. Delegate-heavy WinRT code
is one place the managed projection pays dearly; the native projections treat a delegate as little
more than a vtable pointer.

### Collections and iteration

`IterateVector` and `GetMany` walk the same component-owned `IVector<Int32>` two ways. Like
the pure-ABI loops, iteration is **component-invisible** — the matrix shows `Rust→Rust` `1` vs
`Rust→C++` `1` and `C++→Rust` `12` vs `C++→C++` `13`, so the callee's collection storage does
not matter. What matters is the *consumer's* projection, and here the picture inverts the usual
"native projections tie" result: at 1M, idiomatic iteration is sub-millisecond in Rust, ~12 ms in C++,
and ~50 ms in C#. Rust now leads C++ by an order of magnitude, and at 10M the headline is `4`
vs `127`.

The reason is **batching**. A WinRT `IIterator` exposes both per-element access and a bulk
`GetMany`. cppwinrt's range-`for` reads one element per ABI call, so 10M elements cost 10M
vtable round-trips. windows-rs's `for x in &v` now drives a `BufferedIterator` that fills a
small buffer with one `GetMany` call and yields from it, cutting the boundary crossings ~100×
— which is why idiomatic Rust (`4`) lands far under cppwinrt (`127`) and near the explicit
`GetMany` loop. `GetMany` itself is essentially free everywhere native (~0–6 ms); C#/WinRT
still pays per-call interop (~24 ms) but bulk-copying is far cheaper than its per-element loop.

Two fairness points. First, the C++ component returns a `multi_threaded_vector`, the thread-safe
equivalent of windows-rs's stock vector; `single_threaded_vector` would skip locking and unfairly
flatter C++. With both thread-safe, iteration stays consumer-driven, confirming the ABI thesis.
Second, batching is a real ABI-throughput win, not a measurement trick: it is a transparent
windows-rs improvement, so existing `for x in &v` code gets the speedup with no change — the
naive `GetAt`-per-element shape would have tied cppwinrt's `127`.

`Map` walks an `IMap<String, Int32>` the same idiomatic way (`for pair in &map`), and — like
the vector — that loop is **batched**: `IMap<K, V>` implements `IIterable<IKeyValuePair<K, V>>`,
so its iterator exposes `GetMany`, and windows-rs drives it through the same `BufferedIterator`
(bindgen emits the `IntoIterator` for every iterable, maps included, so the speedup is general).
The catch is *what* `GetMany` returns: for `IVector<Int32>` it bulk-copies the `Int32` values
inline, but for a map it hands back a block of `IKeyValuePair` **COM objects** (one `AddRef`
each), and reading every `pair.Value()` is still a per-pair vtable crossing. So batching erases
the iterator-stepping cost but not the per-pair accessor — which is why `Map` stays an order of
magnitude above `IterateVector` (at 10M, Rust `775` vs `4`) yet only modestly ahead of C++
`955` and C#/JIT `1813`, since all three pay the same per-pair `Value()` crossing. It stays
consumer-driven (`Rust→Rust` `74` vs `Rust→C++` `84` at 1M), so Rust again leads. Getting there exposed — and
fixed — a quadratic in windows-rs: the stock map iterator originally located each element with
`map.iter().nth(current)`, an O(n) walk per step, turning a full traversal into O(n²) (a 1M C#
loop took ~11 minutes). Snapshotting the keys/values once at `First()` makes each step O(1);
the same fix applies to `IMapView` and `IObservableMap`, so all stock-map iteration is now
linear with no caller change. cppwinrt instead holds a *live* `std::map` cursor across the ABI
and bumps it O(1) per step, guarding against mutation with a version counter; windows-rs
snapshots because Rust can't safely store a borrowing `BTreeMap` iterator inside the COM
object. Both are linear — neither cheats; the snapshot just trades memory for resilience.

### Async

`Async` awaits an `IAsyncOperation<Int32>` the component completes synchronously: Rust
`.join()`, C++ `.get()`, C# `await` — the analogous blocking wait in each. Because the operation
is already complete when the consumer waits, both native projections take the same short-circuit
path: windows-future's `join` and cppwinrt's `wait_get` each read `Status()`, see `Completed`
rather than `Started`, skip the event/wait entirely, and call `GetResults()` — and both even pay
the same `QueryInterface` to `IAsyncInfo` that `Status()` requires. So the consumer-side wait is
mechanically equivalent in Rust and C++; the difference is **component-side**, which is why
`Async` sits in the matrix's exception set.

The Rust component returns a ready operation via `windows-future`
(`IAsyncOperation::<i32>::ready`), a tiny object holding only an `AtomicBool` and the result:
`Status()` reports `Completed` directly and `GetResults()` clones the value — no coroutine, no
lock, no condition variable. The C++ component `co_return`s, which instantiates cppwinrt's
coroutine promise — a heap-allocated coroutine frame plus a full async state machine (atomic
status, a `slim_mutex`, an agile completed-handler slot, exception/cancel slots) that is run to
completion and torn down each call. That extra machinery is the gap, and the matrix shows it is
driven entirely by the component: swapping the consumer's language leaves it unchanged
(`C++→Rust` `46` ≈ `Rust→Rust` `46`), while swapping the component moves it (`Rust→C++` `94` ≈
`C++→C++` `95`). At 10M the headline figures are Rust `452` and C++ `994` (both against the Rust
component; that consumer gap is run-to-run variance, not a projection difference — the controlled
matrix shows the consumer's language does not move `Async`), while C#/WinRT's await machinery
costs `49,902` (JIT). Native AOT is the worst case at `479,320 ms` (~48 µs/call, 10× JIT) — the
same pattern as `AddRemove`, where AOT's per-call interop overwhelms steady-state throughput.
Rust and C++ are both cheap native projections here; C#/WinRT trails by two orders of magnitude.

### Boxed values

`Reference` is an `IReference<Int32>` property, exercised **both ways** every iteration: the getter
boxes an `Int32` in the component and the consumer unboxes it; the setter boxes in the consumer and
the component unboxes it. A single iteration therefore pays for *two* independent boxes — one on each
side of the ABI — and each side boxes with its own language. The Rust side boxes through the
`windows-reference` crate (`IReference::<i32>::from`), the C++ side through `box_value`, and C#
through its nullable `int?` projection.

Because the two boxes are independent, the matrix cost is simply the **sum of each endpoint's boxing
implementation**, and it is symmetric — `Rust→C++` (`187`) ≈ `C++→Rust` (`188`), since the same two
boxes are paid regardless of who calls whom. Decomposed per side, windows-rs's compact `IReference`
costs ~30 ms/1M and cppwinrt's `box_value` ~155 ms/1M:

| combo       | consumer box (set) | component box (get) | total |
| :---------- | -----------------: | ------------------: | ----: |
| `Rust→Rust` |               ~30  |                ~30  |  `62` |
| `Rust→C++`  |               ~30  |               ~155  | `187` |
| `C++→Rust`  |              ~155  |                ~30  | `188` |
| `C++→C++`   |              ~155  |               ~155  | `324` |

So windows-rs is ~5× cheaper than cppwinrt on *either* side of the boundary — boxing out of the
component and boxing into it — and the advantage compounds when both ends are Rust. The gap is
structural, not incidental. cppwinrt's `box_value(0)` routes through `reference_traits<int32_t>::make`
to `PropertyValue::CreateInt32`, which calls the cached `Windows.Foundation.PropertyValue` activation
factory (`call_factory`) and makes a cross-DLL ABI call into combase, where the OS allocates a
general-purpose stock `IPropertyValue` — an object carrying the discriminated-union machinery for all
~20 scalar and array property types. windows-rs's `IReference::<i32>::from` instead allocates a single
in-process `StockReference` ComObject that holds just the `i32`: no activation factory, no combase
round-trip, no cross-DLL hop, and its `IPropertyValue` accessors resolve the type with a compile-time
`TypeId` match rather than a runtime union. So each Rust box is one small local allocation against
cppwinrt's factory dispatch plus combase allocation. The win is no
longer just the callee's: even calling the same Rust component, the C++ *consumer* pays `2154` at 10M
against Rust's `703`, because boxing *in* is the consumer's cost and cppwinrt's box is the heavier one.

C#/WinRT pays its usual per-call interop on top, and here the consumer-side set-boxing dominates: at
10M against the Rust component it costs `25,166 ms` (~2.5 µs/call for the round trip), and Native AOT
is again the worst case (`141,045 ms`, ~5.6× JIT) — the same `AddRemove`/`Async` pattern where AOT's
per-call interop overwhelms steady-state throughput. So boxing reinforces the thesis from both ends:
the native projections are cheap and windows-rs is the cheapest on each side of the ABI, while the
managed projection pays per call — most steeply under AOT.

### A note on benchmark structure

A reasonable critique of microbenchmarks is that running every scenario in one method can let
the compiler reorder, hoist, or otherwise optimize across scenarios in ways that distort the
result, and that each scenario should instead live in its own non-inlined method with warmup
and cleanup in between. That concern does not apply here, and we verified it empirically.

Every measured operation is an opaque cross-DLL WinRT vtable call. No compiler — rustc, the
C++ compiler, or the C# JIT — can inline into the component, constant-fold the result, or
hoist anything out of the loop across the ABI boundary, so there is nothing for the method
structure to interfere with. To confirm, we restructured the C# consumer so each scenario ran
in its own `[MethodImpl(MethodImplOptions.NoInlining)]` method with a warmup pass and a
`GC.Collect()` between timed loops, then measured it head-to-head against the single-method
form at 10,000,000 iterations:

| Scenario | Single method | Per-scenario method + warmup + GC |
| -------- | ------------: | --------------------------------: |
| Create   |     ~10000 ms |                         ~10300 ms |
| Int32    |       ~49 ms  |                           ~39 ms  |
| String   |      ~1090 ms |                          ~980 ms  |
| Object   |      ~1170 ms |                         ~1090 ms  |
| Cast     |      ~1410 ms |                         ~1320 ms  |
| Error    | ~14000 ms     |                       ~15000 ms   |

Every value is within run-to-run noise, and the order-of-magnitude gap between C# and the
native projections is fully preserved. The one genuinely C#-specific factor — JIT tiering, since
the AOT-native Rust and C++ builds never tier — is negligible at this scale: the tiering and
on-stack-replacement transition spans a few dozen iterations and is lost against billions of
steady-state calls, which is why the explicit warmup changed nothing. The C# gap is real
projection overhead (runtime-callable-wrapper lookups, garbage-collector pressure, interop
thunks), not a measurement artifact. If method structure were the cause it would have to show
up in Rust and C++ too; it does not.

### A note on Native AOT

The `C#/AOT` column publishes the same C# program with
[Native AOT](https://learn.microsoft.com/dotnet/core/deploying/native-aot/)
(`PublishAot`). Native AOT optimizes process *startup*, not steady-state ABI throughput, so
it does not help this benchmark — and across the WinRT ABI it usually *hurts*. At
10,000,000 iterations it is slower than JIT on every loop except `String`, `IterateVector`,
and `GetMany`, and the loops that do real interop work regress sharply: `Async` runs ~9.6×
slower than JIT (`479,320` vs `49,902 ms`) and `Reference` ~5.6× (`141,045` vs
`25,166 ms`), with `AddRemove` ~2.8× and `Cast` ~1.9×. AOT's ahead-of-time interop stubs
lack some of the dynamic runtime's COM-interop fast paths and lean harder on the garbage
collector, so paying JIT's one-time warm-up actually buys cheaper steady-state calls. AOT's
real advantage is process startup, which this steady-state benchmark deliberately amortizes
away (see [Sample results](#sample-results)); it never closes the order-of-magnitude gap to
the C++ or Rust projections, and JIT is the representative C# result here.

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

### Future exploration

Candidate loops not yet implemented, kept here so the rationale is not lost:

- **Struct by value** — pass/return a WinRT struct (e.g. a few-field value type) across the
  ABI to measure blittable-vs-marshaled struct copies.
- **Arrays** — fill/receive a large `Int32[]` to compare bulk array marshaling against the
  per-element vector path.
- **windows-reactor iteration** — profile real WinUI collection iteration now that stock-map
  iteration is linear; confirm the reactor's `for`-based walk beats `Size`+`GetAt`.
- **C# `String` anomaly** — investigate why C#'s `String` loop is comparatively cheap (leaner
  string marshaling) versus its other loops.

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
