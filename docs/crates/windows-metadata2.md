# windows-metadata2

`windows-metadata2` is an unpublished replacement candidate for
[`windows-metadata`](windows-metadata.md). It is developed beside the existing implementation so
each layer can be compared against the current Windows metadata corpus before consumers migrate.

The first layer owns and structurally validates a PE/CLI metadata image. Successful construction
guarantees that the metadata root, stream directory, table header, row counts, table widths, and
table byte ranges are in bounds. The complete ECMA-335 table schema is declared once and drives
row-width calculation.

Current exclusions are intentional:

- higher-level semantic table wrappers;
- general metadata construction and serialization beyond the bounded authoring proof;
- common, Win32, and WinRT validation;
- architecture merging and namespace remapping;
- `windows-bindgen` and `windows-rdl` integration.

These capabilities will be added in independent changes with differential tests against the
existing implementation. The crate remains unpublished until both `windows-bindgen` and
`windows-rdl` can use it without changing generated output.

## Design constraints

- Each layer must establish a small invariant that later layers can test.
- Invalid external input must return a structured error rather than panic.
- The image owns its bytes; row identities must not require leaked or self-referential storage.
- ECMA table identities, columns, and coded-index relationships must have one declaration.
- Raw blobs remain available for lossless copying; semantic decoding is a separate layer.
- Windows validation, architecture merging, namespace remapping, and `Apis` projection stay
  outside the ECMA storage layer.
- Consumer migration must use differential output tests rather than assumed compatibility.

## Implementation plan

| Step | Status | Acceptance criteria |
| --- | --- | --- |
| PE/CLI image and table layout | Done | WinRT and Win32 images parse; malformed ranges fail without panics. |
| Typed row IDs and checked heaps | Done | One table declaration generates IDs and layout; string/blob/GUID access is bounded. |
| Raw row and coded-index access | Done | Every column kind decodes through checked table metadata. |
| ECMA signatures | Done | One fallible decoder handles every signature-bearing row and reports byte offsets. |
| Multi-image database and indexes | Done | Owned file IDs and row IDs replace leaked indexes and borrowed identities. |
| Custom-attribute values | Done | Constructor-directed fixed and named arguments decode without losing serialized types. |
| `windows-bindgen` proof | Done | Owned selection and representative output proved the metadata2 boundary. |
| `windows-bindgen2` foundation | Started | Existing projections render through deterministic modules. |
| Deterministic metadata builder | Started | Bounded enum/struct images are accepted by both readers. |
| `windows-rdl2` authoring proof | Started | A separate source model emits through metadata2. |
| Consumer diagnostics boundary | Done | Builder rollback preserves consumer-defined errors. |
| Common and Windows validation | Planned | Existing validation corpus passes through explicit profiles. |
| Merge and namespace remap | Planned | Transformations use one lossless copier outside the core image. |
| Replace `windows-metadata` | Planned | Both consumers and generation pipelines have migrated. |

## Progress notes

The initial parser revealed two assumptions during review. Optional-header data directories must be
bounded by the declared optional-header size, and table row counts must be bounded by the declared
table stream rather than adjacent stream bytes. Both checks are now explicit. Table stream
finalization also rejects more than three bytes of zero padding or any nonzero trailing data, which
checks the declared ECMA schema against real WinRT and Win32 images.

The next review found that the first schema implementation still repeated table identities in the
enum, conversion array, and schema list. One table declaration now generates the table identifiers,
typed marker types, and column schemas.

`RowId<T>` uses the ECMA one-based row number and is explicitly local to one image. It contains no
borrowed image reference, so a later multi-image database can combine it with an owned `FileId`
rather than leak an index. `Image` checks every string, blob, and GUID column during construction.
The committed WinRT and Win32 images match the existing reader for every non-nested type identity.

Heap work exposed another container-boundary issue: checking only the containing file allowed a
malformed CLI or metadata directory to borrow bytes from adjacent section data. CLI headers,
metadata roots, stream directories, and streams are now bounded by both their declared directory
sizes and the raw PE section. The string and blob heaps must begin with their empty entry, GUID
heaps cannot end with a partial GUID, and blob lengths must use canonical compressed integers.

Raw row views retain typed table-local identities while validating the requested column kind.
Direct indexes, list starts, and coded indexes are separate column kinds. This matters because a
list start may use the target table's one-past-end sentinel while an ordinary table index may not.
Coded-index declarations record explicit tags rather than relying on target order; this preserves
the sparse tags 2 and 3 used by `CustomAttributeType`.

