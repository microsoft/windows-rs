# windows-clang replacement plan

## Goal and focus

Replace windows-clang with a compiler-backed pipeline that is better in architecture, accuracy,
correctness, testability, and maintainability, and produces usable Rust bindings for x86, x64, and
ARM64. Preserve the required public surface and semantics except for reviewed corrections or
exclusions. Parity is a compatibility constraint, not success; do not reproduce legacy bugs.

**Pause breadth expansion. Focus on architecture and acceptance.** Use the existing workload to
expose and verify design problems. Add coverage only to resolve an agreed contract or acceptance
gap. Resuming breadth expansion requires maintainer approval.

This is the steering contract, not a progress log. Keep findings, measurements, and decisions in
`clang-findings.md` beside this file, linking supporting artifacts rather than duplicating them.
Changes to strategy, scope, deferrals, or acceptance gates require approval.

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

1. **Establish controls and remove the immediate reproducibility hazard.** Make companion
   freshness the first code change: detect changed native build inputs, not just missing DLLs or
   ABI mismatches. Validate explicit external runtimes without silently replacing them. Record the
   baseline, reviewed control, and experimental candidate separately. Agree on required scope,
   budgets, and the PCH GUID decision below. Classify gaps by root cause and affected contracts,
   separating dependent losses. Report obligations, not a completion percentage.
2. **Review the architecture through difficult cases.** Start with import ABI and calling
   conventions, native identity versus public naming, incomplete types and packing, bitfields,
   annotations, and macro/definition inputs. Run bounded consumer probes alongside this review:
   compare baseline and candidate API presence, signatures, layout, linkage, and behavior for
   imports, callbacks, COM, and bitfields. Compilation alone is insufficient.
   Deliver one ranked finding per listed case, naming the owner, evidence, preserved facts,
   rejection rules, proposed resolution, and completion check. Review shared-runtime changes
   independently from parser acceptance. Stop when every case has an evidenced disposition and
   each unresolved item has a bounded fix or maintainer decision; approve that list before step 3.
3. **Resolve provisional mechanisms.** Review SDK include configurations, import corrections, the
   stdcall experiment, and core alignment changes. Each must become a justified, tested part of the
   design, be replaced by a simpler correct mechanism, or be removed with an explicit unsupported
   case. Investigate per-function x86 import decoration as ABI evidence, checking provider, target,
   symbol identity, and compiler-verified signature compatibility. Do not rewrite captured ABI
   from names or change callback defaults; ambiguous or conflicting evidence remains an error.
   Prefer established named exports; ordinal archive records do not require ordinal consumers.
   Require checked SDK publication policy or pinned export evidence for those names, retaining the
   original records. Recovering names alone is never justification. Exclusions require approval.
4. **Make correctness reproducible and end-to-end.** Establish a clean-checkout reproduction path,
   reusable semantic comparisons, and selected SDK-backed CI coverage. Promote the review's
   consumer probes into repeatable acceptance checks. Complete the metadata compatibility gate
   below; successful round trips alone cannot establish correctness.
5. **Consolidate and qualify the replacement.** Remove abandoned experiments and redundant paths.
   Keep repository instructions short and crate documentation about the current design. Run the
   complete agreed workload and consumer checks, account for every difference, and obtain
   maintainer acceptance before switching production callers or removing the old implementation.

## Change discipline

- Make small, independently reviewable changes. State the contract and completion check first.
  Use focused header/RDL fixtures with native and consumer expectations independent of the
  implementation. Preserve useful assertions for rejection, target variation, and input order.
  Checkpoint each validated mechanism change with its diff and remaining risks ready for review.
  Continue unless blocked on evidence or a maintainer decision; the maintainer handles commits.
- Format, build/check, run warnings-denied clippy, and exercise affected regressions for each code
  change. Native changes also require native boundary checks. Review the actual diff in a separate
  pass for correctness, ownership, complexity, and error reporting.
- Use focused checks during implementation and full semantic comparisons at mechanism boundaries.
  Start with concrete RDL differences and validate them against native/header evidence; the baseline
  is a reference, not an authority. Repair and check header/RDL fixtures before escalating to Rust
  consumers or package generation. Do not make baseline infrastructure the iteration's focus.
  Record configuration, phase timings, peak memory, review findings, and unresolved risks. Do not
  accept unexplained regressions or repeatedly run the full workload for minor fixture changes.

## Controls and finish line

Comparison baseline: `6e68e56081340eaa932f4dbc92ec54c56ed51473`, plus the approved `WM_CTLCOLOR`
source correction in `crates/tools/win32-2/src/baseline`. The existing 370-header manifest
is a workload control, not proof of complete required scope. Record compiler/SDK pins, companion
identity, targets, TU sources, arguments, macro selection, and import policy for reproducible runs.
Keep partial candidates visible in the working tree under `metadata/win32`, not committed or
promoted to production generation before approval. Never hand-edit or hide their changes.

**Scope and budget proposal, to agree in step 1:** require the frozen baseline's Win32 surface and
contracts on all three targets, except approved corrections and exclusions. Target a full run no
slower than a matched legacy run, provisionally <=480 seconds wall time and <=8 GB peak process-tree
working set. Record machine, CPU/core count, inputs, configuration, and warm dependency state.
Ask the maintainer to quiet other workloads before controlled measurements; historical timings
are not acceptance measurements.

**PCH decision in step 1:** inventory affected required declarations in the chosen configuration;
do not treat the historical count of about 460 ksmedia GUIDs as a current loss count. For remaining
required gaps, agree on a bounded fix or explicit exclusion; otherwise acceptance stays blocked.
Preserve the compiler reproducer even if validated definition inputs supply the required values.
Only a small companion adjustment is within the existing fix scope; libclang rebuilds, parser
duplication, and per-failure full-source retries remain out of scope.

**Metadata compatibility gate:** review bitfield `MemberType` and per-pointer-level `const`
encoding separately. Specify their semantics and compatibility with current and supported older
readers/generators, including silent misinterpretation. Require focused compatibility probes and
maintainer approval of migration/version requirements before accepting either format change.

Acceptance requires all of the following:

- Reviewed architecture with clear ownership and no unresolved workaround-dependent required path.
- Required API and contract accounting, with justified corrections and approved exclusions.
  `_ARM64_MULT_INTRINS_SUPPORTED` remains an approved compiler-internal exclusion.
- Reproducible native, metadata, and consumer checks across x86, x64, and ARM64, including required
  API presence and behavior. Preserving annotation strings is not implementing their semantics.
- Approved metadata compatibility and migration decisions, not just same-version round trips.
- Deterministic full-scope generation, no unexplained losses, and agreed performance budgets met.
- Maintainable code, fixtures, tooling, and documentation that do not depend on this conversation.

An explicit omission is honest progress, not completion. Only maintainer acceptance ends this plan.
