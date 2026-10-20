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
   The approved first slice uses frozen-baseline named linkage with exact compiler/SDK symbol,
   provider, and target evidence. Do not copy baseline signatures or infer ABI from decorated names.
   Close the Winsock losses before expanding to another affected family.
   The next approved slice resolves implicit x86 import conventions using compiler-derived
   per-function ABI candidates and exact SDK symbol/provider evidence. Preserve original
   observations; explicit conventions or changes to nested callback, parameter, layout, or
   annotation contracts must not be overridden. Do not enable the global stdcall experiment.
   The maintainer also approved bounded attribute-based handling of inherited interface slots.
   Preserve compiler-proven override relationships and their written contracts rather than
   discarding redeclarations or adding vtable entries. Do not expand this into general C++ support
   or infer that old declarations are buggy merely because their annotations differ.
   The maintainer approved compiler-evidenced function aliases while retaining native public names.
   Preserve the referenced declaration's ABI and DLL/export ownership; PSAPI aliases must not switch
   to downlevel providers or depend on a `K32` spelling rule.
   The maintainer approved target-local unresolved imports in partial publication: retain
   independently validated target variants and report missing import evidence per target.
   Strict publication still fails; native and unrelated contract failures retain their existing
   rejection rules. Missing import evidence does not prove that a target is unavailable.
   The maintainer approved 88 x86 baseline availability corrections for the pinned SDK workload:
   the mask-6 entries in `crates/tests/libs/clang2/input/sdk_import_availability.txt`. These are
   acceptance corrections, not parser rules or claims about every Windows release. Preserve
   captured declarations and missing-import diagnostics. Both missing `RtlCaptureContext2` target
   pairs remain required pending stronger platform evidence.
   `crates/tools/win32-2/src/approved-availability.txt` records that exact approved set for
   `--compare-functions`; report raw and unapproved differences separately and reject stale entries.
   The maintainer also approved the three x86 NDR availability corrections in
   `sdk_ndr_import_availability.txt`, excluding its all-target `NdrClientCall2` control. The
   approved availability set is 91 pairs. Both `RtlCaptureContext2` pairs remain required.
   Four provider corrections are approved for the pinned workload: x86 `RtlCompareMemory`,
   `RtlRaiseException` and `RtlRestoreContext`, plus ARM64 `RtlRaiseException`, use the SDK's
   `ntdll.dll` imports instead of baseline `kernel32.dll`. `approved-providers.txt` records the
   exact transitions; export names and compiler ABI remain unchanged. This does not approve other
   provider differences or replace native ARM64 runtime qualification.
   The maintainer approved equivalent transparent value typedefs in import-redeclaration
   comparisons after native agreement. Preserve every observation and emit an observed signature;
   do not erase nominal identity, qualifiers, callback contracts or typedef-owned annotations.
   The maintainer approved fixed 64-bit data-pointer storage, including `PVOID64`. Retain native
   pointer identity, pointee and qualifiers in metadata, and use typed address values in Rust.
   Keep 32-bit extension variants and unqualified callable/function-pointer uses rejected.
   Ordinary pointers to supported storage records and adjusted array parameters remain supported.
   The maintainer approved bounded namespace ownership support: preserve qualified native identity,
   resolve external WinRT ownership through explicit authoritative metadata references, and retain
   flat Win32 names only when unambiguous. Reject collisions rather than merge unrelated native
   declarations or invent public names. Start with memorybuffer/robuffer interop helpers and
   collision/reference fixtures, without expanding the header workload or inferring WinRT identity
   from namespace spelling or inheritance alone.
   The maintainer also approved bounded SDK delegate ownership support after the namespace run
   exposed `IDispatcherQueueHandler`. Resolve the configured SDK delegate spelling against
   authoritative metadata, then check UUID, base, slots and signatures. Do not emit a duplicate
   local COM interface, add per-name exceptions, or infer ownership from an `I` prefix alone.
4. **Make correctness reproducible and end-to-end.** Establish a clean-checkout reproduction path,
   reusable semantic comparisons, and selected SDK-backed CI coverage. Promote the review's
   consumer probes into repeatable acceptance checks. Complete the metadata compatibility gate
   below; successful round trips alone cannot establish correctness.
5. **Consolidate and qualify the replacement.** Remove abandoned experiments and redundant paths.
   Keep repository instructions short and crate documentation about the current design. Run the
   complete agreed workload and consumer checks, account for every difference, and obtain
   maintainer acceptance before switching production callers or removing the old implementation.

## Landing priorities from the ca1d04954b review

Treat the updated `clang-ground` review as an acceptance checklist, not proof that every reported
failure is a parser regression. Its measurements describe an earlier source/output checkpoint.
The 91 availability corrections and four provider corrections are approved; fixed-pointer storage
and combined Win32/WDK metadata compilation have local evidence. Consumer qualification is not a
paired release, hosted CI result, or native ARM64 execution.

Finish the active namespace/reference boundary, then work through these gates in order. Do not
start another general breadth sweep.

