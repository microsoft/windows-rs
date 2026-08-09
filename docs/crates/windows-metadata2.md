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
- metadata construction and serialization;
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
| `windows-bindgen` reader adapter | Next | Full generated Rust output matches the existing reader. |
| Deterministic metadata builder | Planned | Finalization returns a queryable image and stable row remapping. |
| `windows-rdl` builder adapter | Planned | WinRT, Win32, and WDK output remains equivalent. |
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
Assembly-scope narrowing and nested TypeRef resolution remain future resolution layers; callers
can see ambiguity explicitly in the meantime.

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
