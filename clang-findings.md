# Clang replacement findings

This is the bounded review required by `plan.md`, not a declaration of replacement readiness.
The maintainer approved the resolution list and permits a documented reader/generator upgrade
requirement. Ranks order the remaining work; individual exclusions and cutover are not approved.

## Controls

| Control | Identity and status |
| --- | --- |
| Frozen comparison baseline | `6e68e56081340eaa932f4dbc92ec54c56ed51473` |
| Working source checkpoint | `58ee55b4d3`, plus contracts, named linkage, context, per-function ABI, GUID-less interfaces, and allocator repairs |
| Working `metadata/win32` | Regenerated GUID-less inheritance partial candidate; visible, uncommitted |
| Reviewed metadata control | `breadth-noreturn-repeat`; not an accepted replacement |
| Ordinal candidate | `breadth-ordinal-imports`; unresolved x86 import gaps |
| Stdcall trial | `breadth-sdk-stdcall`; not accepted as the canonical configuration |
| Scratch output | `target/win32-2/Windows.Win32.winmd` matches the GUID-less inheritance run |
| Toolchain | LLVM 22.1.8, companion ABI 17, SDK 10.0.28000.2270; x86/x64/ARM64 |

The configuration uses the existing 370 headers and seven translation units, with unchanged compiler
defaults. The global stdcall switch is removed. Historical controls and SHA-256 identities are in
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

**Resolution:** preserve established named linkage using the frozen-baseline/SDK join described
below. No ordinal consumer changes remain in bindgen or windows-link. The initial 42-entry policy
covered the 18 functions before
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

### RPC output arrays and ordinary pointer failure contracts

`_Outptr_result_nullonfailure_` and its optional-storage variant use `NativeReturnedPointer`.
They preserve required non-null results on success and null results on failure without supplying
COM IID-output intent. The pinned macro cases distinguish these from both `_Outptr_` and
`_COM_Outptr_`. `sdk_shell_item.h`/`.rdl` checks `IShellItem::GetDisplayName`.

RPC output capacity, full-array, and terminated-array forms now replace fully represented SAL
payloads with typed attributes. Full inout forms also normalize. Output arrays preserve the SDK's
extra `PostWritable` guarantee; inout arrays do not acquire it. A known capacity retains that
guarantee even when the separate initialized-length expression is unsupported. Partial/full aliases
agree only when the counts and postconditions agree. RPC output and SAL2 output forms with different
writable guarantees remain distinct.

`rpc_full_buffers.h`/`.rdl` covers capacity, initialized length, termination, and optional inout
storage. The 97 breadth and seven annotation cases pass, including pinned SDK definitions and
round trips. The affected SDK interface/function fixtures, warnings-denied clippy, formatting, and
no-default-features check also pass.

`output-contract-comparison.json` compares the regenerated candidate to the preceding validated
candidate, not the restored Git baseline. Exactly 33 partitions change: 321 RPC array
target-parameter payloads normalize, 225 capacities gain their post-writable guarantee, and 135
ordinary returned-pointer contracts normalize. Declaration inventory, types, ownership,
availability, and unrelated attributes remain unchanged. Configuration, all 73,532 omission
diagnostics, and constant accounting are byte-identical. Tracked `metadata/win32` is regenerated
and left visible. The source repair reuses the shared normalizer; no new metadata vocabulary or
bindgen changes are needed.

Generation reports 318.588 seconds for capture/merge/project and 5.060 seconds for compilation.
These are observations, not controlled measurements; peak memory was not measured.

Two further inspected differences remain separate:

- `CFunctionDiscoveryNotificationWrapper` is a C++ implementation helper with method bodies,
  rather than a separately identified COM interface. Capture reports an unsupported class; the
  baseline's empty `#[no_guid]` interface is not proof that this helper should be restored.
- `__RPC_unique_pointer` and `__RPC_full_pointer` expand to the phase-independent `_Maybenull_`
  property. The examples in `activation`, `comcat`, and `docobj` annotate typedefs, not merely
  optional call arguments. They belong with the remaining type-property work, not a blanket
  `#[opt]` replacement.

### Typed field buffers

The approved `NativeFieldBufferAttribute` preserves writable capacity, optional readable extent,
byte/element units, and nullable storage for twelve SAL2 field-size forms. Counts must name direct
integer sibling fields, not bitfields, enclosing-record fields, or member paths. These properties
hold when the containing value is valid; they are not callable pre/postconditions or ownership
guarantees. Native and public-family comparisons share normalization. Different composed contracts
remain evidence, and invalid count/storage types fail publication.

`field_buffers.h`/`.rdl` covers aliases, nested owners, and inline arrays. The SDK fixture checks the
actual twelve macros plus Authz and BCrypt records. Agreement/conflict, retained native evidence,
unsupported-expression, and enclosing-owner rejection cases pass. All 101 breadth and seven
annotation cases pass with SDK cases included, as do the SDK function/interface fixtures,
warnings-denied clippy, no-default-features check, and formatting.