Complete type shapes now match the existing reader across the committed WinRT and Win32 images.
The comparison includes duplicate architecture variants, field and method lists, and base-type
coded indexes. Nonempty pointer tables and edit-and-continue tables are rejected explicitly.
Pointer tables change list interpretation and will remain unsupported until that indirection has a
tested model.

The signature layer uses one bounded `BlobReader` for primitive reads and canonical compressed
integers. Method, field, property, local, member-reference, type-specification, and method-spec
signatures share one recursive type decoder with a nesting limit. Every signature-bearing row in
the committed WinRT and Win32 images is decoded during image construction. Representative
`Point` fields and `IStringable::ToString` signatures also match the existing reader.

The first corpus failure was the Win32 `NativeTypedefAttribute` value field encoded as `void`.
Rejecting it in the decoder repeated the old mistake of mixing semantic policy into structural
reading. The decoder now preserves `void` in any encoded position; common and Windows validators
will decide where it is accepted. Signature errors retain the owning table and row as well as the
absolute blob byte offset.

Custom-attribute planning showed that fixed enum values cannot be decoded correctly until a
constructor parameter's TypeRef can be resolved to its enum definition. Assuming an `i32` backing
type would repeat known debt, so the multi-image database was moved ahead of attribute decoding.

`Database` owns its images and combines `FileId` with typed row IDs as `Entity<T>`. Its nested
namespace/name index does not allocate on lookup and preserves every matching definition,
including architecture variants. Exact raw TypeDef name multiplicities match the existing index
across WinRT and Win32. TypeRef resolution returns all candidates rather than selecting the first.
AssemblyRef and local module scopes now filter those candidates through a borrowed iterator, so
resolution adds no second type-name index or per-query allocation. Nested TypeRef resolution
remains a future layer.

The first authoring checkpoint adds a bounded writer rather than porting the old mutable metadata
file object. Typed build identities distinguish definitions and references. A type-definition
callback owns field creation as one ordered operation, so callers cannot interleave field lists.
Failed callbacks roll back type, field, and constant rows. String and blob heaps are deduplicated,
constants are sorted by parent, and PE/CLI container emission is isolated from table construction.

`windows-rdl2` provides the second consumer. Its programmatic `Document` and `Module` model emits a
primitive enum and struct through metadata2. Both metadata2 and the existing reader accept the
image. The existing RDL compiler builds the same fixture, and the test compares normalized type
categories, flags, field lists, signatures, and enum values. The only raw encoding difference is a
direct `TypeDef` enum self-reference instead of the old writer's same-module `TypeRef`; both resolve
to the same type identity.

This is not yet evidence that the complete writer will be smaller. The table/heaps builder is about
580 lines and the PE/CLI container is about 135 lines while supporting only seven tables and
16-bit indexes. That cost is acceptable for the proof but must be reviewed before adding methods,
attributes, or parser compatibility. Named value fields and forward references are complete. The
external-reference slice is also complete: AssemblyRef rows are deduplicated, TypeRef identity
includes its scope, and RDL2 emits external value fields. A general parser remains deferred.

The follow-up complexity review removed a retained assembly-name string per image. Scoped
resolution now reads the already-owned Assembly row only when a same-named candidate must be
filtered. `TypeCandidates` remains a borrowed iterator over the existing exact-name index, so the
feature adds no second type map and no per-query allocation.

The writer still lists its seven row shapes and serialization order explicitly. Replacing that
with a general table-building framework would add indirection before a second table family proves
the required abstraction. Keep the bounded writer explicit until a concrete authoring feature
cannot be added without duplicated row-width, index, or ordering logic.

The native bindgen inventory added one read-only relationship: `Database::nested_types` streams
direct nested/enclosing TypeDef pairs from `NestedClass`. Metadata2 retains no reverse map.
Bindgen2 owns the ordered parent map because repeated enclosing lookup is projection policy.

Native interface projection added `Database::interface_implementations`, a streaming
`InterfaceImpl` view returning the implementing TypeDef and the referenced interface identity.
TypeSpec identities remain intact rather than being forced into a namespace/name pair. Bindgen2
owns the native base-interface map because inheritance and dependency closure require repeated
lookup.

Architecture projection added no new index or owner abstraction. TypeDef, Field, and MethodDef
views expose their existing custom-attribute relationship and call one shared decoder for
`SupportedArchitectureAttribute`.

