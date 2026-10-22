# Clang replacement acceptance

See [windows-clang](crates/windows-clang.md) for the implementation and reproduction commands, and
[acceptance evidence](clang-evidence.md) for current accounting and qualification limits.

## Goal and focus

Replace windows-clang with a compiler-backed pipeline that is better in architecture, accuracy,
correctness, testability, and maintainability, and produces usable Rust bindings for x86, x64, and
ARM64. Preserve the required public surface and semantics except for reviewed corrections or
exclusions. Parity is a compatibility constraint, not success; do not reproduce legacy bugs.

**The active phase is shipping integration.** Replace the legacy parser, generator and tests with
the compiler-backed implementations under the established crate names. Migrate dependent generators,
integrate remote master, and construct review commits before repairing the build in those buckets.
Native evidence remains separate from the canonical publication vocabulary. Zero baseline
differences is not the goal.
The RDL must be practical and concise without losing meaningful captured contracts. Redundant
spelling can be removed when decoding reconstructs the same metadata; reducing the published
meaning requires a separate, justified decision. Lack of a current consumer is not enough.

The [attribute inventory](clang-attributes.md) owns counts, meanings, consumers and proposed
dispositions. It is an assessment, not approval to delete attributes or weaken contracts.

This is the steering contract, not a progress log. Keep the decision register here and supporting
findings and measurements in [acceptance evidence](clang-evidence.md), rather than duplicating it.
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

1. **Preserve the committed work.** Keep a recovery branch before replacing implementations or
   rebuilding history.
2. **Replace the legacy pipeline.** Use `windows-clang`, `tool-win32` and `test_clang` for the new
   implementations. Migrate Clang-sys, WebView2, WDK and combined WinMD generation rather than
   retaining parallel parsers or silently dropping generator responsibilities.
3. **Integrate remote master.** Resolve conflicts before the broad repair/regeneration pass.
   Preserve the frozen semantic comparison baseline separately from the updated review base.
4. **Build the review stack.** On a fresh master-based branch, group source changes by crate and
   metadata changes by directory. Keep generated package/default metadata and shared wiring
   identifiable. These commits need not build independently. Preserve the integration history;
   do not force-push without approval.
5. **Establish green, repeatable generation.** Repair fixtures, metadata assertions and consumer
   defects at their owning layers. Regenerate canonical metadata and packages, then repeat the
   generators to check cleanliness. Passing tests do not approve missing APIs or changed contracts.
6. **Finish acceptance.** Review unresolved omissions, API/signature changes, native ARM64,
   clean-checkout generation and delivery budgets. Annotation-family expansion remains deferred.

## Remaining work and deferred gates

