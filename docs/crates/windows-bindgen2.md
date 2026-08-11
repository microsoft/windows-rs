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

## Milestone status

The current milestone proves the native sys path, a bounded native COM path, the reusable request
boundary, both layouts, and WinRT value, delegate, interface, and class projection.

| Area | Status | Evidence or blocker |
| --- | --- | --- |
| Metadata ownership | Complete | One shared `Metadata` database; typed row identities per request. |
| WinRT enums and structs | Full unfiltered corpus | 1,731 enums and 125 structs render. |
| Native sys projection | Full corpus | Types, constants, functions, delegates, and interfaces render. |
| Native filtering and closure | Proven on real requests | All nine `tool_bindings` sys files match committed output. |
| Module layout | Focused differential coverage | Nested WinRT/Win32 fixtures match. |
| Flat layout | Complete for current projections | Explicit layout context covers cross-namespace WinRT and Win32 references. |
| Filtered WinRT values | Complete for enums and structs | Transitive value dependencies are selected; recursive values remain errors. |
| WinRT delegates | Full corpus | All 137 definitions lower and render; focused default-style output matches. |
| WinRT interfaces | Full structural corpus | All 8,105 definitions lower and render; focused ordinary, generic, and required-interface output matches. |
| WinRT classes | Full structural corpus | All 4,516 definitions lower and render; focused activation, hierarchy, static, async, and agile policy is covered. |
| Member filtering | Real request proof | WinRT routing and native COM slot selection preserve ABI prefixes. |
| ABI canonicalization | Consolidated for native sys | Namespace-qualified aliases and one callable lowering path match all nine requests. |
| Request reuse | Complete for current catalogs | The database, nested map, and interface-base map are shared across requests. |
| Formatting and file writing | Tool policy | Kept outside the projection core. |
| Sys request differential | Complete | A test-local parser proves nine real request files. |
| Projection styles | Internal WinRT and native proof | Minimal output is proven internally; no public style option exists yet. |
| Tool requests | Eight real proofs | Numerics, time, future, collections, window, reactor, animation, and core match. |
| Package output | Not started | Requires stable filtering, layout, and formatting first. |

Approximate hand-written production source size is 8,000 lines for bindgen2 versus 12,829 for the
existing bindgen crate, about 38% less. Bindgen2 also has about 2,160 lines of tests. This is a
useful maintenance signal, not a complete comparison: bindgen2 still lacks several output
policies and package generation.

## Stabilization gate

This gate is complete:

1. [x] Render flat output in a flat name-resolution context and add cross-namespace tests.
2. [x] Add filtered WinRT value dependency closure with cycle and missing-dependency tests.
3. [x] Centralize native ABI canonicalization:
   - qualify string-alias rewrites by namespace;
   - treat unspecified parameters consistently with input parameters;
   - keep closure and rendering on the same lowered signature path.
4. [x] Harden metadata2 facts used by bindgen2:
   - validate both `NestedClass` columns;
   - classify interfaces from the ECMA interface flag rather than only from the base type.
5. [x] Move immutable native catalogs needed by every request - nested relationships and interface
   bases - behind the shared `Metadata` boundary.
6. [x] Re-run the nine sys requests, focused layout tests, corpus inventories, and performance
   measurements, then do another milestone review.

This is a bounded cleanup pass, not a redesign. The current separation between metadata,
selection, closure, projection, rendering, and output should remain.

## Progress

The first layer selects WinRT enums and structs from an owned database. API-contract marker structs
are excluded as bindgen policy. Construction uses temporary namespace/name sort keys, then discards
them and stores only typed `TypeDef` entities and value categories. Iteration resolves short-lived
semantic views through the owned database and is deterministic by namespace, name, file, and row.

This intentionally does not reproduce the old reader map. Exact-name lookup already belongs to
`windows-metadata2::Database`; bindgen2 has no measured need for another permanent name index.

The ownership boundary separates reusable metadata from generation requests. `Metadata`
eagerly builds one shared database, WinRT value graph, nested-type map, and interface-base map.
Each `Generator` shares that immutable root while storing request-local typed entities. This lets
`tool_bindings` share one parse, index, and lowering pass across its 17 filter files without
exposing projected models or adding public lifetimes.

