# windows-bindgen2

`windows-bindgen2` is an unpublished replacement candidate for
[`windows-bindgen`](windows-bindgen.md). It uses
[`windows-metadata2`](windows-metadata2.md) directly and is developed beside the current generator
so every output layer can be compared before migration.

## Design constraints

- Own the metadata database; do not leak it.
- Store typed metadata entities rather than copied names or borrowed rows.
- Keep selection, dependency closure, projection, rendering, and output layout separate.
- Build indexes only for measured lookup requirements.
- Lower methods, delegates, interfaces, and classes one item at a time.
- Keep only the recursive value graph global.
- Return structured errors rather than using malformed-input panics.
- Require deterministic output and differential tests at every checkpoint.

## Progress

The first layer selects WinRT enums and structs from an owned database. API-contract marker structs
are excluded as bindgen policy. Construction uses temporary namespace/name sort keys, then discards
them and stores only typed `TypeDef` entities and value categories. Iteration resolves short-lived
semantic views through the owned database and is deterministic by namespace, name, file, and row.

This intentionally does not reproduce the old reader map. Exact-name lookup already belongs to
`windows-metadata2::Database`; bindgen2 has no measured need for another permanent name index.

The ownership boundary now separates reusable metadata from generation requests. `Metadata` owns
one `Arc<Database>`, and each `Generator` clones that root reference while storing request-local
typed entities. This is one reference count per request, not per row. It lets `tool_bindings`
eventually share one parse/index pass across its 17 filter files without leaking the database or
adding lifetimes to every public projected model.

Generation options currently contain only `Layout::Modules` and `Layout::Flat`. Both consume the
same collected output items and deterministic sort. Flat output checks cross-namespace generated
name collisions and returns a structured error. Style options are not exposed yet because current
rich/minimal projection coverage is incomplete; accepting ignored flags would create a false
compatibility API.

The first filter layer is also intentionally programmatic. `Filter` stores bare names, exact
namespace/name pairs, and namespace roots in ordered sets. Selection performs borrowed lookups and
does not parse Rust paths, combine member policy with name resolution, or retain a metadata-sized
match index. An empty filter selects nothing, while requests without a filter retain the existing
all-items behavior. This is enough to prove selective request cost and output before adding
dependency closure.

`Generator` retains the resulting WinRT and Win32 typed-entity selections. `Win32Items` is a
borrowed lowering view over the shared database and the request-owned selection, so repeated
rendering does not scan metadata again. The retained native state is limited to namespace groups,
typed entities, the nested parent-to-children map required by recursive structs, and inventory
counts. Native projected models remain streaming values.

Filtered native selection now closes over decoded field and method signatures. A temporary ordered
entity set and work queue pull in referenced aliases, enums, structs, delegates, interfaces, and
base interfaces transitively. Name resolution reuses metadata2's exact TypeDef index and adds every
architecture row for a referenced name. The queue is discarded after selection; no dependency
graph or per-type role map is retained.

The value layer lowers all 1,731 selected enums and 125 structs into owned models. Enum, struct,
type, GUID, and graph policy live in separate modules. The graph uses nested ordered maps so
namespace and type lookups borrow existing strings rather than allocating lookup keys. It detects
recursive value cycles and conflicting GUID definitions explicitly.

Every selected value now renders successfully. Focused enum and struct fixtures match the existing
golden output token for token. `HttpProgress`, the one gap in the earlier bindgen-side proof, now
uses its parameterized `IReference<u64>` interface signature. GUID folding remains bindgen
projection policy and is performed only for generic named types that require it.

The first flat Win32 slice selects 83,641 constants and 14,559 functions from non-WinRT `Apis`
containers. Selection stores one namespace string per container plus typed field and method
entities; per-item namespace copies would have been expensive at this scale. Temporary names are
used only for deterministic sorting.

All 14,559 functions lower through a separate native signature model. The supported surface
includes primitive and named types, const and mutable pointers, symbol aliases, C and system
calling conventions, and variadic declarations. Focused no-parameter and const-pointer functions
match existing flat sys tokens.

