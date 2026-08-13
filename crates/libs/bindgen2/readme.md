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
typed entities and selection policy for one request.

The `builder` facade supports build-script and command-file consumers that use path or default
metadata, include and exclusion filters, default or sys projection, implementation generation,
flat output, rustfmt settings, and changed-file writes. Package layout is implemented for
differential testing but does not yet match the published crates.

The current implementation covers WinRT enums, structs, delegates, interfaces, and classes plus
native sys types, constants, functions, delegates, and interfaces. Filtered dependency closure,
nested and flat layouts, architecture gates, and deterministic ordering are implemented.
Member filtering and explicit interface implementation selection are implemented. A bounded native
COM path covers filtered interface identity, vtable placeholders, callable HRESULT wrappers, shell
dependencies, complete-interface producer vtables, and generic query methods for the animation and
core requests.

`tool_bindings` is the first production consumer. Its 17 flat requests share one `Metadata` value
and select default, sys, or minimal output. The tool owns command parsing, rustfmt compatibility,
and file writes. Twenty standalone build-script consumers now match their committed output
exactly.
They include the activation, overload, constructor, and composable client/producer pairs,
composable aggregation, ref-parameter and `NoException` producers, benchmark component and client,
robot component and client, the reference library test, the context-alignment and service-time
samples, the `windows-core`-only API slice, and the Win32 metadata slice. These requests prove
class-wide overload naming, exclusive WinRT implementation generation, composable activation and
aggregation, output-array and `IReference<T>` authoring, WinRT field-name policy, custom native
derives, canonical external crates, rich standalone native functions, and direct-return native COM
producers. `tool_composition`, `tool_reactor`, and `tool_webview` also run through bindgen2 with
exact committed-output parity. `tool_package` remains on `windows-bindgen`; bindgen2 package
generation still differs across hundreds of generated files.

The design and differential evidence are tracked in
[`docs/crates/windows-bindgen2.md`](../../../docs/crates/windows-bindgen2.md).
