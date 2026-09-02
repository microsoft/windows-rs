# windows-metadata

> A low-level reader and writer for the ECMA-335 metadata format.

- 📦 [crates.io](https://crates.io/crates/windows-metadata)
- 📖 [docs.rs](https://docs.rs/windows-metadata)
- 🚀 [Getting started](../../crates/libs/metadata/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/metadata)

`windows-metadata` reads and writes the ECMA-335 metadata format used by .NET, WinRT, and the Win32
metadata. It is the foundation [`windows-bindgen`](windows-bindgen.md) builds on. The
`reader::Index` type loads one or more `.winmd` files and lets you query namespaces, type
definitions, and their members.

Applications should start with a focused crate or `windows-bindgen`; binary applications may use
[`windows`](windows.md) or [`windows-sys`](windows-sys.md) when they want a broad pre-generated
surface. Use `windows-metadata` directly when writing a metadata inspector, generator, merger, or
namespace transformation. The API exposes metadata facts; it does not project them into a safe Rust
API.

Start with the crate [README](../../crates/libs/metadata/readme.md) for dependency setup and a
focused type query.

## First workflow: inspect an API definition

1. Load a file with `reader::Index::read`, or construct `reader::File` values and pass them to
   `Index::new`.
2. Locate a type with `get` when duplicates are possible or `expect` when exactly one definition is
   an invariant.
3. Inspect `category`, `extends`, fields, methods, signatures, attributes, and nested types.
4. Keep rows borrowed from the index; use `leak` or `read_static` only when a tool requires
   process-lifetime rows.

For methods, match ECMA-335 parameter rows with
`MethodDef::params_by_sequence(signature.types.len())`. Sequence 0 belongs to the return value;
nonzero sequences are one-based signature positions. `params()` instead returns physical table
order and is appropriate for lossless copying, not semantic parameter association.

`tool_reactor` provides a larger example. Its metadata resolver loads WinUI winmd files, adds
`windows-default` byte inputs, indexes classes and interfaces, then uses method signatures and
attributes to drive generated UI code. `tool_features` uses `Index::iter_items` to enumerate types,
free functions, and constants.

## Input and output model

`reader::File` owns one winmd byte stream. `reader::Index` combines files into searchable
namespace, type, nested-type, and expanded Win32 `Apis` indexes. A zero architecture selector keeps
all rows; `Index::new_for_architecture` selects neutral rows plus one of X86 (1), X64 (2), or Arm64
(4).

The writer API is lower-level. `writer::File` builds ECMA-335 rows and `into_stream` returns the
finished bytes. Prefer [`windows-rdl`](windows-rdl.md) when a reviewable source format is useful;
use the writer directly when a tool is copying or synthesizing table rows.

## Common tool tasks

| Task | API |
| --- | --- |
| Read one winmd | `reader::File::read` or `reader::Index::read` |
| Read bytes already in memory | `reader::File::new` |
| Query several files together | `reader::Index::new` |
| Select one architecture | `reader::Index::new_for_architecture` |
| Merge ordinary winmds | `merge().input(...).output(...).merge()` |
| Merge per-architecture winmds | `merge().arch_input(path, bits)` |
| Union compatible duplicate enums | `Merger::union_enums` |
| Remap flat namespaces | `remap().source(...).routes(...).fallback(...)` |
| Author metadata tables | `writer::File` |

The merger accepts files or directories. Architecture merge retains one neutral definition only
when the same shape is present on every merged architecture; divergent shapes are tagged and kept.
The remapper rewrites definitions and references together and splits Win32 `Apis` members across
their routed target namespaces.

## Integration and pitfalls

- [`windows-default`](windows-default.md) supplies the standard WinRT and Win32 files as byte
  slices. Construct `reader::File` values from those bytes when a custom tool needs them.
- [`windows-rdl`](windows-rdl.md) provides a higher-level text authoring and decompilation path.
- [`windows-bindgen`](windows-bindgen.md) applies Rust projection policy to these raw metadata
  facts.
- A metadata parameter marked neither In nor Out is `ParamDirection::Unspecified`. Do not infer
  input direction in the metadata layer.
- Optional, reserved, retval, and buffer-count annotations are independent facts. Consumers decide
  how those facts affect their public API.
- Duplicate type definitions can be valid in an unfiltered multi-architecture index. Do not call
  `expect` unless uniqueness is guaranteed.
- Preserve physical row order when copying malformed or sparse metadata; use semantic helpers when
  interpreting it.

The focused fixtures in `crates/tests/libs/rdl` and `crates/tests/libs/bindgen` show metadata
shapes. The main tool consumers are `tool_winrt`, `tool_win32`, `tool_package`, `tool_features`,
and `tool_reactor`.

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is for contributors and
is **not needed to use `windows-metadata`**.

### How it's built

Consumed by `windows-bindgen`. `src/bindings.rs` is generated by `tool_bindings` from
`crates/tools/bindings/src/metadata.txt`; the ECMA-335 tables and readers are hand-written.

### Multi-arch merge

`windows-metadata`'s `merge` module coalesces the per-architecture winmds that `tool_win32` scrapes
(x64, arm64, x86) into a single winmd. A type identical across every arch is emitted once as
arch-neutral; a type that diverges is split into per-arch copies, each tagged with a
`SupportedArchitectureAttribute` (spelled `#[arch(X86|X64|Arm64)]` in RDL).

`reader::Index::new_for_architecture` selects one of those copies while retaining neutral rows.
Its bit values match the metadata attribute: 1 for X86, 2 for X64, and 4 for Arm64. Plain
`Index::new` keeps every copy for tools that need to inspect or merge the complete metadata.

The collapse-or-split decision is structural, driven by `type_sig()` - a hash of everything that can
legitimately differ between architectures:

- **fields** - name, type, and constant value (so an enum whose members hold different per-arch
  values splits instead of silently dropping the divergent values);
- **method signatures** - Win32 callbacks and WinRT delegates have no fields and diverge *only* in
  their `Invoke` signature, so a fields-only signature would wrongly collapse arch-divergent
  callbacks into one untagged copy;
- **layout** - `#pragma pack` and `ClassLayout`;
- **`AlignmentAttribute`** - `__declspec(align(N))` raised alignment is encoded *only* by that
  attribute (`ClassLayout` can only *lower* alignment), so a type that differs solely in forced
  over-alignment must fold the attribute into the signature or the divergent copy is lost;
- **flags** - the type's attribute flags.

Types present on only a *subset* of arches still go through the same structural split, so a type
that is present on x64+arm64 (but not x86) and *diverges* between those two is split per arch rather
than collapsing to whichever copy happened to be first. A group is emitted arch-neutral only when
its signature spans every arch in the run.

Unmanaged callbacks have one narrow semantic reconciliation before that split. If at least one
architecture explicitly uses `isize` or `usize` and every other copy uses the same native-sized
integer for its architecture (`i32`/`u32` on x86, `i64`/`u64` on x64 or arm64), the callback keeps
one native-sized signature. It becomes arch-neutral when present on every merged architecture;
otherwise it keeps its subset arch tag. This handles SDK declarations such as `FARPROC`, whose
return is spelled `INT_PTR` on 64-bit but legacy `int` on x86. A plain `i32`/`i64` pair does not
qualify: the explicit native-sized spelling is required as semantic evidence.

The merge is deterministic: it stages through `BTreeMap`s and insertion-ordered `Vec`s, with no
`HashMap` reaching the output.

The merger and namespace remapper accept strings, `Path`, or `PathBuf` for input and output paths
and retain them as `PathBuf`. The remapper provides singular/plural `input`/`inputs`,
`source`/`sources`, and `route`/`routes` methods.

### Method parameter association

ECMA-335 `Param` rows are not positional. `Param.Sequence == 0` describes the return value, and a
nonzero sequence identifies a one-based signature parameter. Rows may be absent, sparse, or stored
out of order.

`reader::MethodDef::params_by_sequence(parameter_count)` returns a separate optional return row and
one `Option<MethodParam>` per signature position. It reports duplicate sequences and nonzero
sequences outside the signature as `MethodParamSequenceError`; consumers do not choose a
first-wins or last-wins interpretation. If several rows are invalid, the first invalid physical row
is reported. `MethodDef::params()` still iterates physical table order. The merge and remap writers
use that physical iterator on purpose so lossless copying preserves row order, sparse rows, and
malformed input for a later validator.

`MethodParam::direction()` decodes only the `In` and `Out` bits into `ParamDirection::{Input,
Output, InputOutput, Unspecified}`. `is_optional`, `is_reserved`, and `is_retval_attribute` expose
their independent metadata facts. These helpers do not inspect pointer mutability, infer a default
for an unspecified direction, combine `ReservedAttribute` with `Optional`, or decide whether a
parameter should become a language return value. Array and byte-count attributes remain available
through `attributes()` because each projection validates different public-surface shapes.

### Determinism and the winmd writer

The writer is the foundation of the pipeline's reproducible builds. It stages `Constant` /
`Attribute` / `GenericParam` (and the bindgen `TypeTree`) in `BTreeMap`s / `BTreeSet`s, and the
module MVID is a fixed zero GUID, so regenerating a winmd is byte-for-byte identical across
platforms (CI validates the committed winmds on Linux).

Notable invariants in the ECMA-335 tables:

- The `HasSemantics` coded index and the `Property`/`Event`/`MethodSemantics` tables are emitted
  only when non-empty, and `MethodSemantics` rows are sorted by `Association` - strict readers
  reject an unsorted table.
- `write_index` asserts on the run-list one-past-end sentinel rather than silently wrapping to `0`
  for a target table with exactly 65535 rows (the guard lives on the write side so the reader's
  width threshold stays ECMA-conformant for external winmds).
- `Value::Bool` constants round-trip symmetrically (the reader maps `ELEMENT_TYPE_BOOLEAN`); the CLR
  element-type vocabulary (`ELEMENT_TYPE_*`) is hand-authored in `src/clr.rs` because its defining
  header (`corhdr.h`) ships only in the .NET SDK and cannot be scraped.

### Testing

Run `cargo test -p windows-metadata`; see also the workspace test crates. The arch-merge
collapse/split rules are pinned by `arch_roundtrip.rs` (divergent fields, callbacks, forced
alignment, enum constant values, subset-present divergence) and `merge.rs` (native-sized callback
reconciliation). `method_params.rs` authors metadata directly with `writer::File` and covers dense,
absent, return, sparse, out-of-order, duplicate, and out-of-range parameter rows. It also covers all
four raw directions and verifies that optional, reserved, retval, and count attributes remain
independent facts. `remap.rs` covers explicit and fallback namespace routing, singular/plural
builder methods, missing outputs, and invalid inputs.