The regenerated SDK candidate changes 64 partitions: 1,754 target-field payloads become typed
contracts in 1,142 target declarations. Exact comparison preserves every declaration, type, layout,
owner, target, and unrelated attribute. All 73,532 omission diagnostics, configuration, and constant
accounting are byte-identical to the preceding candidate. Nested references such as
`D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU`'s union field referring to enclosing `ObjectCount` remain
raw evidence; the direct outer field normalizes.

Artifacts are `field-buffer-comparison.json`, `field-buffer-sdk.log`, and the field-buffer regression
logs in the current session. Generation reports 353.419 seconds for capture/merge/project and 5.763
seconds for compilation. Tests overlapped this run; these are not controlled timings, and peak
memory was not measured. This batch establishes a metadata contract, not consumer acceptance.

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

### Winsock and general named-linkage repair

The maintainer approved frozen-baseline named linkage checked against native SDK import evidence.
`named_exports.rs` uses a generated snapshot of target, declaration name, DLL, and export name.
Refresh compiles the frozen Win32 RDL with its matching metadata definitions and external WinRT
references, then uses the metadata reader. No ABI, layout, signature, or ordinal numbers are copied
from the baseline. Normal generation uses the checked-in snapshot without Git history.

Publication requires the compiler's exact native linker symbol in the selected target's SDK code
imports, plus matching baseline name/provider evidence. It never strips linker decoration to infer
names or rewrites calling conventions. Missing evidence remains omitted, provider conflicts fail,
and explicit import overrides retain precedence. The handwritten 42-entry policy is removed.

`sdk_winsock.h`/`.rdl` covers all 125 baseline WS2_32 functions from Winsock2 and WS2tcpip. All 109
Winsock2 baseline functions are present on all three targets, closing its 37 missing names.
Generated consumers check exact representative signatures, byte-order conversions, error state,
and `select` behavior. Host and isolated x86/x64 executions pass; all three targets compile/link
and contain the expected named PE imports. ARM64 execution was not available. Existing DHCP/DNS
x86 mismatch checks still reject the default configuration.

Full regeneration restores 1,436 baseline functions, or 4,308 target declarations, across 38
partitions. This is the shared policy's effect, not additional per-family fixes. Every retained
declaration, type, attribute, owner, and target is identical to the preceding candidate. Exactly
1,436 missing-import omission roots disappear; constants remain byte-identical. The candidate has
13,766 functions and 72,096 omission diagnostics. Other recovered families still need their own
consumer acceptance, rather than inheriting Winsock's behavioral coverage.

`--compare-functions` is a checked-in reader-based comparison of current Win32 RDL against the
frozen named-linkage snapshot. It reports 445 absent names, 1,321 missing target pairs, and the
same three known Rtl provider differences. The largest remaining provider groups include kernel32
(81 names), avifil32 (54), ole32 (50), DHCP (41), and DNS (22). Missing imports are not proof of
target absence.

Canonical accounting corrects another limitation in the earlier script: its library pattern missed
50 functions with explicit import-name attributes. The Win32 baseline has 14,171 named functions,
not 14,121; the pre-repair loss was 1,881 names, not 1,831. The compiled default WinMD has a larger
scope than `metadata/win32`: it includes WDK inputs. The final snapshot uses the Win32 RDL scope,
not that combined file. Removing its 489 extra rows changed no SDK publication decision.

An attempted combined Win32/WDK compile also exposes an existing integration failure:
`metadata/wdk/ntddk.rdl`'s `KPCR` needs missing `NT_TIB`. The pre-Winsock candidate already lacks
that definition. This remains a separate acceptance issue; no WDK inputs or fallback definitions
were changed to conceal it.

Artifacts are `winsock-comparison.json`, `winsock-sdk.log`, and the policy/consumer logs in the
current session. The named-linkage snapshot regenerates identically. Formatting, warnings-denied
clippy, focused policy/ABI regressions, and consumer checks pass. Generation reports 296.401 seconds
for capture/merge/project and 5.117 seconds for compilation; this was not a controlled benchmark,
and peak memory was not measured.

## Independent review disposition

### Residual function roots after named-linkage repair

The 445 missing frozen-baseline functions split into 182 missing import-ownership failures,
221 capture/dependency/projection failures, and 42 names without a `Value(...)` root diagnostic.
The last group includes PSAPI aliases, CLFS inline helpers, and other public-name questions;
absence of a root diagnostic is not evidence that these APIs can be excluded.

The largest direct dependency groups are `IAVIStream` (38 functions), `_CONTEXT` (15),
`IAVIFile` (11), `ID3DInclude` (7), and `_TP_CALLBACK_ENVIRON_V3` (5). AVI capture rejects
inherited-slot redeclarations. `ID3DInclude` has no native UUID. Neither problem authorizes
inventing COM identity or vtable slots. Import ABI mismatches remain separate from absent SDK
archive entries; the global stdcall experiment is not an accepted correction.