Rendering accepts only `Layout::Modules` and `Layout::Flat`. Both consume the same collected output
items and deterministic sort. The layout is also the explicit name-resolution context: module
output emits relative namespace paths, while flat output emits unqualified names. Flat output
checks cross-namespace generated name collisions and returns a structured error. Style options
are not exposed yet because current rich/minimal projection coverage is incomplete; accepting
ignored flags would create a false compatibility API.

The first filter layer is also intentionally programmatic. `Filter` stores bare names, exact
namespace/name pairs, namespace roots, and private method selections in ordered maps and sets.
Selection performs borrowed lookups and does not parse Rust paths, combine member policy with name
resolution, or retain a metadata-sized match index. An empty filter selects nothing, while requests
without a filter retain the existing all-items behavior. Filtered WinRT structs add referenced
value types transitively with a temporary entity set and queue; no permanent value dependency graph
is retained.

`Generator` retains the resulting WinRT and Win32 typed-entity selections. An internal lowering
view combines the shared database and catalogs with the request-owned selection, so repeated
rendering does not scan metadata again. Request-local native state is limited to namespace groups,
typed entities, and enum variant policy. Native projected models remain streaming values.

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
mixed fixture proves that WinRT values and Win32 items pass through the same output path.
Formatting, file writing, and package policy remain deferred.

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

- bindgen2 remains smaller than bindgen, but native COM implementation policy is incomplete, and
  it does not include complete rich native policy, file writing, or packages;
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
- bindgen2 production source is currently 9,505 lines versus 12,829 for bindgen, but bindgen2 does
  not yet implement packages or file writing. Nine warmed `tool_bindings` differential tests take
  about 2.5 seconds while the legacy tool takes about 2.2 seconds. The bindgen2 tests rebuild
  metadata per request, so this is encouraging but not an end-to-end speed claim;
- callable and interface projection are the main bindgen2 growth risks. Collection conveniences
  now live in `winrt_collection.rs`; future named policies should get similarly narrow modules
  rather than accumulating in `winrt_interface.rs` or `winrt_delegate.rs`.

The current advantage is clearer ownership and policy boundaries, explicit unsupported-shape
accounting, and complete corpus tests for implemented slices. Full output equivalence is still the
standard required before claiming the replacement is objectively better overall.

## Deep review

The current consumers do not share one orchestration shape:

| Consumer | Required policy |
| --- | --- |
| `tool_bindings` | Seventeen independent flat requests spanning sys, minimal, and default output. |
| `tool_reactor` and `tool_composition` | Custom metadata inputs, minimal output, dead-code policy, member filters, and implementation scaffolding. |
| `tool_webview` | Header-authored metadata, minimal COM and WinRT output, implementation scaffolding, and member filters. |
| `tool_package` | Multi-file package layout, feature generation, exclusions, sys and rich styles, and custom rustfmt settings. |
| Build scripts and samples | A stable builder facade over path and in-memory metadata inputs. |

Tool orchestration should remain in each tool. Bindgen2 core should own metadata, typed selection,
dependency closure, projection, and deterministic rendering. Legacy command-file parsing,
rustfmt process policy, filesystem writes, package staging, and compatibility diagnostics belong
in a later facade or in the consuming tool.

The review found several signs that test scaffolding was shaping the production API:

- `SysRequest` recognized only nine simple sys files but was public as though it were a general
  request model.
- `Generator` stored `Options::layout` while also exposing methods that rendered either layout,
  creating two ways to choose the same policy.
- Public value and native model types, inventory counters, and per-item lookup methods had no
  repository consumer outside bindgen2 tests.
- `Metadata` lazily initialized native catalogs with `OnceLock`, while request construction rebuilt
  the WinRT candidate catalog and lowered selected structs once for closure and again for
  rendering.
- `lib.rs` contained about 290 production lines and 1,230 test lines. The test module obscured the
  public boundary and encourages making internals public for test access.
- The crate readme repeated milestone history and corpus inventories that belong on this page.

## Cleanup gate

This gate is complete:

