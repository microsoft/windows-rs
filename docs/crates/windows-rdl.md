# windows-rdl

> A parser for RDL (Rust Definition Language) and an ECMA-335 metadata generator.

- 📦 [crates.io](https://crates.io/crates/windows-rdl)
- 📖 [docs.rs](https://docs.rs/windows-rdl)
- 🚀 [Getting started](../../crates/libs/rdl/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/rdl)

`windows-rdl` is the front of the metadata authoring pipeline. It parses RDL (Rust Definition
Language), a small Rust-like syntax for Windows APIs. It emits ECMA-335 `.winmd` metadata for
[`windows-bindgen`](windows-bindgen.md). It also writes canonical RDL from `.winmd` files.

Most applications should use existing bindings rather than author metadata. Use `windows-rdl` when
you are defining a custom WinRT or Win32 surface, adapting header output from
[`windows-clang`](windows-clang.md), or maintaining reviewable text for a winmd generator.

Start with the crate [README](../../crates/libs/rdl/readme.md) for setup and the two basic builder
calls. The full language reference is in
[`crates/libs/rdl/rdl.md`](../../crates/libs/rdl/rdl.md).

## First workflow: define a custom component surface

1. Create an `.rdl` file with one or more `#[winrt]` or `#[win32]` modules.
2. Add standard metadata as a reference if the declarations mention Windows types.
3. Compile the RDL to a winmd with `reader`.
4. Feed the winmd to [`windows-bindgen`](windows-bindgen.md) or C++/WinRT.
5. Treat the RDL as the reviewed source and regenerate the binary winmd.

`crates/samples/robot/component` follows this path. Its build script compiles `src/robot.rdl`,
references standard Windows metadata, then asks bindgen for the `Robotics` namespace and WinRT
implementation support. `crates/samples/robot/component_cpp` sends the same kind of output to
[`cppwinrt`](cppwinrt.md).

## Input and output model

| Direction | Builder | Inputs | Output |
| --- | --- | --- | --- |
| RDL -> winmd | `reader()` | `.rdl` files, directories, or text | One `.winmd` file |
| winmd -> RDL | `writer()` | `.winmd` files, directories, or bytes | One file or partition |

Reader references resolve names but are not emitted as definitions. Use `reference_default` for
the bundled standard metadata, or the path and byte reference methods for custom dependencies.
Writer inputs are definitions to render; `input_default` renders the bundled metadata.

Writer filters accept namespace prefixes, qualified or unqualified names, and `!` exclusions.
`split` writes one file per namespace. `partition` accepts an item-name -> file-stem map and writes
one file per defining header. `item_names` reads the names declared under one namespace without
compiling the file. `enum_member_names` reads enum member names separately, including scoped
members and deduplicating architecture variants. These queries report names, not type, scope,
value, or architecture equivalence.

## RDL syntax

The shared formatter sorts each owner's attributes alphabetically by full name, case-sensitively.
This applies to both clang publication and metadata-to-RDL output. Equal-name attributes retain
their relative order because some reader paths use the first occurrence. Arguments are not sorted.

Win32 parameter direction is explicit: `in`, `out`, both, or neither. No attributes means unspecified
direction, regardless of pointer mutability or typedef spelling. `opt` is independent. The writer
and clang share `emit::write_param_direction`, which does not inspect types or suppress flags.
Unflagged or absent metadata parameter rows need no marker. Property setters and event
add/remove methods have input parameters by their member contracts, not by type inference.

WinRT interface methods and delegates require exactly one of `#[in]` or `#[out]` on every
explicit argument. The reader rejects missing or combined directions at the argument's source
location, naming its interface/delegate, method and parameter. Return values, `&self`, and
metadata attribute constructors are not subject to this rule. Prefer explicit directions in
Win32 authoring too, but retain unspecified direction when the native contract is unknown.

This reader does not accept `direction(unspecified)` or apply legacy direction defaults. Existing
inputs must materialize their old flags before migrating. The repository migration preserves
metadata meanings; the binary encoding is unchanged.

Win32 pointer types preserve qualifiers at each depth: `*mut *const T` and `*const *mut T` have
different metadata signatures and round-trip without flattening. Homogeneous chains retain their
existing binary encoding. Mixed chains use per-level modifiers and require an updated metadata
reader; see [pointer constness](windows-metadata.md#pointer-constness).

Win32 fixed 64-bit data-pointer storage uses `ptr64<*mut T>` or `ptr64<*const T>`, including on
x86. It retains pointee type and constness rather than becoming an integer or native-width pointer.
It requires updated paired metadata readers and generators. Rust bindings expose a typed address
value, not pointer dereferencing. WinRT, fixed-width callable values, and fixed function/interface
pointers are unsupported; see [fixed-width pointers](windows-metadata.md#fixed-width-pointers).

Win32 inline function-pointer types use `extern "C" fn(T, ...) -> R` for C or
`extern fn(T, ...) -> R` for system/stdcall. Omit `-> R` for void results. Parameters are positional
types without names or attributes. Nested function pointers, pointers, arrays, and named record
references retain their types. These encode ECMA-335 `FNPTR` signatures without declaring named
callbacks; named `extern fn Name(...)` declarations keep their delegate representation.
Inline types reject WinRT use, unsupported ABIs, variadics, lifetimes, and `unsafe` qualifiers.
The writer rejects unsupported imported signature flags rather than changing their ABI.

`#[must_use]` encodes the parameterless `Windows.Win32.Metadata.MustUseAttribute` on imports,
interface methods and named callbacks. It says the caller should use the result rather than
accidentally discard it. It is not a success predicate, purity promise or signature change.
Language-specific diagnostics and output-parameter inspection are separate concerns.

`#[success(return == 0)]`, `!= 0`, `== 1`, `>= 0` and `> 0` encode the enum-valued
`SuccessConditionAttribute`. The formatter treats these arguments as expressions, preserving
comparison tokens. Callable conditions override return-type defaults; they govern ordinary
postconditions rather than defining a language binding's error model. The reader rejects other
expressions and parameter/field owners. The writer retains full spelling for unfamiliar payloads.
Canonical success syntax requires an updated RDL reader.

Custom attributes on `type Name = Type;` declarations attach to the metadata typedef and survive
RDL/WinMD round trips, including architecture-specific aliases. The provisional `NativeSalAttribute`
in `metadata/metadata.rdl` uses this support to retain header annotation evidence without new syntax.
Enum variant attributes attach to the corresponding constant fields and survive both directions,
including deprecation payloads.
GUID constants retain their declared type, architecture tags, and custom attributes in both
directions. Property-key constants retain those attributes alongside their format GUID and ID.

`#[override_slot(N)]` encodes `Windows.Win32.Metadata.NativeOverrideSlotAttribute(slot: u32)`
and marks a Win32 method redeclaration
at an absolute inherited vtable slot. The reader clears `MethodAttributes::NewSlot`; the writer
preserves the attribute and all method/parameter contracts. It is not supported on WinRT methods.
Bindgen validates the slot against the base hierarchy and reuses the inherited entry. Older
generators can ignore the attribute and append a wrong vtable entry, so this encoding requires a
paired reader/generator upgrade.

RDL uses Rust tokens and module syntax, but it describes metadata rather than executable Rust.
A top-level `mod` is a metadata namespace. Tag it `#[winrt]` or `#[win32]` to select the type
system. Items include classes, interfaces, delegates, callbacks, enums, structs, unions, typedefs,
constants, and imported functions.

```text
#[win32]
mod Example {
    #[repr(i32)]
    enum Color {
        Red = 1,
        Green = 2,
        Blue = 3,
    }

    struct Point {
        x: i32,
        y: i32,
    }

    const MAX: u32 = 42;

    #[library("example.dll")]
    extern fn GetPoint() -> Point;

    #[guid(0x00000001_0002_0003_0004_000000000005)]
    interface ICustom : Windows::Win32::System::Com::IUnknown {
        fn Method(&self, #[in] value: i32) -> i32;
    }
}
```

Most attributes name a metadata attribute type directly. Some attributes use short pseudo-attribute
names. The reader expands those names to full metadata attributes. See `PSEUDO_ATTRS` in
`windows-rdl`.

Struct bit fields use their own syntax. A run of bit fields packed into one backing integer is
written as a C-like block on that field. Each member uses `Name: width`. Anonymous padding uses
`_: width`.

```text
struct D3D11_VIDEO_PROCESSOR_COLOR_SPACE {
    _bitfield: u32 {
        Usage: 1,
        RGB_Range: 1,
        YCbCr_Matrix: 1,
        YCbCr_xvYCC: 1,
        Nominal_Range: 2,
        Reserved: 26,
    },
}
```

Member offsets are implicit. Each offset is the total width of earlier members, including padding.
The reader writes one `Windows.Win32.Metadata.NativeBitfieldAttribute(name, offset, width)` custom
attribute per named member. The writer renders it back to block form.

An optional `MemberType: System.Type` named argument retains a member type independently of
storage. The compact spelling is `delta: i32: 3` inside the backing field's block. Primitive, enum,
and typedef names survive both metadata round trips; namespace remapping also updates named type
values. Untyped members retain `name: width` without acquiring a member type. Full attributes
remain accepted and are retained when extra properties or overlapping offsets cannot fit a block.
Updated metadata readers are required for the named `System.Type` serialization tag.

`#[out_ptr]` and its `nullable` and `null_on_failure` flags encode returned-pointer guarantees.
They do not replace outer `opt`, parameter direction or `retval`. The reader expands both Boolean
properties of `NativeReturnedPointerAttribute`; the writer uses the short form only when those
two properties are the complete payload. Field-owned `AlignmentAttribute` uses `#[align(N)]`.

`#[null_terminated(entry, double_success)]` preserves single/double zero termination separately
at entry and successful completion. The available flags are `entry`, `double_entry`, `success`
and `double_success`; each phase may appear once. This contract adds no direction, encoding,
capacity or failure guarantee. Shortening requires exact known metadata properties; extended or
inconsistent payloads retain their full attribute spelling.

See [`windows-clang`](windows-clang.md#bit-field-member-scraping) for how the scraper emits bit
fields. See [`windows-bindgen`](windows-bindgen.md#bit-field-accessors) for the accessors they
drive.

WinRT types use the `#[winrt]` namespace flavor. They also add runtime-class and property syntax.

```text
#[winrt]
mod Robotics {
    #[Activatable(1)]
    class Robot {
        IRobot,
    }

    #[ExclusiveTo(Robot)]
    interface IRobot {
        fn Speak(&self, #[in] message: String);
        Name: String;
    }
}
```

A property is read-write unless marked `#[get]` or `#[set]`. Other attributes on a property or
`event` apply to each of its accessor methods, as in MIDL:

```text
interface IRobot {
    #[get]
    #[NoException]
    Battery: i32;
}
```

When the accessors carry different attributes, the writer emits each one on its own, such as a
`#[get]` property followed by a `#[set]` property of the same name. Accessors that the shorthand
cannot represent, such as a getter with a return attribute, stay explicit `#[special] fn` methods.
In a WinRT interface, the reader groups every accessor by name and shape, whether shorthand or
explicit, so each property or event still gets one metadata row with all of its accessors.

Classes are sealed unless marked `#[unsealed]`. A class can only extend an unsealed class, so a
class hierarchy marks each base class:

```text
#[unsealed]
class Visual {
    IVisual,
}

class SpriteVisual: Visual {
    ISpriteVisual,
}
```

The first interface is the default interface. Other attributes on a class interface, such as
`Overridable` and `Protected` for a composable class, apply to that interface implementation:

```text
#[unsealed]
#[Composable(IVisualFactory, Public, 1)]
class Visual {
    IVisual,
    #[Overridable]
    IVisualOverrides,
    #[Protected]
    IVisualProtected,
}
```

A class without interfaces, such as a static class, is marked abstract in metadata because it has no
instances. C#/WinRT requires this flag.

The `crates/tests/libs/rdl/input` directory has focused `.rdl` files for syntax examples. It covers
structs, flags, delegates, generic interfaces, unions, and more.

## Common tasks and neighboring crates

The complete source-to-binding path is:

```text
C/C++ headers -- clang() --> .rdl -- reader() --> .winmd -- bindgen() --> bindings.rs
 (windows-clang)             (windows-rdl)        (windows-bindgen)
```

- Skip RDL when a suitable winmd already exists.
- Use `windows-clang` to create RDL from headers; `tool-webview` demonstrates the full path.
- Use `windows-metadata` for table-level inspection, merge, and namespace remapping.
- Use `writer().split()` to maintain namespace-partitioned reviewable metadata.
- Use `merge_arch_rdl` only for generators that have per-architecture RDL directories and winmds.
  It merges structural differences and restores the defining-header partition.
- `tool-reactor-metadata` compiles hand-authored `extras.rdl` to fill metadata gaps before binding
  WinUI.

## Pitfalls

- RDL references must be supplied separately; naming an external type does not locate its winmd.
- The reader rejects unsupported types, constants, callback ABIs, variadic callback parameters,
  and function ABIs instead of dropping them.
- Mixed pointer chains require a metadata reader that understands per-level `IsConst` modifiers.
- `len_param` and `size_param` store raw zero-based signature positions. Update the attribute when
  parameters move.
- Omitting direction preserves neither In nor Out; it does not infer a contract from the type.
- Attributes on a void return row cannot round-trip because there is no return type to carry them.
- `split` and `partition` clear existing `.rdl` files in the output directory before writing.

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is for contributors and
is **not needed to use `windows-rdl`**.

### How it's built

The RDL grammar uses `syn`, `quote`, and `proc-macro2`. It reuses Rust's tokenizer so the syntax
stays Rust-shaped. The `reader` lowers the syntax tree to ECMA-335 and emits `.winmd` through
[`windows-metadata`](windows-metadata.md). The `writer` reads metadata through the same crate and
writes canonical RDL.

The `clang` path uses `clang-sys` to parse C or C++ translation units. It projects the declarations
into the RDL syntax tree. The header path and hand-authored RDL path share the same lowering code.
The `formatter` module pretty-prints generated RDL.

### Testing

Dedicated test crates cover the crate:

- `test_rdl` covers RDL to winmd round trips with `input/*.rdl` fixtures and invalid input with
  `errors/input/*.rdl` and `errors/expected/*.txt` diagnostic fixtures.
- The `windows-clang` integration tests cover header-to-RDL output.
- `tool-roundtrip` re-derives committed RDL files from committed winmd files. The `gen` workflow
  enforces a clean `git diff`.
- `test_bindgen` covers the `.winmd` to Rust step that consumes this crate's output.
- `test_interop` compiles `fixtures/*/api.rdl`, implements each API in Rust, and runs C++/WinRT
  and C#/WinRT clients against the Rust component. See `crates/tests/interop/readme.md`.

Diagnostic fixtures compare the full message, file name, line, and column. A mismatch updates the
expected file and fails the test so the change can be reviewed.

Run targeted tests with:

```sh
cargo test -p test_rdl
cargo test -p test_interop
cargo test -p windows-clang
```

### Default metadata files

`windows-rdl` builds the default metadata files used by the in-repo generators and library crates.

| File | Source | Writer |
|------|--------|--------|
| `crates/libs/default/Windows.winmd` | Merged SDK contract winmds | `tool-winrt` |
| `crates/libs/default/Windows.Win32.winmd` | Scraped SDK and WDK headers | `tool-win32` |

The committed RDL files are the reviewable source for these metadata files:

- `metadata/winrt/*.rdl` is partitioned by namespace.
- `metadata/win32/*.rdl` is partitioned by defining header.
- `metadata/wdk/*.rdl` is partitioned by defining header.

The binary winmd files are derived artifacts. Generation is deterministic. The metadata writer
stages tables in `BTreeMap`s and uses a fixed zero GUID for the module MVID.

Every maintained crate that needs Win32 metadata resolves against the in-repo `Windows.Win32.winmd`.
Minimal-binding crates and `windows-reactor` use it directly. The `windows` and `windows-sys` crates
use it through `tool-package`.

### Multi-arch merge

`tool-win32` scrapes x64, arm64, and x86 into separate RDL sets. Then `merge_arch_rdl` combines them
into one winmd. A type with the same shape on every architecture is emitted once. A type that
differs by architecture is split into per-architecture copies tagged `#[arch(X86|X64|Arm64)]`.

The merge compares type structure through [`windows-metadata`](windows-metadata.md).
`merge_arch_rdl` handles orchestration. It reads each architecture's RDL, runs the merge, and writes
the combined output. `ArchInput` stores its RDL directory and winmd as `PathBuf`.

### Import libraries

`implib::read` reads COFF short-import members in archive order, preserving duplicate entries.
Each import retains the machine, code/data/const kind, linker symbol, DLL, and `ImportName`: either
the named PE export or its ordinal. Name-prefix removal, undecoration, and export-as names follow
the short-import header, not assumptions about the linker symbol. Machine selection, library
precedence, and corrections belong to the driver.

### Published crates and namespace remap

The in-repo Win32 and WDK metadata lives in flat namespaces. Published `windows` and `windows-sys`
APIs are partitioned behind many Cargo features.

`tool-package` remaps the flat metadata into header-stem namespaces under `target/package/`. It uses
the committed `metadata/win32` RDL directory as the routing signal. Then it runs `windows-bindgen`
over that partition. `tool-features` uses the same remap so feature search reports the same header
stems.

The in-repo WinRT `Windows.winmd` is projected with the remapped Win32 and WDK metadata.

### Round-trip rules

RDL is the reviewable source for WinRT, Win32, and WDK metadata. The `.winmd` files are derived
artifacts. The `gen` workflow runs the generators and `tool-roundtrip`. It fails when regeneration
changes tracked files.

| Family | External source | RDL layout | Winmd build path |
|--------|-----------------|------------|------------------|
| WinRT | SDK winmds | `metadata/winrt`, per namespace | merge -> RDL -> winmd |
| Win32 | SDK headers | `metadata/win32`, per header | scrape -> arch merge -> winmd |
| WDK | WDK headers | `metadata/wdk`, per header | scrape -> arch merge -> winmd |

`tool-roundtrip` validates the reverse direction:

- WinRT uses `writer(Windows.winmd).split()` to write `metadata/winrt`.
- Win32 and WDK cannot recover header files from flat winmd alone. The tool reads the committed RDL
  layout to map type names back to header stems. Then it writes `metadata/win32` or `metadata/wdk`
  with `writer(winmd).partition(map)`.

### Current normalization rules

Some metadata forms have canonical RDL spellings. These rules are intentional. They keep generated
RDL stable.

| Form | RDL spelling | Reason |
|------|--------------|--------|
| WinRT `System.Char` | `Char16` | Keeps the type distinct from `u16`. |
| Property setter parameter | `value` | Shorthand does not retain its original name. |
| Event add parameter | `handler` | Shorthand stores the event shape. |
| Event remove parameter names | `token` | The remove accessor takes an event token. |
| Property and event accessors | Shorthand | Only when the shorthand can represent all accessor metadata. |
| Other interface methods | Full method form | Non-accessor methods stay explicit. |
| Input direction | `#[in]` | The reader accepts `#[r#in]`; the formatter emits `#[in]`. |
| Raw identifiers, GUIDs, delegate ABIs | Canonical output | Equivalent text has one spelling. |

The reader rejects unsupported forms with errors. It does not silently drop them. Examples include
unsupported types, constants, callback ABIs, variadic callback parameters, and function ABIs.

### Nested records

Inline structs and unions have nested TypeDefs linked by `NestedClass` rows. Their field
signatures reference those definitions through enclosing-TypeRef chains, including every level
of nesting. Unshared direct fields retain inline syntax when the nested name matches the reader's
generated name. The writer also accepts the unscoped leaf references in existing metadata.

Shared, named, and indirectly referenced records use declarations inside their owning record:

```rust,ignore
struct Outer {
    struct Inner {
        value: i32,
        next: *mut Outer::Inner,
    }
    first: Outer::Inner,
    second: Outer::Inner,
}
```

Both fields reference the same nested TypeDef. Nested declarations do not occupy field positions.
The reader supports forward references and unqualified lookup from the nearest record scope;
canonical output uses qualified paths such as `Outer::Inner`. Nested structs and unions can
contain further records, and references can occur inside arrays, pointers, and function signatures.
Nested types remain under their owner rather than becoming top-level helper declarations.

`test_rdl::nested_references` checks physical TypeRef scopes before and after the text round trip,
shared identity, lexical lookup, reference assemblies, and name collisions, alongside the canonical
`struct_nested_anon.rdl` and `struct_nested_named.rdl` fixtures. `test_metadata::nested_references`
independently covers the underlying signature reader, merge, and namespace remapper.

### Method parameter rows

The winmd writer uses `MethodDef::params_by_sequence` from
[`windows-metadata`](windows-metadata.md). Sequence 0 supplies return-type attributes. Nonzero rows
are matched to one-based signature positions, so sparse or out-of-order metadata cannot rename or
reflag another parameter. Every signature parameter is emitted; a missing row uses `pN` with no
direction or optional marker and retains neither direction flag. Duplicate and out-of-range
sequences stop the write with a diagnostic.

Direction and optionality come from `MethodParam::direction()` and `is_optional()`. Reserved,
retval, and count attributes remain independent pseudos or custom attributes. The writer applies
RDL's format boundary only when spelling them: In+Out emits both markers, while Unspecified emits
neither. Optionality remains independent in all four Win32 direction states. The writer does not
repair invalid WinRT metadata: emitted RDL with missing or combined WinRT argument directions is
rejected when compiled.

`test_rdl::method_params` authors sparse and out-of-order rows directly, checks parameter and
return pseudos in the generated RDL, and compiles the RDL back to dense metadata with the same
associations. Its parameter matrix covers all four direction states independently of optionality,
pointer mutability and alias spelling.

### Lossless round-trip limits

- **Void return rows:** Return attributes are written after `->`. A void method has no return type
  to carry them, so attributes on its sequence 0 row are not represented.
- **Buffer extents:** `readable` and `writable` bind count names to their owning signature or
  record. Parameters name entry/success phases; field ownership establishes valid-record state
  without a phase argument. `on_entry(count)` retains a call-entry count in a success contract.
  `if_present(*count)` leaves an extent unknown when its count pointer is null; it cannot combine
  with a null fallback. Indirect pointer initialization and nullability do not depend on whether
  the count was requested.
  Parameter/field short forms validate count and storage types; unrecognized metadata payloads
  keep full spelling.
- **Legacy count relationships:** `#[len_param(N)]` and `#[size_param(N)]` store raw zero-based signature
  positions. Reordering parameters without updating `N` changes the relationship.
- **Function pointers:** Inline signatures support plain C and system/stdcall only. Other flags,
  WinRT use, and parameter annotations are rejected rather than normalized.