`_CONTEXT` is blocked by Clang's `NoTrivialAutoVarInit` attribute. LLVM 22.1.8
`clang/lib/CodeGen/CGDecl.cpp`, in `EmitAutoVarInit`, checks the tag or enclosing function for this
marker before inserting automatic initialization. It does not alter record layout, an import's
calling convention, or a caller's initialization obligations. Retain the marker in native evidence
and reconciliation without adding a metadata initialization attribute. `_EXCEPTION_POINTERS` and
`_EXCEPTION_REGISTRATION_RECORD` depend on `_CONTEXT`; the latter also blocks `NT_TIB` and WDK
integration. The focused context fixture covers exact native sizes, alignment, and instruction
pointer offsets on all targets, plus generated bindings and a real `RtlCaptureContext` call.

The context repair restores 19 baseline functions on 54 target pairs. Current accounting is
426 missing names / 1,267 target pairs: 183 import-ownership failures, 201 capture/dependency
failures, and 42 without a value-root diagnostic. Eight partitions change; every retained
declaration, owner, target, type, and attribute is identical. Constants and capture configuration
are byte-identical. The candidate contains 10,270 records, 4,532 enums, 13,286 aliases,
93,085 constants, 13,785 functions, and 4,016 interfaces.

Six existing omissions acquire more specific reasons rather than losing additional declarations:
three minidump records contain aligned context storage under packing, and `RtlCaptureContext2`,
`__C_specific_handler`, and `_except_handler` lack import ownership. The recovered x86
`RtlRestoreContext` uses the SDK's `ntdll.dll` provider instead of the baseline's `kernel32.dll`;
this joins the three recorded Rtl provider differences and still needs acceptance review.

`NT_TIB` and `EXCEPTION_REGISTRATION_RECORD` are recovered. Combined Win32/WDK compilation
passes that point but fails at `ntifs.rdl`'s `READ_LIST.List`: missing `FILE_SEGMENT_ELEMENT`.
Its native `PVOID64 Buffer` is a fixed-width pointer even on x86, currently rejected as unsupported
qualifiers. Do not replace it with a target-sized pointer or copy the baseline union as a fallback.
WDK integration remains open.

All five initialization regressions pass, including native/public conflicts and attributed forwards.
The shared import consumer verifies layouts and named linkage on all three targets and executes
on x86/x64; ARM64 execution remains unavailable. Warnings-denied clippy, the no-default-features
check, annotation/visibility/fidelity regressions, and formatting pass. Full generation reports
299.363 seconds capture/merge/project and 5.464 seconds compilation with concurrent validation.
A mid-run process query observed at least 6,671,699,968 peak working-set bytes; this is not a final
process-tree peak or a controlled budget measurement. Artifacts are `context-comparison.json`,
`context-sdk.log`, and the context controls in the current session.

### Compiler-derived per-function import ABI

The maintainer approved per-function ABI resolution, not a new global compiler default.
`windows_clang_getImportConvention` reads written/defaulted outer convention provenance, including
function typedefs. For eligible implicit C imports on MSVC x86, it constructs a stdcall type and
linker symbol through Clang. Explicit conventions, definitions, variadics, asm labels, and
unsupported function properties have no candidate. Synthetic declarations are not added to TU
traversal, and original declarations are not mutated.

Capture retains the original types, symbols, parameters, and annotations. It checks that the
candidate differs only in its outer convention. SDK policy permits publication only with an exact
candidate symbol/provider match. Explicit public mappings and ordinary native-symbol mappings win.
Every declaration observation must permit resolution; candidates cannot conceal native conflicts
or override explicit cdecl declarations. The production `--x86-stdcall` option is removed.

Full regeneration restores exactly 94 baseline functions on 282 target pairs across 13 partitions,
including 41 DHCP and 22 DNS functions. Every retained declaration, callback, function-pointer field,
owner, target, type, and attribute is unchanged. There are no new omissions; exactly 94 ownership
omissions disappear. Constants, source inputs, compiler arguments, explicit imports, and all existing
target import mappings remain identical. The configuration records 94 per-function decisions.
The output has 13,879 functions and 71,938 omission diagnostics; other declaration totals are
unchanged.

Frozen-baseline accounting now leaves 332 missing names / 985 target pairs: 89 import-ownership
failures, 201 capture/dependency/projection failures, and 42 names without a value-root diagnostic.
The four Rtl provider differences remain unchanged and need acceptance disposition. Required API
losses, including AVI interfaces and missing SDK import entries, are not approved exclusions.