All 83,641 constants now lower and render. The implementation supports primitive and pointer-sized
values, UTF-16 and ANSI strings, boolean coercion, native typedef and enum chains, direct GUIDs,
and GUID-backed property keys. Only 75 named types account for the typed constant corpus, so alias
chains are resolved while lowering one constant and then discarded. There is no global native type
graph.

Native type selection now retains 30,109 top-level typed entities:

| Category | Count |
| --- | ---: |
| Native typedef aliases | 12,667 |
| Native enums | 4,728 |
| Native structs and unions | 12,714 |

Each definition lowers and renders independently. The native model supports primitive and named
fields, pointers, fixed arrays, typedef aliases, enum values, explicit-layout unions, forced
alignment, packing, and empty definitions. Class layout is exposed by a new focused metadata2
semantic module rather than decoded directly in bindgen2. Streaming iterators lower native types,
constants, and functions without retaining projected models or adding a name index.

Selection, native types, constants, and functions are separate modules. The corpus test lowers and
renders every selected top-level type, constant, and function, so an unsupported signature shape
cannot disappear from output. Focused aliases, enum, struct, union, primitive, string, GUID,
no-parameter, and const-pointer fixtures match the corresponding flat sys tokens.

The first output layer consumes only those existing projections. It groups rendered items by
metadata namespace, sorts by item name and category, builds nested Rust modules, and emits one
token stream. A focused nested Win32 fixture matches the existing module golden output, and a
mixed fixture proves that WinRT values and Win32 items pass through the same output path. It does
not add filtering, dependency closure, formatting, file writing, or package policy.

Bitfield accessors are not a sys-output gap: the existing sys generator emits only their coalesced
backing fields. Of 11,264 direct primitive handle shapes, all but
`OVERRIDE_PREFETCH_PARAMETER` carry `NativeTypedefAttribute`; the unannotated exception now follows
the same alias policy. All six `Value: void` shapes are native typedefs and were already aliases.

Native delegate projection is complete for all 2,159 rows, including 43 architecture-gated
variants and system/C calling conventions. A focused callback fixture and an architecture
dependency fixture match the existing sys output. Delegates and imported functions share one
owned native signature model; delegate attribute decoding uses a narrow resolver for the framework
`CallingConvention` enum rather than moving framework policy into metadata2.

Native interface projection is complete for all 4,290 rows and 25,868 methods. Sys output contains
ordered ABI vtables, direct base-vtable fields, COM IIDs for `IUnknown` hierarchies, architecture
gates, special method names, and overload suffixes. One non-value generic interface signature is
erased to its raw ABI pointer rather than introducing a generic native type model. The retained
`InterfaceImpl` map supports inheritance and filtered closure; individual interface projections
are not retained.

The first `tool_bindings` integration probe runs all nine `--sys` request files against one shared
`Metadata` value and compares each in-memory result with its committed binding file. The outputs
match after ignoring function-pointer parameter names and rustfmt-only trailing commas. This probe
found policy gaps that corpus lowering alone did not reveal:

| Area | Required sys policy |
| --- | --- |
| Output order | Imported functions precede the alphabetically sorted remaining items. |
| `GUID` | Lowercase fields, derived `Default`, and local struct-literal IID constants. |
| Class signatures | Interfaces erase to raw ABI pointers; native delegates stay named aliases. |
| String aliases | `BSTR`/`PCSTR` canonicalize to their projected pointer element types. |
| SAL input strings | Input-only `PWSTR`/`PSTR` parameters become `PCWSTR`/`PCSTR`. |
| Closure | Follow projected ABI types so erased or canonicalized metadata types do not leak in. |
| Scoped enums | Dependency enums retain only explicitly requested variant constants. |

## Native shape inventory

The committed Win32 metadata contains:

| Shape | Count |
| --- | ---: |
| Top-level rows with `SupportedArchitectureAttribute` | 1,054 |
| Distinct architecture-specific names | 671 |
| Names with multiple architecture rows | 374 |
| Selected native type rows with architecture gates | 997 |
| `NestedClass` rows | 2,633 |
| Direct parents with nested structs | 1,925 |

