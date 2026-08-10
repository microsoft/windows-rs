# windows-rdl2

`windows-rdl2` is an unpublished authoring proof for a smaller replacement of
[`windows-rdl`](windows-rdl.md). It is a second consumer of
[`windows-metadata2`](windows-metadata2.md), complementing the read-only projection exercised by
`windows-bindgen2`.

## Current boundary

The first checkpoint intentionally uses a programmatic model rather than a text parser:

- `Document` owns assembly-level input.
- `Module` owns one metadata namespace.
- primitive integer enums have checked backing values;
- simple structs contain primitive or named value-type fields;
- definitions may reference types declared later in the document;
- fields may reference a value type in a named external assembly;
- source validation errors retain definition, variant, and field identities;
- compilation emits a PE/CLI image through `windows-metadata2`.

The resulting image is read by both metadata2 and the existing metadata reader. The focused test
also compiles the same fixture with the existing RDL compiler and compares normalized type
categories, flags, field lists, signatures, and enum constant values.

## Metadata2 authoring foundation

The initial metadata builder supports only the tables required by this proof: `Module`, `TypeRef`,
`TypeDef`, `Field`, `Constant`, `Assembly`, and `AssemblyRef`. It has typed definition/reference
identities, deduplicated string and blob heaps, checked compressed signatures, sorted constants,
and separate declaration and ordered-definition phases. Failed field callbacks roll back their
rows. The callback error is consumer-defined, so RDL2 validation failures are not represented as
metadata construction errors.

Table/heaps and PE/CLI container serialization are separate modules. The writer currently uses
16-bit heap and table indexes and returns an explicit error when the bounded proof exceeds them.
This is preferable to adding untested large-image machinery before another consumer requires it.

## Critical assessment

The proof establishes that metadata2 is not limited to reading and that a second consumer can use
a small API without leaked rows or the old writer's broad mutable file object. RDL2 now owns source
diagnostics, while metadata2 reports only construction failures. It does not yet prove that the
authoring design scales:

- metadata2 authoring is already about 715 lines for seven tables and the PE/CLI container;
- text parsing, attributes, methods, and interfaces are absent;
- deterministic row remapping and 4-byte indexes are not implemented;
- generated output is semantically equivalent for the first fixture but is not byte-for-byte equal.

The first comparison found one harmless encoding difference: enum member fields reference their
enclosing enum through a direct `TypeDef`, while the old writer emits a same-module `TypeRef`.
Metadata2 resolves both to `Test.Color`, so the differential test compares resolved identities
rather than preserving an incidental row encoding.

Review against bindgen2 found one immediate common layer: metadata2 now uses the same
`ConstantValue` and ECMA element codes for reading and writing. Primitive source types remain in
RDL2, while bindgen2 keeps its separate WinRT and native projection types. Those types encode
different policy and should not be merged just because their primitive cases overlap.

The larger shared issue was definition identity. Metadata2 now declares all TypeDefs before
emitting fields and requires field bodies in declaration order. RDL2 keeps only its source-name
index and maps those names directly to stable metadata IDs; it has no metadata patch table.

External fields add one source form carrying assembly, namespace, and type name. RDL2 collects
those references before field emission. Metadata2 deduplicates AssemblyRef and TypeRef rows, and
database resolution filters same-named definitions by the encoded assembly scope. A fixture
includes a same-named type in an unrelated assembly to prove the scope is honored.

The complexity review kept local definitions and external references as separate maps because
their failure policy differs, but both maps now borrow names from the document instead of copying
them. No shared source/projection type model or general metadata row builder is justified.

Do not expand the parser or authoring table set next. Return to bindgen2's architecture and nested
native definition inventory before choosing another implementation slice.
