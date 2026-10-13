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
aliases remain separate evidence. Active profile-scoped descriptors attach to their actual closed
native targets without renaming or erasing ANSI declarations.
The matched bridge also captures UUID-bearing COM interfaces with a single public nonvirtual base,
native inherited prefixes and slots, pure-method signatures, and separate inline-helper inventory.
Pilot SAL/MIDL parameter contracts preserve direction, optionality, COM output markers, raw
evidence, and positional byte extents. Unannotated parameters stay unspecified. Source-contract
completeness and callbacks remain outside accepted capture.
Compiler-owned macro history supports object-like annotation wrapper chains without guessing
redefinition order. Physical source contexts preserve return-prefix SAL and explicit named MIDL
interface sections. Unsupported transformations and ambiguous consumed ownership reject.
`Snapshot::agree` compares complete original native observations across same-target input scopes
and orders, after reconciling nominal ownership rather than comparing snapshot IDs or RDL.
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

Run `cargo run -p tool-clang --quiet -- pilot` to reproduce the matched-compiler, three-target pilot
acceptance gates. It returns nonzero for a failed worker, lost native semantics, unapproved
architecture variation, or an unexplained legacy difference. Production is unchanged.
The shellscalingapi profiles emit and compile with pinned SDK import evidence on x86/x64/ARM64.
Toolhelp's ANSI profiles also emit and compile, including all 13 value macros and 16 functions.
All 30 profiles emit and compile; all 24 same-target native agreement checks pass. Candidate WinMD
readback checks DLL/export/convention routes, captured parameter contracts, active aliases, and
function/method/interface source evidence. Exact return-prefix, wrapper-provenance, and
interface-context controls pass on all three targets, including the real selected COM sections.
Full pilot signature/value/layout readback passes independently of rendering. All 30 cross-target
profile comparisons pass with checked pointer-width and Windows ABI variation. Legacy facets are
classified against the compiler/source evidence, with exact preservation of byte-capacity bindings.
The frozen pilot accepts 2/2 SDK partitions and the selected COM closure. The 2026-10-13 decision is
to retain this architecture; no production promotion or broader replacement schedule is authorized.

See [the replacement plan](../../../docs/crates/windows-clang.md#fixed-implementation-matrix)
and [crate internals](../../../docs/crates/windows-clang2.md).