| Priority | Work | Completion check |
| --- | --- | --- |
| 1 | Finish namespace and external delegate ownership; account for composition interop losses | Header/RDL/WinMD/consumer evidence for the memorybuffer/robuffer helpers and delegate references; inspect `ICompositorInterop` and `ICompositionDrawingSurfaceInterop` in the existing workload; no unclassified gains or losses |
| 2 | Add a current-master acceptance control alongside the frozen baseline | Pin the inspected master commit and verify the reported #5083/#5084 PDH correction; compare required surface against both controls without silently resetting approvals or historical differences |
| 3 | Correct publication ownership | Reproduce #4943; keep resource, CRT and driver-input declarations out of public partitions without removing required dependency contracts or weakening native reconciliation |
| 4 | Port issue-linked legacy regressions individually | Three-target `.h`/`.rdl` fixtures and explicit dispositions for #5047, #4932, #5062, #4935, nested enums, and the collision/string/direction cases; retain the legacy fixtures |
| 5 | Make closed-issue SDK coverage executable | A checked-in acceptance command covers PDH constants, sapiddk, IWTS APIs, `IMAGE_DOS_HEADER`, `NtQuerySystemInformation`, and the required interop interfaces |
| 6 | Resolve publication and scope decisions | Review WinRT `InterfaceName_*`/`RuntimeClass_*` constants, new function pairs, SAL/NativeInput semantics and remaining GUID contexts; do not accept additions or strip contracts merely to improve counts |
| 7 | Qualify and prepare delivery | Paired reader/generator/core versions, clean-checkout and hosted CI evidence, native ARM64 execution, agreed budgets, and concise maintained documentation |

The maintainer approved metadata-owned external COM definitions with explicit native identity and
use-site checks. Capture their identity, attributes, layout and bases without traversing member
definitions. Record that ownership in native evidence, reject inconsistent ownership across inputs,
and require matching authoritative metadata at publication. This replaces native member-body
validation for explicitly external definitions; locally owned definitions retain full reconciliation.
Use-site pointer shape, qualification and calling conventions remain native contracts. Do not emit
local stand-ins or infer external ownership from spelling alone. By-value use and local inheritance
from an externally owned definition require separate support and remain rejected.

The maintainer approved excluding WinRT name-string support declarations from automatic Win32
publication while retaining required native dependencies. Use source ownership, not a blacklist
of `InterfaceName_*` or `RuntimeClass_*` symbols. Explicit selections remain eligible. This does
not approve unrelated new function or constant additions.

The maintainer approved scoped single- and double-NUL termination through capture, reconciliation,
and metadata. Preserve raw pre/post qualifiers and bind their property ownership before comparing
annotation sets. Keep entry and successful-return contracts distinct; conditional, grouped, and
indirect forms remain scoped evidence unless their semantics are supported. Do not infer encoding,
capacity, or direction, and do not treat arbitrary user annotation strings as configured SAL.

The maintainer approved bounded member-alignment support for #4967. Preserve public field types,
carry native member alignment in metadata, and implement layout in the Rust generator. Reject
by-value uses until the calling ABI is established, including containing records and arrays.
Correct offsets do not establish aggregate calling conventions.

The maintainer approved native reference ABI and query-selection fixes end-to-end. Preserve
reference metadata, lower native callable references with their pointer ABI and mutability, and
leave WinRT byref behavior unchanged. Cover imports, callbacks, and COM methods; counted buffers
must not become query interface-ID parameters.

For the legacy-fixture port, distinguish setup differences from semantic failures. Configure
explicit imports, macros, C linkage and x86 COM conventions where the fixture intends those
contracts; do not rewrite the behavior being tested to make it pass. `data_classes.h` itself has
no constructors or destructors, and `template_private_records.h` checks that unused private
templates do not suppress unrelated public declarations. General C++ object-lifetime or template
support is a separate decision, not an automatic consequence of those fixture names.

Keep #4982 and #4995 as issue-linked repro obligations, and independently confirm #5081 before
proposing closure. #5111 is a header-workload scope decision; #5107 requires an explicit
declaration/import policy. #5042, #4977 and #4967 need their own semantic dispositions. Do not
close issues or claim supported contracts from a superficially similar fixture.

Do not rebase a dirty worktree or replace the frozen baseline as part of a comparison run. A later
maintainer-approved baseline transition must preserve the old report, carry forward only still
valid corrections, and explain master-side fixes separately from parser gains.

The inspected current-master audit control is `afcb0ab1f8cf7a71ec563f659451748b26e7f5b6`.
#5084 merged as `cc1bf12e71bc6c355de0a5090b6a8b529491e498`, adding `PdhMsg.h` to the legacy
workload and publishing `pdhmsg.rdl`. The local `origin/master` ref is older and must not substitute
for that inspected control. Neither the frozen comparison baseline nor the worktree has been
rebased or replaced.

Before landing, consolidate the large projection/reconciliation functions along existing ownership
boundaries, remove abandoned paths, and move the plan/findings into maintained repository
documentation. Replace session-only acceptance dependencies with checked-in repros and commands;
do not turn this cleanup into another rewrite or a reporting framework.

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

**Metadata compatibility gate:** the maintainer approved requiring updated, paired readers and
generators for the new metadata; older consumers are not a compatibility target. Qualify inherited
override slots, bitfield `MemberType`, and per-pointer-level `const` separately, including silent
misinterpretation by historical consumers. Use a matching validated source revision during
development and record the paired release versions before production cutover. Do not strip
contracts or reproduce older generator mistakes to preserve compatibility.

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
