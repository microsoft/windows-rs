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
compiling the file.

`inventory(path)` parses one RDL file without resolving references or writing metadata. It returns
declarations and members in source order, including duplicates, normalized type/signature tokens,
attributes, architecture masks, and source lines. File-local IDs and parent IDs preserve nested
records and callable parameter order. This is an inventory of the RDL as written, not evidence
that it correctly describes the native API. See the
[header replacement inventory](windows-clang.md#baseline-inventory-command) for the repository tool.

## RDL syntax

RDL uses Rust tokens and module syntax, but it describes metadata rather than executable Rust.
A top-level `mod` is a metadata namespace. Tag it `#[winrt]` or `#[win32]` to select the type
system. Items include classes, interfaces, delegates, callbacks, enums, structs, unions, typedefs,
constants, imported functions, and declaration-only imported data.

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
        fn Method(&self, value: i32) -> i32;
    }
}
```

Most attributes name a metadata attribute type directly. Some attributes use short pseudo-attribute
names. The reader expands those names to full metadata attributes. See `PSEUDO_ATTRS` in
`windows-rdl`.

### Native type qualifiers

In `#[win32]` declarations, `Const<T>` and `Volatile<T>` qualify the enclosed type node.
They are type constructors, not named record wrappers or parameter-direction attributes.
Each accepts exactly one type argument. WinRT declarations reject them.

| RDL | Distinction |
| --- | --- |
| `*const i32` | Existing pointer to const `i32` |
| `Const<*mut i32>` | Const pointer value, not const pointee |
| `*mut Volatile<i32>` | Pointer to volatile `i32` |
| `Volatile<*mut i32>` | Volatile pointer value |
| `*const Const<*mut Volatile<i32>>` | Separate qualifiers at the pointer and scalar nodes |
| `[Volatile<Record>; 2]` | Volatile record elements |
| `Const<[Record; 2]>` | Qualified array node, not a replacement element type |

Nested qualifiers preserve their written order, including `Const<Volatile<T>>` and
`Volatile<Const<T>>`. The two orders do not claim different native const/volatile semantics.
Qualifiers do not add storage, erase aliases, or turn an opaque value into a complete record.
Opaque storage checks follow them without changing whether the surrounding use needs storage.

RDL/WinMD round-trips preserve qualifier placement in fields, aliases, arrays, references,
parameters, and results. Existing pointer signatures remain unchanged. The legacy default
parameter-direction rule examines the type below top-level qualifiers; explicit `#[in]` and
`#[out]` remain independent. Source annotation direction is not inferred from these qualifiers.

These experimental native qualifiers are not yet supported by Rust projection. Bindgen rejects
them explicitly rather than erasing them. Native source capture is a separate replacement gate.

### Native source evidence and direction

Raw native annotations use vocabulary attributes, independently of structural qualifiers and
lowered buffer/direction attributes:

| Attribute | Positional arguments |
| --- | --- |
| `Windows::Win32::Metadata::NativeAnnotation` | Source family, nonnegative ordinal, exact payload |
| `Windows::Win32::Metadata::NativeAnnotationBinding` | Source family, annotation ordinal, source identifier, zero-based parameter position |

An annotation's identity is its owner, source family, and ordinal. Ordinals record order within
one family's evidence on that owner; they need not be contiguous. Attribute table order and the
writer's sorted attribute text are not source order. SAL and MIDL retain separate evidence even
when their payloads disagree. Family names are open, nonempty strings, not a fixed macro list.

A binding refers to evidence on the same owner. Its parameter position excludes `&self` and the
return row; zero refers to the first declared parameter. Method, parameter, and return owners
can bind identifiers from their evidence to parameters of that method. Source identifiers need
not equal the published parameter names. Renaming `length` below does not change what `count`
means in the annotation:

```text
#[win32]
mod Example {
    extern "C" fn Callback(
        #[Windows::Win32::Metadata::NativeAnnotation("SAL", 0, "_In_reads_(count)")]
        #[Windows::Win32::Metadata::NativeAnnotationBinding("SAL", 0, "count", 1)]
        #[unspecified]
        buffer: *mut u8,
        length: usize
    );
}
```

Compile this example with `metadata/metadata.rdl` as input or its compiled vocabulary as a
reference. Annotations also survive on fields and typedef declarations. Return evidence on a
void callable uses `-> #[Attribute(...)] void`; an absent result value does not erase evidence.

| Parameter spelling | Stored `In`/`Out` flags |
| --- | --- |
| `#[in]` | `In` |
| `#[out]` | `Out` |
| `#[in] #[out]` | `In | Out` |
| `#[unspecified]` | Neither |
| No direction spelling | Legacy default: mutable pointer/reference is `Out`, otherwise `In` |

`#[unspecified]` accepts no arguments and conflicts with `#[in]` or `#[out]`. Optionality stays
independent. A replacement extractor must emit it when native evidence supplies no direction,
rather than apply the legacy default. The writer preserves direction-less existing `Param`
rows with this spelling; missing raw rows still use legacy defaults.

Shared checks reject invalid argument shapes, empty families/payloads/identifiers, negative or
duplicate ordinals, duplicate identifier bindings, missing same-owner evidence, out-of-range
positions, and bindings without a callable owner. WinRT declarations reject these native
annotations. Checks run before output writes and split/partition cleanup.

These attributes preserve evidence; they do not parse SAL, rewrite pointers, resolve source
ambiguities, or assert that every identifier in a payload has been bound. Native capture must
prove those relationships from its original declaration context.

### Declaration-only native data

`static Name: Type;` declares an imported native object, not a constant or a local allocation.
It requires one explicit `import` attribute:

```text
#[win32]
mod Example {
    #[import("example.dll", name = "NativeCount", kind = data)]
    static Count: Volatile<i32>;

    #[import("example.dll", ordinal = 17, kind = constant)]
    static Limit: Const<u32>;
}
```

The DLL and exactly one export `name` or u16 `ordinal` are required. `kind` must be `data` or
`constant`, matching the COFF import-object kind rather than C/C++ constness. A constant-kind
import still has no value; use `Const<T>` for an exact native const qualifier. Ordinals range
from 0 through 65535. An export literally named `"#17"` remains a name, not an ordinal.
DLL/export text is preserved exactly; no source-name default or DLL spelling repair is applied.

These items emit `Public | Static` fields with `NativeDataImportAttribute`, no `Literal` flag,
and no Constant row. Mixed pointers, native interface/callback identities, record aliases,
fixed arrays, qualifiers, architecture attributes, and raw member evidence retain their existing
type/attribute representation. A declaration of an opaque external object retains its nominal
type without claiming local storage; opaque array elements still require storage.

Initializers, `static mut`, missing/duplicate routing options, function import kinds, empty/NUL
names, out-of-range ordinals, WinRT owners, and unsupported types reject before output replacement.
The existing `const Name: Type;` grammar is unchanged: a missing initializer is not automatically
reclassified as imported data.