Native controls cover explicit conventions and typedefs, nested callbacks, unsupported properties,
unchanged original types/symbols, and unchanged TU traversal on all targets. Parser controls cover
mapping precedence, same-TU and cross-TU redeclarations, input order, partial projection, and native
conflicts. All nonignored parser regressions, warnings-denied clippy, no-default-features checks,
and the SDK policy suite pass. Generated consumers execute DNS validation without changing DHCP
configuration on x86/x64; all three targets compile, link, and retain named PE imports. These
selected consumers do not establish behavior parity for every restored function.

Generation reports 312.210 seconds capture/merge/project and 5.260 seconds compilation. The driver
wall time is 319.534 seconds; the sampled process peak is 7,333,953,536 bytes. These are observations
with an unquiet machine, not a process-tree budget measurement or a matched legacy benchmark.
The session's `implicit-abi-before.zip`, `implicit-abi-comparison.json`, `implicit-abi-sdk.log`,
and `implicit-abi-resources.json` retain the control and exact retained-declaration comparison.

### Selected SDK checks in CI

The x64 test job explicitly runs the SDK driver's policy/configuration fixtures, the context
layout fixture, and the missing-import negative fixture. It adds Rust's x86/ARM64 targets and runs
the named import consumer matrix, including Winsock, context, and DHCP/DNS. Native x64/ARM64 host
consumers remain part of the ordinary test suites. Unsupported ignored reproducers remain excluded.
The crate page gives checkout-only commands and prerequisites; no session files enter these checks.
The exact selected commands pass locally with warnings denied: four driver cases, context capture,
the missing-import negative case, and the cross-target consumer matrix. The workflow still needs
its first hosted run. Full-workload semantic comparison and acceptance controls remain separate
reproducibility work.

### GUID-less interface recovery

Native vtable structure, not UUID presence, identifies an interface for projection.
GUID-less hierarchies use the existing `#[no_guid]` representation; native layout, calling
conventions, access, purity, and contiguous inherited slots remain checked. No IID or IUnknown
ownership is invented. Const pointers whose qualification would be erased by the
interface-reference encoding remain rejected.

Dedicated native/header/RDL fixtures cover bare, extended, and inherited-only interfaces, aliases,
pointer storage, and rejection boundaries. Metadata round trips preserve absent GUID attributes.
Generated high-level consumers verify inherited calls, pointer-sized storage, and no AddRef/Release
on clone/drop. The real `D3DCompile` consumer uses a supplied `ID3DInclude` object and verifies both
Open and Close callbacks. All three targets compile/link; x86/x64 execute. The selected cases are
wired into CI alongside the prior import consumers.

The full run restores seven required shader functions on 21 target pairs, plus 20 interfaces,
three records, four alias names, and two callbacks across eight partitions. Every retained
declaration, owner, target, type, and attribute is identical. Constants and capture configuration
are byte-identical. Ten already-omitted roots receive deeper diagnostics, including unsupported
GUID-less COM inheritance, interface references, volatile parameters, and interface-valued numeric
sentinels. There are no new lost declarations. Required losses fall to 325 names / 964 target pairs.

This run reports 612.523 seconds capture/merge/project, 10.765 seconds compilation, 625.867 seconds
driver wall time, and 7,789,744,128 sampled peak process bytes. It is substantially slower than the
preceding uncontrolled run; no cause is established and no performance acceptance is claimed.
The following allocation run records phase diagnostics to separate capture from projection. A
matched benchmark still requires a quiet machine and the agreed legacy control.
Artifacts are `guidless-before.zip`, `guidless-comparison.json`, `guidless-sdk.log`, and
`guidless-resources.json` in the current session.

### Allocation instrumentation

Six required allocation imports are blocked by `MSAllocator`, not an unsupported calling ABI.
Pinned LLVM 22.1.8's `MSAllocatorDocs` in `clang/include/clang/Basic/AttrDocs.td` explicitly
distinguishes this CodeView profiling marker from GCC `malloc` optimization semantics.
`clang/lib/CodeGen/CGCall.cpp` adds heap-allocation debug metadata only when debug information is
enabled. Microsoft's [allocator documentation](https://learn.microsoft.com/cpp/cpp/allocator)
describes its profiling purpose.

Capture retains a typed `MSAllocator` marker and reconciliation compares its presence on functions
and methods. Publication adds no ownership, no-alias, nullability, or deallocator attribute.
The fixtures preserve allocation and reallocation signatures and reject conflicting observations
in both input orders. SDK consumers exercise HeapAlloc/HeapReAlloc, GlobalAlloc/GlobalReAlloc, and
LocalAlloc/LocalReAlloc with data-preservation and release checks on x86/x64; all targets compile
and link with named imports. Global/local reallocation permits relocation rather than assuming
an in-place resize must succeed. Existing SAL evidence remains independent.

