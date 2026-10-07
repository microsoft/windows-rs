# windows-clang2

`windows-clang2` is an unpublished, side-by-side prototype for replacing `windows-clang`. This
slice implements native capture, checked resolution, and a narrow RDL projection. It does not
replace any production generator.

See the [crate readme](../../crates/libs/clang2/readme.md) for a small API example.

## Current pipeline

```text
Inputs + compiler arguments + named roots
    -> libclang translation units and declaration index
    -> owned native dependency graph, including cross-TU observations
    -> checked native groups, completions, and annotation evidence
    -> closed projection plan or diagnostic
    -> RDL
```

| Module | Responsibility |
| --- | --- |
| `capture.rs` | Owns libclang handles, indexes declarations, and captures selected evidence. |
| `native.rs` | Private native model with explicit typedef edges and incomplete/unavailable states. |
| `validate.rs` | Checks available evidence with an iterative node-pair worklist. |
| `project.rs` | Applies explicit projection policy and renders only planned items. |
| `lib.rs` | Experimental input, snapshot, report, and diagnostic inspection API. |

There is no dependency on `windows-clang` in the library. The comparison test crate depends on both.
The native model is private so the prototype can change it without maintaining a second public
projection model.

## Root selection and identity

Callers select qualified native names, such as `API::Packet`. Capture indexes the inputs before
decoding selected native dependencies and their matching observations. Roots can be
functions, records, aliases, callbacks, enums, variables, or selected object-like macros. Unknown
roots are errors. Capture supports more kinds than the current projection.

Canonical cursors identify entities within one TU. Each source declaration remains a separate
observation, with its own written dependencies, parameter names, and annotation context. A redundant
declaration cannot replace earlier evidence. Original USRs are retained and find cross-TU
candidates, but do not establish agreement.

Typedef USRs include source filenames even for namespace-level aliases. Alias candidates therefore
use their qualified native names. Internal-linkage and anonymous-namespace entities stay TU-local.
This is candidate matching, not permission to accept different underlying types.

The current comparison is conservative: named type identity, native scalar kind, qualifiers,
reference kind, and callable shape must agree. Alias declarations and callable signatures also
retain canonical types so spelling differences do not require identical typedef paths. Written
dependencies from every observation are checked independently. This is not a general C++ ODR
checker or ABI-equivalence solver. Anonymous-type identity across different files is not inferred.

## Captured evidence

| Surface | Evidence |
| --- | --- |
| Builtins | Native kind, size, alignment, and qualifiers. |
| Indirection | Typedef edges, pointer levels, and distinct lvalue/rvalue references. |
| Arrays | Constant extent or explicitly incomplete extent. |
| Records | Kind, completeness, layout, fields, bit widths, simple bases, and method signatures. |
| Enums | Completeness, scopedness, underlying type, and member values. |
| Callables | Prototype kind, calling convention, exception specification, result, and parameter types. |
| Parameters | Names, annotate strings, original callable context, and annotation locations. |
| Variables | Native type and supported integer/floating initializer values, or an explicit unavailable value. |
| Provenance | Input name, compiler arguments, resolved target, USR, and spelling location. |

Annotation comparison binds parameter identifiers in argument expressions to their original
positions. Missing annotation evidence does not contradict present evidence, but two nonempty,
different sets for the same parameter conflict. Evidence is combined across all observations, not
just compared against one anchor. This tokenizer is not a general annotation-expression parser.
Nested callback annotations and source-comment annotations are outside the current slice.

Method evidence includes virtual/pure/static/const flags and reference qualifiers. Virtual bases,
multiple or data-bearing inheritance, constructors/destructors, anonymous aggregate members,
template specializations, member pointers, and unsupported initializer expressions are reported as
unavailable evidence. Non-ABI member templates are not modeled. Available record fields remain in
the graph when another member is unsupported.

Selected object-like macros use C++ expression probes. Capture reads the initializer's written type,
not the probe's deduced `auto` type, so a direct or indirect interface-pointer cast keeps its typedef
chain. A separate integer probe evaluates pointer bits when needed. Function-like macros, arbitrary
macro expressions, discovery/export policy, and poison-expression recovery are outside this slice.

## Agreement and completion

