# Clang replacement findings

This is the bounded review required by `plan.md`, not a declaration of replacement readiness.
The maintainer approved the resolution list and permits a documented reader/generator upgrade
requirement. Ranks order the remaining work; individual exclusions and cutover are not approved.

## Controls

| Control | Identity and status |
| --- | --- |
| Frozen comparison baseline | `6e68e56081340eaa932f4dbc92ec54c56ed51473` |
| Working source checkpoint | `5246ebc3fd7e8b9cbb1d5fb3fd16effc6d5d74fd`, plus freshness and named-publication changes |
| Working `metadata/win32` | Regenerated named-publication partial candidate; visible, uncommitted |
| Reviewed metadata control | `breadth-noreturn-repeat`; not an accepted replacement |
| Ordinal candidate | `breadth-ordinal-imports`; unresolved x86 import gaps |
| Stdcall trial | `breadth-sdk-stdcall`; not accepted as the canonical configuration |
| Scratch output | `target/win32-2/Windows.Win32.winmd` matches the named-publication run |
| Toolchain | LLVM 22.1.8, companion ABI 16, SDK 10.0.28000.2270; x86/x64/ARM64 |

The named-publication configuration uses the existing 370 headers and seven translation units,
without `--x86-stdcall`. Historical control artifacts and SHA-256 identities are recorded in
`rewrite-handoff.json` in Copilot session
`e8bd0450-292d-42c8-a2b2-45d86a4711e5`; the original artifacts are in session
`1810274b-2842-4eda-8537-79e6ac99f61c`. These are recovery evidence, not a clean-checkout
reproduction path. They must not silently become the accepted source/toolchain control.

The proposed scope is the frozen baseline's required Win32 surface and contracts on all three
targets, with explicit corrections and exclusions. The proposed resource limits remain a matched
legacy run, provisionally 480 seconds and 8 GB peak process-tree working set. Neither scope
exceptions nor performance acceptance have been established by this review.

## Implemented: managed companion freshness

**Owner:** native build/provisioning, not capture or projection.

`crates/libs/clang-sys/native/build.ps1` checks repository native inputs, dependency pins, target,
and deployed DLL hashes under an exclusive cache lock. Failed builds invalidate the stamp.
Changed fingerprints force clean compilation, including when source timestamps are preserved.
`tool-clang path` validates the selected runtime before returning a path. Explicit external
`LIBCLANG_PATH` builds remain caller-owned and are not replaced.

The SDK-independent regression covers reuse, changed/added/removed inputs, version/target
changes, missing/replaced DLLs, invalid stamps, forced/failed builds, inputs changing during
compilation, incomplete deployment, and concurrent checks. The real companion was rebuilt,
reused without rewriting its stamp, and exercised by the native semantic fixture.

**Limits:** fingerprints do not authenticate cached upstream dependency contents or external
build provenance. Loader ABI validation still follows full symbol-table loading; improving that
diagnostic is separate from detecting same-ABI stale implementations.

## 1. Imports: metadata correctness does not ensure consumer API presence

**Owners:** SDK policy for library selection and compilation context; capture for compiler ABI;
bindgen/link for emitted imports.

**Evidence:** `test_clang2`'s `breadth::sdk_import_targets` verifies real ILClone, ILFree, and
WSAStartup ordinal mappings and WinMD on all targets. `sdk_import_gaps` verifies the unresolved
DHCP/DNS x86 convention mismatch and missing BSTR_UserSize64 entry. Both pass.

**Resolution:** preserve established named linkage. No ordinal consumer changes remain in bindgen
or windows-link. The SDK driver owns a checked 20-entry policy: the 18 functions covered before
the ordinal experiment, plus ILClone and ILFree. Every entry is checked against all three pinned
SDK archives, retaining exact native symbol, machine, provider, kind, and ordinal evidence.
Unknown entries are reported rather than renamed from linker spelling.