1. [x] Remove `SysRequest`, formatter process management, and file writing from the projection core.
   Preserve the nine-request proof with a test-local parser until a real tool migration needs a
   facade.
2. [x] Replace `Options` plus four generator constructors with one typed request/selection entry
   point. Make layout an explicit render argument rather than stored request state.
3. [x] Make `Metadata` construction fallible and eager. Share native catalogs and the immutable
   WinRT value catalog directly, removing `OnceLock` and duplicate value lowering.
4. [x] Reduce the public API to consumer-facing generation types. Keep projected models,
   inventories, and lookup helpers crate-private until an external consumer requires them.
5. [x] Remove stored diagnostic counts from `Win32Selection`; derive inventory data in tests.
6. [x] Move the 1,230-line test module out of `lib.rs` and remove redundant inventory-only
   production helpers.
7. [x] Rewrite the crate readme as a short user-facing API and scope page. Keep design status and
   milestone evidence here.

## WinRT delegate milestone

WinRT delegates are projected one definition at a time from typed metadata identities. The model
owns only generic parameter names, the delegate GUID, and the `Invoke` signature. It reuses the
shared WinRT value graph for struct copyability and ABI decisions rather than adding another type
registry.

The committed metadata contains 137 delegates. Their signatures use primitives, strings, objects,
named classes and values, generic type parameters, generic instances, and one input vector. There
are no output vectors, vector returns, architecture-gated delegates, or no-exception delegate
methods. The corpus test rejects any new unsupported shape or policy assumption.

Generic and non-generic identity, public `Invoke`, ABI vtables, constructor closures, and upcall
bodies match focused output from the existing generator. A broader fixture covers strings,
objects, enums, copyable and non-copyable structs, vectors, and non-copyable returns. Filtered
requests use projected generic names without metadata arity suffixes and close over referenced
WinRT values and delegates.

## WinRT interface milestone

WinRT interfaces reuse the delegate callable model for parameter lowering, public calls, ABI
signatures, implementation signatures, and upcalls. Interface-specific code owns identity,
runtime names, required-interface inheritance, method naming, and vtable construction. This keeps
callable ABI policy in one place without introducing a larger universal COM model.

The committed metadata contains 8,105 WinRT interfaces. Every definition lowers and renders,
including generic parameters, input and output arrays, by-reference output parameters, property
and event accessors, overload names, and referenced value and interface types. Focused fixtures
match existing output for ordinary, generic, void-return, and required-interface cases.

Interface relationships are decoded once into the shared metadata root. Filtered requests close
over direct methods, required interfaces, and their transitive value dependencies. Each interface
is otherwise materialized only while closing or rendering; no second global projected interface
graph is retained.

Exclusive interface suppression and class-owned factory policy belong to class projection rather
than the interface model. Package output remains a separate artifact-planning problem rather than
a larger `Layout` variant.

## WinRT class milestone

WinRT classes reuse the interface relationship catalog and callable model rather than introducing
a class-wide type graph. The per-class model owns its default and required interfaces, base
classes, factory interfaces, activation policy, and lowered methods. Filter closure follows those
same relationships and their callable dependencies.

The committed metadata contains 4,516 classes. Every class lowers and renders, including 257
classes without default interfaces, 718 derived classes, 719 default activations, 415 factory
activations, 1,564 static factories, and 374 composable factories. Focused fixtures match existing
output for ordinary activation, class hierarchy, and static factories.

`DefaultAttribute` is read from the `InterfaceImpl` relationship rather than inferred from row
order. Classes flatten instance methods, use direct calls for the default interface, cast for
other instance interfaces, and cache static or activation factories by interface. Classes without
a default interface remain opaque static types but still expose factory methods.

Marshaling behavior is decoded by enum value rather than by attribute presence: 4,116 classes are
agile and receive `Send` and `Sync`. The ten classes whose default interface is one of the WinRT
async interfaces render as aliases to the corresponding `windows_future` type.

Composable non-aggregating constructors use the ordinary factory path. The additional subclassing
`*_compose` helpers belong with implementation selection and remain deferred.

## WinRT member filtering

Member filtering is request-local and remains private until a tool migration needs the builder
surface. Each selected WinRT entity has one role:

