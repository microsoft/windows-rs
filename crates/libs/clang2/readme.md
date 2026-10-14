# windows-clang2

Unpublished native header capture using libclang. The native IR is independent of RDL and WinMD.

```rust,no_run
use windows_clang2::{Input, Target, capture, merge};

let input = Input {
    name: "example.h".into(),
    source: "struct Record { int count; void* data; };".into(),
    roots: vec!["Record".into()],
    ..Default::default()
};
let snapshots = capture(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
let merged = merge(&snapshots).unwrap();
println!("{} records with target variants", merged.records.len());
```

Supply the matched `windows-clang-sys` DLL pair through `LIBCLANG_PATH` or its loader search paths.
`capture` returns one owned snapshot per target/TU, in target-major order. `resolve` checks native
record definitions from the same target and compiler configuration, retaining original observations
and completion decisions.

`Input.arguments` supplies include paths and compiler flags. `Input.roots` selects global declaration
names and their transitive dependencies within each TU; empty selects all declarations. Missing
roots and unsupported reachable declarations fail. `merge` reconciles each target, then groups
native identities with separate target layouts and type graphs. An unobserved target is not proof
of absence.

Indices are created before parallel capture and reused across TUs. Batches are serialized; other
libclang callers must not create indices during a batch. Native hangs affect the whole process.

Supported: named structs, unions, and integer-backed enums; typedefs, non-template `using` aliases,
forwards, scalars, qualified pointers, arrays, pragma packing, explicit record alignment, and
plain C/stdcall callback prototypes. Written aliases and attributed calling conventions are retained.
Fixed array extents are compiler-evaluated; array callback parameters use native pointer adjustment.
Default field initializers remain unsupported, including initializers hidden from cursor traversal.
Native record compatibility uses canonical types. Alias groups retain all name-corresponding
bindings; their public compatibility is checked during projection, not assumed by native merge.
Unresolved attributes block `resolve` and `merge`. No packing request is guessed from layout.
Named and anonymous nested records retain owner-relative identity, including shared types and
out-of-line definitions. RDL preserves their scope through inline fields or nested declarations
and references such as `Outer::Inner`. Nested enums and aliases declared inside records remain
unsupported.
Callback capture checks native function properties before accepting a prototype. Explicit
`regparm`, `noreturn`, variadic or exception-qualified functions, other calling conventions, and
unknown attributes remain unsupported rather than being normalized to ordinary C.
The native qualifier query rejects `__unaligned` and other non-CVR qualifiers before interning,
including qualifiers inherited through typedefs.
Default type visibility is retained and compared as native evidence. It needs no metadata
attribute for plain records and enums. Interface publication handles its own vtable contract;
C++ RTTI is not projected.
Hidden/protected type visibility and ordinary symbol visibility remain unsupported.

The optional `projection` feature provides `projection::generate(inputs, namespace)` for
three-target capture, merge, and partitioned RDL output. It emits structs, unions, enums, constants,
value typedefs, and named callbacks, checks metadata ABI layout, and rejects unsupported declarations
or ambiguous names. Callback signatures retain calling convention, result and parameter types;
parameter names are positional on named callbacks. Unnamed signatures have structural function types.
Written value-typedef and callback names are preserved in fields and callback signatures, including
their dependencies. Alias declarations retain canonical storage types. Unnamed function pointers
use compatible callback names resolved across the target, independent of the representative TU.
Without a compatible name they use inline `extern "C" fn(...)` or `extern fn(...)` types, encoded
as ECMA-335 `FNPTR` signatures. No public callback names are synthesized.

External C-linkage functions retain native signatures, parameter names, and mangling.
`project_with_imports` and `project_partial_with_imports` take explicit DLL/export ownership;
it is not guessed from headers. UUID-bearing, single-vptr interfaces support a public non-virtual
base and ordered public pure virtual methods with the platform COM convention. C++ lvalue
reference parameters preserve constness through metadata byrefs. Non-vtable member templates
remain provenance, not exported methods or a claim of full C++ compatibility.

