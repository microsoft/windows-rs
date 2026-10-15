# windows-clang replacement plan

## Goal

Replace windows-clang with a smaller, maintainable compiler-backed pipeline that reproduces the
required canonical Win32 metadata and usable Rust bindings for x86, x64, and ARM64. Preserve the
baseline public surface and semantics, except for explicitly reviewed corrections or scope changes.
Do not reproduce legacy bugs merely to obtain identical text.

This is a temporary steering document. Update the checklist as evidence is established; discard it
after the replacement is accepted.

## Architecture and rules

`Headers -> owned native IR -> reconciliation -> publication -> RDL/WinMD -> Rust bindings`

- Keep typed IDs, indexed lookup, separate declaration observations, and target-specific evidence.
  Use compiler-owned identities, types, values, layouts, and vtable slots, not spelling heuristics.
- Keep native validation independent of publication pruning. Partial mode must not hide known
  conflicts or turn unvalidated evidence into a claim of compatibility.
- Separate discovery, capture, compatibility, public naming, and consumer semantics. Unsupported
  cases must be identifiable; never silently discard contracts or choose a convenient declaration.
- Reuse existing metadata contracts. Add a representation only with a concrete producer, consumer,
  and end-to-end fixture. Preserving annotation strings is not implementing their meaning.
- Extend the current design, not another rewrite or speculative framework. Keep policy at explicit
  boundaries; do not patch individual headers or copy legacy output to conceal missing support.

## Ordered checklist

- [ ] **Freeze the acceptance contract.** Record the baseline commit, SDK/compiler versions, header
  set, configurations, and time/memory budgets. Define required APIs and approved scope exclusions.
- [ ] **Account for the baseline.** Track constants, types, functions, and interface members across
  moved partitions. Classify each gap as undiscovered, capture-rejected, incompatible,
  unrepresentable, consumer-incomplete, or an approved change. Compare semantics, not only names.
- [ ] **Repair validation boundaries.** Preserve the evidence needed for reconciliation despite
  exclusions. Cover conflicting definitions behind an unsupported dependency, input ordering,
  partial mode, and architecture-specific failures.
- [ ] **Complete constant discovery.** Discover macros and declaration-only GUID values through
  compiler-backed paths. Recover all 22 missing certbcli constants without a hand-selected list.
  Verify types, values, ownership, and architecture variants.
- [ ] **Complete annotation semantics.** Carry required buffer relationships, direction,
  optionality, and termination through existing metadata contracts into generated bindings.
  Start with ReadFile's missing size relationship; test behavior, not just transport round trips.
- [ ] **Close representation and consumer gaps.** Resolve required bitfield member types/storage
  alignment, union accessors, and COM binding identity in supported output modes.
- [ ] **Close import contracts.** Resolve conflicting public prototypes, required ordinal imports,
  and target-specific DLL/export ownership without guessed mappings.
- [ ] **Demonstrate replacement readiness.** Regenerate the full agreed surface repeatedly with no
  unexplained losses. Verify consumer compilation/behavior, deterministic output, existing
  regressions, and the agreed performance budgets before switching production generation.

## Iteration and completion

Take one bounded gap at a time: failing header fixture -> minimal change -> native/metadata/consumer
evidence -> full-workload gap comparison. Count closed baseline gaps, not net new declarations.
Review architectural boundaries after each batch; stop and resolve violations before more breadth.

Partial output is a diagnostic artifact, not a successful replacement. Completion requires every
required baseline case to pass its semantic checks, with all differences explicitly reviewed.
Only then switch callers, remove obsolete implementation paths, and discard this plan.
