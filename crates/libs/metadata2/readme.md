# windows-metadata2

An unpublished replacement candidate for `windows-metadata`.

The crate is being developed beside the existing implementation so its parser, data model, and
writer can be proven independently before any consumer migration. Its current scope is checked
PE/CLI container parsing, ECMA-335 table layout, typed table-local row IDs, and checked string,
blob, and GUID heaps. Raw row views decode primitive, heap, direct-table, list-start, and
coded-index columns, and the signature layer decodes all signature-bearing rows in the committed
Windows metadata. An owned multi-image database adds stable file/entity identities and preserves
duplicate type definitions during name lookup. Constructor-directed custom-attribute decoding
preserves fixed and named values and resolves enum storage from local definitions or an explicit
dependency resolver. Initial semantic views expose type categories and field and method list ranges
for a read-only `windows-bindgen` projection. Sorted relationship tables are validated once and
queried without reverse indexes, and Constant table values are decoded losslessly. The crate does
not yet provide the remaining semantic table relationships, validation policy, merging, or
namespace remapping.

The first bounded writer emits modules, assemblies, type references, type definitions, fields, and
constants with primitive and named field signatures. It uses typed build identities and a scoped
declaration/definition sequence so forward references are stable while field-list ranges remain
ordered. Builder callbacks may return consumer-defined errors while metadata2 still rolls back
partial fields and constants. Shared type and field attribute wrappers remove raw ECMA flag masks
from consumers. Deduplicated assembly references and scope-aware type references support external
value fields. Database resolution prefers file or assembly scope without another type-name index.
Assembly references fall back to exact-name candidates when the requested assembly contributes no
definition, which supports merged contract metadata. The writer is a proof for `windows-rdl2`,
not yet a general metadata construction API.

The reader exposes `NestedClass` as a streaming sequence of nested/enclosing semantic type pairs.
It retains no reverse nested-type index; consumers that need repeated parent lookup own that map.
Type, field, and method semantic views share one checked `SupportedArchitectureAttribute` decoder.
Parameter semantic views expose the same checked custom-attribute lookup used by other members.