Those three owners also share one private typed custom-attribute row scan. This removes duplicated
coded-index logic without adding a public owner trait or retaining another relationship index.

## Consumer overlap review

The first bindgen2/RDL2 comparison separates missing metadata support from valid consumer policy:

| Overlap | Decision |
| --- | --- |
| Constant encoding | Share in metadata2. Done. |
| Type and field flags | Shared typed wrappers. Done. |
| Primitive type lists | Review after named fields. |
| Definition identities | Stable declaration phase. Done. |
| External type scope | Shared AssemblyRef and TypeRef identities. Done. |
| Name indexes | Keep consumer-specific. |
| Errors and rendering | Keep consumer-specific. |

Primitive overlap alone does not justify merging bindgen2's ABI/output types with RDL2's source
types. Name lookup also serves different policies in each consumer. Typed ECMA flags and stable
build identities are the shared metadata mechanisms and now live in metadata2.

The builder declares TypeDefs first and defines their fields later in declaration order. This gives
named and forward-referenced fields stable typed IDs without sacrificing ECMA field-list ranges.
Failed definitions roll back their fields and constants and can be retried. RDL2 uses a nested
source-name map only to resolve its language names to those IDs. Builder callbacks preserve the
consumer's error type, so RDL2 can report source context without turning validation failures into
metadata errors.

The second differential fixture declares `Pixel` before its `Color` enum dependency. Metadata2
emits a direct TypeDef signature reference, and the old RDL writer emits a TypeRef. Both normalize
to the same named value field, with matching flags and constants.

The custom-attribute decoder preserves fixed and named argument types, field/property tags, null
strings, boxed values, arrays, `System.Type` names, and enum identities. Enum values use the
definition's `value__` field rather than assuming an `i32` backing type. A generated test covers a
local `u8`-backed enum, and every attribute in the committed WinRT and Win32 images decodes.

The corpus exposed one necessary dependency boundary: Win32 metadata references
`System.Runtime.InteropServices.CallingConvention` from the framework rather than defining it.
`EnumResolver` supplies backing types for enum dependencies outside the database; unresolved types
remain errors. This keeps dependency policy out of the ECMA decoder and avoids the old reader's
blanket `i32` assumption.

Top-level type indexing now uses the `NestedClass` table rather than treating an empty namespace as
nested. This preserves valid global-namespace definitions while excluding actual nested types and
the synthetic `<Module>` definition.

The next checkpoint is an inventory and benchmark of `windows-bindgen` lookups. New indexes should
be added only when a measured consumer operation cannot use table ordering, list ranges, or the
existing exact-name index.

The initial bindgen inventory separates metadata relationships from its output projection:

| Bindgen operation | Proposed source |
| --- | --- |
| Resolve namespace and type name | Existing exact-name index. |
| Enumerate fields, methods, and parameters | ECMA list ranges. |
| Attributes, constants, interfaces, and implementation maps | Binary search sorted tables. |
| Walk nested C structs | Derive from `NestedClass`; measure before storing a map. |
| Expand Win32 `Apis` and unscoped enum constants | Bindgen adapter projection. |
| Filter architecture variants | Decode attributes while projecting. |
| Trim generic arity for generated names | Bindgen adapter policy. |

This is materially smaller than reproducing the old `Index`: the database needs type identity and
resolution, while the bindgen adapter owns its one-time map of generated items.

One local optimized run over the committed WinRT and Win32 images measured 56.9 ms for image
parsing and 14.6 ms for database construction. The old reader and index took 82.4 ms together.
One hundred complete lookup passes over 50,943 distinct names took 232.8 ms. Separate optimized
processes for the new database and old index both used about 22 MB of working set; that process-wide
measurement is too coarse to claim an allocation improvement, but it found no memory regression.
These numbers are a development baseline, not performance thresholds.

The existing exact-name index is therefore retained. There is no evidence for replacing it with a
more complicated hashed-offset or interning design.

The first bindgen-facing semantic views expose type names, flags, categories, field and method list
ranges, and decoded member signatures without borrowed self-references. Their complete type shapes
match the existing reader. A test in `windows-bindgen` independently projects top-level types and
Win32 `Apis` functions and constants; namespace, name, and item-kind multiplicities match the old
index across both committed images.

