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

`tool-reactor-metadata` provides a larger example. Its metadata resolver loads WinUI winmd files,
adds `windows-default` byte inputs, indexes classes and interfaces, then uses method signatures and
attributes to drive generated UI code. `tool-features` uses `Index::iter_items` to enumerate
types, free functions, and constants.

## Input and output model

`reader::File` owns one winmd byte stream. `reader::Index` combines files into searchable
namespace, type, nested-type, and expanded Win32 `Apis` indexes. A zero architecture selector keeps
all rows; `Index::new_for_architecture` selects neutral rows plus one of X86 (1), X64 (2), or Arm64
(4).

Expanded API items distinguish `Item::Data` from `Item::Const` using an explicit native data
descriptor, not field flags or the absence of a value.

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
shapes. The main tool consumers are `tool-winrt`, `tool-win32`, `tool-package`, `tool-features`,
and `tool-reactor-metadata`.

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is for contributors and
is **not needed to use `windows-metadata`**.

### How it's built

Consumed by `windows-bindgen`. `src/bindings.rs` is generated by `tool-bindings` from
`crates/tools/bindings/src/metadata.txt`; the ECMA-335 tables and readers are hand-written.

### Multi-arch merge

`windows-metadata`'s `merge` module coalesces the per-architecture winmds that `tool-win32` scrapes
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

### Pointer signatures

`Type::PtrConst` and `Type::PtrMut` store a uniform pointer run and its depth. Their pointee can
be another pointer node, preserving mixed qualifier chains without a separate type model.

`System.Runtime.CompilerServices.IsConst` applies to an entire contiguous run of `PTR` bytes.
A mutable run immediately inside a const run starts with the required signature modifier
`Windows.Win32.Metadata.NativeMutableAttribute`. Without that boundary, the two runs would have
the same bytes as one uniform const run.

| Type spelling | Signature after any field/method prefix |
| --- | --- |
| `*mut *mut i32` | `PTR PTR I4` |
| `*const *const i32` | `CMOD_REQD(IsConst) PTR PTR I4` |
| `*mut *const i32` | `PTR CMOD_REQD(IsConst) PTR I4` |
| `*const *mut i32` | `CMOD_REQD(IsConst) PTR CMOD_REQD(NativeMutableAttribute) PTR I4` |

Uniform signatures are byte-compatible with the existing format and do not acquire the new
modifier. Mixed signatures require a reader that understands recursive run boundaries. The
vocabulary definition lives in hand-authored `metadata/metadata.rdl`; it is not a custom attribute
instance on a field or parameter.

The reader preserves pointer nesting through arrays and references. It asserts against
conflicting const/mutable modifiers, a mutable-run modifier applied to a non-pointer, and a
missing pointee. The marker carries no direction, optionality, buffer size, or native volatile
contract. Merge and namespace remapping preserve the runs and remap their named pointees.

### Native node qualifiers

`Type::Const` and `Type::Volatile` wrap the exact native type node being qualified.
`Windows.Win32.Metadata.NativeConstAttribute` and `NativeVolatileAttribute` are required
signature modifiers, not custom attribute instances on declarations. The vocabulary seed
defines their no-argument constructors.

| Type spelling | Signature after any field/method prefix |
| --- | --- |
| `Const<i32>` | `CMOD_REQD(NativeConstAttribute) I4` |
| `Const<*mut i32>` | `CMOD_REQD(NativeConstAttribute) PTR I4` |
| `*mut Volatile<i32>` | `PTR CMOD_REQD(NativeVolatileAttribute) I4` |
| `Volatile<*mut i32>` | `CMOD_REQD(NativeVolatileAttribute) PTR I4` |
| `Const<Volatile<i32>>` | `CMOD_REQD(NativeConstAttribute) CMOD_REQD(NativeVolatileAttribute) I4` |

