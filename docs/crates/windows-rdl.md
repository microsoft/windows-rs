# windows-rdl

> A parser for RDL (Rust Definition Language) and an ECMA-335 metadata generator.

- 📦 [crates.io](https://crates.io/crates/windows-rdl)
- 📖 [docs.rs](https://docs.rs/windows-rdl)
- 🚀 [Getting started](../../crates/libs/rdl/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/rdl)

`windows-rdl` is the front of the metadata authoring pipeline. It parses RDL (Rust Definition
Language), a small Rust-like syntax for Windows APIs. It emits ECMA-335 `.winmd` metadata for
[`windows-bindgen`](windows-bindgen.md). It also writes canonical RDL from `.winmd` files.

Use `windows-rdl` when an API needs metadata first. You can write RDL by hand. You can also generate
RDL from C or C++ headers with [`windows-clang`](windows-clang.md). Then pass the `.winmd` output to
`windows-bindgen`.

## Getting started

Add `windows-rdl` as a build dependency. It usually runs from a codegen tool or `build.rs`. It is
not a runtime dependency.

```toml
[build-dependencies]
windows-rdl = "0.100"
```

The crate exposes two builders:

- `reader()` compiles RDL source to `.winmd` metadata.
- `writer()` writes canonical RDL source from `.winmd` metadata.

Input, reference, and output paths accept strings, `Path`, or `PathBuf`, so build scripts can pass
paths without converting them to UTF-8 strings. `.input_text(..)` and `.input_texts(..)` compile RDL
source already in memory.

Use `.input_text_named(name, source)` or `.input_texts_named(sources)` for in-memory sources whose
names should appear in diagnostics. `Diagnostic` carries a severity, optional code, source labels,
notes, and help. `DiagnosticReport` stores collected diagnostics with their original source text.
`Error` is a small owned wrapper that dereferences to its `Diagnostic`.

### RDL to winmd, and back

Use `reader` to compile `.rdl` into `.winmd`. Use `writer` to regenerate canonical `.rdl` from
`.winmd`.

```rust,no_run
// RDL source -> winmd metadata.
windows_rdl::reader()
    .input("example.rdl")
    .output("example.winmd")
    .write()
    .unwrap();

// winmd metadata -> canonical RDL source.
windows_rdl::writer()
    .input("example.winmd")
    .output("example.rdl")
    .write()
    .unwrap();
```

RDL can reference types it does not define. Examples include `HRESULT` and
`Windows::Win32::System::Com::IUnknown`. Add the standard metadata so those references resolve.

```rust,no_run
windows_rdl::reader()
    .input("example.rdl")
    .reference_default()
    .output("example.winmd")
    .write()
    .unwrap();
```

The reader treats the default metadata as references while compiling the input RDL. Add other
reference metadata with `.reference(path)`, `.references(paths)`, `.reference_bytes(bytes)`, or
`.reference_byte_sets(byte_sets)`. The writer has the corresponding `.input`, `.inputs`,
`.input_bytes`, and `.input_byte_sets` methods and treats default metadata as input to render.

### C/C++ headers to RDL

Use [`windows-clang`](windows-clang.md) when an API ships only a C or C++ header. The `clang()`
builder parses the header into RDL. Then `reader()` compiles that RDL to metadata.

Each header is parsed as its own translation unit. The scraper emits only that header's top-level
declarations. It does not emit declarations from `#include` files. List each header you need as a
separate input.

```rust,no_run
windows_clang::clang()
    .args(["-x", "c++", "--target=x86_64-pc-windows-msvc"])
    .input("Example.h")
    .reference_default()
    .output("example.rdl")
    .namespace("Example")
    .library("Example.dll")
    .write()
    .unwrap();
```

## RDL syntax

RDL looks like a small Rust module. A top-level `mod` is a metadata namespace. Tag it `#[winrt]` or
`#[win32]` to select the type system. Attributes map to metadata attributes. Item keywords mirror
metadata kinds.

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

See [`windows-clang`](windows-clang.md#bit-field-member-scraping) for how the scraper emits bit
fields. See [`windows-bindgen`](windows-bindgen.md#generating-bit-field-accessors) for the accessors
they drive.

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

## How it fits with windows-bindgen

`windows-rdl` and `windows-bindgen` are two stages in one pipeline.

```text
C/C++ headers -- clang() --> .rdl -- reader() --> .winmd -- bindgen() --> bindings.rs
 (windows-clang)             (windows-rdl)        (windows-bindgen)
```

Skip `windows-rdl` when metadata already exists. Use it when you need to create metadata first. You
can write RDL by hand or lift declarations from a header.

Two in-repo tools show both uses:

- `tool_webview` runs the full path. WebView2 ships only a C/C++ header. `clang()` produces
  `WebView2.rdl`. `reader()` compiles it to `WebView2.winmd`. Then `windows_bindgen::bindgen`
  generates bindings for [`windows-webview`](windows-webview.md).
- `tool_reactor` hand-writes COM interfaces and bootstrap functions in
  `crates/tools/reactor/src/extras.rdl`. These declarations fill gaps in the WinUI and Windows App
  SDK metadata. The tool compiles them with the standard Win32 winmd into `extras.winmd`. Then it
  feeds that winmd to `windows_bindgen::bindgen` for [`windows-reactor`](windows-reactor.md).

In both tools, `reader` also gets the standard metadata as references. That lets RDL names resolve
against the standard definitions.

---

## Internal documentation

The rest of this page covers how the crate is built and maintained. It is for contributors and is
not needed to use `windows-rdl`.

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

- `test_rdl` covers RDL to winmd round trips with `input/*.rdl` fixtures.
- `test_clang` covers header to RDL output with `expected/*.rdl` goldens.
- `tool_roundtrip` re-derives committed RDL files from committed winmd files. The `gen` workflow
  enforces a clean `git diff`.
- `test_bindgen` covers the `.winmd` to Rust step that consumes this crate's output.

Run targeted tests with:

```sh
cargo test -p test_rdl
cargo test -p test_clang
```

### Default metadata files

`windows-rdl` builds the default metadata files used by the in-repo generators and library crates.

| File | Source | Writer |
|------|--------|--------|
| `crates/libs/default/Windows.winmd` | SDK Contracts winmds, merged and written through RDL | `tool_winrt` |
| `crates/libs/default/Windows.Win32.winmd` | Windows SDK + WDK headers scraped to RDL, um + km merged | `tool_win32` |

The committed RDL files are the reviewable source for these metadata files:

- `metadata/winrt/*.rdl` is partitioned by namespace.
- `metadata/win32/*.rdl` is partitioned by defining header.
- `metadata/wdk/*.rdl` is partitioned by defining header.

The binary winmd files are derived artifacts. Generation is deterministic. The metadata writer
stages tables in `BTreeMap`s and uses a fixed zero GUID for the module MVID.

Every maintained crate that needs Win32 metadata resolves against the in-repo `Windows.Win32.winmd`.
Minimal-binding crates and `windows-reactor` use it directly. The `windows` and `windows-sys` crates
use it through `tool_package`.

### Multi-arch merge

`tool_win32` scrapes x64, arm64, and x86 into separate RDL sets. Then `merge_arch_rdl` combines them
into one winmd. A type with the same shape on every architecture is emitted once. A type that
differs by architecture is split into per-architecture copies tagged `#[arch(X86|X64|Arm64)]`.

The merge compares type structure through [`windows-metadata`](windows-metadata.md).
`merge_arch_rdl` handles orchestration. It reads each architecture's RDL, runs the merge, and writes
the combined output. `ArchInput` stores its RDL directory and winmd as `PathBuf`.

### Published crates and namespace remap

The in-repo Win32 and WDK metadata lives in flat namespaces. Published `windows` and `windows-sys`
APIs are partitioned behind many Cargo features.

`tool_package` remaps the flat metadata into header-stem namespaces under `target/package/`. It uses
the committed `metadata/win32` RDL directory as the routing signal. Then it runs `windows-bindgen`
over that partition. `tool_features` uses the same remap so feature search reports the same header
stems.

The in-repo WinRT `Windows.winmd` is projected with the remapped Win32 and WDK metadata.

### Round-trip rules

RDL is the reviewable source for WinRT, Win32, and WDK metadata. The `.winmd` files are derived
artifacts. The `gen` workflow runs the generators and `tool_roundtrip`. It fails when regeneration
changes tracked files.

| Family | External source | RDL layout | Winmd build path |
|--------|-----------------|------------|------------------|
| WinRT | SDK Contracts winmds | `metadata/winrt`, per namespace | merge SDK winmds, write RDL, read RDL to winmd |
| Win32 | SDK headers | `metadata/win32`, per header | scrape headers to RDL, merge architectures, read RDL to winmd |
| WDK | WDK headers | `metadata/wdk`, per header | scrape headers to RDL, merge architectures, read RDL to winmd |

`tool_roundtrip` validates the reverse direction:

- WinRT uses `writer(Windows.winmd).split()` to write `metadata/winrt`.
- Win32 and WDK cannot recover header files from flat winmd alone. The tool reads the committed RDL
  layout to map type names back to header stems. Then it writes `metadata/win32` or `metadata/wdk`
  with `writer(winmd).partition(map)`.

### Current normalization rules

Some metadata forms have canonical RDL spellings. These rules are intentional. They keep generated
RDL stable.

| Form | RDL spelling | Reason |
|------|--------------|--------|
| WinRT `System.Char` | `Char16` | It stays distinct from `u16` and maps to `metadata::Type::Char`. |
| Property setter parameter names | `value` | RDL property shorthand does not store the original parameter name. |
| Event add parameter names | `handler` | RDL event shorthand stores the event shape, not the accessor parameter name. |
| Event remove parameter names | `token` | The remove accessor takes an event token. |
| Property and event accessors | Property or event shorthand | The writer tracks consumed `get_`, `put_`, `add_`, and `remove_` methods. |
| Unconsumed interface methods | Full method form | Methods that are not part of shorthand stay explicit. |
| Input direction | `#[in]` | The reader accepts `#[r#in]`; the formatter emits `#[in]`. |
| Raw identifiers, GUID constants, and delegate ABI spelling | Canonical writer output | Text can differ while metadata stays equivalent. |

The reader rejects unsupported forms with errors. It does not silently drop them. Examples include
unsupported types, constants, callback ABIs, variadic callback parameters, and function ABIs.

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
- **Pointer constness:** `metadata::Type` stores one constness bit with a pointer depth. Uniform
  chains such as `*mut *mut T` and `*const *const T` round-trip. Mixed chains are rejected before
  metadata is written, including chains nested inside a reference.

## Future development

RDL should remain an explicit description of the API and its ABI. Concision is useful when it
removes repetitive syntax without hiding interface boundaries, factory calls, allocations,
marshaling, versioning, compatibility obligations, or other costs that API authors need to review.
MIDL3 syntax is therefore an input for comparison, not a feature checklist.

The first development work should focus on correctness, lossless conversion, diagnostics, and
source-level usability. Runtime-class shorthand should be evaluated later and adopted only where
the lowering remains visible and predictable.

### Semantic validation

The reader currently combines parsing, name resolution, validation, and metadata emission. This
makes it difficult to report more than the first error and allows some invalid source shapes to
reach the writer. Add a resolved RDL model and a validation pass before emitting metadata:

```text
source -> syntax tree -> resolved model -> validation -> metadata writer
```

The resolved model should retain source spans for declarations, names, types, attributes, attribute
arguments, parameters, and references. Validation should produce a list of diagnostics rather than
stopping at the first error. Metadata emission should run only when the list contains no errors.

Validation rules should be grouped by profile:

| Profile | Examples |
|---------|----------|
| Common | Names, attributes, generic arity, overloads |
| Win32 | Calling conventions, libraries, pointers, arrays, architecture, layout |
| WinRT | Type graph, classes, factories, contracts, versions, overloads |
| Round trip | Source or metadata forms that cannot be represented |

Duplicate handling needs an explicit symbol model rather than a blanket uniqueness check. A scope
may contain several declarations with the same source name only when the declaration kind permits
it and the variants are distinguishable:

- Method overloads must have distinct signatures and valid overload metadata.
- Architecture variants must have nonconflicting architecture masks.
- Repeated partial declarations must follow a defined merge rule.
- Properties, events, fields, enum members, generic parameters, and ordinary types must be unique.
- A function, constant, and type sharing a metadata scope must be checked against the output
  representation rather than accepted because they live in separate internal maps.

Each collision diagnostic should identify the new declaration and the earlier declaration. This
work should resolve the rules tracked by
[`windows-rdl` duplicate symbols](https://github.com/microsoft/windows-rs/issues/4186).

The reader now runs an initial duplicate-symbol pass after indexing and before metadata emission.
`RDL0001` diagnostics label both declarations. The pass rejects collisions between top-level
types, functions, and constants; duplicate members and parameters; overlapping architecture
variants; and method overloads with the same parameter types. Distinct method signatures,
disjoint architecture variants, and matching split get/set properties remain valid.

Top-level symbol and property checks still compare syntax spellings. Method overloads and attribute
constructors now compare resolved `metadata::Type` signatures, so an import alias and a qualified
path to the same type collide. `RDL0005` rejects missing or extra generic arguments. Moving the
remaining checks to a resolved model and preventing every validation failure from reaching the
encoder remain part of the work below.

The winmd writer now preflights Property and Event rows before reconstructing shorthand. Custom
attributes, nonzero flags, property constants, unsupported or duplicate semantics, missing
accessors, mismatched accessor names, non-special accessors, and accessor custom attributes are
rejected instead of dropping the row or synthesizing a different one. This keeps MethodSemantics
authoritative while RDL lacks syntax for attributes on property/event rows or their accessors.
The writer also scans every custom-attribute row before writing output and rejects attributes on
TypeRef, MemberRef, and TypeSpec parents, which are not represented by RDL declarations.
FieldLayout rows are preserved by metadata merge and remap. RDL unions represent only explicit
layouts where every instance field has offset zero; missing or nonzero offsets are rejected rather
than normalized to overlapping fields.

The same pre-emission pass now rejects accepted syntax that the encoder cannot represent.
`RDL0002` covers attributes on event shorthand, generic functions/callbacks/interface methods,
variadic callbacks/delegates/interface methods, generic bounds/defaults/attributes, attribute
constructors with returns or variadic parameters, and enum variants with payload fields.

Where metadata has a direct representation, the conversion now preserves it instead of rejecting
it. Custom attributes on typedefs, enum variants, GUID constants, and property-key constants
round-trip in both directions. The writer rejects equivalent winmd states that have no RDL
spelling, including generic methods, variadic non-function methods, generic parameter flags,
attributes on synthetic typedef, enum, or attribute-property fields. Callback and delegate
`Invoke` attributes use the explicit `#[invoke(Attribute(...))]` wrapper and round-trip without
moving the attribute to the generated type.

Initial validation work:

1. Done: define initial syntax-level symbol keys and legal duplicate categories.
2. Done: add negative fixtures for duplicate properties, events, fields, methods, types,
   constants, functions, architecture variants, and parameter names.
3. In progress: separate resolve and validate from `Encoder` so validation cannot partially mutate
   a winmd. Type and attribute lookup now share `Resolver`; signature and generic-arity checks run
   before encoding.
4. Add target validation for every built-in and metadata-defined attribute.
5. Done: add checks for parsed syntax that is currently ignored or not represented, including
   attributes on event shorthand, method generics, and variadic interface methods.
6. Done: run the validator over the committed WinRT, Win32, and WDK RDL as a compatibility
   baseline.

### Lossless metadata conversion

RDL cannot serve as the reviewable source for arbitrary winmd files while metadata tables are
silently discarded. `windows-metadata` now reads Property, PropertyMap, Event, EventMap, and
MethodSemantics rows and preserves them through merge and namespace remapping. Those copy paths
also retain WinRT runtime-class methods, property constants, flags, signatures, and custom
attributes on properties and events.

The desired rule is:

> Every metadata fact is preserved, represented explicitly, or rejected with a diagnostic.

Initial losslessness work:

1. Done: add reader row types and traversal APIs for Property, PropertyMap, Event, EventMap, and
   MethodSemantics.
2. Done: preserve those tables and WinRT class methods through `windows-metadata` merge and remap.
   FieldLayout rows are also read and preserved through both paths.
3. Done: add focused winmd -> winmd tests that compare methods, property and event rows, flags,
   signatures, constants, custom attributes, and accessor semantics before and after conversion.
4. Add winmd -> RDL -> winmd tests for properties, events, class methods, return rows, and custom
   attributes on every supported parent. Property/Event row states without a lossless shorthand are
   now rejected and covered by negative tests. TypeRef, MemberRef, and TypeSpec attributes are also
   rejected before output because RDL has no declaration site for them.
5. Inventory every ECMA-335 table that the reader skips and classify it as preserved, irrelevant
   to Windows metadata, or unsupported with an error.
6. Replace known silent losses with errors until a lossless spelling or copy path exists.

The existing round-trip limits above should become machine-readable capabilities so the writer can
report the exact unrepresentable row rather than relying only on documentation.

### Diagnostics

Diagnostics should read naturally in a terminal and follow the useful parts of `rustc` output:

```text
error[RDL0007]: duplicate property `Name`
  --> src/widget.rdl:18:9
   |
12 |         Name: String;
   |         ------------ first declared here
...
18 |         Name: String;
   |         ^^^^ duplicate property
   |
   = help: remove one declaration or use distinct property names
```

A diagnostic should contain:

- Stable code, severity, message, and primary span.
- Zero or more labeled secondary spans.
- Notes and actionable help.
- Source name and source text supplied independently, including named in-memory inputs.
- A rendering API separate from the diagnostic data model.

The library should return structured diagnostics and leave color, terminal width, and final
rendering to the caller. The default renderer should support color auto-detection, `--color`, short
and human-readable formats, and one final error count. JSON output should be available for editors
and build systems.

Parser recovery is needed to report several useful errors from one file. It does not need to accept
an invalid tree for metadata emission. Recovery can synchronize at module items, interface members,
fields, enum variants, and semicolons. Semantic validation can then continue for unaffected items.

`Reader::check_all` now returns a `DiagnosticReport`. Parsing continues across input files, and
validation collects independent errors across declarations and namespaces. Resolved method and
attribute-constructor signatures, unresolved types, import ambiguity, and generic arity are checked
before encoding. The report retains source text for named in-memory inputs and files, so terminal
rendering does not need to re-read them. When one source name identifies different texts, the
report omits that ambiguous lookup rather than showing the wrong source. Stable source IDs, parser
recovery within one file, and collection of all errors still discovered during encoding remain
future work.

### `riddle` command-line tool

`riddle` is a small binary crate built on the library APIs rather than the removed bindgen
argument forwarder. The binary contains argument parsing, terminal rendering, standard input, and
exit-code policy; parsing, validation, resolution, and metadata encoding remain library code.

The initial implementation provides `riddle check` and `riddle build`. Both accept repeated file
or directory inputs, repeated winmd references, standard input, and the default Windows metadata.
`Reader::check` runs the same pipeline as `Reader::write` without creating a winmd. `riddle check`
uses `Reader::check_all`, rendering every independent diagnostic with source locations, labeled
lines, notes, help, and a final error count. Invalid RDL uses exit code 1, while invalid command
lines use exit code 2.

An initial command set:

| Command | Purpose |
|---------|---------|
| `riddle check` | Parse, resolve, and validate RDL without writing a winmd |
| `riddle build` | Validate and compile RDL to winmd |
| `riddle fmt` | Format files, with `--check` for CI |
| `riddle dump` | Write canonical RDL from winmd |
| `riddle validate` | Validate an existing winmd and report unsupported or malformed metadata |

Future commands should use the same input and diagnostic behavior. Response files can be added if
Windows command-line limits become relevant.

### Formatting

The formatter validates a complete RDL file before formatting and returns a named diagnostic for
invalid input. A comment-aware lexer protects regular and documentation comments before the
canonical token layout runs, then restores them at their source positions. Formatting is
idempotent across comments between attributes, declarations, members, parameters, and closing
delimiters.

`riddle fmt` formats files or directories in place and writes formatted standard input to standard
output. `riddle fmt --check` reports files that differ without changing them. All inputs are read,
parsed, and formatted before any file is replaced, so one invalid file cannot cause a partial
update.

The layout pass still operates on generic Rust tokens after RDL syntax validation. Moving layout to
the RDL syntax tree and adding source-range formatting remain future work. That change should
preserve source constructs that are metadata-equivalent but meaningful to authors.

### Imports and name resolution

Imports are an authoring convenience for types and attributes. They are scoped to one RDL file,
including every metadata namespace declared by that file. Import paths therefore name absolute
metadata namespaces rather than paths relative to a module.

```rust
use Windows::Foundation::Point;
use Windows::Foundation::Collections as Collections;
use Windows::Foundation::{Point, Size as Extent};
use Windows::Foundation::Metadata::*;
```

Named imports, aliases, grouped imports, `self` within a group, and namespace globs are supported.
An imported name can identify a type or a namespace according to where it is used, so a namespace
alias such as `Collections` resolves `Collections::IIterable`. The same model applies to
attributes. Importing `Marker` resolves the metadata type `MarkerAttribute` when `#[Marker]` is
used.

Names resolve in this order:

1. Generic parameters, primitive types, and core spellings.
2. A declaration in the current namespace.
3. An explicit named or aliased import.
4. Namespace glob imports.
5. Core aliases such as `Type`, `GUID`, and `HRESULT`.

An explicit import can disambiguate competing globs. Multiple glob matches produce `RDL0004` with
a label for each candidate instead of selecting the first match. `RDL0003` reports an import whose
target is not a known namespace, type, or source-spelled attribute. Reusing one local import name
for different targets also produces `RDL0004`; repeating the same import is accepted.

Leading `crate`, `self`, and `super` imports are rejected because their meaning would depend on
which namespace in the file used them. `self` remains valid within a group such as
`use Windows::Foundation::{self as Foundation, Point};`.

The writer does not infer or emit imports. Canonical winmd -> RDL output uses qualified or
namespace-relative paths, so metadata round trips do not depend on import style. Unused-import and
shadowed-import warnings can follow after diagnostics support collecting warnings alongside
errors.

### Overload authoring

Overloads are a suitable convenience because the metadata already carries the distinction and the
author must still write every ABI method signature. RDL should let authors use the public method
name while supplying or deriving the metadata ABI name.

Investigate a source-level spelling along these lines:

```rust
#[overload(MethodWithValue)]
fn Method(&self, value: i32);
```

The final design should make these facts clear:

- The public projected name.
- The unique metadata method name.
- The full signature used to distinguish overloads.
- Whether a default overload is required.

Automatic metadata-name generation may be offered, but canonical RDL should expose the generated
name so ABI changes remain reviewable. Validation must detect duplicate signatures, reused metadata
names, inconsistent overload groups, and invalid `DefaultOverloadAttribute` placement. This work
should address [`windows-rdl` overload attribute should be supported directly][rdl-overloads].

### Runtime-class authoring

MIDL3 runtime-class bodies are much shorter because the compiler synthesizes default, factory,
static, and composable interfaces. Those interfaces are real ABI and versioning boundaries. RDL
should not copy this design without showing authors what is generated and how it changes.

Investigate class conveniences with these constraints:

- No hidden interface is added without a stable, inspectable name.
- Constructor and static-member lowering is available through `riddle dump` or another expansion
  view.
- Interface assignment and method order are deterministic.
- Adding a constructor or member cannot silently change an existing interface ABI.
- Version and contract placement is explicit or derived by a documented, reviewable rule.
- Authors can always write the fully lowered interface form.

Compare three designs before implementation:

1. Keep classes explicit and add only diagnostics and templates for common factory patterns.
2. Add class-body syntax that requires authors to name the target interface for each member group.
3. Add MIDL3-like synthesis, but require an expansion manifest that is committed and checked for
   ABI changes.

Use the remaining MIDLRT-backed activation, constructor, overload, composable, `noexcept`, and
reference-parameter tests as study cases. Replacing MIDLRT in those tests is useful only when the
resulting RDL makes the ABI at least as reviewable as the current explicit interface form.

### Initial implementation order

1. Done: introduce structured diagnostics and named source inputs without changing parser behavior.
2. Done: add duplicate-symbol validation and negative diagnostic fixtures.
3. Done: reject syntax and metadata states that are currently ignored or silently lost.
4. Done: add Property/Event/MethodSemantics reading and preserve those tables through merge and
   remap.
5. Done: restore a minimal `riddle check` and `riddle build` on the new library APIs.
6. Done: replace the formatter's silent parse fallback, preserve comments, and add `riddle fmt`.
7. Done: add named imports, aliases, grouped imports, and ambiguity diagnostics.
8. Deferred: implement explicit overload authoring after the semantic foundation described below.
9. Deferred: evaluate runtime-class conveniences after overload lowering is explicit and reviewable.

### Review after the initial implementation

Steps 1-7 removed several silent-loss paths and made the existing compiler usable from a terminal.
They also made the next architectural limit clearer: validation and resolution still run partly
inside `Encoder`, return the first error, and use syntax spellings for some semantic comparisons.
Adding overload authoring directly to that design would make the coupling worse.

The main findings are:

- **Validation:** Most passes stop at the first error. Collect independent semantic diagnostics
  before encoding.
- **Resolution:** Type and attribute lookup now share `Resolver` and produce canonical
  `metadata::Type` identities. Declaration-level resolved nodes are still needed.
- **Duplicate checks:** Method and attribute-constructor signatures now compare resolved types and
  generic arity. Properties, class interface lists, and other checks still use syntax spelling.
- **Encoding:** Some unresolved or invalid states are found while the winmd writer is being
  mutated. Make the encoder consume only a validated resolved model.
- **Diagnostics:** The data model supports labels, but source text is external and `riddle` renders
  one label at a time. Add a source registry, diagnostic collections, color/short/JSON rendering,
  and a final count.
- **Formatting:** Token formatting is safe but does not understand all RDL constructs. Grouped
  imports initially exposed poor brace layout. Keep focused fixes now; move final layout to the RDL
  tree later.
- **Losslessness:** Known losses are rejected or documented, but the ECMA table and parent inventory
  is incomplete. Finish a machine-checked support inventory before claiming arbitrary winmd round
  trips.

Grouped imports are now formatted inline rather than as ordinary brace blocks. This is a useful
example of why each new authoring feature needs parser, resolver, diagnostic, formatter, CLI, and
round-trip coverage rather than parser coverage alone.

The next phase should proceed in this order:

1. Done: introduce a source registry, canonical type identities, and a shared name resolver.
   `DiagnosticReport` and `Resolver` provide these layers while preserving the current
   `Result<T, Error>` APIs as convenience wrappers. Stable numeric source and declaration IDs can
   be added with the resolved model.
2. Build a resolved model for declarations, types, attributes, parameters, and imports. Move
   duplicate checks and import ambiguity checks onto that model, and emit no metadata when it has
   errors.
3. Add generic-arity, attribute-target, profile, and resolved-signature validation. Finish the
   metadata table and custom-attribute parent inventory with rejection tests for unsupported rows.
4. Implement explicit overload authoring and canonical expansion using resolved signatures and
   stable metadata names.
5. Upgrade `riddle` rendering and add `dump` and `validate` once the library can return complete
   diagnostic collections and unsupported-metadata findings.
6. Move formatting to the RDL syntax tree and add range formatting if editor use justifies it.
7. Evaluate runtime-class conveniences only after the expanded ABI can be inspected and compared.

[rdl-overloads]: https://github.com/microsoft/windows-rs/issues/4166
