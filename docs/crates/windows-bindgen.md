# windows-bindgen

> The code generator that turns Windows metadata (`.winmd`) into Rust bindings.

- 📦 [crates.io](https://crates.io/crates/windows-bindgen)
- 📖 [docs.rs](https://docs.rs/windows-bindgen)
- 🚀 [Getting started](../../crates/libs/bindgen/readme.md)
- 🧩 [Samples](../../crates/samples/bindgen)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/bindgen)

`windows-bindgen` generates Rust bindings from Windows metadata. It powers the `windows` and
`windows-sys` crates. You can also use it from `build.rs` to make a small binding set for the APIs
that your crate calls.

The crate includes the standard Windows metadata through
[`windows-default`](windows-default.md). Most projects only need an output file and a filter. If an
API has no metadata, use [`windows-rdl`](windows-rdl.md) to create a `.winmd` file. Then pass that
file to `windows-bindgen`.

## Getting started

Add `windows-bindgen` as a build dependency. Add `windows-link` or `windows-core` as the runtime
dependency that the generated code uses:

```toml
[dependencies]
windows-link = "0.100"

[build-dependencies]
windows-bindgen = "0.100"
```

Generate bindings from `build.rs` with command-line-style arguments, a command file, or the builder:

```rust,no_run
windows_bindgen::bindgen([
    "--out", "src/bindings.rs",
    "--flat",
    "--sys",
    "--filter", "GetTickCount",
]);
```

```rust,no_run
windows_bindgen::Bindgen::new()
    .output("src/bindings.rs")
    .flat()
    .sys()
    .filter("GetTickCount")
    .write();
```

Include the generated file as a module in your crate.

## Filters

A filter selects which APIs appear in the output. Rule specificity controls how much of a type is
generated. This works like a Rust `use` declaration. Use a bare name for the full item. Use braces
to select a smaller surface.

- A namespace, such as `Windows.Win32.System.Com`, includes all types under it.
- A bare type includes the full type. Examples are `HWND`, `OSVERSIONINFOEXW`, and
  `Windows.Win32.Foundation.HWND`.
- `Namespace.Type::{}` emits a name-only shell. Use it for a dependency that you only pass through
  signatures.
- `Namespace.Type::{Method1, Method2}` emits only the named methods. `Namespace.Type::Method` is the
  single-method form.
- `Property` and `Event` names expand to accessor pairs. Properties expand to `get_` and `put_`.
  Events expand to `add_` and `remove_`.
- `Namespace.Class::CreateInstance` emits class activation support. A bare class projects its
  default interface but no constructor.

Prefix a rule with `!` to exclude it. A selected type also pulls in the types that its signatures
require. Those dependency types are emitted as shells.

For a complete command file, use `--etc`. Blank lines and lines whose first non-whitespace
characters are `//` are ignored:

```text
--out crates/libs/version/src/bindings.rs
--flat --sys

--filter
    RtlGetVersion
    OSVERSIONINFOEXW
    VER_NT_WORKSTATION
```

```rust,no_run
windows_bindgen::bindgen(["--etc", "bindings.txt"]);
```

When only the filter list is large, keep it in a filter-only file and use
`Bindgen::filter_file`/`filter_files` or the textual `--filter-file` option.

The in-repo crates use both patterns. `tool_bindings` runs
`bindgen(["--etc", "crates/tools/bindings/src/<crate>.txt"])` for each library.

## Choosing the output shape

Two independent choices control the generated code. The first choice is style. The second choice is
layout.

Style:

- Default style emits rich bindings. It includes class wrappers, inherited-interface forwarders,
  handle types, and free-function wrappers. The `windows` crate uses this style.
- `--sys` or `.sys()` emits raw FFI. It emits bare `extern` functions and plain structs. It links
  through `link!` macros. Add `--extern` or `.extern_fns()` to emit `extern { fn ... }` blocks
  instead of `link!`. The `windows-sys` crate uses this style.
- `--minimal` or `.minimal()` starts from default style. It omits per-class wrappers, inherited
  forwarders, handle helpers, and free-function wrappers. Use it for small binding sets.
  `windows-canvas` and `windows-reactor` use it. It is mutually exclusive with `--sys`.

WinRT event accessors are always collapsed into an `Event` wrapper. This applies to all styles and
layouts. See [Event accessors](#event-accessors).

### Variadic native functions

Native variadic exports carry `MethodCallAttributes::VARARG` in the method signature. Only
`--sys` emits them, because sys output can retain the literal `...` tail in a raw foreign
declaration. Both `link!` output and `--sys --extern` preserve metadata `C` and `system` calling
conventions. Rust lowers a Windows `system` C-variadic declaration to the compatible C variadic ABI
on X86 while retaining `system` for fixed signatures.

Default and minimal output cannot forward an unknown variadic tail through a Rust wrapper. Broad
filters omit those exports. Selecting one by exact function name reports that rich and minimal
bindings cannot project it and directs the caller to `--sys`; it never emits the fixed prefix as a
callable function. Stable Rust cannot declare a `fastcall` C-variadic function, so broad sys
generation omits that metadata shape and exact selection reports the unsupported convention.

The published `windows-sys` crate retains raw declarations such as
`AuthzReportSecurityEvent(...)`. The `windows` crate omits Win32 and WDK variadic exports because
the default projection cannot preserve their argument tails.

Layout:

- The default layout emits one Rust module per metadata namespace.
- `--flat` or `.flat()` emits one flat list of items.
- `--package` or `.package()` emits one file per namespace. It also writes a `Cargo.toml` with
  per-namespace features. The `windows` and `windows-sys` crates use this layout. It is mutually
  exclusive with `--flat`.

The style and layout choices are independent. The repository uses only the combinations below.

| Style + layout        | Purpose                                           | Examples                                  |
| --------------------- | ------------------------------------------------- | ----------------------------------------- |
| default + `--flat`    | Helper crate with one bindings file              | `windows-collections`, `windows-future`   |
| default + `--package` | Published umbrella crate                         | `windows`                                 |
| `--sys` + `--flat`    | Raw FFI helper crate with one bindings file      | `windows-result`, `windows-registry`      |
| `--sys` + `--package` | Published raw FFI crate                          | `windows-sys`                             |
| `--minimal` + `--flat`| Small binding set                                | `windows-core`, `windows-canvas`, `windows-reactor` |
| any + modules         | Namespace-per-module output for direct consumers | External binding generation               |

`--minimal` and `--package` are not used together. Minimal output targets small binding sets.
Package output targets the full API surface.

### Empty modules in package mode

In `--sys --package` mode, a namespace can contain only COM interfaces. Raw FFI style emits no
interface bodies, so the namespace has no items. `write_package` prunes that empty namespace. It
removes the module declaration, file, Cargo feature, and feature dependency references.

Pruning is recursive. A parent namespace is pruned only when it and all children are empty. This
applies only to `--sys`. The full `windows` crate emits interfaces, so those modules are not empty.

### Event accessors

Each WinRT `add_X` and `remove_X` pair becomes one method:

Event add methods return `Result<EventRevoker>`.

The method takes the closure directly. It returns an
[`EventRevoker`](https://docs.rs/windows-core/latest/windows_core/struct.EventRevoker.html). The
revoker calls the matching `remove_X` slot on drop. Call `.forget()` or `.into_token()` to opt out.

This rule changes only the consumer side. Implementing an event source still requires both `add_X`
and `remove_X`.

### Other useful options

- `--in`, `.input(..)`, and `.inputs(..)` add `.winmd` files or directories. The builder uses the
  standard metadata implicitly when no input is supplied. Builder inputs accept strings, `Path`, or
  `PathBuf`. Use `.input_default()` to combine the bundled metadata with custom inputs; the textual
  `--in default` form provides the same behavior.
- `.output(..)` accepts a string, `Path`, or `PathBuf`.
- `--derive` and `.derive(..)` add derives to generated types.
- Bare `--implement` and `.implement_all()` emit `_Impl` scaffolding for every WinRT interface in
  scope. Use `.implement(name)` or `.implements(names)` to limit scaffolding to type names or
  namespace prefixes.
- `--rustfmt` and `.rustfmt(..)` set the formatter for the output.
- `--dead-code` and `.dead_code()` emit `pub(crate)` for callable items. This lets the compiler flag
  unused generated callables.

## Committing generated bindings

A `build.rs` can regenerate bindings on each build. Published crates usually use a different
pattern. Commit `src/bindings.rs` as source. Depend only on [`windows-link`](windows-link.md) at
runtime. Consumers then build without code generation, metadata files, or a `windows-bindgen`
dependency.

The pattern has three parts.

**1. The published crate depends only on `windows-link`** and includes the committed bindings:

```toml
# tickcount/Cargo.toml
[dependencies]
windows-link = "0.100"
```

The library module includes the generated bindings and exposes the safe API used by the binary.

**2. A separate, unpublished binary owns code generation.** Keep it as a workspace member. It does
not become a dependency of the published crate:

```toml
# gen/Cargo.toml
[package]
name = "gen"
publish = false

[dependencies]
windows-bindgen = "0.100"
```

```rust,no_run
// gen/src/main.rs
windows_bindgen::bindgen([
    "--out", "tickcount/src/bindings.rs",
    "--flat",
    "--sys",
    "--filter", "GetTickCount64",
]);
```

`--out` is resolved relative to the current directory. Run the tool from the workspace root:

```sh
cargo run -p gen
```

**3. A CI check keeps the committed bindings current.** Regenerate, then fail if the result differs
from the checked-in file:

```yaml
- run: cargo run -p gen
- run: git diff --exit-code
```

This repository uses the same arrangement.
[`tool_bindings`](https://github.com/microsoft/windows-rs/tree/master/crates/tools/bindings)
regenerates each crate's `bindings.rs` from a `.txt` filter. The
[`gen.yml`](https://github.com/microsoft/windows-rs/blob/master/.github/workflows/gen.yml) workflow
runs the tools and rejects any diff.

## Consuming APIs outside the default projection

The published `windows` crate projects public, documented APIs behind Cargo features. Some consumers
need a smaller slice or an API that is not in public metadata.

Use `windows-bindgen` for these cases instead of expanding the `windows` crate.

- If the API is public but belongs to a broad feature, generate a small binding set with a filter.
  For example, a crate can select `IPropertyStore` and `PROPVARIANT` without enabling the full
  feature surface that contains them.
- If the API is not in public metadata, author metadata with [`windows-rdl`](windows-rdl.md). Then
  feed that metadata to `windows-bindgen`. This keeps the FFI surface generated and typed.

---

## Internal documentation

The rest of this page covers how the crate is built and maintained. It is for contributors and is
not needed to use `windows-bindgen`.

### How it's built

`windows-bindgen` is hand-written. It is the generator that other crates use. It reads ECMA-335
metadata through [`windows-metadata`](windows-metadata.md). The bundled metadata inputs live in
the [`windows-default`](windows-default.md) crate.

Two tools drive it in this repository:

- `tool_bindings` reads the per-crate `.txt` filters in `crates/tools/bindings/src`.
- `tool_package` produces the published `windows` and `windows-sys` crates.

### Output policies

The generator models output style as named policies. These policies keep style checks in one place
and make call sites describe intent.

- `Style::emit_class_methods` emits per-class wrapper methods.
- `Style::emit_inherited_forwarders` emits inherited-interface forwarders.
- `Style::emit_iterable_into_iterator` emits the `IntoIterator` bridge for an inherited
  `IIterable<T>`.
- `Style::minimal_string_input` and `Style::minimal_string_return` expose `HSTRING` parameters and
  returns as `&str` and `String`.
- `Config::emit_runtime_name` emits the WinRT `NAME` runtime-name constant.
- `Style::derive_std_traits` emits `Default`, `Debug`, and `PartialEq` derives.
- `Style::emit_core_traits` emits the `windows-core` trait block.
- `Style::emit_bare_typedef` emits handle structs and unscoped enums as type aliases.

`--dead-code` visibility is centralized in `Config::item_vis()`. It is used for callables such as
methods and delegate constructors. Nameable public items stay `pub`, because hand-written crates can
re-export them or reference them from exported macros.

Repeated layout helpers also live on `Config`:

- `Config::doc_hidden_in_package` emits `#[doc(hidden)]` in package mode.
- `Config::write_value_name_const` writes the `RuntimeType::NAME` constant for value types.

### Type selection

For precise filters, `TypeClosure::build` starts from the selected types and walks signature
dependencies. It emits selected entry points as full types. It emits dependency types as shells
unless they are selected directly.

For broad filters and package generation, `TypeMap::filter` scans namespaces from the top down. This
is used for full namespace and package output.

The `--minimal` flag affects rendering only. It does not change which referenced types are included.

### WinRT and Win32 code generation

The metadata reader classifies types as WinRT or Win32/COM from the metadata type attributes. Shared
code handles names, signatures, dependencies, and type remapping. The writers stay separate where
the ABI rules differ.

The main differences are:

- WinRT vtable methods return `HRESULT`. The projection wraps them in `Result`.
- COM methods keep their native return shape. `ReturnHint` controls the projected shape for common
  COM patterns.
- WinRT supports generics, runtime signatures, activation, and `RuntimeType`.
- WinRT delegates are COM interfaces with `Invoke`. COM callback types can be raw function pointers.
- Win32 also has free exports, constants, handles, unions, nested types, and architecture-specific
  layout.

Some writer pieces are shared. Interface vtable method fields and `_Impl` method iteration use
common helpers. Enum constant and flag operator emission also use common helpers.

### Generating bit-field accessors

Win32 structs frequently pack several logical members into one storage unit with C bit-fields:

```c
typedef struct _MIB_IF_ROW2 {
    // ...
    struct {
        BOOLEAN HardwareInterface : 1;
        BOOLEAN FilterInterface : 1;
        BOOLEAN ConnectorPresent : 1;
        // ...
    } InterfaceAndOperStatusFlags;
} MIB_IF_ROW2;
```

The winmd format has no bit-field concept. The scrape coalesces each run of bit-fields into one
backing integer field named `_bitfield`. If a struct has more runs, the fields are named
`_bitfield1`, `_bitfield2`, and more. The backing field is emitted as public FFI data:

`windows-bindgen` also generates a typed getter and setter for each logical member. The data comes
from `NativeBitfieldAttribute` metadata on the backing field. The accessors are generated for
non-`sys` styles.

Generated accessors read and update each member without manual bit arithmetic.

A width-1 member projects as `bool`. Wider members project as the backing integer type. Reads shift
through the backing type so signed backing fields sign-extend and unsigned backing fields
zero-extend. Writes clear the target bits and OR in the masked value. Identity shifts are omitted so
generated code stays clean under `-D warnings`.

The RDL spelling is a C-like bit-field block on the backing field:

```text
_bitfield: u8 { HardwareInterface: 1, ... }
```

See [`windows-rdl`](windows-rdl.md) for RDL input. Test coverage lives in
`crates/tests/libs/clang/input/bitfields.h` and
`crates/tests/libs/bindgen/input/struct_bitfield.rdl`.

### Counted-buffer metadata

`NativeArrayInfoAttribute` and `MemorySizeAttribute` can replace an input or input/output
pointer/count pair with a slice parameter in rich output.
`windows-metadata::reader::MethodParam::buffer_relationship` decodes the literal signed
relationship:

- `CountParamIndex` identifies an element-count parameter.
- `BytesParamIndex` identifies a byte-count parameter.
- `CountConst` supplies a fixed element count.

The metadata reader checks property names and value types. Invalid or conflicting relationships
return `None`, which keeps the raw ABI shape. It does not interpret parameter positions, pointer
shapes, or public projection policy.

Strictly output-only buffers keep their pointer and count parameters so callers may supply
uninitialized storage. For other buffers, before `CppMethod` indexes a related parameter, it
rejects negative, out-of-range, and self-relative indexes and verifies that the count is one input
scalar used by one buffer. Byte counts still require byte-sized elements. A fixed `CountConst` must
be nonnegative and fit the maximum Rust object size on 32-bit Windows. If any check fails,
generation keeps the pointer and count parameters exactly as the ABI declares them and adds no
slice or array sugar.

### Parameter direction and retval policy

`windows-metadata::reader::MethodParam` supplies the raw direction, optional, reserved, and retval
facts. Bindgen's local `Param::is_input_only` then applies Rust policy: `Input` and `Unspecified`
are input-only, while `Output` and `InputOutput` take the output-capable branch. An eligible In+Out
counted buffer becomes `&mut [T]`; treating the presence of `In` as input-only would incorrectly
make writable storage const. An output-only counted buffer remains `*mut T` plus its count because
`&mut [T]` would require initialized values before the call.

Bindgen also keeps its optional-or-reserved `Option` shaping local. Metadata does not combine those
facts because they are projection policy rather than metadata structure.

A trailing parameter becomes a projected return only when it is an output-only, required,
non-reserved, uncounted pointer. An explicit `RetValAttribute` bypasses the heuristic requirements
that every preceding parameter be input-only, that the pointee is not void, and that it fit the
existing 128-bit size limit; it does not bypass the other candidate checks. Without that attribute,
any preceding `Output` or `InputOutput` parameter keeps the trailing pointer in the parameter list.

### Testing

Dedicated test crates cover the generator and related metadata tools: `test_bindgen`, `test_rdl`,
and `test_clang`. `variadic_fn*` covers rich, minimal, sys-link, sys-extern, `C`, `system`, and
unsupported `fastcall` output. `buffer_relationships` covers valid, negative, out-of-range,
self-relative, byte-counted, and fixed-count metadata. The `interface_out_array` golden pins raw
output buffers and In+Out mutable slices. `method_params` pins In+Out mutable projection, and
`method_return` covers explicit and heuristic retval selection with In+Out, optional, reserved, and
counted exclusions plus explicit void-pointer and large-pointee returns.