Full regeneration restores all six imports on 18 target pairs in `heapapi.rdl` and `winbase.rdl`.
Every retained declaration, owner, target, type, and attribute is unchanged; constant accounting and
capture configuration are byte-identical. Already-omitted CRT functions reach deeper `Restrict` or
missing-import diagnostics, without adding lost declarations. Required losses fall to 319 names /
946 target pairs. The candidate has 13,892 functions and 71,878 omission diagnostics.

This profiled run reports 652.878 seconds capture/merge/project, 10.592 seconds compilation,
666.109 seconds driver wall time, and 7,521,808,384 sampled peak process bytes. The phase log records
528.237 seconds in workers and 106.874 seconds in projection. Both contribute to the elapsed time;
these observations do not establish the cause of the regression against the earlier run or meet
the controlled performance gate. Artifacts are `allocator-comparison.json`, `allocator-sdk.log`,
`allocator-sdk.err`, and `allocator-resources.json` in the current session.

### GUID-less inheritance and consumer identity

The initial GUID-less repair added an unjustified rejection for a child without a UUID whose base
has one. Native UUID presence and base identity are independent facts. Existing RDL can represent
both with `#[no_guid] interface Child: Base`; no inherited or name-matched IID is needed.
The synthetic inheritance fixture and SDK `IAVIPersistFile` case check that the child remains
GUID-less through both metadata round trips while the base retains its UUID.

`bindgen` `CppInterface::write` emits `define_interface!(..., 0)` for a GUID-less interface with an
IUnknown base. This pre-existing consumer behavior is not a native IID and must not establish
identity/cast correctness. Keep it as a separate consumer acceptance defect rather than rejecting
representable native metadata. AVI's overridden methods remain a distinct capture/contract gap;
preserving inheritance does not authorize dropping their annotations or inventing vtable slots.

Full regeneration restores `IAVIPersistFile`, `IBaseVideoMixer`, `IRicheditUiaOverrides`, and
`PAVIPERSISTFILE` on all three targets in three partitions. Every retained declaration is unchanged;
there are no new omissions, and constants/configuration remain byte-identical. The native SDK
fixture checks `IAVIPersistFile::Reserved1` at slot 9; generated raw consumers check the matching
offset and ten-pointer vtable on all targets. The candidate has 4,039 interfaces, 13,293 aliases,
and 71,874 omission diagnostics. Required function losses remain 319 names / 946 target pairs:
89 ownership, 188 capture/dependency/projection, and 42 without a value-root diagnostic.

The profiled run reports 312.501 seconds capture/merge/project, 5.142 seconds compilation,
319.296 seconds driver wall time, and 7,412,654,080 sampled peak process bytes. Workers take
253.267 seconds and projection 50.006 seconds. Timings returned to the earlier range, but these
uncontrolled runs do not explain the variation or establish performance acceptance.
`guidless-inheritance-comparison.json`, `guidless-inheritance-sdk.log`/`.err`,
`guidless-inheritance-resources.json`, and `guidless-inheritance-regressions.log` retain the results.

The AVI override boundary needs a metadata/consumer decision, not an IID naming rule.
SDK `vfw.h` redeclares `IAVIStream::QueryInterface` with `_Outptr_` and the `STDMETHOD` nothrow
contract. `unknwn.h` declares the inherited method with `_COM_Outptr_`, an `iid_is` comment, and no
written nothrow contract. These are not identical declarations whose annotations can be discarded.
`rdl` `Encoder::encode_interface` gives every declared method `MethodAttributes::NewSlot`;
appending the overrides would shift the vtable. The maintainer approved bounded attribute-based
handling, without general C++ expansion or an assumption that these old declarations are buggy.

`Method::overrides` retains Clang's override result alongside the absolute slot. Capture requires
one overridden method, the same slot, and an identical canonical return type; covariance and
adjusting overrides remain rejected. Reconciliation and public families compare override evidence.
`NativeOverrideSlotAttribute` preserves each redeclaration, including parameter names/defaults and
output/exception annotations. RDL clears `NewSlot`; bindgen validates the inherited name/signature,
including pointer depth and qualifiers, and reuses the inherited Rust callable and vtable field.
It adds no duplicate `_Impl` requirement.

Synthetic multi-level and UUID-bearing fixtures, conflicting native evidence, malformed slots,
pointer-depth mismatches, and real AVI metadata pass. Consumers verify the physical vtable sizes
and offsets on all three targets, execute inherited calls on x86/x64, and create/clone an empty
editable AVI stream before releasing it. The prior procedural override-rejection case is ported
to the positive fixture. Affected parser, RDL, and bindgen regressions pass.

The frozen `6e68e56081` generator and metadata reader were built together for a compatibility probe.
On x64 they silently emit `IAVIStream`/`IAVIFile`/`IAVIStreaming` vtables of 136/104/64 bytes instead
of 112/80/40, and place stream `Info` at offset 56 instead of 32. This is a required paired-upgrade
boundary, not a compatible annotation addition. The probe is in session `override-generator-baseline`.