The distinct const marker avoids changing `IsConst`'s existing pointer-run/reference meaning.
The volatile marker carries a native type qualifier, not a CLR field-access or synchronization
instruction. Readers reject optional forms of either marker and modifiers without a type code.
Existing signatures without these markers retain their bytes and decoded types.

Qualifiers remain at their own nodes during readback, merge, and namespace remapping, including
named targets inside arrays and references. `Type::unqualified` removes only qualifiers on the
current node; it does not dereference a pointer or follow an alias. Opaque validation follows a
qualified type with the same storage requirement as the surrounding use.

The format preserves both const/volatile nesting orders without claiming they differ in native
meaning. It does not perform source qualifier normalization, capture annotations, or establish
native ABI layout. Rust projection rejects the new variants until its qualifier policy is defined.

### Native annotation evidence

`NativeAnnotationAttribute(source: String, ordinal: i32, payload: String)` retains raw native
evidence on its owning TypeDef, field, method, or parameter/return row. Its namespace is
`Windows.Win32.Metadata`. A source family and nonnegative ordinal identify one annotation on
one owner. Family strings are open; ordering comes from ordinals, not attribute table order.

`NativeAnnotationBindingAttribute(source: String, ordinal: i32, identifier: String,
parameter: i32)` associates an identifier in that annotation with an original zero-based
parameter position. Positions exclude the receiver and return. Bindings have the same owner
as their evidence and require a method or method parameter/return context. They do not depend
on the current `Param.Name`. Physical parameter table order is not positional order.

`MethodParam::parent()` supplies that method context, including return rows. Its
`params_by_sequence` map checks the ECMA-335 associations before positional bindings are used.
Raw SAL/MIDL evidence and lowered direction/buffer attributes remain separate.

| API | Contract |
| --- | --- |
| `Index::validate_native_annotations()` | Checks owners, positional argument shapes, evidence identities, and parameter bindings |
| `Index::validate_native_contracts(reference)` | Checks opaque storage, annotations, bit-fields, data imports, native slots, and values against supplied references |
| `writer::File::try_into_stream()` | Returns checked bytes or `NativeContractError`, retaining writer references during validation |

The checker rejects duplicate annotation identities or identifier bindings, missing evidence,
invalid positions, empty family/payload/identifier strings, and WinRT owners. It checks the
metadata namespace rather than matching attribute names alone. Rendering, merge, and remap
validate before replacing output. Raw unchecked emission remains available for negative fixtures.

This is representation validation, not SAL interpretation, source ownership proof, or a complete
expression-binding check. Consumers must not choose a direction or pointer shape from raw text
without native capture/reconciliation evidence.

### Logical bit-fields

One backing record field holds storage. Logical members are descriptors on that field in
`Windows.Win32.Metadata`; none adds an ECMA-335 Field row.

| Attribute | Positional arguments | Meaning |
| --- | --- | --- |
| `NativeBitfieldAttribute` | name, offset: i64, length: i64 | Named member interval, with the existing wire shape |
| `NativeBitfieldTypeAttribute` | name, logicalType: Type | Declared integral primitive, enum, or typedef identity |
| `NativeBitfieldQualifierAttribute` | name, ordinal: i32, qualifier: i32 | Outermost-first qualifiers; 1 = const, 2 = volatile |
| `NativeBitfieldAnnotationAttribute` | name, source, ordinal: i32, payload | Raw evidence owned by that logical member |

Type-valued arguments use CLR scalar identities (`System.SByte`, `System.Int32`, `System.UIntPtr`,
and so on) or the original nominal enum/typedef name. They do not copy the backing type.
`Field::bitfields()` reconstructs ordered `Type::Const`/`Type::Volatile` nodes and member evidence.
Legacy interval-only members return `logical_type: None`. Namespace remapping routes a logical
type-valued reference independently of the record.

`Index::validate_native_bitfields(reference)` checks instance-record ownership, companion
identities, nonempty names, nonoverlapping nonzero ranges, contiguous qualifier ordinals, and known
integral targets. Padding cannot acquire a descriptor. Known enum/typedef backing targets supply
capacity within the owner's architecture; alias cycles and known nonintegral targets reject.
A declared width may exceed the logical scalar's value bits, but must fit known backing storage.

