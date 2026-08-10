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

Corpus inventory found 1,054 architecture-specific top-level rows, of which 997 are selected
native enum/struct definitions, and 2,633 nested native structs under 1,925 direct parents.
Bindgen2 retains one parent-to-children map for those nested rows; metadata2 keeps only a streaming
relationship view. Native lowering recursively attaches all nested rows, assigns positional names
such as `Outer_0_0`, and resolves empty-namespace field references only against the current
enclosing definition.

Architecture gates are emitted for all 997 selected native types, 512 constants, and 261 functions
that carry the attribute. Duplicate variants sort by name and architecture bits. Native enum
members are flattened at the module-output boundary so their ordering matches the existing
generator without adding a global enum graph.

A true `NestedClass` fixture matches the existing generator for multiple children, deep nesting,
packing, unions, and inherited architecture gates. Nested unions also force manual `Default`
implementations through their enclosing subtree, matching the existing layout policy.

Native `Default` policy now matches the existing sys generator for explicit layout propagated
through by-value fields, direct fixed arrays, fixed-array typedef chains, and scoped-enum fields.
The corpus contains 8,584 deriving structs and 4,131 manual implementations. Resolution is an
ephemeral per-root traversal rather than another retained native graph.

All 10 scoped native enums now use the existing transparent-newtype sys projection with associated
constants. Ordinary C enums remain integer aliases with module-level constants. The distinction is
stored on each independently lowered enum and requires no enum registry.