Every captured declaration is checked for unavailable evidence, including graphs with only one TU.
No unsupported type is accepted merely because another declaration has the same unsupported reason.

All matched complete definitions must agree. Incomplete observations keep their available facts,
including enum representation, and can refer to a checked completion in another TU. An unrelated,
unselected declaration is not admitted merely because it conflicts elsewhere.
UUID evidence is retained on forward record declarations. All supplied UUIDs in a native group
must agree, including when the chosen complete definition has no UUID.

The report separately lists records and enums with no captured completion. A successful report with
an incomplete list is not proof of complete native definitions. Variables without initializers
retain that absence; it does not override or hide contradictory observed constant values.

Comparisons expand each new declaration pair once, check their local facts, and queue their type
dependencies. Recursive assumptions are published to the local completed-pair cache only after the
whole comparison succeeds. The cache is confined to one validation call; failed validation does not
leave state on the snapshot.

`Snapshot::resolve` returns a checked `Resolved` view containing the representative for every
observation, all groups, and merged annotation evidence. `Snapshot::validate` returns just the work
report. Projection consumes the checked view rather than repeating candidate resolution.

Captured data owns no libclang pointers. Validation and diagnostic inspection work after the
translation units and loader guard are dropped. An existing caller-owned thread-local loader
registration is preserved.

## Projection boundaries

`Resolved::project` accepts a destination namespace, an optional import library, and explicit
external type bindings. It returns an owned `Plan`; `Plan::rdl` needs neither the snapshot nor
libclang. The renderer performs no native type classification or dependency discovery.

| Surface | Current policy |
| --- | --- |
| Records | Ordinary, nonempty structs; verify field offsets, final size, and alignment. |
| Local dependencies | Schedule checked complete records; unsupported dependencies fail the plan. |
| Functions | Fixed prototypes, supported Windows calling conventions, compiler link names. |
| Parameters | Positional names for functions, native names for methods; supported SAL below. |
| External value types | Explicit native record/enum bindings; no assumed by-value layout. |
| External scalar typedefs | Explicit value bindings; retain the checked native scalar representation and layout. |
| External interfaces | Bind a native record; consume exactly one native pointer/reference level. |
| Local interfaces | UUID-bearing, fieldless records with pure virtual system-ABI methods and at most one base. |
| Aliases | Preserve explicitly bound scalar contracts; otherwise peel at uses. Same-name tag/typedef roots schedule one record or interface definition. |
| Constants | Supported scalar and raw pointer values; omit interface values with a reason. |
| Raw pointers | Collapse mixed mutability to const if any raw-pointer level is const. |

External bindings are trusted metadata contracts, not native ABI proof. All captured native
dependencies must agree before an external binding can suppress local output. Metadata readers
still need the referenced WinMD files when compiling RDL. Scalar typedefs may bind to external
metadata value types through `ProjectionOptions::references`. The native canonical type must be a
supported non-void scalar with matching builtin layout; the caller guarantees the external type has
that same scalar representation. Pointer, array, and other arbitrary alias bindings are unsupported.

Callable projection retains written typedef paths, not just canonical ABI types. For example,
explicitly binding native `HRESULT` to `Windows.Win32.Foundation.HRESULT` preserves error and COM
query-result semantics in generated Rust; unbound `long` remains `i32`. The planner has no built-in
Windows typedef names. Every callable observation and alias chain must agree on projected contracts.
A bound typedef versus its unbound primitive is a projection conflict even when native resolution
accepts them as ABI-compatible. Successful alias projections are cached within one builder.
Destination namespaces with multiple segments render as nested RDL modules.

Functions and methods share annotation lowering. `_In_`, `_Out_`, and `_Inout_` retain direction.
`_In_opt_` adds input optionality for pointer parameters. `_Out_opt_` and `_Inout_opt_` require
writable pointers and preserve output/inout direction with `#[opt]`. `_COM_Outptr_` requires a writable
`void**` or interface output pointer. A `void**` receives `#[out] #[iid_is]` (the metadata
`ComOutPtrAttribute`); typed interface outputs retain their interface type and receive `#[out]`.

Buffer annotations preserve their direction and length units:

