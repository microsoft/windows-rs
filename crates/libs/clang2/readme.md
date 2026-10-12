## windows-clang2

An unpublished replacement experiment for the `.h -> .rdl -> .winmd` path. Production generators
continue to use `windows-clang`. This crate does not depend on the legacy extractor, default
metadata, or Rust projection.

`Compiler` owns a reusable libclang library. `capture` parses one explicit Windows target/profile
and required declaration scope, returning owned evidence. `Snapshot::rdl` constructs checked
output without retaining compiler handles.

The C2 slice supports scalar typedefs, written alias chains, pointers, lvalue references,
fixed arrays, const/volatile qualifiers, and incomplete named record identities. C3-A adds named
plain-data structs/classes and unions with checked natural storage, preserving fields, aliases,
and recursive pointer identity. Adjusted/anonymous/bit-field storage remains outside that slice.
The pilot adds complete integer enums and external free-function prototypes. Compiler-evaluated
members, enum aliases, written signature types, parameter positions, effective calling conventions,
and native linker symbols survive capture. `Snapshot::rdl_with_imports` requires explicit named
export contracts keyed by those symbols; it does not guess DLLs.
Selected object-like integer macros use compiler evaluation in a recorded derived TU with the same
profile. Active definitions must match the selected source roots. Compiler-resolved type/function
aliases remain separate evidence; publication rejects until profile-specific naming is defined.
Callbacks, COM methods, and source annotations remain outside this producer.
The active delivery gate combines these families in a frozen SDK/COM pilot.

Index creation excludes active native sessions to honor LLVM's target-registration contract.
Deadline-controlled lifecycle regressions cover the SDK parse stall reproduced by direct native
controls. The selected C2/C3-A gates pass; broader capture and production performance remain open.
This path is not ready for production promotion.

C3-B adds record-level packing and MSVC `__declspec(align(...))` through
`Compiler::with_layout_bridge`.
The matched native bridge reads requests from the same TU, including requests with identical
storage. Required alignment propagates through named records, aliases, and arrays; reviewed RDL,
WinMD readback, and independent MSVC assertions cover x86/x64/ARM64. Field adjustments and
bit-fields remain unsupported.

Build the pinned parser/bridge pair with `cargo run -p tool-clang --quiet -- native-layout`
on an x64 Windows host with MSVC/CMake. Set `LIBCLANG_PATH` to the printed directory for bridge
captures. The loader checks parser identity, SHA256, and bridge ABI before reading ASTs.
`Compiler::new` keeps the ordinary C2/C3-A path; natural-storage agreement alone does not prove
the absence of explicit packing requests.

C3-C adds public named nested records/typedefs and direct unnamed struct/union storage. Native
owner and unnamed-field descriptors preserve source meaning independently of publication names.
The bridge supplies implicit-field identity; canonical cursor equality keeps records distinct even
when their USRs match. Compiling this RDL requires a reference built from `metadata/metadata.rdl`.
Arrays/pointers/shared unnamed identities and private/behavioral owners remain outside this slice.

Run `cargo run -p tool-clang --quiet -- pilot` to stage the matched-compiler, three-target pilot
baseline. It reports unsupported required APIs and pending acceptance checks and returns nonzero
while the pilot is incomplete. Further isolated storage work is paused; production is unchanged.
The shellscalingapi profiles emit and compile with pinned SDK import evidence on x86/x64/ARM64.
Toolhelp's ANSI profiles also emit and compile, including all 13 value macros and 16 functions.
Compilation does not establish source-contract preservation, agreement, or partition acceptance.

See [the replacement plan](../../../docs/crates/windows-clang.md#fixed-implementation-matrix)
and [crate internals](../../../docs/crates/windows-clang2.md).
