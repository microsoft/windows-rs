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
| Native typedef aliases | 12,666 |
| Native enums | 4,728 |
| Native structs and unions | 12,715 |

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

This is structural coverage, not complete native output equivalence. Architecture variants still
need selection attributes, nested types are not yet attached to their enclosing definitions, and
bitfield accessors, handle policy, native delegates, and interfaces are absent. Derive policy also
needs full generated-output comparison. These are separate policies rather than reasons to build a
global native type graph.

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
streaming semantic pair iterator. Bindgen2 builds one ordered parent-to-children map because
enclosing-aware field lowering will need repeated lookup. It does not add nested types to the
top-level name index or create a second native type graph.

Architecture gating should be implemented before nested rendering. It is local item policy and is
already required to prevent same-name variants from colliding. Nested rendering is the larger
slice: it needs enclosing-aware type resolution, deterministic generated names, recursive output,
and focused comparison with true `NestedClass` metadata rather than sibling RDL structs.

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

- bindgen2 remains much smaller than bindgen, but it does not yet include interfaces, classes,
  complete native type policy, filters, closure, module output, or packages;
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

The bounded module-output, RDL2 external-reference, native-shape inventory, and architecture-gating
checkpoints are complete. Next, design nested native rendering around the retained parent map and
enclosing-aware field lowering. Do not add dependency closure, a second name index, or a global
native graph in that slice.