| Annotation family | Length metadata |
| --- | --- |
| `_In_reads_`, `_Out_writes_`, `_Inout_updates_` | `#[len_param(index)]` or `#[len_const(count)]` |
| `_In_reads_bytes_`, `_Out_writes_bytes_`, `_Inout_updates_bytes_` | `#[size_param(index)]` |

Each of these six buffer annotations also supports its `_opt_` suffix, such as
`_In_reads_opt_` or `_Out_writes_bytes_opt_`. Optionality applies only to the buffer parameter;
its count parameter is not made optional. Length and direction rules are unchanged.

Resolution binds length parameter names using each annotation's original declaration context.
Projection uses those zero-based positions, not the representative declaration's parameter names;
method positions exclude `self`. A referenced count must be a by-value integer parameter, and its
index must fit `i16`. Constant element counts are decimal literals from zero through `i32::MAX`.
Output buffers must be writable. Element-counted `void*` buffers, indirect or arithmetic counts,
constant byte counts, non-decimal literals, and multiple length annotations are errors.
Other annotations remain errors, including unsupported COM output variants.
Compiling length attributes to WinMD requires the standard metadata attribute definitions, supplied
by `windows_rdl::Reader::reference_default()` in the fixtures.

Scalar string annotations `_In_z_`, `_In_opt_z_`, `_Out_z_`, and `_Inout_z_` require explicit
`ProjectionOptions::string_references`. Each binding is a trusted pointer-sized metadata value
type, supplied by the caller's reference WinMD. The planner never invents local aliases or assumes
a Windows metadata namespace. Missing bindings, interface bindings, and malformed names are errors.

| Native single-pointer target | Mutable binding | Const binding |
| --- | --- | --- |
| Signed/unsigned 8-bit character | `StringKind::Ansi` | `StringKind::AnsiConst` |
| Unsigned 16-bit character, including Windows `wchar_t` | `StringKind::Wide` | `StringKind::WideConst` |

For example, bind `StringKind::WideConst` to a `TypeReference` naming the caller's `PCWSTR` with
`ReferenceKind::Value`. Its pointer layout is part of that trusted contract, not inferred from an
external record definition. Native constness and SAL direction remain independent: `_In_z_ char*`
uses the mutable binding with input direction. Output/inout strings require writable pointers.
Other character widths and pointer depths fail projection. Unannotated pointers are unchanged;
typedef spelling alone does not establish termination. Counted `_z_` forms remain unsupported.

Local COM methods must be non-static, non-const, and non-ref-qualified. Their calling convention
must match the RDL system ABI: stdcall for x86, the platform convention for x64 and ARM64.
Inheritance is represented by a base-interface reference, with declared method order preserved.
An interface parameter consumes exactly one native pointer/reference level. By-value record results
are rejected for COM methods on all targets until the downstream aggregate-return ABI is covered.
Record pointers and free-function record results remain supported.

Packing, explicit alignment, bitfield projection, array projection, callback emission, and local enum
emission remain outside this slice. Qualified native names can select roots, but local output names
currently require unqualified identifiers. UUIDs decode from `__declspec(uuid(...))` only (no
`GUID`-typed value decoding yet). General SAL lowering, MIDL recovery, WinRT mapping, header
ownership, and export-name policy are not implemented.

## Evaluation

```powershell
cargo test -p windows-clang2 -p test_clang2
cargo clippy -p windows-clang2 -p test_clang2 --all-targets
cargo fmt -p windows-clang2 -p test_clang2
cargo test -p test_clang2 --test abi --test com --target i686-pc-windows-msvc
```

Inspect a real file or fixture:

```powershell
cargo run -p windows-clang2 --example inspect -- Native `
    crates\tests\libs\clang2\input\native.h -- -x c++ --target=x86_64-pc-windows-msvc
