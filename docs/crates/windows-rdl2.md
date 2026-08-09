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
- simple structs contain primitive fields;
- compilation emits a PE/CLI image through `windows-metadata2`.

The resulting image is read by both metadata2 and the existing metadata reader. The focused test
also compiles the same fixture with the existing RDL compiler and compares normalized type
categories, flags, field lists, signatures, and enum constant values.

## Metadata2 authoring foundation

The initial metadata builder supports only the tables required by this proof: `Module`, `TypeRef`,
`TypeDef`, `Field`, `Constant`, `Assembly`, and `AssemblyRef`. It has typed definition/reference
identities, deduplicated string and blob heaps, checked compressed signatures, sorted constants,
and a scoped type-definition callback that preserves field-list ordering. Failed callbacks roll
back their rows.

Table/heaps and PE/CLI container serialization are separate modules. The writer currently uses
16-bit heap and table indexes and returns an explicit error when the bounded proof exceeds them.
This is preferable to adding untested large-image machinery before another consumer requires it.

## Critical assessment

The proof establishes that metadata2 is not limited to reading and that a second consumer can use a
small API without leaked rows or the old writer's broad mutable file object. It does not yet prove
that the authoring design scales:

- the builder is already about 650 lines for seven tables and the PE/CLI container;
- text parsing, named field types, attributes, methods, interfaces, and references are absent;
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

The larger shared issue is definition identity. Named fields and forward references need stable
metadata definition IDs before field bodies are emitted. That belongs in the metadata2 builder;
RDL2 should not grow a separate metadata name/patch map.

Do not expand the parser next. Add named value-type fields and a second fixture that exercises
cross-definition references after metadata2 supports stable declarations. Then review whether the
low-level builder remains simpler before adding another table family.
