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

`Input.arguments` supplies include paths and compiler flags. `Input.roots` selects global type
names and their transitive dependencies within each TU; empty selects all declarations. Missing
roots and unsupported reachable declarations fail. `merge` reconciles each target, then groups
native identities with separate target layouts and type graphs. An unobserved target is not proof
of absence.

Indices are created before parallel capture and reused across TUs. Batches are serialized; other
libclang callers must not create indices during a batch. Native hangs affect the whole process.

Supported: named structs, unions, and integer-backed enums; typedefs, non-template `using` aliases,
forwards, scalars, qualified pointers, arrays, pragma packing, explicit record alignment, and
plain C/stdcall callback prototypes. Written aliases and attributed calling conventions are retained.
Native record compatibility uses canonical types. Alias groups retain all name-corresponding
bindings; their public compatibility is checked during projection, not assumed by native merge.
Unresolved attributes block `resolve` and `merge`. No packing request is guessed from layout.
Nested records, bitfields, and exported function declarations are not implemented.
Callback capture checks native function properties before accepting a prototype. Explicit
`regparm`, `noreturn`, variadic or exception-qualified functions, other calling conventions, and
unknown attributes remain unsupported rather than being normalized to ordinary C.
The native qualifier query rejects `__unaligned` and other non-CVR qualifiers before interning,
including qualifiers inherited through typedefs.

The optional `projection` feature provides `projection::generate(inputs, namespace)` for
three-target capture, merge, and per-header RDL output. It emits structs, unions, enums, and value
typedefs and named callbacks, checks metadata ABI layout, and rejects unsupported declarations or
ambiguous names. Callback signatures retain calling convention, result and parameter types;
parameter names are positional. Nested function pointers require a named callback declaration.
Written value-typedef and callback names are preserved in fields and callback signatures, including
their dependencies. Alias declarations retain canonical storage types. Unnamed function pointers
use compatible callback names resolved across the target, independent of the representative TU.
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
Enums retain nominal identity, scopedness, exact signed/unsigned values, and backing types.
Forward declarations must agree with complete definitions. Projection preserves `#[scoped]` and
the integer representation; it does not infer flags or combine distinct enums by matching values.
By-value aliases can connect distinct native records only after their definitions are checked.
Each target-local family uses its alphabetically first by-value alias as the public record name;
other reachable value names remain transparent aliases. Pointer aliases cannot establish equivalence.
Header ownership prefers the unique alias header containing its native definition, otherwise it
requires a unique declaration header. Native identities and source observations remain distinct.
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
