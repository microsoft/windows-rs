# windows-metadata2

An unpublished replacement candidate for `windows-metadata`.

The crate is being developed beside the existing implementation so its parser, data model, and
writer can be proven independently before any consumer migration. Its current scope is checked
PE/CLI container parsing and ECMA-335 table layout. It does not yet provide semantic row access,
metadata writing, validation policy, merging, or namespace remapping.