| Role | Meaning |
| --- | --- |
| `All` | The type was selected directly; emit its full projection. |
| `Names` | Emit the selected public methods and the metadata required to call them. |
| `Shell` | Emit only the identity needed to name or cast the dependency. |

Values and delegates reached by callable signatures are complete projections. Classes and
interfaces reached only as reference types are shells. Class method selections are routed to the
instance or factory interface that owns the metadata method. Roles can only grow from `Shell` to
`Names` to `All`, so closure order cannot discard a stronger direct selection.

Public methods follow the requested names, including metadata accessor names and projected property
or overload names. ABI vtables retain every slot through the last selected method. This prefix rule
is required because removing an earlier slot would change every later slot number. Event selection
also retains the paired remove method in the ABI while exposing only the add-side revoker wrapper.

Focused differential tests cover interface methods, class routing, properties, value, delegate,
and class dependencies, minimal output, static factories, event pairs, and later vtable slots.
Complete minimal requests now cover both WinRT and native COM member filters; the internal
`Filter::include_method` entry point is not yet public.

## Projection style boundary

Projection style is one internal enum with `Sys`, `Default`, and `Minimal` variants. These are the
three request-wide styles used by current consumers, not independent rendering flags. The profile
is passed through rendering and the shared callable context rather than copied into each projected
model. Selection and dependency closure use the same roles in all three styles.

The focused minimal enum, struct, delegate, interface, and class fixtures match existing output
apart from the delegate safety correction below. Default event output matches the existing
fixture, and focused minimal event tests cover the intentional constructor difference. The full
delegate, interface, and class corpora also lower and render in both modes. The minimal differences
currently stay at the rendering boundary:

| Area | Minimal policy |
| --- | --- |
| Values and interfaces | Omit runtime type names when implementation support does not need them. |
| Strings | Use `String` and `&str` in public wrappers while preserving `HSTRING` ABI types. |
| Delegates | Use infallible `Send` closures, preserve non-void returns, and omit public `Invoke`. |
| Interfaces | Do not emit inherited forwarders; expose exclusive interfaces for class `Deref`. |
| Classes | Keep identity and interface calls, add `Deref`, and return classes directly. |
| Events | Use the same add/remove pairing and revoker model with mode-specific delegate closures. |

This mode remains private. Exposing it before native COM output and more real tool request
comparisons would create another compatibility surface before its semantics are complete.
Implementation selection remains a separate request policy rather than a projection style.

Event projection is shared by default and minimal output. Lowering pairs special-name `add_` and
`remove_` methods, resolves the handler delegate, validates the registration-token shape, and
stores only the handler callable needed by the public wrapper. ABI methods and implementation
traits still retain both metadata methods; public output emits one closure-based
`EventRevoker` method.

The existing minimal generator accepts non-`Send` closures even though `DelegateBox` advertises an
agile COM object, and it returns success from non-void delegate upcalls without writing the result.
Bindgen2 does not reproduce those behaviors. Minimal delegates retain `Send`, non-void closures
return the projected value, and the upcall writes it to the ABI result. Minimal event wrappers call
the public delegate constructor instead of reaching across namespaces into private delegate-box
implementation types.

The first complete tool comparisons use a test-local metadata-aware parser for existing filter
paths and brace groups. `numerics.txt` proves a pure minimal WinRT value request. `time.txt` proves
a mixed request with WinRT values, a native struct, a linked native function, and a
`windows_core::BOOL` remap. `future.txt` proves default generic delegates and interfaces, async
agility, required generic interfaces, multiple generic parameters, and special WinRT value
remaps. Native sys rendering remains unchanged.

`collections.txt` proves the rich collection adapters without introducing a general convenience
registry. `IIterable<T>` emits owned and borrowed `IntoIterator` adapters, `IIterator<T>` emits
`Iterator`, and interfaces requiring `IIterable<T>` receive the same adapters with the substituted
item type. The request also proves self-referential output parameters and event sender types.