Unresolved external nominal targets and native-sized capacity remain unknown without supplied
definition/target evidence. This validation neither closes the external graph nor calculates
native layout. Checked emission, RDL rendering, merge, and remap compose it through
`validate_native_contracts`; failures use `NativeContractError::Bitfield` before replacing output.
Rust projection rejects the new companions rather than selecting a logical accessor policy.

### Native data imports

`Windows.Win32.Metadata.NativeDataImportAttribute(dll: String, name: String, ordinal: i32,
kind: i32)` marks a declaration-only `Public | Static` field in a non-generic, architecture-neutral
native `Apis` container. The field has no `Literal`, Constant row, GUID value attribute, or
storage/default flags. Architecture tags belong to the field.

| Target | `name` argument | `ordinal` argument |
| --- | --- | --- |
| Named export | Nonempty exact export text | -1 |
| Ordinal import | Empty string | 0 through 65535 |

The DLL must be nonempty. DLL/export strings cannot contain NUL. Kind 1 means COFF data and kind 2
means COFF constant; function/unknown kinds reject. Kind does not imply an initializer, type
constness, or read-only storage. No Field `ImplMap` or ordinal-as-export-string convention is used.

`Field::is_imported_data()` identifies the marker by namespace and name.
`Field::data_import()` returns a checked `NativeDataImport` with a named or ordinal target and a
`NativeDataImportKind`. Unmarked/foreign-marker fields return `None` without being reclassified.
`Index::validate_native_data_imports(reference)` checks descriptor shapes, ownership, flags,
absence of values, native type nodes, and known typedef/WinRT targets. Native interfaces and
callbacks retain their existing reference-type representation. Unresolved external nominal
definitions remain unknown; this is not source closure or native layout validation.

Shared native checks report `NativeContractError::DataImport` before checked emission, RDL
rendering/cleanup, merge, or remap. Namespace remapping changes declaration/type namespaces, not
DLL/export names. API merge keys include data import contracts and non-architecture member
attributes, so differing imports or raw evidence do not collapse across architectures. Legacy
constant keys retain their existing behavior.

Rust projection, feature generation, and legacy clang references reject `Item::Data` rather than
inventing a constant or an unsupported binding.

### Native constant values

`Windows.Win32.Metadata.NativeConstantAttribute(version: i32, payload: String)` carries a
semantic initializer on a `Public | Static` field in a neutral nongeneric native `Apis` container.
It has no `Literal`, ECMA Constant row, GUID value descriptor, or data-import descriptor. Legacy
scalars, strings, GUIDs, and property keys keep their existing wire formats.

`Field::has_native_value()` identifies the exact marker. `Field::native_value()` decodes a checked
`NativeValue`: typed scalar, fixed-array contents, ordered named record members, or literal
kind/code units. `NativeValue::to_payload` and `from_payload` share the version 1 codec.
The payload is lowercase hexadecimal of a little-endian semantic tree, not native object bytes.
A standard custom-attribute string carries the bytes without extending ECMA scalar Constant rows.

| Tag (decimal) | Contents after tag |
| --- | --- |
| 1 | Boolean byte, exactly 0 or 1 |
| 2 / 3 | u8 / i8 |
| 4 / 5 | u16 / i16 |
| 6 / 7 | u32 / i32 |
| 8 / 9 | u64 / i64 |
| 10 / 11 | usize / isize value in u64 / i64; not a target-layout claim |
| 12 / 13 | f32 / f64 IEEE bits in u32 / u64 |
| 20 | u32 count, then child trees |
| 21 | u32 member count; each has u32 UTF-8 name length, name bytes, and a child tree |
| 22 | Kind byte, u32 unit count, and units of that kind's width |