Relationship queries use sort keys declared beside each table schema. The committed Windows images
leave the ECMA sorted-mask bits clear even though these tables are physically ordered. Image
construction now validates actual ordering once, uses binary range searches for proven ordered
tables, and falls back to a checked linear scan for unsorted input. This avoids reverse indexes
without trusting incorrect header hints.

Type attributes and field constants now use that range primitive. Attribute constructor ownership
is resolved through TypeDef method-list ordering rather than a method-to-type map. Attribute names,
field constant counts, type categories, flags, and member counts all match the existing reader
across the committed images.

`SupportedArchitectureAttribute` decoding now matches the existing reader for every indexed type.
The bindgen-side comparison also reproduces the complete `Reader` selection policy: remapped types,
WinRT categories, API contracts, Win32 `Apis`, scoped enums, and projected enum constants all have
the same namespace, name, kind, and multiplicity.

The output adapter must not recreate the old leaked row API merely to fit the current bindgen type
structures. The preferred next prototype is a bindgen-owned reader that owns `Database` and stores
`Entity<T>` identities in its projected type map. Semantic views stay borrowed and short-lived.
This will require some bindgen call sites to resolve entities through the reader, but it preserves
the metadata foundation instead of adding `Arc` ownership to every row handle or leaking the
database.

That reader prototype now exists beside the current bindgen reader in tests. It owns the database,
stores typed entities for every projected type, function, and constant, supports exact full-name
lookup, and resolves every stored identity back to its metadata name. A local optimized run built
the entity reader in 148 ms and the current leaked reader in 158 ms. These are development
measurements rather than performance thresholds.

Constant table values now decode booleans, characters, signed and unsigned integers, native-sized
integers, floating-point values, UTF-16 strings, and null class constants. Every constant in the
committed WinRT and Win32 images matches the existing reader.

The next output step is a bindgen design change rather than another metadata index. Existing
bindgen types embed `'static` row aliases and `TypeName` string references from the leaked index.
Adapting metadata2 to those types would reintroduce the leak or spread ownership into every row
handle. Instead, extract one metadata-neutral output model - starting with WinRT structs - that
both readers can populate and the existing renderer can consume. Continue only if that boundary
stays small; do not build a compatibility copy of the old reader API.

The first struct-model prototype remains test-side while its boundary is evaluated. It owns type
and field names and represents primitive, string, named value, named class, and generic-instance
field identities without metadata row lifetimes. Both readers produce equal models for every
supported struct definition; more than 3,000 committed definitions are covered.

A flat owned map of WinRT enum and struct models now supplies recursive copyability, equality,
runtime signatures, and relative namespace paths. It is not a general type index: classes and
interfaces are retained only as field identities, and their runtime signatures are not modeled.
Both readers produce the same value map, including enum backing types and generic-name trimming.
Models populated from metadata2 exactly render more than 120 of the 123 WinRT structs selected by
the existing bindgen reader. The remaining gap is the generic `IReference<u64>` class field. Do not
add the interface graph merely to close that gap; first decide whether production rendering should
consume this owned value map.

The same owned map now carries enum backing types and literal constants. Metadata2 models render
more than 1,000 projected WinRT enums exactly, including runtime signatures and the `u32` flag
operators. The test prototype is named `value_model` rather than `struct_model` because enums and
recursive struct semantics share the same small boundary. Constants remain integer-only in this
model because ECMA enums cannot use the other constant forms. Stable bindgen policy such as the
flag-operator renderer is reused rather than copied into the metadata migration layer.

A critical review replaced coverage thresholds and silent skips with exact accounting. Every
projected WinRT enum renders, and the only projected struct that does not render is
`Windows.Web.Http.HttpProgress`, whose generic `IReference<u64>` fields need parameterized
interface runtime signatures. Value-map construction rejects duplicate full names rather than
allowing `BTreeMap::collect` to overwrite them. The shared field/signature type is named
`ModelType`; it is not struct-specific.

The owned value map should remain small and global because recursive struct traits and signatures
need cross-type queries. This does not justify a global owned model for classes, interfaces,
delegates, and every method. Those items should be resolved from their stored `Entity<T>` and
materialized one at a time for rendering. This keeps database ownership in `Reader2`, avoids both
leaked rows and duplicated metadata strings, and prevents the migration layer from becoming a
second bindgen type graph.

The revised bindgen sequence is:

