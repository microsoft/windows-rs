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

## Acceptance checklist

- [ ] **Freeze the acceptance contract.** Record the baseline commit, SDK/compiler versions, header
  set, configurations, and time/memory budgets. Define required APIs and approved scope exclusions.
- [ ] **Account for the baseline.** Start with a small, repeatable constant comparison; extend it
  as other categories are tackled, not as a separate framework project. Track moved ownership and
  distinguish undiscovered, rejected, incompatible, unrepresentable, consumer-incomplete, and
  approved differences. Name presence alone does not establish semantic parity.
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

The next work is a control batch, not another unrestricted SDK sweep:

1. Freeze the baseline and make the constant-name comparison reproducible, including certbcli.
2. Add the pruning/conflict reproducer as a regression. Review and repair the
   capture/reconciliation/pruning boundary before relying on partial-mode results.
3. Recover certbcli's 22 constants through general discovery and evaluation, with exact semantic
   checks. Then take ReadFile's buffer relationship through metadata to its consumer.

For each subsequent batch, choose one measured gap family and state its failing case, intended
change, and completion check before editing. A batch is complete only when:

- The regression fails before the fix and passes afterward.
- The required native facts, metadata semantics, and consumer behavior agree.
- The full-workload comparison shows the expected closures and no unexplained new losses.
- The implementation uses a general rule, with any added policy or complexity explained.

Report cases closed, remaining blockers, and architectural changes, not just output totals or
passing test counts. Review the affected boundaries after each batch. Stop if the solution needs
header-specific exceptions, weakens an invariant, or cannot meet its stated completion check;
report the blocker instead of switching to easier additions.

Partial output is a diagnostic artifact, not a successful replacement. Completion requires every
required baseline case to pass its semantic checks, with all differences explicitly reviewed.
Only then switch callers, remove obsolete implementation paths, and discard this plan.