Kind 0 is ordinary narrow text and kind 1 is UTF-8; both retain u8 units. Kinds 2 and 3 retain
UTF-16 u16 and UTF-32 u32 units. Narrow does not infer an execution charset. Units are not decoded
as Unicode or normalized, and no terminator is added/removed. Floating values retain their exact
bits, including NaN payloads. Member names are nonempty, NUL-free, unique UTF-8 strings.
Counts must fit the remaining input, nesting is bounded to 64, and trailing/truncated data,
unknown tags/kinds/versions, invalid booleans, and noncanonical hex reject.

`Index::validate_native_values(reference)` follows qualified typedef/enum targets and checks
all architecture-compatible definitions. Records require every instance field in declaration
order; arrays require exact lengths and element types. Literal kind checks the character width
without rewriting a pointer to `String`. Value definitions must be available in the local or
supplied reference index. Known x86 native-sized values must fit 32 bits.

Opaque/union/bit-field aggregate values, generic/WinRT/nonvalue definitions, cyclic types,
missing definitions, and conflicting value/storage descriptors reject. These are representation
checks, not compiler evaluation, source closure, native layout, padding, lifetime, or ABI proof.
The tree has no relocation or active-union-member form.

Shared checks report `NativeContractError::Value` before checked emission, rendering/cleanup,
merge, or remap. Payloads contain no nominal type references: field/record signatures retain
the types and remap independently of value contents. API merge keys retain payloads and member
evidence, including architecture presence. GUID/property-key merge keys also retain their
existing value descriptors rather than collapsing different UUIDs by field name.

Rust projection, feature generation, and legacy clang references reject native value descriptors
rather than treating a missing scalar Constant row as a missing initializer.

### Native interface slots

Three attributes in `Windows.Win32.Metadata` carry an opt-in single-native-vtable model:

| Attribute | Positional arguments | Owner |
| --- | --- | --- |
| `NativeVtableAttribute` | size: i32, inherited: i32 | Nongeneric native interface |
| `NativeSlotAttribute` | index: i32 | Local MethodDef |
| `NativeSlotReuseAttribute` | owner: Type | Local MethodDef reusing an inherited declaration |

`TypeDef::native_vtable()` and `MethodDef::native_slot()` decode checked descriptors. Sizes and
indices are nonnegative absolute native counts/positions, independent of MethodDef order.
The inherited prefix cannot exceed the table size. Local indices are unique and in range.
Every local method requires a slot; new slots cover the complete suffix, while reuse occupies
the inherited prefix. No fake inherited method rows are added.

New methods carry `Public | HideBySig | Abstract | Virtual | NewSlot`; reuse omits `NewSlot`.
`SpecialName` is permitted independently. Implementation flags are default and signature flags
are exactly `HASTHIS`; generic/variadic methods are unsupported. Architecture attributes belong
to interface variants rather than methods.

`Index::validate_native_slots(reference)` checks single native inheritance, cycles, known base
sizes, ancestor ownership, descriptor placement, arguments, and flags. A known modeled reuse
owner must locally declare that index, not merely inherit it. Architecture-compatible ancestry
paths are intersected for neutral owners. Known WinRT/generic/noninterface bases reject.
An unavailable or unmodeled base does not provide native slot evidence from method row counts;
the explicit prefix and direct base identity remain representable without proving external closure.

Shared validation reports `NativeContractError::Slot` before checked emission, RDL rendering,
merge, or remap. Type-valued reuse owners remap independently. Native-model architecture keys
include bases, slots, flags, and method/parameter/return evidence so distinct contracts retain
separate variants. Unmarked interface merge behavior stays unchanged.

These descriptors do not prove native order, override legality, signature covariance, receiver
qualifiers, or thunks. Capture/reconciliation must establish those contracts from native evidence.
Rust projection rejects modeled interfaces and derived consumers instead of using method row
order as an ABI oracle.

### Native typedef targets

A native typedef is a value TypeDef marked `Windows.Win32.Metadata.NativeTypedefAttribute`.
Its single field carries the target type. RDL authors that field as `Value`; it is an alias
payload rather than a copied native record member.

