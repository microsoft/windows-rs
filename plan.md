# windows-clang replacement plan

## Goal and focus

Replace windows-clang with a compiler-backed pipeline that is better in architecture, accuracy,
correctness, testability, and maintainability, and produces usable Rust bindings for x86, x64, and
ARM64. Preserve the required public surface and semantics except for reviewed corrections or
exclusions. Parity is a compatibility constraint, not success; do not reproduce legacy bugs.

**Pause breadth expansion. Focus on architecture and acceptance.** Use the existing workload to
expose and verify design problems. Add coverage only to resolve an agreed contract or acceptance
gap. Resuming breadth expansion requires maintainer approval.

This is the steering contract, not a progress log. Keep findings, measurements, and decisions
outside this file. Changes to strategy, scope, deferrals, or acceptance gates require approval.

## Architecture and invariants

`Headers -> owned native IR -> reconciliation -> publication -> RDL/WinMD -> Rust bindings`

| Layer | Responsibility |
| --- | --- |
| Capture | Preserve compiler-owned identities, types, values, layouts, attributes, and provenance |
| Reconciliation | Validate native compatibility across observations before publication pruning |
| SDK policy | Declare supported compilation contexts and justified publication choices |
| Projection | Encode validated contracts without inventing native facts |
| Consumers | Implement metadata contracts with correct ABI and behavior |

- Keep typed identities, separate observations, and target-specific evidence. Failed observations
  are not agreement; missing import evidence is not proof of target absence.
- No name-specific parser exceptions, guessed values or layouts, silent fallbacks, or workarounds
  that manufacture support. Unsupported contracts remain explicit failures.
- Give each decision one owner. Fix conflicting interpretations at their boundary rather than
  patching several layers. Prefer simpler shared mechanisms over duplicated logic or abstractions.
- Improve the existing design. No new rewrite, speculative framework, or reporting-system project.

## Work sequence

1. **Establish the acceptance picture.** Agree on required API/configuration scope and time/memory
   budgets. Record the baseline, accepted control, and experimental candidate separately. Classify
   remaining gaps by root cause and affected contracts, separating dependent losses. Report
   obligations satisfied, unresolved, or explicitly excluded, not a completion percentage.
2. **Review the architecture through difficult cases.** Start with import ABI and calling
   conventions, native identity versus public naming, incomplete types and packing, bitfields,
   annotations, and macro/definition inputs. Identify the owner, evidence, preserved facts, and
   rejection rules. Produce ranked findings and proposed resolutions before another feature
   batch. Review shared-runtime changes independently from parser acceptance.
3. **Resolve provisional mechanisms.** Review SDK include configurations, import corrections, the
   stdcall experiment, and core alignment changes. Each must become a justified, tested part of the
   design, be replaced by a simpler correct mechanism, or be removed with an explicit unsupported
   case. Recovering names alone is never justification. Resolve required gaps without weakening
   invariants; exclusions require approval.
4. **Make correctness reproducible and end-to-end.** Establish companion freshness checks, a
   clean-checkout reproduction path, reusable semantic comparisons, and selected SDK-backed CI
   coverage. Use bounded consumer probes now, comparing baseline and candidate API presence,
   signatures, layout, linkage, and behavior. Cover ordinal imports, callbacks, COM, and bitfields;
   successful compilation or round trips alone cannot establish correctness.
5. **Consolidate and qualify the replacement.** Remove abandoned experiments and redundant paths.
   Keep repository instructions short and crate documentation about the current design. Run the
   complete agreed workload and consumer checks, account for every difference, and obtain
   maintainer acceptance before switching production callers or removing the old implementation.

## Change discipline

- Make small, independently reviewable changes. State the contract and completion check first.
  Use focused header/RDL fixtures with native and consumer expectations independent of the
  implementation. Preserve useful assertions for rejection, target variation, and input order.
- Format, build/check, run warnings-denied clippy, and exercise affected regressions for each code
  change. Native changes also require native boundary checks. Review the actual diff in a separate
  pass for correctness, ownership, complexity, and error reporting.
- Use focused checks during implementation and full semantic comparisons at mechanism boundaries.
  Record configuration, phase timings, peak memory, review findings, and unresolved risks. Do not
  accept unexplained regressions or repeatedly run the full workload for minor fixture changes.

## Controls and finish line

Comparison baseline: `6e68e56081340eaa932f4dbc92ec54c56ed51473`. The existing 370-header manifest
is a workload control, not proof of complete required scope. Record compiler/SDK pins, companion
identity, targets, TU sources, arguments, macro selection, and import policy for reproducible runs.
Keep partial candidates visible in tracked `metadata/win32`; never hand-edit or hide their changes.

The PCH GUID bug remains deferred pending an explicit decision. Preserve its reproducer and
omissions. Only a small companion adjustment is within the existing fix scope; libclang rebuilds,
parser duplication, and per-failure full-source retries remain out of scope.

Acceptance requires all of the following:

- Reviewed architecture with clear ownership and no unresolved workaround-dependent required path.
- Required API and contract accounting, with justified corrections and approved exclusions.
  `_ARM64_MULT_INTRINS_SUPPORTED` remains an approved compiler-internal exclusion.
- Reproducible native, metadata, and consumer checks across x86, x64, and ARM64, including required
  API presence and behavior. Preserving annotation strings is not implementing their semantics.
- Deterministic full-scope generation, no unexplained losses, and agreed performance budgets met.
- Maintainable code, fixtures, tooling, and documentation that do not depend on this conversation.

An explicit omission is honest progress, not completion. Only maintainer acceptance ends this plan.