Full SDK regeneration restores 73 required functions / 219 target pairs, including AVI, Direct3D,
DirectDraw, DirectSound, and XAudio2 entry points. It adds 221 interfaces, 246 aliases, and ten
records across 24 partitions, without changing or losing retained declarations. Constant accounting
is byte-identical. Twelve changed omission messages expose deeper failures on already omitted
roots: variadic or non-pure methods, missing proxy imports, and an unsupported interface typedef.
Required function losses are now 246 names / 727 target pairs: 89 ownership, 115 capture/dependency/
projection, and 42 without a value-root diagnostic. The four Rtl provider differences remain open.

The generic session comparison stopped on two assumptions, not a generation failure: a projection
diagnostic prefixes its existing alias identity, and the configuration report includes derived
import ownership. `verify-overrides-comparison.py` verifies all twelve prior omission roots and
the exact configuration delta: six recovered DirectSound/XAudio2 names gain named ownership on
three targets, replacing eighteen unresolved ordinal diagnostics. All other configuration lines
are unchanged. The original comparison failure remains visible; no production rejection is relaxed.

The candidate has 10,283 records, 4,532 enums, 13,539 aliases, 93,085 constants, 13,965 functions,
4,260 interfaces, 789 partitions, and 71,216 omission diagnostics. The run reports 309.173 seconds
capture/merge/project, 5.557 seconds compilation, 316.390 seconds driver wall time, and
7,528,910,848 sampled peak process bytes. These are uncontrolled observations, not budget acceptance.
`overrides-comparison.json`, `overrides-comparison-reviewed.json`, `overrides-resources.json`, and
`overrides-sdk.log`/`.err` retain the evidence. The input control is `overrides-before.zip` (SHA-256
`991C2F5C88DFAF8312BF5CA143D1CC926A4F85DBB371F86A20F8A6827AEC72C3`).

Bindings and Reactor regeneration leave tracked Rust/manifests byte-identical. Final
warnings-denied clippy and the parser's no-default-features check pass. On identical remapped
metadata, both current bindgen and pre-change `ce6188b027` bindgen reach the same bitfield-storage
failure in `CppStruct::write_bitfield_accessors`; all 554 generated `windows-sys` package files
match byte-for-byte. This establishes the failure predates the override change, not that full
high-level package generation passes. `override-package-comparison.json` and
`override-package-head.log` retain the matched control result. The corrected frozen inputs and
repository package outputs remain intact.

### PSAPI public aliases

Twenty-seven of the 42 missing names without value-root diagnostics are PSAPI public aliases
(81 target pairs). Every corresponding `K32*` native function is already present with the baseline
DLL and export. For example, the baseline's `EnumProcesses` imports `kernel32.dll!K32EnumProcesses`;
the candidate publishes that import as `K32EnumProcesses`. All 27 native names are additions
relative to the frozen baseline, so native capture completeness is not public-name compatibility.

Pinned `psapi.h` selects these macro aliases under `PSAPI_VERSION > 1`. A separate version-1 input
would instead declare the downlevel `psapi.dll` family and would not preserve the required linkage.
The attempted version-1 input and its failing fixture were removed; production configuration and
tracked RDL are unchanged. `psapi-alias-investigation.json` checks all 81 pairs against current
function accounting. The maintainer approved compiler-evidenced aliases while retaining native
public names and exact native linkage. Do not strip `K32` by spelling, copy baseline signatures,
or change DLL providers to make the presence count pass.

The compiler classifies macro expressions through `__typeof__` before value or reference probes.
Only a direct function reference, with optional source parentheses, supplies an alias target.
The target must have a captured global declaration and external C linkage. Capture retains its
signature, parameter contracts, canonical identity and macro owner. Native companion ABI 18 extends
the existing operand query to typedef/using declarations; LLVM remains pinned to 22.1.8.
Calls, comma expressions, pointer relocations, unsupported scopes and failed ASTs supply no alias
evidence. Import policy resolves the native target, not the public macro spelling.

The full workload exposed `Shell_GetCachedImageIndex`: a native wide-string function and a macro
alias to the ANSI function coexist. These are separate identities, not conflicting native
redeclarations. Reconciliation and target association retain both; strict publication rejects their
public collision, and partial publication omits the macro without replacing the native signature.
The focused fixture also checks conflicting native declarations in both input orders, preserving
fatal reconciliation before pruning. Seven alias regressions, native suites, the SDK fixture,
warnings-denied clippy and both parser checks pass. Nine import-consumer tests include exact
three-target linkage and real x86/x64 calls through both PSAPI spellings.

Full tracked regeneration restores 42 required names / 126 target pairs: all 27 PSAPI aliases,
ten CLFS LSN helpers, three `Rtl` aliases, `CM_WaitNoPendingInstallEvents`, and `UrlIs`. Every
restored pair has baseline DLL/export ownership. All 6,412 added target declarations have another
published declaration with identical linkage, signature and parameter attributes. No retained
declaration, owner or target changes or disappears; constants and capture configuration are
byte-identical. Existing four `Rtl` provider differences remain unchanged and unapproved.