Several aliases can target the same record while retaining distinct `TypeName` identities.
An alias chain retains each edge, and signatures retain the selected alias name. Neither
matching record fields nor a one-field shape implies a typedef without the metadata marker.
Namespace remapping routes an alias and its target independently, including targets behind
pointer/reference qualifiers and arrays.

Architecture merge can keep an alias neutral when its named target varies by architecture.
The target TypeDef selected by `Index::new_for_architecture` supplies that architecture's record
definition; the neutral alias does not copy one architecture's fields or packing.

The RDL writer requires exactly one target field and validates that requirement before it
clears split or partitioned output. It does not select the first field of a malformed typedef.
This does not reconcile native tag/typedef observations or calculate native ABI layout.

### Opaque records

`Windows.Win32.Metadata.NativeOpaqueAttribute` marks a native struct or union with resolved
nominal identity and no exposed storage definition. Its constructor has no arguments. It is
different from an empty complete record and from an unresolved source forward declaration.
`TypeDef::is_opaque()` checks the metadata namespace and attribute name.

An opaque TypeDef has no fields, methods, generic parameters, nested definitions, ClassLayout,
or AlignmentAttribute. It cannot be WinRT or also be a native typedef. Struct/union identity
retains the existing sequential/explicit layout flag; neither flag supplies storage for an
opaque record.

| API | Contract |
| --- | --- |
| `Index::validate_opaque_types(reference)` | Checks declarations and value/array uses through aliases, with optional external type references |
| `Index::validate_opaque_storage(ty)` | Rejects storage use of an opaque value or array element, including typedef chains |
| `writer::File::try_into_stream()` | Checks native representation contracts against the writer's references before returning bytes |
| `writer::File::into_stream()` | Low-level unchecked table emission, also used to author negative fixtures |

Pointers and references retain opaque nominal identity without claiming pointee storage. An
array requires element storage even when reached through a pointer or reference. A
`NativeTypedefAttribute` TypeDef's one target field records an alias, not a native value member;
the checker follows it with the caller's storage requirement.

An imported-data field declares an external object without claiming local storage. It may retain
an opaque object's nominal type; arrays still require element storage. A consumer calculating
the object's layout must use `validate_opaque_storage` and resolve its native definition.

The checker rejects overlapping opaque/complete definitions. Member architecture attributes
and inherited nested-type architecture select the applicable variants. Architecture merge
includes opacity in its structural key, so an opaque record and a complete empty record do not
collapse into one neutral TypeDef. RDL compilation, rendering, merge, and remapping fail before
writing output when an opaque contract is invalid.

These checks do not calculate native size/alignment or establish external-reference closure.
Missing external definitions are not inferred to be opaque or complete. Consumers that calculate
layout must check opacity and still resolve and validate their native layout evidence.

### Nested type identity

Decoded signature `TypeName` values retain the outermost namespace and a slash-separated name,
such as `Test` and `Outer/Child`. `TypeDef::full_name`, `TypeRef::full_name`, and
`TypeDefOrRef::full_name` expose that identity; their `name` and `namespace` accessors still return
the raw table columns. TypeDef names follow `NestedClass`, while TypeRef names follow enclosing
TypeRef scopes.

The writer turns the qualified name into an enclosing-TypeRef chain. Merge preserves that chain,
and namespace remapping routes its outermost type without changing the nested path. A
`NestedClass` row alone does not resolve an unqualified, module-scoped reference to a nested type
(ECMA-335 II.7.3 and II.22.38).

Nested generic instantiations are outside the writer's supported input. Both signature and
TypeSpec emission assert against them rather than derive a nested name's arity from the total
generic argument count. Top-level generics and non-generic nested records remain supported.

`test_metadata::nested_references` checks physical scope chains, signature decoding through both
TypeRef and TypeDef tokens, repeated leaf names under different parents, rejection of nested
generic instantiations, merge, and namespace remapping.

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
