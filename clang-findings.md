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
or windows-link. The SDK driver owns a checked 42-entry policy: the 18 functions covered before
the ordinal experiment, ILClone and ILFree, and 22 Active Directory helpers. Every entry is checked
against all three pinned
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

RDL triage found all 22 functions in `adshlp.rdl` omitted solely because their SDK records select
ordinals. The installed System32 and SysWOW64 `activeds.dll` export every established name with the
matching SDK ordinal, including forwarded exports; the public provider remains `ACTIVEDS.dll`.
The reviewed table now includes those 22 exact symbol/provider/ordinal tuples, without admitting
other ordinal entries from that DLL. The SDK policy test checks all three archives and captures
all 22 declarations and their native dependencies into `sdk_adshlp.rdl`.
Microsoft documents named DLL exports such as
[ADsGetObject](https://learn.microsoft.com/windows/win32/api/adshlp/nf-adshlp-adsgetobject) and
[AllocADsMem](https://learn.microsoft.com/windows/win32/api/adshlp/nf-adshlp-allocadsmem).
Installed export-table evidence remains limited to x86/x64 on version 10.0.26709.1000, not proof
of every supported Windows version or ARM64 runtime availability. The `adshlp-*` session artifacts
retain the evidence. No parser, ABI, header-selection, or consumer-linkage mechanism changes.

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
| `BCryptDeleteContext` duplicated by architecture | Clang retains explicit stdcall in written attributed types even where the native ABI is C; callable publication | Preserve the written request on every target. Native evidence, compiler defaults, and genuine signature/import-name variants remain unchanged. |
| Resource callbacks duplicated by architecture | `libloaderapi.h` declares `ENUMRESNAMEPROC*` and `ENUMRESTYPEPROC*` with `CALLBACK`; callable publication | Share written-convention handling across imports, named callbacks, and structural pointers. This also covers 64-bit-only declarations without cross-target inference. |
| `BCryptFinishHash` lacks output buffer semantics | SDK `bcrypt.h` uses `_Out_writes_bytes_all_`; `sal.h` defines full initialization | Shared SAL lowering emits output, byte capacity, and initialized length from the direct count. Eight full-buffer forms are covered; unsupported expressions retain evidence. |
| `BCryptEnumContexts` lacks output/optional flags | SDK `_Outptr_opt_result_bytebuffer_all_maybenull_`; the baseline also misses these flags | Lower outer direction/optionality for the 24 returned-buffer forms. Preserve returned extent and inner nullability as evidence, not caller capacity. |
| BITS mixes `[out]` with directional-looking `NativeSal` | `rpcsal.h` maps `__RPC__deref_out_opt` to `_Outptr_`; shared SAL normalization and publication | Approved bounded `NativeReturnedPointerAttribute` replaces fully represented RPC/SAL2 output-pointer evidence. Keep output storage, result nullability, and COM failure guarantees separate. |

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

The subsequent `callback_interop` consumer reproduced a named/structural assignment failure on
Win64 despite compiler-proven native type equivalence. Projection now preserves explicit written
conventions and checks them throughout nested signatures during callback-name selection.
The fixture checks both assignment directions without casts, callback returns and adjusted function
parameters, direct and round-tripped metadata, and both binding styles. All three targets compile;
x86/x64 execute the calls. ARM64 execution remains unavailable on this host.

The written-convention regeneration changes 188 target declarations in 14 partitions, exclusively
C/system spelling. No declaration, owner, availability, constant accounting, or capture-configuration
changes occur. Normalized omissions are unchanged. `written-conventions-comparison.json` and
`written-conventions-changes.json` record the exact comparison against the preceding candidate.

For returned pointers, `MayBeNull` describes successful result nullability; `NullOnFailure`
preserves COM's additional guarantee. A non-null successful result supplies one readable element.
Outer optionality remains `[opt]`; allocation and ownership are not inferred. Native observations
remain unchanged. Unsupported returned-buffer extents, SAL1 contracts, and conditional wrappers
remain explicit evidence. `pointer_parameter_flags` covers all ten supported spellings and actual
SDK definitions; `sdk_background_copy_error` checks the real BITS methods through RDL/WinMD.
This is typed metadata, not a claim that bindgen consumes the new guarantee.

The full returned-pointer regeneration replaces 14,573 target-parameter payloads across 249
partitions. Declaration inventory, ownership, availability, types, values, other attributes,
configuration, omissions, and constant accounting remain unchanged. The SDK writes
`_Outptr_opt_ _Inout_` on `MprConfigTransportGetInfo::lplpwsDLLPath`; these distinct contracts retain
both raw annotations rather than acquiring a permissive combined contract. This restores `_Inout_`
evidence on that parameter on all three targets. A dedicated golden and SDK-macro case cover it.
`returned-pointer-comparison.json` records the exact comparison and `returned-pointer-tests.log`
records the focused regression run. The working SDK metadata remains a partial candidate.

The BITS enumerator `Next` contract supplies a separate array-capacity and initialized-length case.
`__RPC__out_ecount_part(celt, pceltFetched ? *pceltFetched : celt)` uses the fetched count when its
pointer is present and `celt` otherwise. `NativeInitializedLength.NullFallbackParamIndex` records
that fallback without parsing arbitrary expressions. `NativeArrayInfo.PostWritable` also preserves
the RPC macro's successful-output writable-capacity guarantee; omitting it would lose information.
Both are optional named fields, and existing contracts without them keep their encoding.

The full SDK delta removes 183 fully represented RPC parameter payloads in 22 partitions, including
15 target-parameter instances with conditional lengths. Every declaration, owner, availability,
type/value, unrelated attribute, omission, configuration, and constant comparison remains unchanged.
`conditional-length-comparison.json` records the exact checks. Dedicated header/RDL fixtures cover
all callable kinds, real SDK macros and BITS, metadata round trips, invalid types, conflicting
fallbacks, and unsupported expressions. Consumers must understand the new fallback before using
initialized-length metadata to dereference an optional count pointer; consumer-version acceptance
remains separate from this metadata repair.

The checked named-publication policy restores all 22 `adshlp.h` helpers as named exports from
`activeds.dll`. Installed x86/x64 export tables and all three pinned SDK import archives verify
their name/ordinal identities. `sdk_adshlp.h/.rdl` independently captures every signature.
`adshlp-comparison.json` confirms 66 added target declarations in the restored `adshlp.rdl`,
exactly 22 removed omission identities, and no changes to existing declarations or constants.
No ordinal consumer linkage or parser exception is involved.

Explicit `_COM_Outptr_` contracts also require the existing `ComOutPtrAttribute` (`#[iid_is]`).
Shared normalization retains COM intent separately from null-on-failure; publication requires
native unqualified `void**` storage, including aliases. Ordinary output pointers, typed interfaces,
const pointees, and deeper chains are negative controls. `com_output` covers all callable kinds,
SDK macros, native evidence, and metadata round trips. The SDK regeneration restores 462
target-parameter markers in 294 target declarations across 28 partitions. Exact comparison in
`com-output-comparison.json` confirms no other declaration, ownership, availability, type, value,
attribute, configuration, omission, or constant-accounting changes.

`accountssettingspaneinterop.rdl` supplies the concrete input-contract fixture. The approved
`NativeInputAttribute` records valid input on entry, with pointer nullability governed by `[opt]`.
`ReadOnly = false` represents `__RPC__in`/`__RPC__in_opt`; true preserves the additional read-access
contract of direct SAL1/SAL2 input forms. False does not require writable storage or change native
qualifiers. The full SDK delta replaces 13,215 RPC target-parameter payloads and makes 90,634 SAL
read-access contracts explicit across 416 partitions. Every declaration, type, owner, availability,
and unrelated attribute remains unchanged. Configuration and constant accounting are identical;
12 embedded omission-signature lines reflect the new encoding, with no omission changes.
`input-contract-comparison.json` records these checks.

Leading parameter comments retain compiler-bounded ownership and source locations. Literal MIDL
bracket lists supply `[retval]`, output-only `[iid_is]`, and accompanying direction flags.
`IAccountsSettingsPaneInterop` and `IAudioClient::GetService` have dedicated SDK fixtures.
No IID association is inferred from GUID parameters or names. A full SDK run exposed valid input
`[iid_is]` contracts on `IBindStatusCallback::OnObjectAvailable`, text-services sink registration,
and `ICommandStream::SetCommandStream`. They describe supplied interfaces, not returned pointers.
Shared normalization keeps input/output directions distinct; the input relationships remain native
comment evidence rather than acquiring a COM output marker. The `sdk_midl_inputs` fixture checks
these cases, and synthetic tests cover input/inout IID markers, ownership, conflicts, and rejection.
Other MIDL comment contracts and macro-expanded parameter-list ownership remain separate gaps.

The final MIDL SDK comparison restores 16,728 retval, 633 IID-output, and 3,549 output-direction
target-parameter markers across 113 partitions. `midl-comments-comparison.json` verifies unchanged
declaration inventory, ownership, availability, types, other attributes, configuration, omissions,
and constant accounting. MIDL-only callback contracts also participate in callback naming and
metadata-owner checks; a structural pointer cannot acquire an unrelated annotated callback name.
`midl_comments` covers both that boundary and indirect callbacks without a contract owner.
Final generation reports 317.067 seconds for capture/merge/project and 4.917 seconds for compilation.
These are observations, not controlled benchmarks; peak memory and resource-budget acceptance
remain unmeasured.

### RPC input strings and arrays

Argument-free RPC input strings and direct/full input arrays now share typed normalization across
reconciliation and publication. `NativeInput(ReadOnly = false)` preserves their input validity
without inventing SAL1/SAL2's additional read-access guarantee. Counted strings also retain entry
termination. Partial input arrays still preserve their separate readable length and writable
capacity as evidence; they do not acquire a successful-output initialized length.

`rpc-input-comparison.json` records 1,140 input-string and 738 input-array target-parameter payload
replacements, plus 5,576 SAL input read-access contracts. The 226 changed partitions retain every
declaration, type, owner, architecture, and unrelated attribute. Configuration, omissions, and
constant accounting are identical. The 92 breadth tests include pinned SDK macro definitions,
equivalent RPC spellings, read-access conflicts, and residual count expressions. Affected annotation
and callable fixtures, warnings-denied clippy, and the no-default-features check pass.
Generation reports 309.492 seconds for capture/merge/project and 5.129 seconds for compilation;
these are observations, not controlled measurements, and peak memory was not measured.

The wider RDL diff review also found:

| Difference | Native evidence and disposition |
| --- | --- |
| `FACILITY_NS`, `MAX_INTERFACE_NAME_LEN` | Present under `nserror` and `ifmib`; ownership moves, not losses |
| `AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS::TargetProcessId` | Written `DWORD` alias retained instead of flattened `u32` |
| `memorybuffer.rdl` deletion | `IMemoryBufferByteAccess` is inside `Windows::Foundation`; capture descends linkage blocks but not namespaces |
| `MetaDataGetDispenser` output | Repaired through the four SAL1 returned-pointer aliases described below |
| `_Null_terminated_` | SDK buffer property without a pre/post phase, including direct `coml2api` parameters; do not blindly replace it with an entry-only guarantee |
| `_Must_inspect_result_` | Adds `SAL_mustInspect` beyond `_Check_return_`; do not collapse the two return obligations |

Namespace traversal alone is insufficient: native alias correspondence currently assumes global
names. Preserve namespace-qualified correspondence before admitting namespace declarations, and
decide flat-public-name collisions rather than merging unrelated aliases by unqualified spelling.
The maintainer notes that most selected APIs are global, but some `Windows::...` and
`ABI::Windows::...` declarations refer to WinRT types. The existing parser's
`is_flat_declaration` consults supplied metadata references for ABI-prefixed namespaces, and
`tool-win32` supplies `windows_default::WINRT` through `MetadataReferences`. The rewrite has no
equivalent external-type-reference input. Restore that boundary rather than treating every
namespaced declaration as a new flat Win32 type. `IMemoryBufferByteAccess` and `IBufferByteAccess`
are separate IUnknown-based interop helpers in the SDK's `memorybuffer.h` and `robuffer.h`;
their placement under `Windows` is not itself proof of WinRT type identity.

### SAL1 returned-pointer aliases

The four argument-free SAL1 output-pointer forms share `NativeReturnedPointer` normalization with
their SAL2 counterparts. `__deref_out_opt` permits a null result, not a null argument.
`__deref_opt_out` permits a null argument, and `__deref_opt_out_opt` permits both. None supplies
COM's null-on-failure guarantee or an IID-output marker.

The pinned macro cases and `sdk_rometadata.h`/`.rdl` fixture check all targets. The 94 breadth and
seven annotation tests pass, including equivalent aliases, distinct nullability contracts,
malformed arguments, storage rejection, and SDK definitions. Warnings-denied clippy and the
no-default-features check pass. The exact SDK comparison changes only `MetaDataGetDispenser.ppv`
in `rometadata.rdl`: raw `__deref_out` becomes `#[out]` plus the non-null returned-pointer contract.
Every other RDL byte, configuration, omission diagnostic, and constant-accounting row is unchanged.
Artifacts are `sal1-pointer-*` and `compare-sal1-pointer.py` in the session files.
Generation reports 297.203 seconds for capture/merge/project and 4.953 seconds for compilation;
these are observations, not controlled measurements, and peak memory was not measured.

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
