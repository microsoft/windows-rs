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
not yet provide the remaining semantic table relationships, metadata writing, validation policy,
merging, or namespace remapping.
