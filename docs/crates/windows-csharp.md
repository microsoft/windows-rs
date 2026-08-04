# windows-csharp

`windows-csharp` is an experimental code generator that projects Windows metadata directly into
C#. It uses the same metadata model as windows-rs but emits direct COM and native ABI calls instead
of using the managed-wrapper model employed by C#/WinRT.

The current branch is a credible experimental alternative to CsWinRT for performance-sensitive,
metadata-controlled applications. It is not yet a general replacement, but it is broad enough to
build real Composition and WinUI applications without `WinRT.Runtime.dll`,
`Microsoft.Windows.SDK.NET.dll`, or generated Windows SDK NuGet projections.

## Viability summary

| Question | Assessment |
| --- | --- |
| Can it build real applications? | Yes: Composition Minesweeper and direct WinUI Tic-Tac-Toe |
| Does it cover meaningful WinRT breadth? | Yes: activation, generics, collections, and more |
| Can it project Win32 and native COM? | Yes, for a substantial but incomplete first slice |
| Is the implemented ABI surface well tested? | Yes: it has undergone two deep reviews |
| Is it faster than CsWinRT? | Usually, and often by a large margin |
| Is it as mature or complete as CsWinRT? | No |
| Is it ready for broader technical evaluation? | Yes |
| Is it ready to promise as a supported CsWinRT replacement? | No |

There are no known correctness blockers in the implemented surface. The remaining issues are
primarily productization, coverage boundaries, and explicit lifetime tradeoffs.

## Architecture

`windows-csharp` generates direct ABI C# rather than building on the CsWinRT managed-wrapper model.
A projected WinRT object is a sealed C# owner containing one COM interface pointer. Calls dispatch
through `delegate* unmanaged` vtable entries. Failing HRESULTs become exceptions while preserving
the HRESULT and thread error information.

Two ownership policies are available:

| Policy | Behavior |
| --- | --- |
| Default raw owner | Smallest; requires same-apartment disposal and no call/dispose races |
| `.synchronized()` | Coordinates calls, disposal, apartment access, release, and finalization |

The normal owner is 32 bytes per retained managed object, including the containing array slot. The
synchronized owner is 40 bytes. The default profile achieves its performance by making lifetime
and apartment requirements explicit rather than hiding them behind a larger managed wrapper and
garbage-collected release.

C# cannot encode Rust-style move-only COM ownership. Multiple managed references to the same owner
share its disposed state. The generator uses callback-confined `ref struct` borrowing and
stack-owned QI leases where possible to avoid temporary managed owners.

`BorrowAs` performs one QI and keeps the queried reference in a stack-only lease while a callback
receives a non-owning `Borrowed` view. Several inherited or base-interface calls can share that
lease without exposing a copyable owning pointer.

## Current WinRT coverage

The projection supports:

- runtime classes and interfaces;
- default, custom, and composable activation factories;
- static interfaces;
- inherited method and property forwarding;
- scalars, enums, WinRT Boolean, strings, and objects;
- blittable structs, Boolean-containing structs, and nested String-containing structs;
- delegates and events in both directions;
- deterministic event revokers;
- vectors, vector views, maps, map views, entries, and iteration;
- `IReference<T>`;
- `IAsyncOperation<T>` with native completion handlers rather than polling;
- scalar, Boolean, enum, struct, string, and object arrays;
- parameterized interface IID calculation;
- exact type/member selection with transitive dependency closure;
- deterministic diagnostics for unsupported requested metadata.

String-array inputs create temporary owned HSTRINGs. Object-array inputs hold balanced COM
references. Returned string and object arrays consume each native owner and free the native buffer
on both success and failure paths.

String-bearing structs use generated ABI companions containing HSTRING fields. Input conversion
owns temporary handles through the native call. Return conversion consumes each native-owned
handle once and cleans up unconsumed fields if conversion fails.

The async awaiter reads status once and installs the native
`AsyncOperationCompletedHandler<T>` when the operation is still running. It does not poll.
Registration holds the operation reference needed to survive inline completion, and continuation
dispatch is atomic and one-shot.

Generic runtime classes whose default interface is a closed generic interface derive their IID
from the closed signature. For example, `UIElementCollection` uses the generated
`IVector<UIElement>` IID.

### Remaining WinRT work

- reverse delegate Boolean and array shapes;
- reverse delegate `IReference<T>` and `IAsyncOperation<T>` inputs;
- some reverse non-blittable struct signatures;
- broader `IReference<T>` cases where real metadata requires them;
- idiomatic WinRT `Char` as C# `char` rather than `ushort`;
- generated-name collision handling;
- optional diagnostics for omissions during broad namespace generation.