The generic line-based omission comparison rejects shifted probe locations and diagnostic framing.
`verify-function-aliases.py` retains its assertions and separately checks structured root identities:
no new omission root, with 2,159 removed. `function-alias-comparison-reviewed.json` records every
added declaration's matching contract. The candidate adds 2,150 names on at least one target across
96 partitions; these include target extensions of existing names, not just new public names.
Totals are 10,283 records, 4,532 enums, 13,539 aliases, 93,085 constants, 16,085 functions,
4,260 interfaces, 790 partitions and 69,057 omission diagnostics. Required losses are now
204 names / 601 target pairs: 89 ownership and 115 capture/dependency/projection.

Uncontrolled observations are 274.845 seconds capture/merge/project, 5.630 seconds compilation,
281.889 seconds driver wall time and 8,015,290,368 sampled peak process bytes. These do not meet
the matched process-tree budget measurement gate. `function-alias-after.zip` has SHA-256
`9A7B349A9DEB6171A5D6E82B983708AF57816C31E82AB107850AC535B4468209`; matching configuration,
coverage and omission files preserve the candidate independently of later scratch runs.

### Target-local import evidence

The alias candidate's 89 ownership roots omitted all 267 required target pairs, although its
configuration contained matching baseline DLL/export evidence for 177 pairs: all 89 on x64 and
88 on ARM64. Missing x86 ownership removed otherwise supported targets through global value pruning.
`RtlCaptureContext2` also lacks ARM64 evidence. `sdk_import_gaps` reproduces this with
`BSTR_UserSize64`: x86's pinned archive lacks that export, while x64 and ARM64 match the native
symbol.

This is an import-publication boundary, not permission to invent x86 linkage or infer target absence.
The maintainer approved retaining independently validated targets and reporting unproven imports
per target. Projection separates import resolution from callable-contract validation. An unmapped
target's native and public contracts still undergo the same checks; only missing ownership is
target-local. Invalid mappings, conflicting aliases, native disagreements and other unsupported
contracts retain their global/fatal rules. Provisional missing-import messages are discarded if
another contract failure removes the declaration, rather than reporting stale target outcomes.

Four focused tests cover target-local functions and aliases, all-target missing evidence, invalid
mappings and a volatile x86 parameter hidden behind missing ownership. The SDK fixture preserves
x64/ARM64 `BSTR_UserSize64` in `sdk_unresolved_imports.rdl` and checks target masks through both
WinMD round trips. The existing `sdk_import_gaps.rdl` remains the separate compiler-stdcall fixture.
Ten consumer tests pass, including named 64-bit PE imports, their absence on x86, and real 64-bit
DNS calls. BSTR marshaling is not executed. Bindgen's `#[in]` conversion still produces const raw
pointers despite `NativeInput(ReadOnly = false)`; that input-contract refinement remains separate.

Full regeneration restores all 177 required pairs without changing their baseline linkage.
Nine other names gain eleven non-baseline target pairs, including x86 RPC helpers and
compiler-proven aliases. Every added import has matching current SDK evidence. No retained RDL
declaration, owner or target changes or disappears; constants and configuration are identical.
The generic comparison passes. `verify-target-imports.py` also checks exact per-target coverage,
all added DLL/export evidence, unchanged prior linkage differences and no new omission roots.

Nine partitions gain 188 target declarations (91 x64, 89 ARM64, eight x86). Totals are 10,283
records, 4,532 enums, 13,539 aliases, 93,085 constants, 16,183 functions, 4,260 interfaces and
790 partitions. There are 73,012 diagnostics because unresolved imports now name each affected
target; no omission-root identity is added. Required losses are 424 target pairs across 204 names:
90 unresolved import pairs and 334 capture/dependency/projection pairs. Each of the 89 recovered
ownership names still has at least one unresolved target, so the incomplete-name count is unchanged.
Four existing `Rtl` provider differences and the other acceptance gates remain open.

Uncontrolled observations are 300.565 seconds capture/merge/project, 6.056 seconds compilation,
308.591 seconds driver wall and 7,600,218,112 sampled peak process bytes. SDK checks ran concurrently;
this is not a controlled budget comparison. The preserved `target-import-after.zip` has SHA-256
`1A532157EB60602A846ECFF9ADFBE3D046458240378DC1D90A5D435A92EFF1B9`, with matching configuration,
omission and coverage controls. Final parser/driver suites, all five selected SDK driver tests,
warnings-denied clippy, no-default-features checks and formatting pass. No source changes follow
the full generation other than documentation.

The maintainer supplied `D:\clang-ground.md` against `58ee55b4d3`. The field-buffer change above
finishes the approved in-flight contract. Further annotation batches pause for the acceptance
work already required by `plan.md`; no new rewrite or reporting framework is proposed.