| Step | Scope | Stop condition |
| --- | --- | --- |
| Value output | Small global enum/struct map. | Done except `HttpProgress`. |
| Callable prerequisites | Generics, parameters, and GUIDs. | No new indexes. |
| Delegate output | Materialize one delegate at a time. | Do not copy interface/class policy. |
| Interface output | Add inheritance after delegates. | No second global type graph. |
| Class output | Compose proven interface identities. | No leaked-row adapter. |
| Production extraction | Replace one source path at a time. | Full output must match. |

Type-owned generic parameters are now exposed through the existing sorted `GenericParam` table.
Sequence, flags, and names match the current reader across the committed corpus. This is the first
callable prerequisite and required no new index.

Method parameter rows now use the same generic list-range primitive as type fields and methods.
Flags, sequence numbers, and names match the current reader across the committed corpus. Sequence
association is a separate checked semantic step: sequence zero is the optional return row, missing
parameter rows remain `None`, and duplicate or out-of-range sequences return structured errors.
The complete committed corpus matches the current reader's association.

`GuidAttribute` folding stays in the bindgen adapter rather than becoming ECMA metadata policy.
The adapter validates the 11 fixed argument types and reproduces every type GUID from the current
reader. Generic parameters, checked parameter association, and GUID projection are now sufficient
to inventory a per-item delegate model before adding rendering.

The delegate inventory must come before extending `ModelType`. Method signatures contain more
shapes than value fields, and copying every existing bindgen `Type` variant would defeat this
exercise. The next checkpoint should list the shapes used by projected WinRT delegates, then add
only the metadata-neutral projection operations required by that corpus.

The committed WinRT delegate inventory is narrow: primitives, `void`, strings, objects, named
classes and values, generic type parameters, generic instances, and vectors. It contains no
pointers, by-reference types, multidimensional arrays, generic method parameters, function
pointers, typed references, or pinned types. The old and new readers produce the same shape set and
delegate count. A delegate model can therefore start with this subset and reject any newly observed
shape until its projection behavior is designed.

The first per-item delegate model now owns only the data required by one delegate: namespace, name,
generic parameters, GUID, call flags, return metadata, and sequence-aligned parameter metadata.
Models populated by both readers match for every committed WinRT delegate. Delegate signatures
reuse the existing metadata-neutral `ModelType`; review rejected a separate callable type enum as
immediate duplication. Nested generic and vector elements retain the same modifier checks rather
than bypassing them during recursion.

Default parameter names and input-only direction behavior also match the current bindgen
projection for every delegate. This is tested separately from raw metadata-model parity so a
projection-policy change cannot hide inside equal reader models.

Delegate identity rendering is now a shared metadata-neutral helper. Production and metadata2
provide tokenized names, generic constraints, phantom fields, and GUID values to the same renderer;
every generic and non-generic WinRT delegate definition matches exactly. This extraction covers
only the interface identity and runtime-signature declaration. Constructor, public `Invoke`, ABI
vtable, and upcall generation still use the existing `Method` renderer.

The helper intentionally accepts rendering inputs rather than metadata rows or `ModelType`. This
keeps it reusable without turning the production renderer into a metadata2 adapter. The next slice
is the callable signature used by the vtable and public `Invoke`. Stop and split it further if
matching that slice requires copying `Method::write`, `write_abi`, or upcall generation.

The ABI vtable signature now matches for every WinRT delegate. Its first prototype copied the
array, direction, and return-parameter branches from `Method::write_abi`; review rejected that
duplication. Those branches now live in one metadata-neutral `write_abi_signature` helper.
Production supplies ABI tokens from the existing `Type` model, while metadata2 supplies equivalent
tokens from `ModelType` and the value map.

`ModelType` gained only the ABI operations required by the delegate corpus: primitive values,
strings and object references, generic ABI types, vectors, projected system value types, named
interfaces, enums, and copyable/non-copyable structs. The next slice is the public callable
signature. Upcall bodies and method execution policy remain outside the migration model.

## Bindgen migration review

The side-by-side proof has reached its useful limit. Continuing to adapt the existing generator is
not the recommended path.

The existing bindgen source is about 12,800 lines across 49 Rust files. The metadata2 proof already
adds about 1,900 test-side lines, before class, interface, package, filtering, dependency closure,
or full method rendering has migrated. Each new output slice now requires both a metadata2 model
and extraction of policy from production types whose identities still contain leaked rows and
static strings. Although the extracted delegate identity and ABI helpers are clean and
output-neutral, repeating this process across the generator would spend substantial effort
untangling an architecture that would then be replaced.