All 2,633 nested rows are native struct-to-struct relationships. Metadata2 exposes them as a
streaming semantic pair iterator. Bindgen2 builds one parent-to-children map because recursive
lowering needs repeated lookup. It preserves relationship-table order, recursively assigns
positional names such as `Outer_0_0`, and substitutes empty-namespace field references only from a
borrowed list for the current enclosing definition. Nested rows inherit the enclosing projection
namespace rather than their empty metadata namespace, so references to sibling top-level types
remain local to the emitted module. Nested types are not added to the top-level name index and
there is no second native type graph.

Nested rendering is complete for the selected native struct surface. A true `NestedClass` fixture
matches the existing generator for multiple direct children, deep struct/union nesting, packing,
named types in the enclosing namespace, and inherited architecture gates. Explicit layout
propagates through the owned nested subtree for `Default` policy, so structs containing nested
unions use gated manual implementations.

Native `Default` policy is also complete for the selected sys surface:

| Policy | Structs |
| --- | ---: |
| Derive `Default` | 8,584 |
| Manual - explicit layout | 2,164 |
| Manual - direct fixed array | 1,889 |
| Manual - fixed-array typedef chain | 74 |
| Manual - scoped-enum field | 3 |

The check resolves by-value definitions during lowering with an ephemeral cycle guard. It does not
retain a native dependency graph. A focused old-generator fixture covers each suppression reason.
That fixture also covers the 10 scoped native enums. They now render as transparent newtypes with
associated constants, while ordinary C enums remain aliases with module-level constants. None of
the current scoped enums carries an architecture gate.

Architecture gating is now complete for the selected surface:

| Selected item | Gated rows |
| --- | ---: |
| Native types | 997 |
| Constants | 512 |
| Functions | 261 |

Metadata2 uses one checked decoder for TypeDef, Field, and MethodDef owners. Bindgen2 stores only
the decoded bits and emits the existing `x86`, `x86_64`/`arm64ec`, and `aarch64` mappings. Sorting
uses name, architecture bits, and entity identity so duplicate definitions are deterministic.
Enum members are flattened only while building output modules; standalone native models remain
per definition. Focused aliases and duplicate enums match existing golden tokens, while constants,
functions, enum members, and union `Default` implementations are each independently gated.

## Critical assessment

The direction remains better than the current generator, but it is not yet a replacement:

- bindgen2 remains much smaller than bindgen, but it does not yet include WinRT interfaces and
  classes, rich native policy, member-level filters, file writing, or packages;
- metadata2 owns data, uses checked typed identities, and avoids the leaked reader, but its source
  is already close to the old metadata crate's raw line count because parsing and differential
  tests are extensive;
- the measured retained indexes remain small: metadata2 has exact type-name lookup, the WinRT value
  graph exists only for recursive value semantics, and Win32 selection stores typed entities
  grouped by one namespace string;
- WinRT and native type models intentionally remain separate because their projection and ABI
  rules differ. Unifying them now would recreate the broad old `Type` enum;
- `image.rs` and `semantic.rs` are the main metadata2 growth risks. New relationships should be
  split by concern rather than extending one semantic module indefinitely.

The current advantage is clearer ownership and policy boundaries, explicit unsupported-shape
accounting, and complete corpus tests for implemented slices. Full output equivalence is still the
standard required before claiming the replacement is objectively better overall.

## Next checkpoint

The bounded module-output, RDL2 external-reference, native-shape inventory, architecture-gating,
nested-rendering, native-default, scoped-enum, and remaining-surface inventory checkpoints are
complete. Native delegates and interfaces are also complete for sys output. The reusable
metadata/request boundary includes explicit module/flat layout, exact programmatic filters, and
transitive supported-native closure. All nine real `tool_bindings` sys requests now match in
memory. The next checkpoint should decide whether to expose a thin request-file adapter or first
add output formatting and file writing. Do not add general member-filter syntax, a second name
index, or a global native graph without a measured requirement.
