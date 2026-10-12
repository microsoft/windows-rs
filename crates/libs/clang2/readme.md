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
Callables, enums, values, annotations, and imports remain the later C4-C9 packages.

Index creation excludes active native sessions to honor LLVM's target-registration contract.
Deadline-controlled lifecycle regressions cover the SDK parse stall reproduced by direct native
controls. The selected C2/C3-A gates pass; broader capture and production performance remain open.
This path is not ready for production promotion.

See [the replacement plan](../../../docs/crates/windows-clang.md#fixed-implementation-matrix)
and [crate internals](../../../docs/crates/windows-clang2.md).
