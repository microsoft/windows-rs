# windows-bindgen2

An unpublished replacement candidate for `windows-bindgen`.

`windows-bindgen2` projects an owned `windows-metadata2` database into deterministic Rust tokens.
It is developed beside the existing generator so each implemented output layer can be compared
before migration.

The public API separates reusable metadata, request selection, and rendering:

```rust,ignore
let metadata = windows_bindgen2::Metadata::from_images(images)?;
let generator = metadata.generator(windows_bindgen2::Request::filtered(filter))?;
let tokens = generator.render(windows_bindgen2::Layout::Flat)?;
```

`Metadata` validates images and lowers immutable catalogs once. Relationship errors that do not
affect a request are deferred until that request traverses them. Each `Generator` retains only the
typed entities and selection policy for one request. Package staging and legacy command parsing
remain the responsibility of callers.

The `builder` facade supports flat build-script consumers that use path or default metadata,
filters, default or sys projection, implementation generation, rustfmt, and changed-file writes.
It is intentionally narrower than the existing `windows-bindgen` builder. Unsupported policy is
added only when a real consumer has a differential test.

The current implementation covers WinRT enums, structs, delegates, interfaces, and classes plus
native sys types, constants, functions, delegates, and interfaces. Filtered dependency closure,
nested and flat layouts, architecture gates, and deterministic ordering are implemented.
Member filtering and explicit interface implementation selection are implemented. A bounded native
COM path covers filtered interface identity, vtable placeholders, callable HRESULT wrappers, shell
dependencies, complete-interface producer vtables, and generic query methods for the animation and
core requests.

`tool_bindings` is the first production consumer. Its 17 flat requests share one `Metadata` value
and select default, sys, or minimal output. The tool owns command parsing, rustfmt compatibility,
and file writes. Fifteen standalone build-script consumers now match their committed output exactly.
They include the activation, overload, constructor, and composable client/producer pairs,
composable aggregation, ref-parameter and `NoException` producers, benchmark and robot components,
the context-alignment sample, and the Win32 metadata slice. These requests prove class-wide overload
naming, exclusive WinRT implementation generation, composable activation and aggregation,
output-array and `IReference<T>` authoring, canonical external crates, and direct-return native COM
producers. `tool_reactor` remains on `windows-bindgen`; its class selection, hierarchy, event, and
producer policies need bounded parity work before migration. Package output and broader COM policy
remain future work.

The design and differential evidence are tracked in
[`docs/crates/windows-bindgen2.md`](../../../docs/crates/windows-bindgen2.md).