The table below records review-time evidence. The named-linkage source and per-function ABI
mechanism are addressed above; the remaining required surface and other acceptance gates are open.

| Priority | Review-time concern | Required disposition |
| --- | --- | --- |
| 1 | Named export policy has 42 tuples. Current import diagnostics contain 5,593 unreviewed ordinal target records, or 1,865 provider/ordinal pairs; Winsock has 38 per target. | Replace incremental per-name expansion with a reviewed general source of named-export evidence. Preserve provider, target, raw symbol and ordinal identity; do not infer public names by undecoration. |
| 1 | Target-aware frozen-baseline comparison finds 14,121 library-backed function names versus 12,330 candidate names: 1,831 missing, 40 added, and 5,480 missing name/target pairs. `htons`, `select`, and `WSAGetLastError` are absent on every target. | Restore required APIs or obtain explicit corrections/exclusions. A previous-candidate comparison is a local regression control, not an acceptance inventory. |
| 1 | `sdk_import_gaps` still reproduces native C versus decorated x86 stdcall imports and missing x86 entries. The historical 94/88 split has not been freshly reclassified here. `--x86-stdcall` remains available. | Investigate per-function compiler/import ABI evidence; do not change callback defaults. Remove the abandoned global switch without removing the useful negative fixture. |
| 2 | SDK fixtures remain ignored by the default workflow. There are 61 ignore attributes in `test_clang2`, including intentional unsupported/compiler reproducers. | Select passing SDK-backed acceptance fixtures explicitly for CI; do not enable every ignored test indiscriminately. |
| 2 | Golden tests are checked in, but workload controls and semantic comparison scripts still depend on session directories. | Provide a checked-in, clean-checkout acceptance command using pinned inputs and frozen-baseline accounting, reusing existing readers rather than maintaining session-only parsers. |
| 2 | GUID source policy has `guid_definition_scope` and `sdk_guid_definitions` coverage, but the supported compilation contexts and remaining required PCH losses are not accepted. | Review the supported contexts and actual residual required losses together. Neither the old 460-GUID figure nor successful selected GUID capture settles this gate. |
| 2 | Typed annotation repairs preserve more semantics, but required consumer contracts are not selected; namespace/external WinRT references remain absent. | Select representative required contracts and the external-type-reference boundary. Retained SAL strings and metadata round trips are not end-to-end acceptance. |
| 3 | Historical reader probes exist, but generator/release compatibility and the core unaligned-vptr contract remain unresolved. | Complete the separate format migration and runtime decisions; do not infer approval from parser fixtures. |
| 3 | Constant accounting reports 10,491 new names. Scope/additions policy and controlled resource budgets remain unapproved. | Review additions and exclusions as public API choices, then measure the agreed workload against the legacy control on a quiet machine. |
| 3 | Before this disposition, the crate page was 1,488 lines, `breadth.rs` exceeded 7,200, and instructions were 650. | Consolidate by contract, move detailed working history out of permanent design docs, and keep independently reviewable checkpoints. Relocate/remove root planning files only when their handoff purpose is finished. |

Two numerical/coverage corrections matter. The review's roughly 1,458 missing functions understates
the current target-aware inventory. The initial adjacent-`library` regex also overstated the result
as 1,941 because other attributes can intervene before `extern fn`; the declaration-aware count
above is the one to use. Ordinal import records count archive evidence, not necessarily selected
declarations, so 1,865 is not a missing-API count.

The workflow excludes clang crates only for the x86 process job, which the native runtime does not
support. The x64 and ARM64 jobs provision libclang and run nonignored tests, including cross-target
fixtures. The actual gap is selected ignored SDK coverage, not an absence of all clang CI.
GUID input policy likewise has bounded tests; its gap is accepted semantics and required-loss
accounting, not a total absence of testing.

`ground-review.json`, the missing-function list, and import-root list record this check in the
current session. They remain recovery evidence, not the clean-checkout solution. No API exclusion,
general named-export source, format migration, runtime expansion, or production cutover is approved
by this review. The next implementation priority is import evidence and frozen-surface losses,
not another residual annotation family.

### Deferred input-contract refinement

The maintainer requested revisiting `NativeInput(ReadOnly = true/false)`. The boolean distinguishes
SAL's additional pointee-read-access annotation from RPC input validity; false does not mean
writable. Its name obscures that distinction, and blanket emission includes scalar and handle
parameters. Bindgen does not consume this attribute.

Before refining the vocabulary, specify the consumer-visible guarantees, distinguish them from
existing input/optionality flags and native qualifiers, and identify the owners where an additional
contract is meaningful. Preserve original native evidence while deciding what belongs in canonical
metadata. Do not equate fewer `NativeSal` payloads with improved consumer semantics or merely rename
the boolean. This refinement is deferred behind import evidence and required API losses.
