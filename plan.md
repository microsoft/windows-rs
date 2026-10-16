# windows-clang replacement plan

## Fixed execution contract

Replace windows-clang with a smaller, maintainable compiler-backed pipeline that reproduces the
required canonical Win32 metadata and usable Rust bindings for x86, x64, and ARM64. Preserve the
baseline public surface and semantics, except for explicitly reviewed corrections or exclusions.
Do not reproduce legacy bugs merely to obtain identical text.

This plan defines the work, not a running progress log. Changes to its strategy, required scope,
deferrals, or acceptance gates require explicit maintainer approval. Do not rewrite the plan to
justify the current activity. Keep changing counts, per-header results, and investigation details
in the crate documentation and session artifacts. Retain this contract until the maintainer accepts
the replacement.

## Architecture and invariants

`Headers -> owned native IR -> reconciliation -> publication -> RDL/WinMD -> Rust bindings`

- Use compiler-owned identities, types, values, layouts, and vtable slots. Keep typed IDs, indexed
  lookup, separate declaration observations, and target-specific evidence.
- Validate native compatibility before publication pruning. Partial mode must not hide known
  conflicts, treat failed observations as validated, or infer absence from an unobserved target.
- Separate discovery, capture, compatibility, naming, metadata representation, and consumer
  semantics. Unsupported cases must have explicit diagnostics.
- Use general mechanisms and existing metadata contracts. Do not add header/name-specific parser
  exceptions, guess values or layouts, or copy legacy output to conceal missing support.
- Extend the current design. No new rewrite, speculative framework, or reporting infrastructure
  project. Modules with child files use `module/mod.rs`.

## Execution cadence

**Breadth is the default. Small slices isolate new architectural failures; they are not the unit
of coverage rollout.**

1. **Scale across the agreed header set.** The driver already parses the shared 370-header manifest
   across three architectures. Expand macro discovery and definition coverage in substantial
   batches toward that full scope, rather than advancing one header at a time. Dinput is part of
   this breadth work, not a separate milestone.
2. **Compare and classify each breadth result.** Reuse the baseline comparison and omission
   evidence. Group failures by underlying cause, not header or individual missing name. Distinguish
   discovery/configuration gaps, compiler failures, native conflicts, publication/representation
   gaps, downstream issues, and approved differences.
3. **Reduce only new hard failures.** State the failing contract and completion check; create the
   smallest representative header/RDL fixture; fix the general mechanism; demonstrate the failure
   before the fix and correct output afterward. Cover relevant rejection and target/input-order
   cases. Return immediately to breadth once that mechanism is established.
4. **Amortize full-workload runs.** Use focused checks while implementing a mechanism and a full
   comparison at the batch boundary. Do not add a fixture and repeat the entire SDK run for every
   header that needs no implementation change. Smaller batches need a concrete isolation or
   resource reason, not habit.
5. **Measure scaling.** Record phase timings and peak memory with the input/configuration and
   coverage result. Investigate measured bottlenecks that prevent full-scope execution; do not
   optimize tiny fixtures or add costly fallbacks without workload evidence.

Known deferred failures remain explicit omissions; they do not stop unrelated breadth work.
New failures must be classified, not silently ignored. Stop and seek a decision if proceeding would
weaken an invariant, require an exception, change required scope, or exceed an agreed resource
budget. Do not substitute an easier header-by-header queue for the architectural problem.

## Current boundaries

Header-to-RDL iteration is the working loop. Use focused WinMD checks where needed to establish
metadata contracts. Defer Rust generation and consumer execution to final acceptance.

The PCH GUID materialization bug is deferred. Preserve the ignored `uuid_macros` reproducer,
explicit GUID-value coverage, and affected omissions. Do not reopen the investigation during
breadth work. A small `windows-clang-sys` companion adjustment is the only permitted fix scope;
no suitable pre-parse hook is established. Rebuilding libclang, duplicating its parser, and
per-failure full-source retries are out of scope.

The remaining hard contracts are not reasons to serialize header coverage:

| Area | Required resolution |
| --- | --- |
| Constants | Broad macro discovery, declaration/definition coverage, and GUID naming policy |
| Annotations | Required direction, optionality, termination, and buffer relationships |
| Representation | Per-member bitfield types and reduced storage alignment |
| Imports | Conflicting public prototypes, ordinal imports, and target-specific export ownership |
| Consumers | Union accessors, flat COM identity, and required generated Rust behavior |

Resolve shared causes with representative fixtures, then apply them broadly. Preserving annotation
strings is not implementing their meaning. Recording a gap is not satisfying an acceptance gate.

## Controls and acceptance

Comparison baseline: `6e68e56081340eaa932f4dbc92ec54c56ed51473`.
Initial controls: LLVM 22.1.8, companion ABI 15, SDK 10.0.28000.2270, shared 370-header manifest,
and x86/x64/ARM64. Record TU groups, support/definition inputs, macro selection, and import policy
for each run. Version or configuration changes must be explicit, not mixed into unexplained diffs.

Keep generated partial candidates visible in tracked `metadata/win32`; do not hand-edit them.
`--compare-constants` supplies name/owner accounting, not semantic parity. Preserve the preceding
candidate as a comparison control. Compare required types, values, encodings, contracts, layouts,
ownership, and target availability as applicable; explain every loss or change.

Replacement acceptance requires:

- An agreed required API/configuration scope and runtime/peak-memory budgets. The initial manifest
  is a control, not proof that the required scope or budgets are settled.
- Accounting for the baseline across required declaration categories, with semantic comparisons
  and explicit approval of corrections or exclusions. `_ARM64_MULT_INTRINS_SUPPORTED` is the
  approved compiler-internal exclusion; other failures are not automatically excluded.
- Required constant, annotation, representation, and import contracts implemented without weakening
  pre-pruning validation. Deferred required cases must be resolved or explicitly excluded.
- Repeated full-scope generation with deterministic output, no unexplained losses, preserved
  regressions, and acceptable scaling.
- Final downstream Rust generation, compilation, and behavior checks across the required targets.

Only after these gates and maintainer acceptance may production callers switch and obsolete
implementation paths be removed. Partial output and increased name counts are not replacement
readiness.