The public package boundary is much smaller than the implementation. Consumers use the `bindgen`
function or `Bindgen` builder. A new engine can preserve that surface later without preserving the
current internal `Reader`, `Type`, `TypeName`, `Config`, and leaked-index design.

Validation favors a clean replacement:

- committed `bindings.rs` files provide roughly 80,000 lines of immediate output comparison;
- `tool_bindings`, `tool_package`, `tool_webview`, and `tool_reactor` cover the main generation
  modes;
- bindgen golden tests cover small focused WinRT and Win32 cases;
- committed `windows` and `windows-sys` package output provides the full-corpus oracle;
- the metadata2 differential tests already validate the source facts independently.

This review led to the unpublished `windows-bindgen2` crate beside the existing generator. The old
generator remains the oracle and production implementation until the new crate matches all
required output. Bindgen2 does not adapt the old bindgen types.

The initial architecture should have these boundaries:

| Layer | Responsibility |
| --- | --- |
| Metadata | `windows-metadata2::Database` and typed entities. |
| Selection | Bindgen policy for filters, remaps, `Apis`, contracts, and architecture. |
| Closure | Required type and namespace dependencies, stored by owned entity or owned name. |
| Projection | Per-item lowering into small metadata-neutral models. |
| Rendering | Deterministic tokens with no metadata row access. |
| Output | Flat, module, and package layout plus formatting and file writes. |

Only the value graph should remain global because recursive value semantics require it. Methods,
delegates, interfaces, and classes should be lowered one item at a time. No database leak, static
metadata strings, compatibility clone of the old reader, or global second type graph should be
introduced.

Suggested checkpoints:

1. Create `windows-bindgen2` with a `Database`-owning generator and deterministic item selection.
2. Move the proven enum and struct models into it and match focused golden output.
3. Add Win32 constants and functions, then prove flat sys output.
4. Add delegates using the proven per-item model and shared rendering rules.
5. Add interfaces and classes only after callable output is stable.
6. Add filter closure, module layout, and package layout independently.
7. Run every generation tool and require no tracked output differences.
8. Replace the implementation behind the existing `windows-bindgen` API, then retire the old
   engine and temporary crate name.

The current bindgen proof should now be treated as design evidence. Do not continue extracting the
public callable signature or additional production renderers unless a finding is needed to design
the new engine.

The first two checkpoints are complete. `windows-bindgen2::Generator` owns the database and selects
1,731 WinRT enums and 125 projected structs from the committed metadata. It stores only typed
entities and value categories. Temporary namespace/name strings make construction sorting cheap
and are discarded afterward; adding another metadata index would not improve retained state or a
measured lookup path.

The separate owned value graph renders the full selected corpus, including the parameterized
`IReference<u64>` fields that blocked `HttpProgress` in the earlier proof. Nested ordered maps avoid
allocating lookup keys, recursive value cycles are checked, and GUID folding occurs only when a
generic named type needs a parameterized-interface signature. Focused enum and struct fixtures
match existing golden tokens.

The first Win32 checkpoint added checked `MethodDef -> ImplMap -> ModuleRef` semantic views and
field custom-attribute views. Import names, modules, and flags match the existing reader across the
committed Win32 metadata. Bindgen2 now lowers and renders every selected Win32 function and
constant. Native typedef chains are resolved per constant rather than stored in a global graph, and
GUID-backed constants use the same checked attribute decoding as parameterized WinRT signatures.

Native type output required class packing without adding more unrelated code to `semantic.rs`.
`semantic_layout.rs` now owns the checked `TypeDef -> ClassLayout` relationship and its differential
corpus test. Bindgen2 retains 30,109 top-level native type entities and lowers each definition only
while rendering. The corpus contains 12,666 aliases, 4,728 enums, and 12,715 structs or unions.
This proves that fixed arrays, explicit unions, alignment, and packing do not require a global
native type graph.

The count does not establish full output equivalence. Architecture variants, nested native types,
bitfield accessors, handles, native delegates, and interfaces still need independent policy and
golden comparisons. They should not be folded into a broad replacement for the old `CppStruct`
graph.

Raw source size needs continued review. Metadata2 is already near the old metadata crate's line
count even though it does not have writing or merging. Much of that difference is checked parsing
and in-crate differential testing, and the implementation uses fewer concepts and files, but line
count is no longer evidence of simplicity by itself. `image.rs` and `semantic.rs` should be split
by concern before they accumulate more unrelated behavior.