## Win32 and native COM coverage

The implemented native slice includes:

- namespace-local `Apis` classes using `LibraryImport`;
- native functions and integer constants;
- native pointers and pointer-sized values;
- `BOOL` and `HRESULT` conversion;
- opaque typed handles such as `HWND`, `HANDLE`, and `HDC`;
- scalar and struct `out T` and `ref T` shaping;
- pinned UTF-16 input strings;
- mutable UTF-16 `Span<char>` buffers;
- one-to-one element and byte-counted spans;
- native function-pointer callbacks with metadata calling conventions;
- sequential and explicit-layout records;
- unions and nested anonymous records;
- architecture-specific X86, X64, and Arm64 metadata;
- record packing;
- direct native COM owners with inherited slots;
- native COM interface returns with deterministic ownership;
- raw `[GeneratedComInterface]` companions for complete interfaces;
- Microsoft C++ COM record-return ABI.

The private native signature remains pointer-based. Public wrappers introduce string, span,
`out`, `ref`, HRESULT, BOOL, and ownership shaping only when the metadata establishes a
conservative interpretation.

An opaque Win32 handle is emitted as a blittable `readonly struct` containing one `nint`. Scalar
identifiers such as `COLORREF` and pointer aliases such as `PWSTR` remain scalars or pointers.
No close operation or invalid-value convention is inferred because those policies vary by API.

Native callbacks are emitted as literal `delegate* unmanaged[...]` types. Lifetime and callback
state remain the caller's responsibility and no managed delegate is allocated.

Native COM methods returning records use the Microsoft C++ member ABI:

```csharp
Point result = default;
((delegate* unmanaged<nint, Point*, void>)slot)(self, &result);
return result;
```

The result pointer immediately follows `this`, including for small X86/X64 records and Arm64
homogeneous floating-point aggregates. Scalars, enums, handles, callbacks, and pointers remain
direct returns.

An HRESULT function or method promotes an interface double pointer to an owning return only when
metadata identifies one required output-only trailing retval. In/out, optional, reserved, counted,
success-null-capable, and ambiguous candidates remain raw pointers. On failure, a non-null output
is released before throwing. A required null output on success becomes `E_POINTER`.

### Remaining Win32 work

- ANSI string families;
- returned and two-call string APIs;
- fixed arrays;
- records requiring alignment beyond CLR packing;
- broader COM-interface function parameters;
- shared-count buffer convenience;
- runtime loading policy for APIs unavailable on older Windows;
- API-specific handle ownership and invalid-handle semantics;
- richer generated-COM marshalling.

Optional scalar pointers and uncertain ownership shapes remain raw by design. They should acquire
convenience wrappers only after a concrete API establishes a reusable policy.

## Selection and diagnostics

Namespace filtering projects every supported item in a namespace:

```rust,no_run
windows_csharp::builder()
    .input("component.winmd")
    .input_default()
    .filter("Component")
    .output("Component.cs")
    .write()
    .unwrap();
```

Exact selection projects named roots and their transitive dependencies:

```rust,no_run
windows_csharp::builder()
    .input("component.winmd")
    .input("Windows.Foundation.winmd")
    .select("Component.Widget")
    .member("Component.Gadget", "Value")
    .function("Component.GetWidget")
    .constant("Component.WIDGET_DEFAULT")
    .output("Selected.cs")
    .write()
    .unwrap();
```

Exact selection reports deterministic errors for:

- missing items;
- selecting an item as the wrong metadata kind;
- unsupported requested signatures or shapes;
- unavailable architecture variants;
- malformed parameter sequences;
- unsupported required dependencies;
- unsafe variadic exports.

Member selection preserves metadata vtable slots: the complete interface is numbered before
unselected members are removed.

Broad namespace generation retains supported-subset behavior and may omit incidental unsupported
items. Applications intended as coverage proofs should prefer exact selection.

## Performance relative to CsWinRT

### Language benchmark

`crates/samples/test_bench` runs cppwinrt, windows-rs, windows-csharp, and CsWinRT 2 against the
same native component. The following numbers are medians of three 10,000,000-iteration x64 runs.

