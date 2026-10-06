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
| External interfaces | Bind a native record; consume exactly one native pointer/reference level. |
| Local interfaces | UUID-bearing, fieldless records with pure virtual system-ABI methods and at most one base. |
| Aliases | Peel at uses; same-name tag aliases share their record binding. |
| Constants | Supported scalar and raw pointer values; omit interface values with a reason. |
| Raw pointers | Collapse mixed mutability to const if any raw-pointer level is const. |

External bindings are trusted metadata contracts, not native ABI proof. All captured native
dependencies must agree before an external binding can suppress local output. Metadata readers
still need the referenced WinMD files when compiling RDL. Arbitrary alias bindings are not
supported.

Functions and methods share annotation lowering. `_In_`, `_Out_`, and `_Inout_` retain direction.
`_In_opt_` adds input optionality for pointer parameters. `_COM_Outptr_` requires a writable
`void**` or interface output pointer. A `void**` receives `#[out] #[iid_is]` (the metadata
`ComOutPtrAttribute`); typed interface outputs retain their interface type and receive `#[out]`.
Other annotations remain errors, including unsupported COM output variants.

Local COM methods must be non-static, non-const, and non-ref-qualified. Their calling convention
must match the RDL system ABI: stdcall for x86, the platform convention for x64 and ARM64.
Inheritance is represented by a base-interface reference, with declared method order preserved.
An interface parameter consumes exactly one native pointer/reference level.

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

The SDK fixture force-includes `specstrings.h` before the shared `tool-win32` SAL capture shim.
`specstrings.h` redefines COM SAL macros, so loading it after the shim can erase annotation evidence.
Capture records compiler annotation attributes, not arbitrary SAL spelling or MIDL comments;
callers must provide the annotation shim in the correct header order.

The workspace test job includes both crates on supported hosts. Like the existing clang tests,
they are excluded from the x86 test process because the pinned runtime has no x86 DLL.

## Rewrite plan and restart point

This page contains the continuation plan; resuming work must not require conversation history or
session-local notes. The objective is a manageable replacement around libclang, not another planner
over the old extractor's lossy output. Proceed through bounded gates, not an unconditional rewrite.

### Current baseline

The old `windows-clang` implementation and production generators are unchanged. Inspect the
worktree before restarting and preserve any local changes.

The current slice has 40 passing integration tests and one passing doctest, with strict clippy and
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

The test crate is at `crates/tests/libs/clang2`. Real-header local COM metadata is covered, with
synthetic positive and negative controls alongside it. Generated-Rust vtable layout and C/C++ ABI
checks remain **not established**; metadata agreement alone is not proof of executable ABI parity.

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

### Next gate: SAL and MIDL relationships

The next implementation step is richer SAL and MIDL evidence, including strings and representable
length relationships. Retain the local SDK interface case as a regression gate. First record the
exact selected roots, expected metadata, supported shapes, and expected rejections in fixtures.
Do not broaden the slice silently as new cases appear.

| Order | Work | Acceptance condition |
| --- | --- | --- |
| 1 | Local COM and UUIDs from `unknwnbase.h` | Metadata case covered: local `IUnknown` and `IClassFactory`, IID, inheritance, method order, system calling conventions, pointer levels, and COM output attributes on three targets. Executable ABI checks remain in gate 6. |
| 2 | SAL and MIDL relationships | Preserve raw evidence and original parameter bindings; lower direction, optionality, strings, and representable lengths with semantic metadata assertions. |
| 3 | Constants and preprocessing | Cover GUID/property-key forms, redefinition/undefinition, final macro state, and poison expressions with explicit, bounded recovery outcomes. |
| 4 | Record layout | Cover packed, anonymous, bitfield, and a supported inherited record; compare compiler layout with generated Rust size, alignment, and offsets. |
| 5 | Real multi-TU consumers | Select WebView2/core-interop declarations with external references and a WDK case with UM references and an enum overlay. |
| 6 | Target and ABI coverage | Exercise x86 calling conventions and x64/x86/arm64 layouts; compile generated Rust and add selected C/C++ ABI smoke cases. |

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