`test_rdl::native_data` has a reviewed fixture with exact text/binary round-trips, semantic
readback, independent namespace routing, external types, and architecture-specific import
contracts. `native_data_metadata` authors invalid descriptors directly and checks rejection before
emission, rendering, split/partition cleanup, merge, or remap. See
[metadata native data imports](windows-metadata.md#native-data-imports) for the wire contract.
These checks do not prove native DLL/export availability or layout. Rust data projection is a
separate task.

### Native interface slots

An explicit native model separates source declaration order from vtable order:

```text
#[win32]
mod Example {
    #[no_guid]
    #[vtable(size = 2, inherited = 0)]
    interface Base {
        #[slot(1)]
        fn Echo(&self, value: f32);
        #[slot(0)]
        fn Echo(&self, value: i32);
    }
    #[no_guid]
    #[vtable(size = 3, inherited = 2)]
    interface Derived: Base {
        #[slot(2)]
        fn Extra(&self);
        #[slot(1, reuse = Base)]
        fn Echo(&self, value: f32);
    }
}
```

Indices are absolute. New declarations cover every slot from `inherited` through `size - 1`.
A reuse declaration occupies the inherited prefix and names the ancestor declaring that slot;
an ancestor's effective table alone is not a local declaration. New methods set ECMA-335
`NewSlot`; reuse methods clear it. MethodDef order stays in source order, and inherited methods
are not copied into the derived declaration.

The checked model supports nongeneric native interfaces with one base chain. Every local method
requires a slot descriptor. Duplicate/out-of-range slots, missing suffix slots, invalid owners,
known cycles, generic/variadic methods, and conflicting method flags reject before output writes
or cleanup. Architecture variation uses interface-level `#[arch(...)]`, not method-level tags.
Known modeled base sizes must equal the explicit inherited count. A neutral declaration's
transitive reuse owner must occur on every available architecture lineage.

Unavailable or unmodeled external bases retain the explicit inherited prefix without guessing
native order from method counts. A direct base identity remains representable; ancestry through
unavailable definitions stays unknown. Slot checks do not prove override signature legality,
receiver qualifiers, covariance, thunks, or native ABI agreement. Raw receiver annotations retain
source evidence without claiming to lower C++ receiver semantics.

`test_rdl::native_slots` checks reviewed text/binary round-trips, independent namespace routing,
source order, reuse owners, and architecture-specific contracts. `native_slot_metadata` checks
raw malformed descriptors and unchanged method signature bytes. Rust projection rejects this
model, including unmodeled interfaces derived from modeled bases; it is a separate gate.
Interfaces without descriptors retain their existing representation. See
[metadata native interface slots](windows-metadata.md#native-interface-slots) for the wire contract.

### Named record aliases

`type NAME = TYPE;` preserves a declared alias and its written target. Several names can refer
to one complete record without copying its fields:

```text
#[win32]
mod Example {
    struct Record {
        value: u32,
    }
    type First = Record;
    type Second = Record;
    type Chain = First;
    type Pointer = *mut First;
}
```

Each alias emits one value TypeDef marked `Windows.Win32.Metadata.NativeTypedefAttribute`, with
a single `Value` field carrying its target type. `First` and `Second` retain separate names
targeting `Record`; `Chain` targets `First`, not a flattened copy of `Record`. Field and callable
signatures retain those alias names, including pointer qualifiers, references, and arrays.

A one-field ordinary record is not inferred to be a typedef. The metadata marker is required.
Records with identical fields also remain distinct nominal definitions. External references
resolve alias targets without copying the referenced record into the output.

The writer rejects typedefs with zero or multiple target fields before writing or clearing
file, split, or partitioned output. `test_rdl::record_aliases` checks reviewed text and exact
WinMD round-trips, by-value and qualified uses, external assembly references, independently
routed alias/record namespaces, and architecture-specific record targets.

This is the representation contract, not native declaration reconciliation. If a compiler links
a same-spelled tag and typedef to one record, the extractor must select one canonical published
definition using that evidence. Do not emit an additional self-alias or infer that relationship
from spelling or matching fields.

### Opaque nominal records

Canonical RDL has resolved type identities, not pending C/C++ forward declarations. A native
record whose definition is available must be emitted with that definition. A record intentionally
opaque in the selected public source contract uses `#[opaque]`:

```text
#[win32]
mod Example {
    #[opaque]
    struct _CONTEXT {}

    type CONTEXT = _CONTEXT;
    type PCONTEXT = *mut CONTEXT;

    #[library("example.dll")]
    extern "C" fn Close(context: PCONTEXT);
}
```

`#[opaque] struct Tag {}` and `#[opaque] union Tag {}` define nominal identities without exposing
storage. They emit `Windows.Win32.Metadata.NativeOpaqueAttribute`. An unmarked `struct Tag {}`
is a complete empty record, not an opaque type or a claim about its native byte size. `struct Tag;`
is not canonical RDL syntax.

| Use | Opaque record |
| --- | --- |
| Named typedef, including a chain of aliases | Allowed; retains the target identity |
| Pointer or reference | Allowed; retains the pointee identity |
| Declaration-only imported object | Allowed; does not claim local storage |
| Array of pointers | Allowed |
| Storage field, by-value parameter, or result | Rejected, including through aliases |
| Array element, including an array behind a pointer/reference | Rejected |
| Fields, methods, packing, alignment, or other layout on the opaque declaration | Rejected |
| WinRT record or a non-record marked `opaque` | Rejected |

The reader checks emitted metadata against its supplied references before replacing the output.
The writer checks input metadata before rendering or clearing partitioned outputs. Merge and
namespace remapping use the same check. Distinct opaque and complete definitions of one identity
cannot overlap architectures; architecture merging preserves the distinction.

These are opaque-storage checks, not native layout calculations or proof that every external
metadata reference resolves. The extractor must reconcile source forward declarations against
the available definitions before choosing an opaque declaration. See
[metadata opaque records](windows-metadata.md#opaque-records) for the validation APIs.

`test_rdl::opaque` has a reviewed, non-updating fixture with nominal aliases, pointers, references,
callbacks, imported functions, and interface methods. It checks semantic readback, exact text and
WinMD round-trips, external references, merge/remap, architecture variants, and negative storage
controls. `test_metadata::opaque` authors invalid metadata directly and checks rejection before
conversion can erase its meaning.

Struct bit fields use their own syntax. A run of bit fields packed into one backing integer is
written as a C-like block on that field. Each member uses `Name: width` or
`Name: LogicalType: width`. Anonymous padding uses `_: width`.

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

Typed members retain an integral primitive, enum, or integral typedef identity independently of
the backing field. `Const<T>` and `Volatile<T>` retain their ordered placement on that logical type.
Raw member annotations use the same `NativeAnnotation(source, ordinal, payload)` spelling as
declaration evidence; other member attributes are rejected.

```text
struct Packet {
    bits: u32 {
        signed: i8: 3,
        _: 2,
        state: Const<Volatile<Status>>: 3,
        #[Windows::Win32::Metadata::NativeAnnotation("CXX", 0, "[[deprecated]]")]
        flag: bool: 1,
    },
}
```

The type, qualifier, and annotation companions are attributes on the existing backing field,
not extra storage fields. Legacy `Name: width` leaves the logical type unspecified rather than
inferring a source declaration from storage. Named zero-width members and typed/annotated padding
are rejected. Member intervals cannot overlap or exceed known backing capacity, including
architecture-compatible scalar aliases. Width is not limited to the logical type's value bits.
Native allocation units, zero-width barriers, and ABI layout still require compiler evidence.
Trailing padding is represented only by backing storage, not a logical member.

The reviewed `test_rdl::logical_bitfields` fixture checks exact text/binary round-trips, physical
field identity, independent namespace routing, qualifier order, and raw member evidence.
`logical_bitfield_metadata` checks malformed descriptors before checked emission or output cleanup.
See [metadata logical bit-fields](windows-metadata.md#logical-bit-fields) for the wire model.

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
        fn Speak(&self, message: String);
        Name: String;
    }
}
```

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
- Empty record bodies are complete unless marked `#[opaque]`; never use an empty body to stand
  in for an unavailable native definition.
- The reader rejects unsupported types, constants, callback ABIs, variadic callback parameters,
  and function ABIs instead of dropping them.
- Pointer chains preserve each `*mut`/`*const` level, including mixed chains. Pointer mutability
  does not replace explicit parameter direction or source qualifiers.
- `len_param` and `size_param` store raw zero-based signature positions. Update the attribute when
  parameters move.
- RDL cannot spell a metadata parameter with neither In nor Out. Omitting direction invokes the
  type-based default.
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

Run targeted tests with:

```sh
cargo test -p test_rdl
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
| Property and event accessors | Shorthand | The writer tracks consumed accessor methods. |
| Other interface methods | Full method form | Non-accessor methods stay explicit. |
| Input direction | `#[in]` | The reader accepts `#[r#in]`; the formatter emits `#[in]`. |
| Raw identifiers, GUIDs, delegate ABIs | Canonical output | Equivalent text has one spelling. |

The reader rejects unsupported forms with errors. It does not silently drop them. Examples include
unsupported types, constants, callback ABIs, variadic callback parameters, and function ABIs.

### Inline nested records

Inline structs and unions have nested TypeDefs linked by `NestedClass` rows. Their field
signatures reference those definitions through enclosing-TypeRef chains, including every level
of nesting. The writer matches the qualified identity to the containing record and renders the
same inline syntax. It also accepts the unscoped leaf references in existing metadata.

`test_rdl::nested_references` checks physical TypeRef scopes before and after the text round trip,
alongside the canonical `struct_nested_anon.rdl` fixture. `test_metadata::nested_references`
independently covers the underlying signature reader, merge, and namespace remapper.

### Method parameter rows

The winmd writer uses `MethodDef::params_by_sequence` from
[`windows-metadata`](windows-metadata.md). Sequence 0 supplies return-type attributes. Nonzero rows
are matched to one-based signature positions, so sparse or out-of-order metadata cannot rename or
reflag another parameter. Every signature parameter is emitted; a missing row uses `pN` with no
explicit direction or optional marker, which the RDL reader lowers with its existing type-based
direction default. Duplicate and out-of-range sequences stop the write with a diagnostic.

Direction and optionality come from `MethodParam::direction()` and `is_optional()`. Reserved,
retval, and count attributes remain independent pseudos or custom attributes. The writer applies
RDL's format boundary only when spelling them: In+Out emits both markers, while Unspecified emits
the input spelling because RDL has no literal unspecified-direction syntax.

`test_rdl::method_params` authors sparse and out-of-order rows directly, checks parameter and
return pseudos in the generated RDL, and compiles the RDL back to dense metadata with the same
associations. It also checks that every non-return `Param` row in the committed WinRT, Win32, and
WDK metadata has at least one representable direction flag.

### Lossless round-trip limits

- **Direction flags:** RDL has no spelling for a `Param` row with neither In nor Out. Omitting both
  invokes the type-based default, so a metadata row with neither flag reads back as In.
- **Void return rows:** Return attributes are written after `->`. A void method has no return type
  to carry them, so attributes on its sequence 0 row are not represented.
- **Count relationships:** `#[len_param(N)]` and `#[size_param(N)]` store raw zero-based signature
  positions. Reordering parameters without updating `N` changes the relationship.
- **Native qualifiers:** Mixed `*mut`/`*const` pointer chains round-trip, including chains inside
  references and arrays. This spelling does not represent arbitrary native volatile or top-level
  const qualifiers independently of the pointer type.

### Mixed pointer chains

RDL preserves `*const *mut T` and `*mut *const T` as distinct types. Adjacent equal qualifiers
form one `metadata::Type` pointer run; a change in qualifier starts a nested pointer node. Fields,
parameters, results, callbacks, interface methods, and typedef targets use the same encoding.

Uniform pointer signatures keep their existing bytes. A required
`Windows.Win32.Metadata.NativeMutableAttribute` signature modifier separates a mutable run from
an enclosing const run; it is not a parameter attribute and does not imply input/output direction.
The metadata reader decodes run boundaries recursively. See
[pointer signatures](windows-metadata.md#pointer-signatures) for the wire format.

`test_metadata::pointers` checks exact bytes, every qualifier order through depth five, nested
signature contexts, malformed modifiers, and namespace remapping of named pointees.
`test_rdl::mixed_pointers` uses a reviewed fixture outside the auto-updating round-trip directory.
It checks semantic readback, exact RDL text, deterministic WinMD bytes, merge/remap preservation,
and compilation of the modifier vocabulary.