| Metric | cppwinrt | windows-rs | windows-csharp | CsWinRT 2 |
| --- | ---: | ---: | ---: | ---: |
| Create (ms) | 626 | 559 | 715 | 10,858 |
| Int32 (ms) | 24 | 22 | 39 | 65 |
| String (ms) | 299 | 293 | 493 | 1,596 |
| Add (ms) | 14 | 13 | 22 | 30 |
| Cast (ms) | 154 | 144 | 210 | 34 |
| Cast owned (ms) | 158 | 147 | 239 | 188 |
| Interface (ms) | 14 | 12 | 20 | 45 |
| Object (ms) | 135 | 136 | 219 | 1,225 |
| Event (ms) | 232 | 230 | 293 | 931 |
| Add/remove (ms) | 367 | 802 | 440 | 25,614 |
| Vector (ms) | 121 | 119 | 136 | 254 |
| Iterate vector (ms) | 1,236 | 129 | 149 | 4,121 |
| GetMany (ms) | 2 | 2 | 5 | 177 |
| Map (ms) | 623 | 575 | 942 | 16,739 |
| Lookup (ms) | 198 | 187 | 199 | 298 |
| Vector view (ms) | 17 | 24 | 21 | 148 |
| Map view (ms) | 127 | 143 | 155 | 240 |
| Reference (ms) | 1,831 | 684 | 884 | 24,966 |
| Async (ms) | 465 | 473 | 648 | 56,271 |
| Error (ms) | 14,392 | 6 | 1,361 | 2,892 |
| Live owner memory | 8 bytes | 8 bytes | 32 bytes | 296 bytes |

All consumers leave zero native objects above baseline.

The benchmark establishes:

- creation is about 15 times faster than CsWinRT;
- retained owners use about one ninth of the managed memory;
- ordinary owner calls remain close to direct ABI calls;
- activation, strings, events, collections, references, async, and exceptions are materially
  faster than CsWinRT in this workload;
- bulk collection operations must remain direct rather than falling through element adapters;
- CsWinRT can be faster for inheritance-like casts because it may reuse a cached wrapper, while
  `As<T>()` performs QI and creates an independently disposable owner.

The WinUI comparisons currently use CsWinRT 2. A comparable CsWinRT 3 WinUI projection is not
available, so these measurements do not predict a future CsWinRT 3 implementation.

### Direct WinUI benchmark

`crates/samples/test_winui_bench` builds equivalent visible WinUI consumers. The table is the
median of six headless x64 runs.

| Metric | windows-rs | windows-csharp | CsWinRT |
| --- | ---: | ---: | ---: |
| Process to `Main` (ms) | 136.8 | 186.1 | 195.2 |
| Process to window tree (ms) | 208.1 | 301.7 | 346.9 |
| Working set (MiB) | 98.7 | 128.1 | 148.7 |
| Create and set text (ns) | 877 | 1,072 | 2,098 |
| Repeated text update (ns) | 214 | 294 | 399 |
| Projection-specific cast (ns) | 14.1 | 58.5 | 21.9 |
| Build retained 100-node tree (us) | 151.8 | 173.7 | 389.4 |
| Batch update per child (ns) | 218 | 320 | 325 |
| Clear and reattach per child (ns) | 636 | 798 | 1,394 |
| Event registration and revocation (ns) | 352 | 384 | 4,272 |
| Teardown retained tree (us) | 77.8 | 86.4 | 242.3 |

For the retained 4,900-control stress tree:

| Metric | windows-rs | windows-csharp | CsWinRT |
| --- | ---: | ---: | ---: |
| Build (ms) | 18.02 | 20.59 | 41.37 |
| Managed allocation (MiB) | 0.12 | 0.34 | 3.46 |
| Working set (MiB) | 105.99 | 135.36 | 160.98 |
| Update 10% (ms) | 0.12 | 0.16 | 0.17 |
| Update 50% (ms) | 0.61 | 0.79 | 1.03 |
| Update 100% (ms) | 1.32 | 1.69 | 2.20 |
| Managed allocation per update | 0 B | 0 B | 0 B |

windows-csharp builds the tree about 1.9 times faster than CsWinRT and is 1.1-1.3 times faster
across the update sweep. windows-rs remains about 20-28 percent faster than windows-csharp.

The sustained update track measures a deterministic 10 percent text and foreground update every
33 ms:

| Metric | windows-rs | windows-csharp | CsWinRT |
| --- | ---: | ---: | ---: |
| Update time per tick (ms) | 1.15 | 1.29 | 1.31 |
| Managed allocation per tick | 0 B | 0 B | 0 B |
| Working set (MiB) | 173.88 | 203.32 | 221.64 |

