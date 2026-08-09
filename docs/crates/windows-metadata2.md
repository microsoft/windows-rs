# windows-metadata2

`windows-metadata2` is an unpublished replacement candidate for
[`windows-metadata`](windows-metadata.md). It is developed beside the existing implementation so
each layer can be compared against the current Windows metadata corpus before consumers migrate.

The first layer owns and structurally validates a PE/CLI metadata image. Successful construction
guarantees that the metadata root, stream directory, table header, row counts, table widths, and
table byte ranges are in bounds. The complete ECMA-335 table schema is declared once and drives
row-width calculation.

Current exclusions are intentional:

- semantic row and heap access;
- signature and custom-attribute decoding;
- metadata construction and serialization;
- common, Win32, and WinRT validation;
- architecture merging and namespace remapping;
- `windows-bindgen` and `windows-rdl` integration.

These capabilities will be added in independent changes with differential tests against the
existing implementation. The crate remains unpublished until both `windows-bindgen` and
`windows-rdl` can use it without changing generated output.