`window.txt` proves a complete minimal native request with constants, string and handle aliases,
structs, delegates, linked functions, and pointer-width aliases. Rich native derives use recursive
type properties: aliases inherit their target, structs inherit their fields, delegates block
equality, and floats block `Eq`. Explicit-layout, packed, and aligned structs remain conservative.

The webview `reactor.txt` request is the first proof over custom Windows App SDK metadata and an
explicit implementation list. Custom input definitions take precedence over duplicate default
definitions. Unresolved relationships in unrelated metadata are retained and reported only if a
request traverses them. In minimal output, interfaces omitted from the implementation list use
private `usize` placeholders for skipped ABI slots, so those slots preserve offsets without
selecting their signature dependencies. Implemented interfaces retain typed slots and complete
method dependencies. The comparison preserves bindgen2's intentional delegate differences:
closures remain `Send`, delegates retain public constructors, and event wrappers call those
constructors instead of duplicating private `DelegateBox` code.

`animation.txt` is the first bounded native COM proof. Selected interfaces retain their full
vtable layout, selected methods receive typed slots and callable wrappers, unselected slots remain
private `usize` placeholders, and signature-only interface dependencies are emitted as shells.
HRESULT methods project trailing output pointers as `Result<T>`, interface inputs use
`windows_core::Param<T>`, and optional output pointers remain optional raw pointers. This policy is
limited to the shapes required by the animation request; broader COM policy remains request-driven.

`core.txt` proves the matching native COM producer path without adding request-specific names.
Complete selected interfaces emit `_Impl` traits, vtable constructors, ABI upcalls, and IID
matching; interfaces with skipped slots still omit producer machinery. `ComOutPtrAttribute`
identifies generic `Resolve<T>` query methods, while ordinary trailing output pointers retain the
animation retval policy. Native callable ABI spelling also keeps core-owned `HSTRING` values raw,
maps `RPC_STATUS` back to `windows_core`, and constructs projected HRESULT constants.
Native interfaces without COM identity remain sys-only; rich requests return a structured
unsupported-shape error rather than emitting an undefined interface type.

The final `tool_bindings` proof is `canvas.txt`. Its committed output contains 49 COM
interfaces and 75 selected wrappers: 29 return `Result<T>`, 16 return raw HRESULTs, 26 return
`void`, and four return values directly. Thirty-five vtables derive from another projected
interface. Four wrappers use metadata-sized slices, two use generic query projection, and one
query method has an input parameter before the IID/output pair. The complete semantic differential
runs as a normal test.

Canvas should proceed in this order:

1. [x] Non-HRESULT COM methods: void returns, direct scalar returns, and hidden struct-return ABI.
   `BeginDraw`, `GetMaxWidth`, and `GetSize` provide focused coverage.
2. [x] Query methods with ordinary parameters before the IID/output pair.
   `IDXGISwapChain::GetBuffer` provides focused coverage.
3. [x] External minimal-crate routing for canonical Numerics references.
4. [x] Ownership projection for interface-valued struct fields.
5. [x] Native interface inheritance conveniences: `Deref` and complete hierarchy lists.
6. [x] Output-interface projection for non-HRESULT methods.
7. [x] Metadata-sized input slices and their pointer/count ABI pairs.
8. [x] Input `BOOL` sugar.
9. [x] Reassess the complete differential and split any remaining callable conversions.
10. [x] Correct large-struct retval classification.
11. [x] Add `PCWSTR` input conversion where the public API accepts `Param<PCWSTR>`.
12. [x] Review remaining pointer casts, `Self` spelling, and token normalization before adding
    projection policy for cosmetic parity.

The canvas request now renders fully, so later checkpoints can use the complete token differential
rather than discovering one structural error at a time.

The complete canvas request now passes as a normal semantic-parity test. Its narrow normalization
removes only the 12 reviewed differences: redundant pointer casts, one equivalent `Param<Self>`
spelling, and slice token spacing. Bindgen2 does not reproduce those legacy output details.

### Post-canvas review

Native COM projection still has a two-way policy boundary: `sys` emits raw ABI declarations, while
default and minimal projections share owned fields, wrappers, hierarchy, and callable policy.
Minimal adds external crate routing and excludes those definitions from local closure. No other
native COM behavior currently differs between default and minimal.