With 100-cell remove and append churn, windows-csharp and CsWinRT both take about 3.46 ms per tick.
windows-csharp remains allocation-free while CsWinRT allocates 15.63 KiB per tick through its
managed collection path.

Allocation counters are runtime-specific. Rust counts its global allocator and .NET counts the
managed heap; neither measures all native WinUI and COM allocation. Working set is the useful
process-level comparison.

## Generator scale

The generator was measured through 395 closed types per generic shape. At that breadth:

- generated source is 6.78 MB;
- the resulting assembly is about 2.01 MiB;
- filtered and exact generation take about 146 and 164 ms;
- .NET compilation takes about 8 seconds;
- repeated vector, map, and async fake-vtable calls allocate 0 managed bytes;
- hot generic operations remain flat rather than growing with the projected type count.

At 256 map pairs, generation changes from growing `typeof(T)` chains to static closed-pair
selection and direct managed function pointers. At 395 types this reduced:

- total source from 8.73 MB to 6.78 MB;
- generator time from 322 ms to 146 ms;
- assembly size from 2.46 MiB to 2.01 MiB;
- the last object-map lookup from 185.1 ns to 8.3 ns.

The remaining scale risk is cold generic initialization. Touching late object-map instantiations
can take about 55 ms at the tested maximum. Revisit this only if a real projection exceeds the
tested breadth or exposes a cold-start regression.

Generation remains under 3 percent of the measured .NET build. Repeated index scans, namespace
grouping, and formatting allocations are therefore deferred until profiling attributes a material
end-to-end cost to them.

## Real application samples

### Composition Minesweeper

`crates/samples/csharp/minesweeper` is a complete 16x16 Composition application using:

- a generated Win32 HWND and message pump;
- dispatcher queue creation;
- desktop composition interop;
- Composition visual trees, shapes, brushes, and animations;
- mouse input and resize handling;
- deterministic COM ownership.

Run it from the repository root:

```powershell
dotnet run --project crates\samples\csharp\minesweeper\minesweeper.csproj `
    -c Release -p:Platform=x64
```

MSBuild invokes a thin Cargo build-script package to regenerate the committed `Windows.cs`. The
output has no CsWinRT or Windows SDK projection runtime dependency.

A local clean Release build took 3.11 seconds. The bounded visible smoke process used about
32.5 MiB working set.

The sample found and fixed generator defects involving generic validation, empty factories,
forwarded overloads, and owning native COM outputs.

### Direct WinUI Tic-Tac-Toe

`crates/samples/csharp/tictactoe` directly starts WinUI 3 using the same pinned metadata and runtime
setup as windows-reactor.

```powershell
dotnet run --project crates\samples\csharp\tictactoe\tictactoe.csproj `
    -c Release -p:Platform=x64
```

It proves:

- `Application.Start`;
- framework-dependent Windows App Runtime bootstrap;
- composable WinUI controls;
- a programmatic control tree;
- dispatcher-queued startup;
- object-valued content and generic UI collections;
- direct events and revokers;
- deterministic STA teardown;
- generated Win32 bootstrap and COM calls.

The smoke mode programmatically plays a win and a draw, resets the game, closes the window, and
uses a watchdog to turn startup or shutdown hangs into a failing process.

Both samples use a thin Cargo build-script package for metadata generation and MSBuild integration.
This works for repository development but is not the final end-user distribution model.

## Improvements to the wider windows-rs toolchain

The windows-csharp work exposed shared metadata and ABI issues. The resulting fixes are independent
of the C# experiment and remain useful if the crate is later removed.

### windows-metadata

`MethodDef::params_by_sequence` now:

- associates parameter rows through one-based ECMA-335 `Param.Sequence`;
- exposes Sequence 0 as the return pseudo-row;
- preserves absent and sparse rows;
- rejects duplicate and out-of-range sequences;
- retains physical row iteration for lossless metadata merge and remap operations.

Shared policy-free parameter facts now report:

- `Input`;
- `Output`;
- `InputOutput`;
- `Unspecified`;
- Optional;
- Reserved;
- explicit RetVal.

windows-csharp, windows-bindgen, and windows-rdl consume these facts while retaining their
language-specific projection policy.

`Index::new_for_architecture` filters types, functions, constants, nested types, namespace
existence, and assembly lookup for X86, X64, or Arm64 metadata.

### windows-bindgen and generated windows APIs

The Rust projection now:

