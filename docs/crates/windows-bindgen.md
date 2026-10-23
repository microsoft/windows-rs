# windows-bindgen

> Generate Rust bindings for the Windows APIs your project needs.

- 📦 [crates.io](https://crates.io/crates/windows-bindgen)
- 📖 [docs.rs](https://docs.rs/windows-bindgen)
- 🚀 [Getting started](../../crates/libs/bindgen/readme.md)
- 🧩 [Samples](../../crates/samples/bindgen)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/bindgen)

Use `windows-bindgen` to generate a focused binding set from Windows metadata (`.winmd`).
It includes default Win32 and WinRT metadata and can also read custom metadata. For broad
pre-generated bindings, use [`windows`](windows.md) or [`windows-sys`](windows-sys.md).

The [README](../../crates/libs/bindgen/readme.md) shows dependency setup, generation from
`build.rs`, and inclusion of the generated file. This guide covers API selection and generation
options.

## Generate bindings

Configure the builder, then call `write()`:

```rust,no_run
windows_bindgen::builder()
    .output("src/bindings.rs")
    .filters(["GetTickCount", "GetTickCount64"])
    .flat()
    .sys()
    .write();
```

The command-line-style adapter does the same thing:

```rust,no_run
windows_bindgen::bindgen([
    "--out", "src/bindings.rs",
    "--flat",
    "--sys",
    "--filter", "GetTickCount", "GetTickCount64",
]);
```

Both forms require an output path and at least one include filter. Paths are relative to the
generator's working directory.

For a published library, run generation from a separate tool and commit `src/bindings.rs`.
Consumers then need only the generated code's runtime dependencies. Applications can instead
generate into `OUT_DIR` from `build.rs`, as shown in the README.

## Select APIs

Start with the function, constant, or type name in the Windows API documentation. For example,
`.filter("GetTickCount")` selects [`GetTickCount`][get-tick-count]. Include any `A` or `W` suffix
in the metadata name; filters do not expand C preprocessor aliases.

[get-tick-count]:
  https://learn.microsoft.com/windows/win32/api/sysinfoapi/nf-sysinfoapi-gettickcount

Signature dependencies are included automatically. You do not need to list every parameter and
return type. Select an interface or its members explicitly when you also need to call its methods.

Structural C/stdcall `FNPTR` signatures become `Option<unsafe extern "C"/"system" fn(...)>`.
Their result and parameter dependencies are included without creating public callback names.

Nested record references retain their metadata owner. Selecting a consumer of `Outer/Inner` includes
the enclosing record that emits `Outer_0`, its architecture variants, and dependencies of its nested
declarations. Cross-namespace and reference-crate paths use the enclosing record's ownership rather
than the nested TypeDef's empty namespace. Both precise and namespace-wide filters follow this rule.

Native callable byrefs use pointer ABI: const references become `*const T`, mutable references
become `*mut T`, and written pointee aliases remain intact. This applies to imports, named and
structural callbacks, and COM methods. WinRT byref projection has a separate path. Native
references are not heuristic return values: the caller's existing mutable storage must remain
available unless metadata explicitly marks a return value. Query-return projection excludes
counted GUID buffers and counted object outputs from its scalar parameter candidates.

`BufferExtentAttribute` marks buffer storage even when its count cannot become a Rust slice.
Indirect, successful, arithmetic and other unsupported extents do not authorize scalar-return or
COM-query inference. Parameter-owned `NativeSalAttribute` keeps unresolved contracts opaque:
bindgen preserves storage rather than parsing SAL or hiding unresolved count parameters.
This restriction does not transfer COM ownership: interfaces retain borrowed parameter passing,
including optional references. Producers use `Ref<T>` for direct interface arguments even when
direction flags are absent; output pointer storage remains a separate case.
Known simple entry extents still support slices, and unambiguous scalar/query projections remain.
See the [canonical attribute policy](../clang-attributes.md) for owner and phase semantics.

Win32 `NativeOverrideSlotAttribute` methods redeclare inherited entries. Bindgen validates their
slot, name, and resolved signature, preserving pointer depth and qualifiers. Rust exposes these
calls through inherited methods and vtable fields, without additional `_Impl` requirements.
For member-only filters, select the callable on its declaring base.
The redeclaration's annotations remain in metadata. Out-of-range, duplicate, or incompatible
overrides fail generation rather than shifting the vtable. Older generators that ignore the
attribute must not consume metadata using this encoding.

Field-owned `AlignmentAttribute` raises a member's alignment without wrapping its public type.
Bindgen emits target-evaluated `MaybeUninit<u8>` padding and raises the record alignment as needed;
native padding need not be initialized. Padding names avoid authored field names. Records with
this storage, and their by-value containers, omit Debug, PartialEq, and Eq implementations.
Storage traversal uses qualified nested type identities, including nested types referenced by
other records or arrays. Merely declaring or pointing to an aligned nested type does not suppress
these traits on its owner.
Packed combinations are unsupported. The clang producer rejects by-value callable uses until
their calling ABI is qualified; correct field offsets alone do not establish that ABI.
Older generators that ignore member alignment must not consume this metadata.

Legacy x86 `GetWindowLongPtr*`/`SetWindowLongPtr*` aliases are emitted only when no native x86
declaration exists, once across separate wide-target declarations. Dependency selection follows
the same rule. Flat package namespace reexports follow their declarations' architecture masks;
architecture-empty public modules remain available without unused glob reexports.

Unscoped enum members become namespace constants. An explicit namespace constant with the same
name takes precedence on its declared architectures; the enum-derived constant remains on the
other architectures. Metadata retains both definitions and values. Scoped enum members keep their
associated names. Whole-enum, member and direct-constant filters use the same resolved selection.

The bundled Win32 metadata places APIs in `Windows.Win32`. Bare names such as `GetTickCount` and
qualified names such as `Windows.Win32.GetTickCount` both work. The subsystem paths used by
`windows` and `windows-sys` package features, such as `Win32_System_Com`, are not metadata
namespaces: do not turn them into filters such as `Windows.Win32.System.Com`.

Win32 `LPSTR`/`LPCSTR` and `LPWSTR`/`LPCWSTR` project to the same string pointer types as
`PSTR`/`PCSTR` and `PWSTR`/`PCWSTR`. This is a Rust projection rule; metadata retains the native
typedefs. Unrelated pointer typedefs and raw character buffers are not inferred to be strings.

Filters accept dots or `::` between path segments and `{...}` for grouped names:

| Filter | Selects |
| --- | --- |
| `GetTickCount` | A function by name. |
| `HWND` | A type by name. |
| `Windows.Win32.GetTickCount` | A function with its metadata namespace. |
| `Windows::Win32::{GetTickCount, GetTickCount64}` | A group of functions. |
| `Windows.Foundation` | A WinRT namespace and its children. |
| `Windows.Storage.Streams.IDataReader::ReadByte` | One interface method. |
| `Windows.Storage.Streams.IDataReader::{ReadByte, ReadUInt32}` | A group of methods. |
| `Windows.Storage.Streams.IDataReader::ByteOrder` | A property's getter and setter. |
| `Windows.Storage.Streams.IDataReader::{}` | The interface type without callable methods. |

Bare names search all input namespaces; qualify a name when you need to distinguish matching
definitions. For custom metadata, use the namespaces declared in that metadata.

Interface shells do not expose iteration. Iteration helpers require `First`, or `Current`,
`HasCurrent`, and `MoveNext` for an iterator.

For runtime classes, select the instance, static, or factory methods you need. For example,
`Windows.Foundation.Uri::CreateUri` selects a factory method; a class name alone does not select
its factory methods.

Member filters select methods on a type, not free functions in a subsystem namespace.
Property and event names select their accessor pairs. For type and namespace selections, prefix a
filter with `!` to exclude it. More specific rules take precedence; exclusions win ties regardless
of argument order. Prefer narrow filters to generating a namespace and removing items from it.

### Keep longer filter lists in a file

Create `apis.txt` with one filter per line:

```text
// APIs used by this project.
GetTickCount
GetTickCount64
```

Use `.filter_file("apis.txt")` or `--filter-file apis.txt`. Blank lines and lines beginning with
`//` are ignored. A file containing the entire argument list can be read with `--etc`.

## Choose the output

Style controls the generated API. Layout controls where it is written.

| Builder option | Argument | Use |
| --- | --- | --- |
| Default style | No flag | Rich wrappers using `windows-core`. |
| `sys()` | `--sys` | Raw bindings using `windows-link`. |
| `minimal()` | `--minimal` | Bindings for hand-written wrappers, without most conveniences. |
| Default layout | No flag | Nested Rust modules matching metadata namespaces. |
| `flat()` | `--flat` | One flat file, suitable for a private bindings module. |
| `package()` | `--package` | A Cargo package with namespace files and features. |

`sys` and `minimal` are mutually exclusive, as are `flat` and `package`. Narrow filters are needed
in every style. Generated APIs may also use focused support crates such as `windows-numerics`;
add the dependencies referenced by the generated source.

### Bit-field accessors

For native bit fields, default and minimal bindings provide getters and setters over the backing
integer fields. Sys bindings keep the backing fields without accessors.

`NativeBitfieldAttribute.MemberType` supplies an optional member type independently of storage.
Typed accessors retain signedness, bools, scoped enum wrappers, and transparent typedef names.
Signed fields are sign-extended from their declared width, including single-bit signed integers.
Setters truncate to that width without changing neighboring bits. Both dependency walks include
member types. Typed union accessors contain the required unsafe storage reads.
Untyped attributes keep the existing single-bit `bool` and backing-type behavior.

Accessors preserve storage wrappers and aliases, including enum and `BOOL` backings.
Pointer-sized operations use the consumer target's width; out-of-range fields fail compilation
on that target. Architecture-varying storage and member typedefs resolve per target, including
signedness. Nested records inherit their owner's architecture, and identical accessor
implementations share their architecture gates.

### Fixed-width addresses

`ptr64<*mut T>` and `ptr64<*const T>` metadata project to a generated typed 64-bit address.
The helper has size/alignment 8/8 on x86, x64 and ARM64, retains the pointee type and constness,
and provides `from_address(u64)` and `address()`. It has no dereference or pointer-conversion
operations. A native typedef such as `PVOID64` remains an alias to this address type.

The helper is emitted with the bindings, without another runtime dependency. Flat output places
it beside the declarations; module and package output place it in `Windows.Win32`. It does not
require a Cargo feature. Both broad and exact-name dependency selection retain its pointee types.
Ordinary native pointers retain their existing Rust representation.

### Metadata compatibility

Clang2 metadata requires updated, paired readers and generators. Build `windows-clang`,
`windows-rdl`, `windows-metadata`, and `windows-bindgen` from the same validated source revision
during development. Upgrade regeneration tools together and regenerate the bindings.

| Contract | Required reader behavior | Required generator behavior |
| --- | --- | --- |
| Inherited virtual overrides | Preserve `NativeOverrideSlotAttribute` and its slot. | Validate and reuse the inherited vtable entry rather than append one. |
| Typed bitfields | Decode the `MemberType: System.Type` property. | Preserve each member's type, signedness and target-specific alias while retaining storage layout. |
| Mixed pointer qualifiers | Decode each pointer's `IsConst` modifier without flattening the chain. | Preserve qualifier levels in the generated signature. |
| Fixed 64-bit pointers | Decode `NativePointer64Attribute` at its pointer level. | Emit typed 8-byte, 8-aligned address storage, not a target-sized pointer. |

Older consumers are unsupported for this output. Some reject a required encoding; others accept
the file and emit an incorrect ABI. Sys storage-only generation does not prove support for typed
bitfield accessors. Do not strip attributes or qualifiers to make an older generator accept new
metadata.

Record the paired release versions before production cutover. The development version `0.100.0`
spans incompatible source revisions and is not a sufficient compatibility pin. Qualify the chosen
pair with the override, typed/architecture-dependent bitfield and pointer fixtures, including
generated signatures, vtable slots, storage layout and accessor behavior, not only metadata parsing.

## Common options

The [builder reference] documents all methods, and [`bindgen`] lists the command-line-style
arguments.

[builder reference]: https://docs.rs/windows-bindgen/latest/windows_bindgen/struct.Bindgen.html
[`bindgen`]: https://docs.rs/windows-bindgen/latest/windows_bindgen/fn.bindgen.html

| Builder method | Argument | Purpose |
| --- | --- | --- |
| `input(path)` | `--in path` | Read a custom `.winmd` file or directory. |
| `input_default()` | `--in default` | Add the bundled Windows metadata. |
| `output(path)` | `--out path` | Set the output file or package directory. |
| `filter(name)` / `filters(names)` | `--filter names...` | Select APIs. |
| `filter_file(path)` | `--filter-file path` | Read filters from a text file. |
| `implement(name)` | `--implement name` | Generate interface implementation traits. |
| `implement_all()` | `--implement` | Generate implementation traits for all interfaces in scope. |
| `derive(name)` | `--derive name` | Add a derived trait to generated types. |
| `rustfmt(options)` | `--rustfmt options` | Pass comma-separated formatter configuration. |
| `extern_fns()` | `--extern` | Use extern declarations instead of link macros with `sys`. |
| `compose(name)` | `--compose name` | Select a WinRT class for composition with `minimal`. |
| `dead_code()` | `--dead-code` | Use `pub(crate)` visibility to detect unused callable bindings. |

For `compose`, also filter the class and its composable factory methods, and select its override
interfaces with `implement`.

With no explicit input, generation uses the bundled metadata. Adding a custom input replaces
that default; add `input_default()` if it also needs Windows definitions. Compile RDL with
[`windows-rdl`](windows-rdl.md), or scrape headers with [`windows-clang`](windows-clang.md),
before passing the resulting `.winmd` to bindgen.

Formatting uses `rustfmt` from `PATH` and is best effort: unavailable or failed formatting leaves
the output unformatted without a warning. `rustfmt("max_width=120")` sets configuration, not an
executable path. Install `rustfmt` when generating checked-in source.

## Samples

| Sample | Shows |
| --- | --- |
| [VSS backup](../../crates/samples/bindgen/vss_backup) | Rich COM bindings in `OUT_DIR`. |
| [Context](../../crates/samples/bindgen/context_alignment) | Flat sys bindings. |
| [Robot component](../../crates/samples/robot/component) | Custom RDL and implementation support. |
| [Robot client](../../crates/samples/robot/client) | Custom and default metadata together. |

## Contributing

The generator reads metadata through [`windows-metadata`](windows-metadata.md).
`test_bindgen` contains the fixtures; run `cargo test -p test_bindgen` and
`cargo test -p windows-bindgen`. After changing generation logic, run the affected `tool-*`
generators and inspect the output diffs. `tool-bindings` generates focused library bindings;
`tool-package` generates the published `windows` and `windows-sys` packages.