Interface lowering previously walked every base chain twice. The retained root-to-base hierarchy
now also determines whether the interface reaches the canonical `IUnknown`, leaving one checked
traversal. Retval sizing uses metadata layout and the maximum supported pointer width so one
generated API shape remains safe on 64-bit targets.

### WebView2 discovery

The next proof is the native `tool_webview` request generated from the pinned WebView2 headers. It
produces 6,927 lines and explicitly selects 28 COM interfaces for implementation scaffolding. An
ignored complete differential consumes `target/webview/WebView2.winmd` after `tool_webview`
generates it. This request should first prove native implementation filtering; projection policy
must not be broadened merely because another complete selected interface could be implemented.

Native `Request::implementations` now controls producer emission. The request emits exactly its 28
producer traits and no others. Requested implementations close dependencies over every method and
use typed vtable slots for producer upcalls, but consumer wrappers still follow the main member
filter. An implementation-only callback therefore does not expose unrelated consumer methods.
Requests without an implementation filter preserve the earlier complete-interface behavior.

Native method lowering now retains the raw metadata name for member matching and separately stores
the projected Rust name used by wrappers and vtables. This restores accessors and events selected
as `get_*`, `put_*`, `add_*`, and `remove_*` without changing overload naming. The differential
dropped from 534 to 208 token groups.

Explicitly implemented native interfaces in minimal output now expose producer traits and typed
vtables without caller wrappers, matching legacy native COM policy. An explicit implementation
list also limits `RuntimeName` emission to that list; requests without a list retain the earlier
default. WebView2 now matches all 218 consumer wrappers, 28 producer traits, and 28 runtime-name
implementations. The differential dropped from 208 to 130 token groups.

`EventRegistrationToken` versus `i64` remains intentionally unresolved. This resembles the alias
canonicalization already performed by `windows-clang` and `tool_win32`; bindgen2 should not add a
WebView2-specific remap. Revisit the metadata producer after parity or leave the named alias in the
new output if it is the better model. The next local projection category is producer callable
parameters.

Direct input interface parameters in native producer traits now use `windows_core::Ref<T>`.
Consumer methods retain generic `Param<T>` inputs and ABI vtables retain raw COM pointers. Producer
upcalls convert the raw pointer into the borrowed reference without taking ownership. WebView2 now
has the same 39 `Ref<T>` uses as the committed output, reducing the differential from 130 to 91
token groups.

Excluding the deferred event-token alias, the next producer differences are borrowed non-primitive
value inputs and their `transmute` conversions. A few callable casts and one `IStream` optional
interface conversion remain after that.

### Native type identity and projection boundary

The Win32 scrape already canonicalizes Direct2D aliases such as `D2D_POINT_2F` and
`D2D_MATRIX_3X2_F` to `Windows.Foundation.Numerics` types. The generated Win32 RDL and winmd carry
those canonical identities. Bindgen2 must not add a second D2D alias table or reinterpret the
original header names.

The missing `windows_numerics::` prefix in the canvas output was an external-reference routing
problem. Flat layout removes namespace paths, so minimal projection now distinguishes locally
emitted types from types supplied by another crate first. One shared table replaces the
future-only check and routes the five canonical Numerics types. Dependency closure treats those
WinRT definitions as external to the native canvas request.

Interface-valued native struct fields are a separate projection concern. Their metadata identity
remains the interface and their ABI remains a raw COM pointer, while the public field spelling is
`core::mem::ManuallyDrop<Option<Interface>>`. This requires a field-specific writer rather than a
global type remap: method parameters, vtable entries, and public struct fields intentionally use
different spellings of the same metadata type. Rich structs also omit `Copy` when a field owns an
interface, while the sys projection remains copyable because it contains only the raw pointer.

### Projection-mode review

Parity work should preserve observed `sys`, `minimal`, and default output where the distinction is
required, but each new mode-specific branch should be treated as a review item. Some differences
are inherent to ABI exposure, external crate boundaries, and package layout; others may only carry
forward unnecessary generator complexity. Once parity provides a stable baseline, audit the
recorded differences and merge policies that do not serve a concrete output or ownership
requirement.