```

The example provisions the shared pinned libclang and prints the native graph followed by the
agreement report. The library itself does not install or configure dependencies.

Dedicated headers live in `crates/tests/libs/clang2/input`. The native evidence golden specifies
its target environment version so host Visual Studio versions do not change its target line.
Update it only when the model change is intentional:

```powershell
$env:UPDATE_EXPECT = "1"
cargo test -p test_clang2 native_evidence_golden
Remove-Item Env:UPDATE_EXPECT
```

Coverage includes native distinctions erased by the legacy projection model, all selected root
categories, alias chains, non-root completions, contradictory definitions, incomplete enums,
constant observations, qualifiers, calling conventions, and unavailable evidence. Shared graphs
at 32, 64, and 128 levels assert exact declaration-pair expansion counts. Target capture runs
for x64, x86, and arm64. Legacy extraction is also exercised in the same test process.

Projection fixtures compare RDL and read generated WinMD to assert external value/interface kinds,
pointer depth, constants, and parameter directions. Coverage includes the external-reference
conflict beneath non-function roots, indirect interface constants, checked completions, redeclaration
permutations, an externally bound `IUnknown`, and a synthetic local interface pair. The
`sdk_interfaces.h` fixture selects `Use` and emits the pinned SDK's `IUnknown` and `IClassFactory`
locally on x86, x64, and ARM64. It checks IIDs, base interfaces, method order, signatures, pointer
depth, optionality, and COM output attributes. Only the `_GUID` value record is externally bound;
neither interface is substituted. Negative controls cover UUID conflicts, incompatible method
calling conventions, and invalid COM output types.

`sal_buffers.h` covers element and byte relationships for functions and methods, forward parameter
references, direction, and constant count bounds. The redeclaration fixture verifies that renaming
a count does not lose its original binding. `sdk_buffers.h` projects `BCryptHashData` and
`BCryptGenRandom` with byte counts referencing their third parameter. `BCryptDeriveKeyPBKDF2`
adds two optional input buffers with independent byte counts and a required output buffer.
The synthetic buffer matrix covers required and optional forms for functions and COM methods.
Conflicting optionality across redeclarations remains an error even when another declaration has
no annotations.

`strings.h` covers all four string binding kinds, typedef chains, functions, COM methods,
direction, optionality, and raw pointer controls. `sdk_strings.h` projects the SDK's
`WinHttpTimeToSystemTime` with an explicit const-wide binding and a local `SYSTEMTIME` record.

The SDK fixtures force-include `specstrings.h` before the shared `tool-win32` SAL capture shim.
`specstrings.h` redefines COM SAL macros, so loading it after the shim can erase annotation evidence.
Capture records compiler annotation attributes, not arbitrary SAL spelling or MIDL comments;
callers must provide the annotation shim in the correct header order.

The workspace test job includes both crates on supported hosts. Like the existing clang tests,
frontend tests are excluded from the x86 test process because the pinned runtime has no x86 DLL.
An x86 ABI-only CI step runs capture and generation in the x64 build script, then executes the x86
fixtures without loading libclang in that process. Native x64 and ARM64 jobs run the ABI fixtures as
part of the ordinary package tests.

### Executable ABI fixture

The MSVC build script captures `input/abi.h` for Cargo's target, projects RDL, compiles WinMD, and
generates Rust with `windows-bindgen --flat --sys --extern`. It independently compiles `input/abi.cpp`
with the target C++ compiler. All generated artifacts stay in `OUT_DIR`.

`tests/abi.rs` compares record size, alignment, and every field offset against C++. It exercises
by-value aggregate arguments and returns through a C-linkage function, inherited vtable calls from
Rust into C++, and C++ virtual calls into a stateful Rust object using the generated vtable type.
Integer/floating-point results, `this`, buffer mutations, and call counts are checked. Repeated calls
exercise the x86 stack-cleanup convention. `--extern` links the static fixture, so this does not test
DLL export lookup or decorated import-name policy.

An aggregate-returning virtual-method experiment exposed a downstream limit:
`windows-bindgen`'s `CppMethod::write_abi` emits an explicit result pointer with a void return, and
`CppMethod::write_upcall` only writes that result. MSVC x86 callers can read the returned aggregate
through the result-buffer address in EAX; the generated signature does not return that address.
The reverse call failed even though Rust-to-C++ dispatch passed. `com_record_result.h` now checks
explicit rejection for small/large records and aliases on all three targets, with record-pointer
returns as the positive control. Fixing the downstream ABI requires separate bindgen coverage and
regeneration; the prototype does not work around it with a handwritten vtable signature.

### COM ownership fixture

`input/com.h` includes the pinned SDK's `unknwnbase.h`. The build script captures and locally projects
`IUnknown` and `IClassFactory`, with explicit external `GUID` and scalar `HRESULT` contracts from
`input/com_reference.rdl`. Normal bindgen output supplies the high-level `IClassFactory` wrapper;
bindgen maps the standard `IUnknown` metadata identity to `windows_core::IUnknown`. A separate raw
binding bootstraps a native factory owned by the test. No high-level wrapper or COM vtable is
handwritten in Rust.

`input/com.cpp` implements the SDK interfaces with per-object counters held by the caller.
`tests/com.rs` exercises the generated `CreateInstance<T>` wrapper and core interface operations:

| Case | Assertion |
| --- | --- |
| Clone and drop | Exact AddRef/Release counts; final release destroys each object once. |
| Successful QueryInterface | Correct requested IID, one new reference, and controlling-unknown identity. |
| Unsupported QueryInterface | `E_NOINTERFACE`, no new reference, and no premature release. |
| Successful CreateInstance | One owned output reference, independent of the factory lifetime. |
| Unsupported output IID | Error with null output, no child allocation or owned wrapper. |
| Non-null aggregation argument | `CLASS_E_NOAGGREGATION`; passing the outer object borrows its reference. |

These cases execute on x64 and x86 in debug and release. The fixture uses SDK declarations with an
in-process test implementation, not Windows COM activation, apartments, marshaling, or a registered
server. `scalar_references.h` also covers scalar contracts through alias chains, fields, pointers,
function/method signatures, invalid binding shapes, and order-independent redeclaration conflicts.

### Multi-TU review and scale probe

The focused review covers observation selection, typedef projection agreement, forward UUIDs,
same-name tag roots, and the native-comparison and projection caches. It is not a full production
audit. Counterexamples confirmed two defects: forward UUID evidence was discarded, and selecting
an SDK interface by name also selected its same-name typedef as a separate output item. Capture now
retains forward UUIDs, resolution checks all supplied UUIDs, and tag/typedef roots share one output.

The suspected field-selection gap did not require another projection pass: native validation
rejects different written field-type identities, while alias agreement checks the underlying
canonical evidence. Callable projection separately checks written semantic typedef contracts
because native callable comparison uses canonical types. The regression matrix changes both
input order and filename order; reversing inputs alone cannot challenge a filename-sorted capture.

`tests/webview.rs` parses the pinned `WebView2.h` and `WebView2Interop.h` as separate TUs. The
supported roots are `ICoreWebView2Deferral`, `ICoreWebView2StringCollection`, and
`ICoreWebView2HttpHeadersCollectionIterator`. Golden RDL and semantic metadata checks cover both
filename orders. A negative case selects `ICoreWebView2Interop2`: its `ICoreWebView2` dependency
reaches `tagVARIANT` and fails native resolution because anonymous aggregate capture is unsupported.
An external binding must not suppress that unavailable native evidence.

The `test_clang2` example `webview` runs either frontend on these same pinned inputs and prints
phase timings, work counts, interface GUIDs, bases, signatures, and parameter directions. It writes
RDL, WinMD, and generated Rust under `OUT_DIR`. Root selection accepts `1`, `3`, or `interop`;
TU pairs repeat the main and interop inputs:

```powershell
cargo run -p test_clang2 --example webview --release -- new 1 3
cargo run -p test_clang2 --example webview --release -- old 1 3
cargo run -p test_clang2 --example webview --release -- new 4 3
cargo run -p test_clang2 --example webview --release -- new 1 interop
```

The final command is an expected failure, not a fallback to legacy extraction. Both generated
three-root Rust wrapper sets were separately type-checked against `windows-core` on x64. Comparing
the metadata establishes matching IIDs, bases, method order, HRESULTs, and scalar outputs.
String outputs differ: legacy emits `PWSTR` values; the rewrite retains raw UTF-16 pointers.
This is an explicit support gap in string/MIDL semantics, not wrapper parity.

One local Windows x64 release run per cell, with cached packages, measured the following for three
selected roots. Peak working set was sampled from each dedicated process, including metadata and
binding generation; it is not a phase-specific allocation measurement. Timings exclude compilation
of the example, and capture timings exclude package setup:

| TUs | Rewrite capture | Rewrite resolve / project | Rewrite peak MiB | Legacy capture / emit | Legacy peak MiB |
| --- | --- | --- | --- | --- | --- |
| 2 | 647 ms | 78 / 46 us | 185 | 7568 / 274 ms | 379 |
| 4 | 1236 ms | 107 / 57 us | 333 | 15413 / 575 ms | 643 |
| 8 | 2988 ms | 165 / 81 us | 635 | 29819 / 1166 ms | 1189 |

The one-root runs had similar capture cost and memory. Rewrite groups stayed at 17 for the
three-root case; observations grew 48 -> 96 -> 192 and declaration comparisons 31 -> 79 -> 175.
Both backends emitted identical RDL within their own root selection as TU counts increased.
Legacy captures the full header facts before filtering; the rewrite captures only selected native
dependencies after indexing the headers. These numbers demonstrate that selection advantage, not
equal-work production performance or a resource-budget guarantee.

The current pressure point is retained libclang TU/index memory, not projection time. An eight-TU
case already uses about 635 MiB even for three small roots. Full-consumer coverage, many more roots,
macro-heavy reparsing, deep alias chains, and parallel target extraction remain unmeasured.

## Rewrite plan and restart point

This page contains the continuation plan; resuming work must not require conversation history or
session-local notes. The objective is a manageable replacement around libclang, not another planner
over the old extractor's lossy output. Proceed through bounded gates, not an unconditional rewrite.

### Current baseline

The old `windows-clang` implementation and production generators are unchanged. Inspect the
worktree before restarting and preserve any local changes.

The current slice has 69 passing integration tests and one passing doctest, with strict clippy and
formatting complete. This establishes the covered cases, not production parity or completion of
the full acceptance matrix below.

| Established behavior | Primary evidence |
| --- | --- |
| Owned native evidence, native distinctions, completions, and bounded graph comparisons | `test_clang2/tests/native.rs` |
| Redeclarations retain dependencies and annotation contexts; missing annotations cannot hide conflicts | `test_clang2/tests/observations.rs` |
| External references cannot hide conflicts beneath type, function, callback, or value roots | `test_clang2/tests/projection.rs` |
| Direct and indirect interface constants have the same explicit omission outcome | `test_clang2/tests/projection.rs` |
| Closed plans survive snapshot disposal and compile to semantic WinMD | `test_clang2/tests/projection.rs` |
| Real SDK `IUnknown` capture and external-interface pointer projection | `test_clang2/tests/projection.rs` |
| `__declspec(uuid(...))` decoding and local interface emission (GUID, single inheritance, inherited methods, `_In_`/`_Out_` parameters) | `test_clang2/tests/projection.rs::local_interfaces_project_real_com_metadata` |
| Local SDK `IUnknown` and `IClassFactory` metadata on x86, x64, and ARM64 | `test_clang2/tests/projection.rs::real_interfaces_project_locally` |
| Shared COM output lowering and incompatible type/ABI rejection | `test_clang2/tests/projection.rs` |
| Conflicting interface UUIDs fail native resolution | `test_clang2/tests/observations.rs::interface_uuids_must_agree` |
| Buffer units, direction, constant bounds, and original parameter bindings | `test_clang2/tests/projection.rs` |
| SDK BCrypt input/output byte lengths | `test_clang2/tests/projection.rs::real_sdk_buffer_lengths_project` |
| Optional buffers retain direction, lengths, and per-parameter optionality | `test_clang2/tests/projection.rs::real_sdk_optional_buffers_project` |
| Explicit string contracts preserve constness and direction; missing/invalid contracts fail | `test_clang2/tests/projection.rs` |
| Native record layout, free aggregate calls, and bidirectional inherited vtable dispatch | `test_clang2/tests/abi.rs` |
| Unsupported by-value COM record results fail before emitting bindings | `test_clang2/tests/projection.rs::com_record_results_are_rejected` |
| SDK COM wrappers preserve identity, reference counts, output ownership, and failure HRESULTs | `test_clang2/tests/com.rs` |
| Explicit scalar typedef contracts survive alias chains and reject conflicting projections | `test_clang2/tests/projection.rs` |
| Forward UUIDs agree even without a UUID on the complete definition | `test_clang2/tests/observations.rs::forward_interface_uuid_conflicts_are_not_discarded` |
| SDK same-name tag/typedef roots share output; field/callable contracts survive order changes | `test_clang2/tests/projection.rs` |
| Pinned WebView2 core roots agree across main/interop TUs; unsupported interop evidence fails explicitly | `test_clang2/tests/webview.rs` |

The test crate is at `crates/tests/libs/clang2`. Real-header local COM metadata is covered, with
synthetic positive and negative controls alongside it. The synthetic ABI fixture executes on x64
and x86. Local ARM64 linking is blocked by missing Visual Studio ARM64 compiler/runtime libraries;
native ARM64 execution is configured in CI but has not been observed for this change. Coverage
includes raw ABI calls and high-level COM ownership wrappers against a native test implementation,
not Windows COM activation, DLL imports, or general C++ ABI parity.

Resume by inspecting the worktree and these modules, then rerun the baseline without updating
goldens:

```powershell
git status --short
$env:RUSTFLAGS = "-D warnings"
cargo test -p windows-clang2 -p test_clang2 --quiet
cargo clippy -p windows-clang2 -p test_clang2 --all-targets --quiet
```

### Architecture rules to preserve

Keep entity identity, observation agreement, metadata projection, and output ownership separate.
Canonical types supplement written evidence; they never replace the observations or their
dependencies. Native compatibility runs before reference pruning, exclusions, output naming, and
deduplication for every selected root category.

Keep raw compiler handles in capture. Resolve an immutable owned graph into checked groups and
completions. Projection reads that checked result without mutating the native graph. Every planned
dependency must be a local planned item or an explicit external binding. Rendering must not
rediscover dependencies, resolve aliases, classify interfaces, or infer layout.

Missing, conflicting, and unsupported evidence must remain distinguishable. External/opaque
contracts may permit incomplete evidence, but cannot excuse contradictions or supply an assumed
local by-value layout. Unsupported local dependencies must not silently become `void` or disappear.
Report root outcomes explicitly: emitted, omitted by policy, or unsupported with a reason.

The domain is Windows C ABI declarations plus the COM, MIDL, SAL, and closed WinRT forms required
by the generators. General C++ merging, arbitrary templates, arbitrary inheritance/vtable
reconstruction, and whole-language ODR verification are not goals.

### Next gates: interop evidence, ABI limits, and SAL/MIDL relationships

The aggregate-returning COM method limit needs a downstream bindgen fix and bidirectional native
coverage before relaxing the projection rejection. Confirm the native ARM64 ABI run in CI.

The next real-consumer blocker is anonymous aggregate evidence reached through
`ICoreWebView2Interop2` and `ICoreWebView2`. Bound a native-capture slice that preserves union/member
layout and contradictory observations before extending local projection. Do not weaken resolution
to let an external `VARIANT` binding bypass unsupported evidence.

Remaining SAL and MIDL work includes indirect lengths, counted strings, and the string-output
contract difference exposed by WebView2. Preserve explicit string bindings; do not invent unbound
`PSTR`/`PWSTR` names. Retain the local SDK interface, BCrypt, WinHTTP, and WebView2 cases as
regression gates. First record the exact selected roots, expected metadata, supported shapes, and
expected rejections in fixtures. Do not broaden the slice silently as new cases appear.

| Order | Work | Acceptance condition |
| --- | --- | --- |
| 1 | Local COM and UUIDs from `unknwnbase.h` | Metadata case covered: local `IUnknown` and `IClassFactory`, IID, inheritance, method order, system calling conventions, pointer levels, and COM output attributes on three targets. Synthetic executable ABI coverage is in gate 6. |
| 2 | SAL and MIDL relationships | Required/optional buffers, decimal element constants, and scalar null-terminated strings with explicit bindings are covered. Counted strings, indirect lengths, and MIDL recovery remain. |
| 3 | Constants and preprocessing | Cover GUID/property-key forms, redefinition/undefinition, final macro state, and poison expressions with explicit, bounded recovery outcomes. |
| 4 | Record layout | Cover packed, anonymous, bitfield, and a supported inherited record; compare compiler layout with generated Rust size, alignment, and offsets. |
| 5 | Real multi-TU consumers | Three WebView2 core roots pass across main/interop TUs and have bounded scale measurements. The actual interop bridge is blocked by anonymous aggregates; full WebView2 and a WDK case with UM references/enum overlays remain. |
| 6 | Target and ABI coverage | Raw-binding layout, free aggregate calls, bidirectional COM-style dispatch, and SDK COM ownership wrappers execute on x64/x86. Native ARM64 execution, Windows COM activation, DLL imports, and aggregate-returning methods remain. |

Use pinned real generator inputs where practical. Preserve main/satellite and WebView multi-TU
configurations rather than replacing them with a convenient single TU. Keep small synthetic
positive/negative controls alongside the real-header fixtures.

Query layout facts from libclang rather than reconstructing them from size guesses. The pinned
runtime was probed for `clang_getOffsetOfBase`, which the current `clang-sys` bindings do not expose.
If needed, use an updated or narrowly extended binding layer, not scattered dynamic symbol lookups.
An available offset API does not make arbitrary inheritance representable in metadata.

Keep annotation/token decoders small and independently covered; do not build another C++ parser.
Keep macro evidence, compiler evaluation, and projection eligibility separate. Failed evaluation,
nonconstant expressions, and unsupported constant representations need distinct outcomes. Preserve
definition/probe provenance and state a policy for context-sensitive macros.

### Acceptance matrix and review

Exercise valid combinations of root kind, local/external binding, alias depth, completeness,
contradictions, and graph shape. Cover functions, records, callbacks, constants, and typed values;
trees, shared graphs, cycles, and shared cycles; input order, filename changes, comparison direction,
and graph entry point. Document combinations that have no valid language meaning.

Pair accepted cases with controlled mutations of nested fields, pointer levels, calling
conventions, GUIDs, annotation relationships, and completions. Assert semantic invariance under
irrelevant ordering while preserving intentional preprocessing differences and source-order
ownership. Filename sorting must not decide semantic output ownership.

Measure unique comparison obligations and edges, including failure/retry paths, rather than relying
on recursion depth or timeouts. Preserve the existing 32/64/128-node work assertions. Review the
stage boundaries and acceptance results before declaring this gate complete.

### Production differential gate

Only after the fixed slice passes should remaining Windows rules be ported one family at a time.
Run both implementations on the same pinned inputs:

| Consumer | Required coverage |
| --- | --- |
| Win32 | x64, x86, and arm64; main and satellite TUs; DLL routing and header ownership. |
| WDK | `ntifs.h`, `wdm.h`, and `offreg.h`; UM references/exclusions and enum overlays. |
| WebView | WebView2 and interop headers; COM/WinRT types, references, and selected exports. |

Compare semantic metadata, exported symbols, import routing, layouts, calling conventions, pointer
categories, annotations, ownership, and unsupported-root outcomes. Text equality alone is
insufficient. Classify every difference as preserved behavior, an intentional correction, or an
explicitly accepted support change. Matching a known legacy bug is not an acceptance requirement.
No unexplained disappearance or new unsupported root passes.

Agree on wall-time and peak-memory budgets before the production pass. Measure both backends on
the same machine over comparable runs, including macro reparsing separately. Capture may dominate
runtime and retaining more evidence may increase memory; a simpler planner does not prove either
cost acceptable.

### Cutover gate

Keep the prototype unpublished and side by side until each caller passes its production gate.
Share fixtures and semantic assertions, not the old planner. Backend selection must be explicit;
there must be no automatic fallback to `windows-clang`.

Decide the public API migration for the builder and inspection consumers such as `Snapshot::facts`.
A compatibility view may preserve useful inspection, but must not feed lossy old types back into
native resolution. Retain procedural tests until each case is verified and moved to dedicated
`.h`/`.rdl` fixtures where appropriate.

Migrate callers only after their gates pass, then remove the old backend and temporary selector.
Do not keep two public pipelines indefinitely. Commits and merges remain maintainer-controlled.

### Stop conditions

Stop or narrow the rewrite if required evidence can only be guessed, compatibility depends on cycle
entry, production needs unrestricted structural matching, or the fixed slice requires global name
propagation, native graph mutation during projection, silent omissions, or fallback.

Also stop if support accumulates exceptions outside the responsible projection rule or cannot meet
the agreed resource budget. A narrow missing C API may justify updated bindings or a version-pinned
native helper. A broad missing AST surface warrants reassessing LibTooling and its C++/LLVM-version
coupling. Replacing the parser does not remove Windows projection policy.