Const integer declarations and explicitly selected `Input::macros` use compiler evaluation.
Macros are evaluated after the input and retain native definition ownership. A constant-expression
check rejects runtime side effects that libclang's general folding API may ignore. This slice
does not implement string macros, general initializers, or automatic macro discovery.
The header/RDL fixtures and SDK breadth test cover all three categories
through x86, x64, and ARM64 WinMD round trips.

The SDK driver preserves SAL source-hook evidence through the provisional `NativeSalAttribute`,
including capacity, initialized-length, and conditional contracts. Common parameter direction and
argument-optionality flags are also projected. This is not a complete SAL interpreter; expressions
and nested contracts remain explicit evidence. Typedef publication accepts an observed annotation
set that contains all other observations, but never invents a union of incomparable sets.

All same-target record and callback observations must agree on their public names after native
compatibility is checked; conflicting names are errors, including in partial mode.
Requested packing is emitted as `#[packed(N)]`, including requests with no visible layout effect.
Complete-record alignment is emitted as `#[align(N)]`. Native checks retain every requested
alignment and its GNU, standard, or declspec form, even when layout is unchanged. Projection uses
the maximum request; zero adds no alignment attribute. Field, typedef, and enum alignment,
combined packing/alignment, and packed records containing aligned types remain unsupported.
Unions retain their record kind, publish explicit-layout fields at offset zero, and must match
captured size and alignment. Structs and unions never share a public family, even with identical
fields and layouts.
Integer bitfields retain evaluated widths and native offsets. Projection coalesces
compatible members into checked storage fields, preserving unnamed padding and zero-width
barriers in native evidence. Union members use separate offset-zero storage fields. Mixed
signedness between named members in one storage unit and enum/bool bitfields remain unsupported.
Storage layouts must match native size, alignment, and offsets; no packing request is inferred.
Downstream union-accessor defects are tracked separately rather than blocking correct metadata.
Enums retain nominal identity, scopedness, exact signed/unsigned values, and backing types.
Forward declarations must agree with complete definitions. Projection preserves `#[scoped]` and
the integer representation; it does not infer flags or combine distinct enums by matching values.
Anonymous enums without a public type name publish their members as typed constants. Same-target
constants must agree in native backing type and value; architectures can retain different variants.
Type and constant names occupy separate publication namespaces. Typedef-named enums remain enums;
references to otherwise unnamed enum types are rejected rather than replaced with integers.
Mixed const/mutable pointer chains retain per-level qualifiers through RDL and WinMD.
`char16_t` and `char32_t` project to `u16` and `u32`, while native reconciliation keeps their
identities distinct from other equal-sized integer types.
By-value aliases can connect distinct native records only after their definitions are checked.
A single native record or enum uses its unique defining value typedef as its public name; with
zero or multiple defining typedefs it retains its native tag. A multi-record family requires a
unique common defining typedef, or a unique common alias when none defines a tag. Ambiguity is an
error. Other reachable value names remain aliases. Pointer aliases cannot establish equivalence.
Header ownership prefers the unique alias header containing its native definition, otherwise it
retains all declaration headers as candidates. A single owner keeps its header partition; multiple
owners use `foundation.rdl`. Ownership evidence is combined across architectures before routing,
and `Partition::headers` retains the real source-header set. Native identities and source
observations remain distinct; foundation routing never resolves a type conflict.
`tool-win32-2` supplies the SDK configuration and writes canonical files.

`capture_partial` and `projection::generate_partial` return supported output plus omission reports.
`projection::project_partial` accepts an existing partial capture without reparsing.
Failed entities and dependent declarations are removed across all targets; compiler errors and
conflicts in retained definitions or alias bindings still fail. A non-value typedef, such as `void`,
can remain as native evidence and expand at use sites. A named value alias or callback
that cannot be published is removed with its dependents rather than replaced by its canonical type.
Macro constants are not captured.

Fixtures, inspection, and scaling live in `test_clang2`. See
[crate notes](../../../docs/crates/windows-clang2.md) for commands.