The independent review of `d0a38039eb` confirms the canonical contracts and identifies test,
generation and downstream-direction gaps. Its findings are tracked in
[current review disposition](clang-evidence.md#current-review-disposition).

| Work | Status and completion check |
| --- | --- |
| Canonical annotation quality | D38 arithmetic harmonization and termination owner/storage validation are complete. Short termination contracts are parameter-owned; unsupported owners/storage retain explicit full metadata. Single/double-NUL phases share one attribute family, including #5042's native case. Further annotation families are deferred. Residual SAL and C++ evidence publication remain maintainer decisions, not a reason to keep expanding the grammar. |
| Branch test health | Not green. Fixture drift, return-row assertions and the known PCH GUID reproducer remain from the wrap-up sweep. Native import consumers pass after the direction correction. WebView2 generation still exposes canonical value-type/alias ownership gaps, and WDK/combined WinMD generation still needs migration. |
| Existing metadata differences | Open after the attribute model. Account for required API families and `activationregistration`/`windowsfoundation`; no automatic exclusions or zero-difference target. |
| UIA scope and unresolved imports | D23 approves candidate scope only. Keep `CustomNavigationPattern_Navigate` unresolved ownership explicit; generated Rust/live-provider qualification waits. |
| SDK compilation contexts and PCH GUIDs | Open. Re-evaluate Olectl preloads, INITGUID phases, topology context and lean definition inputs together. Preserve the compiler reproducer; no source retries or guessed values. |
| Candidate-aware CI | Confirmed gap, deferred. `package_acceptance` reads worktree RDL; clean CI has legacy output and does not generate the candidate first. Require a generated, identified candidate before claiming this gate. |
| Unspecified-direction projection | Corrected in bindgen: unknown direction preserves native pointer mutability and does not trigger input/output, slice, query or retval conversions. Raw, alias and COM regression fixtures and native import consumers cover the change. |
| Generation cleanliness | The independent review reports legacy generator and package drift. Separate textual direction changes from WinMD/API differences before landing. Do not overwrite the visible candidate with legacy output as part of metadata polishing. |
| Consumer and runtime decisions | Deferred, not waived. Member-alignment traits/packed input, shared unaligned vptr loads, metadata version pairing and unresolved consumer contracts need acceptance. |
| Driver organization | Deferred. Split `run` and `sdk_library_policy` by existing responsibilities, proving configuration/output neutrality. |
| Landing boundaries | The maintainer approved rebuilding review commits after integration, before build repairs. Separate source crates, metadata directories, generated packages and shared wiring; retain the recovery branch. |
| Delivery qualification | Deferred. Actual candidate hosted/clean-checkout runs, native ARM64 execution, production generation ordering, controlled budgets and final independent review remain required. |

Review findings are obligations to investigate, not new public scope approvals. Keep issue-specific
fixtures and distinguish their setup from semantics; do not close issues from superficially similar
coverage. Preserve frozen and inspected-master controls and their approved corrections. Do not
rebase a dirty tree or silently migrate the baseline.

## Decision register for final acceptance

Approved means permission to implement the stated boundary, not final replacement acceptance. Keep
this table current as decisions change; link results in `clang-evidence.md` by decision ID. Do not
silently widen an approval or turn a review suggestion into one. Before cutover, revisit every entry
and record whether it is retained, revised, or removed, with supporting evidence and maintainer
acceptance. Unresolved choices remain visible even when unrelated work passes.

| ID | Decision and current status | Re-evaluate before cutover |
| --- | --- | --- |
| D01 | Approved: pause general breadth expansion; use the existing workload to resolve acceptance gaps. | Agree required public scope and explicitly approve any workload expansion. Counts alone do not establish coverage. |
| D02 | Approved: use frozen named linkage with exact compiler/SDK symbol, target, and provider evidence; retain ordinal evidence without inventing ordinal consumers. | Verify every published import against native evidence; preserve diagnostics for unresolved ownership. |
| D03 | Approved: select compiler-derived implicit x86 conventions per function, not a global stdcall default. | Confirm callbacks, explicit conventions, qualifiers, layouts, and annotations remain unchanged; ambiguous candidates must fail. |
| D04 | Approved: partial publication may retain validated targets when another target lacks import evidence. | Keep strict-mode failure and per-target diagnostics; do not confuse missing imports with native target absence. |
| D05 | Approved: 91 availability-pair corrections and four provider corrections for the pinned SDK workload. | Revalidate the checked approval files and stale-entry checks against final controls. Both missing `RtlCaptureContext2` pairs remain required. |
| D06 | Implemented: explicit target/name selection preserves 130 compatibility pairs across 46 names. Only the four existing x86 Get/SetWindowLongPtr generator aliases extend the frozen metadata's pairs. Native declarations and reconciliation remain independent. | Re-evaluate the checked selection against final public scope. No new target is selected merely because its name exists on another target. D23/D26 approve bounded native additions as candidate scope, not final qualification; aliases cannot change provider/ABI or claim unselected public names. |
| D07 | Approved: transparent value typedefs may compare equal in import redeclarations after native agreement. | Emit an observed signature and retain nominal identity, qualifiers, callback contracts, and typedef-owned annotations. |
| D08 | Approved: inherited interface slots preserve compiler-proven override relationships and written contracts. | Confirm slot continuity and producer/consumer ABI without treating redeclarations as new slots or discarding contracts. |
| D09 | Approved: fixed 64-bit data-pointer storage retains native pointer identity and projects as typed address values. | Keep unsupported extension variants and unqualified callable uses rejected; verify layout and supported consumers. |
| D10 | Approved: bounded qualified namespace identity and authoritative WinRT ownership, including external definitions and configured delegate names. | Recheck collisions, UUID/base/slot/signature and use-site contracts. No duplicate interfaces, spelling-based ownership, external by-value use, or local inheritance without separate support. |
| D11 | Approved: exclude WinRT name-string support roots by source ownership, retaining dependencies and explicit selections. | Confirm required declarations survive; do not generalize this approval to unrelated SDK support declarations. |
| D12 | Approved: scoped single/double-NUL contracts preserve pre/post ownership and residual unsupported evidence. | Qualify an actual reader/generator consumer before closing #5042; round trips alone are insufficient. |
| D13 | Implemented: bounded member alignment preserves field types and rejects unqualified by-value uses and containers. Qualified traversal covers nested and array storage without suppressing traits on pointer-only owners. Affected records omit Debug/equality. | Decide acceptable trait behavior and review packed-input rejection. Correct layout alone does not qualify aggregate calling ABI or all of #4967. |
| D14 | Approved: native callable references retain pointer ABI, mutability, and caller-owned storage; WinRT behavior stays separate. Counted buffers are not query IDs/outputs. | Recheck native imports, callbacks, COM producers/consumers, and WinRT controls on all targets. |
| D15 | Implemented: native x86 variants, legacy generator aliases, and public architecture-empty modules coexist. Full isolated rich/raw packages compile on all three targets; five generators remain output-neutral. | Qualify matched production metadata/routing and hosted CI. Local package compilation does not replace native ARM64 execution or paired-release acceptance. |
| D16 | Retained controls: frozen comparison/approval pin plus inspected-master audit. The requested Git metadata reset does not migrate semantic approvals. | Decide any re-pin explicitly, preserve historical reports, and carry forward only still-valid corrections. Master metadata currently equals the inspected control. |
| D17 | Implemented: approved automatic macro exclusion for `specstrings.h`, `rpcsal.h` and `driverspecs.h` removes only six internal constants; all 674 retained partitions are byte-identical. Required `stdint` aliases remain. | Keep support headers in compilation and SAL evidence. Explicit header/macro requests remain eligible. Preserve dependency ownership; no parser exceptions or symbol-name blacklist. Re-evaluate the scope at cutover. |
| D18 | Superseded by D30: scalar/enum-only omission removed 15,296 NativeInput attributes without other RDL or coverage changes. | D30 extends publication normalization to all parameter kinds while retaining native SAL evidence, reconciliation, direction and other contracts. |
| D19 | Four required storage/smart-card GUIDs recovered through native definition contexts with unchanged values and no other RDL changes. #4977, #5111 and #5107 are separated under D22-D24; D26 accepts the earlier native additions as candidate scope. Remaining GUIDs still need disposition. | Re-evaluate the selection-owned lean GUID includes. Make separate evidence-backed policy dispositions; no guessed GUIDs or implicit scope expansion. |
| D20 | Implemented: bounded SDK namespace/include contexts are declared in `header_contexts.rs`; translation-unit phase application remains driver-owned. | Seven complete before/after configurations are byte-identical. Re-evaluate the bounded contexts and qualify WinRT-before-Win32 production generation ordering; no parser exceptions or scope expansion. |
| D21 | Repository-owned full package reproduction passes locally on the generated candidate on all three targets. The CI invocation reads legacy worktree RDL, so candidate CI qualification is not established. | Require generated candidate input/provenance, paired tooling compatibility, actual clean-checkout/hosted runs, native ARM64 execution, agreed budgets, final review and maintainer sign-off. |
| D22 | Approved: preserve compiler-owned constant types, including signed constants associated with unsigned APIs. Handle #4977's Rust convenience typing separately. | Do not retag constants by name or usage, or claim #4977 is fixed. Re-evaluate any future public typing policy independently of native capture and metadata agreement. |
| D23 | Implemented: approved `UIAutomationCoreApi.h` expansion adds 98 native imports on all targets and 397 constants; all 674 preceding partitions are byte-identical. Focused provider/API-ID fixtures preserve signatures, imports, values and ownership. | One-header exception only. Keep missing `CustomNavigationPattern_Navigate` import ownership and inline helpers explicit. Qualify generated Rust and live providers before issue closure; re-evaluate the expanded public scope at cutover. |
| D24 | Approved: keep #5107 `ProcessPrng` separate from clang replacement capture. Any authored declaration/import supplement is separate work. | Do not synthesize compiler evidence from documentation or DLL exports. This disposition does not implement the API or close the issue; re-evaluate any future supplement's ABI and publication contract independently. |
| D25 | Implemented: publication phases, record storage/nesting/bitfield emission, and native record/value reconciliation follow their existing responsibilities. All 675 SDK partitions, configuration and coverage/provenance reports are byte-identical; normalized omissions are unchanged. | Preserve validation and publication order, shared collision checks, native observations, partial-mode rejection and type-worklist obligations. No new state framework or publication policy; re-evaluate these boundaries in the final review. |
| D26 | Approved: all 51 observed native names / 127 additional target pairs preceding the UI Automation expansion are accepted as candidate public scope. The exact set is the non-UIA native-declaration additions in `structure-after` function coverage. | Preserve the observed names, targets and named providers; no blanket approval of future additions or macro aliases. Scope approval does not waive ABI/consumer qualification or grant cutover acceptance. |
| D27 | Approved: separate native constant and macro identities; preserve native constants in partial publication while reporting failed or colliding macros separately. | Native and macro conflicts within their own observations remain fatal. Do not silently substitute active macro values for native constants, including WIC's version-1 CLSIDs shadowed by version-2 macros. |
| D28 | Approved: partial publication may emit an independently captured constant macro beside a declaration-only native object, retaining the missing-native-definition diagnostic. | The macro does not define the native object. Actual native definitions take precedence; conflicts and failed native definitions cannot enable macro substitution. Strict publication still requires native definitions. |
| D29 | Approved: prioritize canonical metadata/RDL quality and inventory all annotations before further downstream work. | Agree meanings, existing-attribute reuse, native-only evidence and short RDL syntax before semantic changes. Downstream gates are deferred, not waived. Baseline parity and smaller text alone do not establish correctness. |
| D30 | Implemented: approved normalization of SAL1/SAL2/RPC inputs through existing RDL direction and optionality, removing NativeInput publication on all parameter kinds. Inferred input needs no explicit attribute. | Preserve native SAL distinctions and reconciliation. Input is not COM object immutability. Counts, termination, returned pointers, qualifiers and residual contracts remain separate. New attributes require a useful contract beyond existing RDL, not just different source evidence. |
| D31 | Implemented: supported C++ exception specifications remain native-only evidence, with no NativeExceptionSpecification publication. Anonymous non-throwing pointers can use existing C/stdcall FNPTR without an attribute-owning callback. | Preserve native exception comparisons and compatible callback selection. Keep noreturn public and its anonymous-pointer restriction intact. Do not infer a language-neutral unwind ABI, weaken unsupported native forms or treat unannotated APIs as throwing. |
| D32 | Implemented: direct, unscoped, argument-free SAL2 `_Check_return_` and `_Must_inspect_result_` on callable results normalize to parameterless `#[must_use]`. Named callback aliases inherit the contract; duplicate forms emit one marker. | Keep native SAL distinctions and reconciliation. Do not promote output-parameter, field, global, value-typedef or conditional inspection to a callable result contract. Success predicates remain separate. This is not a success, purity or Rust-diagnostic guarantee; downstream generation remains deferred. |
| D33 | Implemented: expression-form success predicates use typed metadata for zero, nonzero, exact-one, nonnegative and positive results. Native type/owner checks normalize 556 SDK annotations; value aliases preserve links to return-default owners. | Callable conditions override typedef defaults. Preserve complex, scoped, conflicting and incompatible evidence, including shared-declarator pointer defaults. Do not infer partial-result error conversion, traverse pointers for defaults or guess precedence between competing defaults. Native reconciliation remains strict; downstream work is deferred. |
| D34 | Implemented: unknown direction remains unknown; complementary pointer-null conditions publish only common direction/optionality facts. The maintainer selected retained conditional evidence over a new conditional-expression vocabulary. D35 defines the RDL spelling. | Preserve missing In/Out flags through RDL/WinMD. Keep both conditional annotations; never replace conditional input with an unconditional requirement or assume an output-only conversion. Other predicates remain evidence. Native agreement stays strict; downstream work is deferred. |
| D35 | Implemented: the maintainer approved improving RDL rather than adding an unspecified-direction escape. Known In/Out flags are explicit; absence means unspecified. | No type-based inference, direction compaction or legacy language mode. Migrate existing authored inputs without changing metadata; regenerate clang from native evidence. Preserve independent optionality and conditional contracts. |
| D36 | Implemented first buffer slice: the maintainer selected named `readable`/`writable` contracts. Parameters name `entry`/`success`; fields omit the phase. `on_entry(count)` selects the entry count for a success contract. `BufferExtent` separates access, phase, units and count source without changing metadata for these spelling refinements. | Preserve entry versus successful-exit count values, null fallbacks, RPC post-writable guarantees and nullable field storage. Native lowering supports direct counts and one dereference. D37 covers returned buffers; return-owned buffers, constant/return expressions and containing-record extents remain separate work. |
| D37 | Returned-buffer slice with the approved extension for optional counts. `if_present(*count)` represents an unavailable extent, not zero. Capacity-only, partial and full SAL2 forms lower to indirect success extents with separate outer optionality and inner nullability. | `_all_` promises readable contents only, not writable capacity or an entry count. Do not add `out_ptr`'s one-element promise or infer ownership. Shared optional-count handling includes ordinary buffers and common conditional optionality, while preserving explicit null fallbacks. D38 covers bounded count arithmetic. |
| D38 | Approved bounded harmonization: new count arithmetic uses the shared typed-expression model; `count + 1` and `count + 1i32` have the same meaning and canonical spelling. Legacy nonzero Offset metadata retains full attribute spelling. | Preserve phase, optional-count, null-fallback, units and returned-storage semantics. No new attribute families or wider SAL campaign. Review producer correctness and clarity; the maintainer owns publication-value decisions and downstream qualification. |

Maintain the projection/reconciliation ownership boundaries and remove abandoned paths before
landing. Keep this register, the evidence page and crate documentation consistent. Acceptance must
use checked-in repros and commands, not session artifacts; do not turn maintenance into another
rewrite or a reporting framework.

## Change discipline

[Success predicates](clang-attributes.md#success-predicate-investigation) now have a bounded typed
form under D33. The remaining 58 direct annotations include compound conditions, partial-result
guarantees and pointer-typedef ownership traps. Further lowering needs its own semantic review;
do not introduce automatic Rust Result conversion or weaken native agreement.

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
source correction in `crates/tools/win32/src/baseline`. The current 372-header manifest is a
workload control, not proof of complete required scope. Record compiler/SDK pins, companion
identity, targets, TU sources, arguments, macro selection, and import policy for reproducible runs.
Keep partial candidates visible in the working tree under `metadata/win32`, not committed or
promoted to production generation before approval. Never hand-edit or hide their changes.

**Scope and budget proposal, to agree in step 1:** require the frozen baseline's Win32 surface and
contracts on all three targets, except approved corrections and exclusions. Target a full run no
slower than a matched legacy run, provisionally <=480 seconds wall time and <=8 GB peak process-tree
working set. Record machine, CPU/core count, inputs, configuration, and warm dependency state. Ask
the maintainer to quiet other workloads before controlled measurements; historical timings are not
acceptance measurements.

**PCH decision in step 1:** inventory affected required declarations in the chosen configuration; do
not treat the historical count of about 460 ksmedia GUIDs as a current loss count. For remaining
required gaps, agree on a bounded fix or explicit exclusion; otherwise acceptance stays blocked.
Preserve the compiler reproducer even if validated definition inputs supply the required values.
Only a small companion adjustment is within the existing fix scope; libclang rebuilds, parser
duplication, and per-failure full-source retries remain out of scope.

**Metadata compatibility gate:** the maintainer approved requiring updated, paired readers and
generators for the new metadata; older consumers are not a compatibility target. Qualify inherited
override slots, bitfield `MemberType`, and per-pointer-level `const` separately, including silent
misinterpretation by historical consumers. Use a matching validated source revision during
development and record the paired release versions before production cutover. Do not strip contracts
or reproduce older generator mistakes to preserve compatibility.

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