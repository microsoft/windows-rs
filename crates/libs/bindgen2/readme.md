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

`Metadata` eagerly validates and lowers immutable catalogs once. Each `Generator` retains only the
typed entities and selection policy for one request. Formatting, file writing, package staging,
and legacy command parsing remain the responsibility of callers or a later tool-facing facade.

The current implementation covers WinRT enums, structs, delegates, and interfaces plus native sys
types, constants, functions, delegates, and interfaces. Filtered dependency closure, nested and
flat layouts, architecture gates, and deterministic ordering are implemented. Rich/minimal
projection, WinRT classes, member filters, and package output remain future work.

The design and differential evidence are tracked in
[`docs/crates/windows-bindgen2.md`](../../../docs/crates/windows-bindgen2.md).
