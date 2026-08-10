# windows-bindgen2

An unpublished replacement candidate for `windows-bindgen`.

The crate is developed beside the existing generator and uses `windows-metadata2` directly.
`Metadata` owns one validated database, and each `Generator` request shares it through one
root-level reference count while retaining only typed metadata entities. A separate owned value
graph now renders the complete committed
WinRT enum and struct corpus. The first flat Win32 layer selects constants and functions without
copying per-item namespaces and renders the complete committed constant and function corpus. It
also lowers the 30,109 top-level native aliases, enums, structs, and unions through a separate
per-item model. A bounded output layer groups these supported items into deterministic namespace
modules and matches the existing nested-module golden output. Filters, dependency closure, nested
native types, native delegates, native interfaces, and flat output are implemented. Rich
projection, member-level filters, file writing, and package output remain future layers.

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
The validated `GUID` shape is the one fixed-array exception and derives `Default`. The corpus
contains 8,584 deriving structs and 4,130 manual implementations. Resolution is an ephemeral
per-root traversal rather than another retained native graph.

All 10 scoped native enums now use the existing transparent-newtype sys projection with associated
constants. Ordinary C enums remain integer aliases with module-level constants. The distinction is
stored on each independently lowered enum and requires no enum registry.

All 2,159 native delegates now lower and render as optional unsafe function-pointer aliases. The
43 architecture-gated rows sort and gate independently. Functions and delegates share one owned
native signature model for parameter naming and type rendering; import policy and delegate
calling-convention attributes remain separate.

All 4,290 native interfaces and 25,868 methods lower to sys vtables. The model stores one optional
base name, an optional COM IID, and ordered ABI methods. Bindgen2 retains one `InterfaceImpl`
relationship map for inheritance and closure; projected interface models remain per-item values.
The 14 architecture-gated interfaces and independently gated methods use the same target mappings
as other native items.

The 1,228 bitfield members belong only to rich bindings; the existing sys generator emits their
coalesced backing fields without accessors. Sys handle aliases already follow native-typedef
policy, including the one primitive `Value` shape lacking `NativeTypedefAttribute`.

This request boundary allows tools with many filter files to parse and index metadata once. It
does not add per-row reference counting or change metadata identities. The compatibility CLI and
file-writing API remain deferred until filtering and dependency closure can make each request
selective.

`Options` currently exposes only the implemented layout choice: nested namespace modules or flat
output. Flat generation rejects names contributed by different namespaces rather than emitting
invalid duplicate Rust items. Style flags are intentionally absent until rich and minimal
projection policy exists.

`Filter` is a programmatic selection model, not a command-line parser. It includes bare item names,
exact namespace/name pairs, or namespace trees. An empty filter selects nothing; an unfiltered
request selects the complete implemented surface. Bare scoped-enum variant names retain only the
requested variants when the enum is otherwise a dependency. General member filters, exclusions,
and string grammar remain separate future layers.

Each request stores its selected WinRT and Win32 typed entities. Repeated writes borrow that
selection instead of scanning the metadata database again. Native projected models are still
lowered one item at a time and discarded after rendering.

Filtered Win32 requests close transitively over supported native definitions referenced by field
and method signatures. This includes aliases, enums, structs, delegates, interfaces, base
interfaces, and every architecture variant of a referenced metadata name. Closure uses an
ephemeral entity set and work queue; it does not retain a dependency graph.

The nine `tool_bindings` `--sys` requests run against one shared metadata database and match their
committed generated output. The comparison ignores only function-pointer parameter names and
rustfmt-added trailing commas. This covers filtered functions, constants, structs, aliases,
delegates, interfaces, IIDs, string aliases, SAL input constness, enum variants, and closure.
