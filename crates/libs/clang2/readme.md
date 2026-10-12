## windows-clang2

An unpublished replacement experiment for the `.h -> .rdl -> .winmd` path. Production generators
continue to use `windows-clang`. This crate does not depend on the legacy extractor, default
metadata, or Rust projection.

`Compiler` owns a reusable libclang library. `capture` parses one explicit Windows target/profile
and required declaration scope, returning owned evidence. `Snapshot::rdl` constructs checked
output without retaining compiler handles.

The C2 slice supports scalar typedefs, written alias chains, pointers, lvalue references,
fixed arrays, const/volatile qualifiers, and incomplete named record identities. Complete records,
callables, enums, values, annotations, and imports remain the later C3-C9 packages.

See [the replacement plan](../../../docs/crates/windows-clang.md#fixed-implementation-matrix)
and [crate internals](../../../docs/crates/windows-clang2.md).