These public functions already have named bindings. Review confirmed all 20 named exports in
System32 and SysWOW64 DLLs, version 10.0.26709.1000. This is x86/x64 export evidence, not proof for
every Windows release or an ARM64 runtime check. Relevant public contracts:
[Winsock](https://learn.microsoft.com/windows/win32/winsock/initializing-winsock),
[Direct3D 12](https://learn.microsoft.com/windows/win32/api/d3d12/nf-d3d12-d3d12createdevice),
[Direct2D](https://learn.microsoft.com/windows/win32/api/d2d1/nf-d2d1-d2d1makerotatematrix), and
[ILClone](https://learn.microsoft.com/windows/win32/api/shlobj_core/nf-shlobj_core-ilclone) /
[ILFree](https://learn.microsoft.com/windows/win32/api/shlobj_core/nf-shlobj_core-ilfree).
The policy is a bounded preservation choice, not clearance for the roughly 1,478 ordinal names
in the experimental output.

The driver compares real SDK capture and named publication against `sdk_named_imports.rdl`.
`named_import_consumers` uses that metadata to check required APIs and exact ABIs. Its isolated
PE probe excludes std: std's own Winsock import library can otherwise select ordinals even when
the generated raw imports are named. The ordinary host consumer still includes std.

The full named-publication run emits 12,308 functions, 93,085 constants, and 788 partitions;
the selected target maps contain 67,244 named imports and no ordinals. Against the reviewed
noreturn control, source and canonical comparisons find four added functions: ILClone, ILFree,
MapDebugInformation, and UnmapDebugInformation. The latter two retain the exact target maps'
named exports. The same 182 unresolved x86 import gaps remove 546 target declarations; there are
no additional losses. Three RtlCompareMemory/RtlRaiseException provider changes match the native
maps. All other retained declarations, ownership, availability, semantics, inputs, arguments, and
constant accounting are unchanged. Four omission identities disappear; the 182 known gaps remain.

The named consumer probes pass for ordinary host execution, isolated x86/x64 execution, and
x86/x64/ARM64 PE imports. The full workload is still partial, not accepted. Supporting artifacts
are `named-publication-*` and `compare-named-publication.py` in the current session's files directory.

The saved stdcall comparison recovers 94 functions but changes 86 existing x86 declarations,
including 84 callbacks. Those gains do not justify the collateral calling-convention changes.

**Preserve/reject:** retain exact target/provider/symbol evidence and native signatures. Missing
imports are not target-absence evidence. Keep conflicts and ambiguous associations explicit.

**Remaining resolution:** investigate per-function decorated import evidence without rewriting
captured ABI or changing callback defaults. Do not adopt `-mrtd`. Additional ordinal-backed APIs
need reviewed named-export evidence, not a larger consumer-linkage mechanism.

**Completion:** required functions exist with correct signatures and named import tables on
all three targets; host execution exercises real imports. Classify the 94 ABI discrepancies
and 88 missing-entry cases without public-name fallback or guessed target availability.

## 2. Bitfields and mixed pointers: compatibility needs an explicit release policy

**Owners:** metadata/RDL for encoding; bindgen for semantics; maintainer for supported readers.

**Evidence:** `projection/bitfields.rs` separates native storage grouping from member type.
`NativeBitfieldAttribute.MemberType` supplies signed, bool, enum, and alias semantics to bindgen.
`metadata::reader::Blob::read_type_signature` recognizes modifiers after each pointer while
retaining the legacy leading-modifier meaning. The bitfield and mixed-pointer fixtures pass
current-tool metadata checks and generated Rust checks. This is not older-reader compatibility.

**Preserve/reject:** preserve native bit offsets, member types, storage layout, and every pointer
qualification level. Reject unrepresentable layouts rather than inventing packing. Do not assume
an older reader understands a new attribute or modifier merely because it accepts the file.

**Resolution:** test each encoding against the frozen reader/generator and establish the minimum
supported consumer versions. The maintainer permits an explicit paired tool upgrade rather than
losing type semantics. Keep the two format approvals separate.

The reader probes now establish the following boundary:

| Reader revision | Mixed pointer fixture | Typed bitfield fixture |
| --- | --- | --- |
| `51eac0a953`, before mixed-pointer support | Seven field signatures fail decoding | MemberType values fail decoding |
| Frozen `6e68e56081` | All field signatures decode correctly | MemberType values fail decoding |
| Current | All field signatures decode correctly | Alias, enum-alias, and bool MemberType values decode correctly |

These are actual historical reader builds against current fixture WinMD, not inferred outcomes.
Release-style builds report unsupported type codes; debug builds can abort during unwinding because
the blob's drop assertion also fails. The frozen baseline already supports mixed pointers, so it
cannot stand in for a pre-change pointer reader.

Mixed-pointer decoding enters at `cf1806f01d`; typed MemberType reading and consumption enter at
`b742a59a59`. Both libraries still identify as 0.100.0, so that version string alone cannot express
the boundary. A paired reader/generator release requirement and explicit format approvals remain
necessary. Historical generator probes and supported release numbers are still outstanding.
The sources, fixture WinMD, commands, and results are in the session's `files/compatibility`.

**Completion:** current and supported older consumers have measured outcomes, including silent
misinterpretation; minimum versions and migration requirements receive maintainer approval.

## 3. Incomplete records and packed interfaces: parser and runtime contracts differ

**Owners:** projection for representability; windows-core for object-pointer access.

**Evidence:** `opaque_records` checks genuine incomplete-record references and rejection of
by-value or failed-definition cases. `windows-core::Interface::assume_vtable` uses an unaligned
object-vptr load. `test_core::packed` exercises a synthetic unaligned object, calls, ownership,
and weak-reference paths. Both fixtures pass. SDK declaration packing is not evidence that
Windows itself returns unaligned objects.

**Preserve/reject:** retain compiler layout and packing evidence. An opaque reference does not
authorize arbitrary value storage, failed-definition fallback, or an unaligned vtable.

**Resolution:** retain the metadata distinction; review the shared runtime contract independently.
Decide whether arbitrary valid COM object-pointer alignment is supported by windows-core, rather
than treating that change as an automatic consequence of parser coverage.

**Completion:** document the object/vtable alignment preconditions and approve or remove the core
contract expansion, with native and generated-consumer checks for the chosen contract.

## 4. Macro/definition inputs: supported SDK contexts need a declared boundary

**Owners:** SDK driver for source configuration; capture for compiler evaluation.

**Evidence:** `tool-win32-2::guid_inputs` selects definition TUs, normalizes storage-header
includes, changes Olectl definition mode, and uses `_KS_` for the topology context. These choices
are outside generic capture, but remain SDK-specific policy. `mmreg` distinguishes explicit
GUID capture from PCH discovery; `uuid_macros::explicit_and_discovered_uuid` remains an ignored
compiler-bug reproducer.

**Preserve/reject:** keep definition provenance, compiler values, and cross-input reconciliation.
No GUID-name exceptions, guessed values, global INITGUID, or per-failure source retries.

**Resolution:** specify and test the supported SDK definition contexts. Inventory remaining
required PCH-dependent losses in that configuration before proposing exclusions. The historical
460-GUID figure is not a current loss count; recovering definitions does not fix the compiler bug.

**Completion:** required GUID values and availability are verified in the accepted configuration.
Any residual required gap has a bounded in-scope fix or explicit exclusion approval; the compiler
reproducer remains even if production definition capture avoids the failing path.

## 5. Annotations: retained evidence is not an implemented contract

**Owners:** capture/reconciliation for evidence and ownership; projection for bounded lowering;
consumers for observable semantics.

**Evidence:** `projection/annotations.rs::covering` requires one observed SAL set containing
every other set. `write_typedef` retains typedef ownership rather than inventing a merged
observation. `annotations` checks round trips, conflicts, owner rejection, and callback boundaries;
four non-SDK tests pass. SDK-specific cases were not rerun in this review.

**Preserve/reject:** retain original observations and distinct capacity, initialized length,
termination, and optionality meanings. Incomparable evidence and unpublishable owners remain
errors, not discarded attributes.

**Resolution:** select concrete required consumer contracts and check their lowering end-to-end;
classify residual annotations as evidence-only or unsupported rather than claiming semantic
coverage from their presence.

**Completion:** agreed representative callable contracts have independently specified consumer
expectations and SDK-backed CI coverage. No generic SAL interpretation expansion is required.

## 6. Native identity and public naming: keep the validation boundary

**Owners:** reconciliation for native identity/agreement; publication for public families/names.

**Evidence:** `runtime::capture_partial` merges captured observations before pruning.
`projection::Plan::new` validates families before primary naming and callback naming.
`validation_pruning` checks conflicts behind failed dependencies in both input orders;
`primary` checks naming/ambiguity and a generated consumer. These fixtures pass.

**Preserve/reject:** keep native identities distinct from chosen public names. Failed observations
are not proof of agreement. Do not select a representative spelling to conceal public conflicts.

**Resolution:** retain this ordering and shared checks; no redesign is justified by the reviewed
cases. Carry these fixtures into the acceptance set and classify omission roots separately from
dependent losses without building a second reconciliation/reporting system.

**Completion:** required naming/ownership differences are classified against the frozen surface,
and the conflict, ordering, and consumer fixtures remain passing.

## RDL-first repairs

| Difference | Evidence and owner | Resolution |
| --- | --- | --- |
| `BCryptDeleteContext` duplicated by architecture | Native x86 stdcall and unified Win64 C ABI; import publication | Use the observed x86 convention spelling for equivalent Win64 imports. Keep native evidence, callback defaults, and genuine signature/import-name variants. |
| Resource callbacks duplicated by architecture | `libloaderapi.h` declares `ENUMRESNAMEPROC*` and `ENUMRESTYPEPROC*` with `CALLBACK`; named callback publication | Share the observed-x86 spelling rule with imports. Keep native conventions, callback naming, and compiler defaults unchanged. Callables observed only on 64-bit targets retain their native spelling. |
| `BCryptFinishHash` lacks output buffer semantics | SDK `bcrypt.h` uses `_Out_writes_bytes_all_`; `sal.h` defines full initialization | Shared SAL lowering emits output, byte capacity, and initialized length from the direct count. Eight full-buffer forms are covered; unsupported expressions retain evidence. |
| `BCryptEnumContexts` lacks output/optional flags | SDK `_Outptr_opt_result_bytebuffer_all_maybenull_`; the baseline also misses these flags | Lower outer direction/optionality for the 24 returned-buffer forms. Preserve returned extent and inner nullability as evidence, not caller capacity. |

Focused header/RDL fixtures cover all callable kinds and all three targets, with real SDK
definitions and metadata round trips. `import_conventions` checks native evidence, real target
differences, and 64-bit-only functions. `buffer_full`, `returned_buffer_flags`, and `sdk_bcrypt`
cover the annotation repairs. No bindgen or Rust package changes are required for these repairs.
The loader callback fixture retains the SDK's `LPSTR`/`LPWSTR` spellings and input annotations,
rather than substituting the baseline's `PCSTR`/`PCWSTR`.

The full RDL comparison checks 305 changed partitions, 22,284 import ABI spellings, 3,108 callback
ABI spellings, 72 full-buffer target parameters, and 1,079 returned-buffer target parameters.
Declaration inventory, owners, availability, other attributes/types/values/imports, and capture
configuration are unchanged. Omission diagnostics differ only in equivalent import ABI spelling
in quoted bodies.

Win64 ABI equivalence does not imply Rust function-pointer type equality: a direct compiler probe
rejects assigning `extern "system" fn` to `extern "C" fn` and vice versa. Named callable spelling
therefore remains a consumer-facing decision. Structural function pointers retain their native
convention; acceptance must check their use alongside named callbacks rather than assuming
interchangeability from layout or calling convention alone.

## Adopted package control correction

`tool-bindings` and `tool-reactor` regenerated without tracked output changes during the initial
review. Bindgen and windows-link have no remaining source or golden changes.

`tool-package` fails before invoking bindgen because remapping leaves a flat `Windows.Win32`
namespace. The frozen `package-before.exe` and its baseline inputs reproduce the same assertion.
The remaining root contains exactly one API field, `WM_CTLCOLOR`. It is present in the compiled
WinMD but absent from the frozen RDL routing inputs. It already ships as
`windowsx::WM_CTLCOLOR: i32 = 25` in both generated Rust crates, and the named-publication candidate
recovers that exact source declaration in `windowsx.rdl`. Dropping it would remove an existing API.

The maintainer approved preserving that declaration in an isolated corrected source control.
The RDL writer recovered it from the frozen WinMD; all 1,025 original control inputs remain
byte-identical. `crates/tools/win32-2/src/baseline/windowsx.rdl` supplies the same correction to
constant accounting. Frozen package generation succeeds with the corrected routing inputs and
matching manifests. Original artifacts remain intact; no name-specific route or fallback namespace
was added. Further package comparison work is deferred while concrete RDL differences are repaired.

## Remaining approval gates

The initial six-case review and resolution ordering are approved. Required SDK scope, controlled
resource budgets and individual format migration requirements
still need acceptance decisions. No metadata candidate, PCH exclusion, blanket stdcall default,
or production cutover is accepted by this document.
