# windows-bindgen2

An unpublished replacement candidate for `windows-bindgen`.

The crate is developed beside the existing generator and uses `windows-metadata2` directly. Its
first layer owns the metadata database and selects deterministic WinRT value items while retaining
only typed metadata entities. A separate owned value graph now renders the complete committed
WinRT enum and struct corpus. The first flat Win32 layer selects constants and functions without
copying per-item namespaces and renders the complete committed constant and function corpus. It
also lowers the 30,109 top-level native aliases, enums, structs, and unions through a separate
per-item model. A bounded output layer groups these supported items into deterministic namespace
modules and matches the existing nested-module golden output. Filters, dependency closure, nested
native types, interfaces, delegates, flat output, and package output remain independent future
layers.