- associates parameters by Sequence rather than physical table order;
- preserves In+Out as output-capable;
- applies conservative retval selection;
- leaves Optional, Reserved, counted, and ambiguous pointers as parameters;
- validates signed buffer relationships before indexing parameters;
- falls back to raw pointer/count signatures for malformed relationships;
- omits variadic functions from rich and minimal projections.

The generated `windows` crate consequently removes 31 fixed-prefix wrappers for variadic native
functions. Those wrappers could not forward the unknown tail and represented a different function
from the native declaration. Affected families include Authz, tracing, RPC, SetupAPI, shell, ICU,
VFW, and user32. Raw `windows-sys` declarations remain where stable Rust can represent the native
calling convention.

This is a correctness improvement and a public API removal. It should receive explicit
compatibility review.

### windows-rdl

RDL now covers and tests:

- sequence-correct parameter and return pseudo-row writing;
- input, output, and input-output attributes;
- optional and retval attributes;
- `iid_is`;
- count and size relationships;
- reserved, noreturn, scoped, and encoding attributes;
- sparse parameter rows;
- canonical `#[in]` spelling.

RDL rejects mixed-constness pointer chains such as `*mut *const T` because its metadata type model
stores one constness for a pointer run. Rejecting that spelling avoids silently writing different
metadata.

A literal `Unspecified` parameter direction still cannot round-trip because omitted direction
uses RDL's type-based authoring default. Fixing that requires an RDL syntax change.

### windows-clang

The clang-to-RDL pipeline normalizes pointer runs into the shape RDL can represent, preserving the
innermost qualifier for each run. This keeps generated SDK RDL writable while direct RDL authoring
rejects mixed constness.

This is a deliberate boundary compromise: per-pointer-level constness is lost for mixed
non-parameter C pointer runs. The normalization should remain specific to clang output rather than
becoming general metadata behavior.

### Review fixes

The branch-wide review fixed:

- synchronized map iterator double-release on constructor failure;
- object-key map-entry owner leakage when value retrieval failed;
- native COM records returned through the wrong CLR aggregate ABI;
- promotion of optional, in/out, reserved, counted, or ambiguous COM outputs;
- non-null COM outputs leaked on failed HRESULT;
- required success-null COM outputs creating invalid owners;
- closed-generic WinUI default-interface IID derivation;
- managed benchmark architecture mismatches.

## Tests and validation

`crates/tests/libs/csharp` contains metadata fixtures, generated C# goldens, generated-source
compilation, runtime fake-vtable tests, real Windows API tests, and a Rust WinRT component used for
managed round trips.

Coverage includes:

- activation, methods, properties, inheritance, and exact selection;
- strings, objects, delegates, events, collections, async, and arrays;
- reverse delegate ownership and failure cleanup;
- raw and synchronized owner behavior;
- real Win32 string, span, handle, callback, and COM calls;
- fake native COM record returns and owning output paths;
- architecture selection, unions, anonymous records, and packing;
- generated native COM implementations;
- generated WinUI slice compilation and visible application smoke tests;
- generic breadth and hot-path allocation measurements.

The branch-wide review passed the X64 and X86 paths locally. Arm64 is covered by the repository's
native Arm64 CI runner; the local X64 environment lacked the Arm64 MSVC runtime libraries needed
for cross-linking.

## Productization gaps

The largest viability gap is distribution rather than ABI implementation:

- the crate remains version `0.0.0`;
- consumers currently need a Rust/Cargo generation step;
- there is no standalone packaged generator;
- there is no NuGet/MSBuild package comparable to CsWinRT tooling;
- compatibility and versioning policy are not defined;
- generated API naming still needs collision handling;
- the recommendation between raw and synchronized ownership needs to be part of the public
  product contract.

The next bar for a broadly supported CsWinRT alternative is packaging, stable generated API design,
name-collision handling, broader Win32 and string coverage, and evaluation by applications outside
this repository. Another internal optimization round is not the priority.

## Current recommendation

The shared metadata, RDL, buffer-validation, native COM ABI, and variadic correctness fixes are
independently valuable to windows-rs and should remain even if windows-csharp stays experimental.

`windows-csharp` is ready for technical evaluation and real application experiments. Its
performance case against CsWinRT 2 is strong, its ownership and ABI design is well tested, and the
two applications demonstrate that it is more than a benchmark projection.

It should currently be described as:

> An experimental direct Windows metadata projection for C# that trades CsWinRT's mature managed
> wrapper ecosystem for deterministic ownership, smaller wrappers, direct ABI calls, and much
> lower allocation.
