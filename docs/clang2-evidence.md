# Clang replacement evidence

This page records the current acceptance evidence, not replacement readiness. The
[acceptance plan and D01-D28 register](clang2-acceptance.md) own scope and approvals.
[windows-clang2](crates/windows-clang2.md) describes implemented contracts and repository-owned
reproduction commands. Implementation approval is not final consumer or cutover acceptance.

## Controls and current accounting

The current candidate includes D27-D28. Their comparison control is `structure-after`, with the
same header selection and compiler configuration.

| Control | Identity |
| --- | --- |
| Frozen comparison baseline | `6e68e56081340eaa932f4dbc92ec54c56ed51473` |
| Inspected-master audit | `afcb0ab1f8cf7a71ec563f659451748b26e7f5b6` |
| Working source | `6b48809df2` plus uncommitted implementation changes |
| Candidate | D27-D28 constant identities and declaration-only macro publication |
| Native toolchain | LLVM 22.1.8, companion ABI 19, SDK 10.0.28000.2270 |
| Workload | 372 selected headers, seven translation units, x86/x64/ARM64 |

The merge incorporates master `b95e10774b`. Its Win32 RDL is byte-identical to the inspected-master
control. The requested reset changed committed output, not the semantic comparison pin: candidate
`metadata/win32` remains visible and uncommitted. Re-pinning requires explicit approval and
revalidation of the availability/provider corrections.

| Current output | Count |
| --- | ---: |
| Records | 10,294 |
| Enums | 4,537 |
| Aliases, including callbacks | 13,528 |
| Constants | 88,454 |
| Functions | 14,251 |
| Interfaces | 4,287 |
| Partitions | 675 |
| Omission diagnostics, including dependents | 29,562 |
| Function-target pairs with unchanged linkage | 42,068 |
| Approved provider corrections | 4 |
| Raw missing function-target pairs | 289 |
| Approved availability corrections | 91 |
| Unapproved required missing pairs | 198 across 70 names |
| Additional function-target pairs | 421 native, 4 compatibility aliases |
| Missing constant names in either control | 15 |

These are presence/linkage and inventory measures, not signature or behavior parity. Moved constant
ownership is not absence; new names do not offset required losses. Unselected macros do not appear
in omission counts, and multiline diagnostic text must not be counted as separate omissions. Refresh
the comparison reports only after successful generation.

## Scope decisions and measured deltas

| Decisions | Evidence retained by the current candidate | Qualification limit |
| --- | --- | --- |
| D02-D03 | Frozen named linkage joins exact compiler symbols, SDK machine/kind/provider records and compiler-derived x86 alternatives. The per-function change recovered 94 functions without changing retained callbacks or signatures. | No decoration parsing, global stdcall default or ordinal consumer expansion. Each restored family still needs consumer qualification. |
| D04-D05 | Target-local import publication preserves validated variants. Checked files own 91 availability and four provider corrections; stale entries fail comparison. | Missing evidence is not target absence. Both missing `RtlCaptureContext2` pairs remain required. |
| D06 | Explicit target/name selection retains 130 compatibility pairs across 46 names and removes 6,302 unapproved macro-alias pairs without required losses or retained linkage changes. | Four x86 Get/SetWindowLongPtr pairs extend the frozen metadata but match existing generator compatibility APIs. No future aliases or targets are implicitly approved. |
| D07-D10 | Native typedef agreement, inherited slots, fixed-width storage and authoritative WinRT ownership have focused native/RDL/WinMD and cross-target consumer fixtures. | Native agreement does not settle Rust typedef usability, general C++ ABI, or external by-value/inheritance support. |
| D11 | Source-owned root selection removes 5,119 WinRT name strings and an unused support record; retained partitions and required coverage stay unchanged. | Required support dependencies retain their actual owners. No symbol-prefix filter. |
| D12 | Direct scoped single/double-NUL properties preserve entry/success phases. Seven SDK partitions change without coverage changes. | Residual scopes remain evidence, and #5042 still needs a consuming reader/generator contract. |
| D13 | Field alignment recovers CLFS scan APIs. Nested and multidimensional-array traversal includes external nested types without suppressing traits on pointer-only owners. | Aligned storage omits Debug/equality. Packed combinations and by-value calling ABI remain unqualified. |
| D14-D15 | Native reference ABI and query-buffer selection are corrected. Isolated rich/raw packages compile on three targets; five generators preserve 3,035 recorded source/configuration hashes at the alias-policy control. | Matched candidate routing is required. Production default metadata is not replaced. |
| D17 | Automatic macro discovery excludes three SAL support headers, removing only six internal constants and three partitions. All 674 retained partitions are identical. | Compilation, explicit selections and required `stdint` aliases remain. |
| D18 | Scalar/enum-only input-attribute omission removes 15,296 attributes in 280 partitions, retaining 28,322. Other RDL, coverage and omissions are unchanged. | Pointer-backed handles, pointers, references, COM objects and records retain their input attributes and native evidence. |
| D19-D20 | Lean definition inputs recover four storage/smart-card GUIDs with baseline values. Seven before/after driver configurations are identical after context extraction. | Remaining GUIDs/PCH policy and production WinRT-before-Win32 ordering stay open. |
| D21 | `package_acceptance` uses checked-in routing, filters, dependencies and lints without session files. | Hosted/clean-checkout execution, native ARM64, paired releases and budgets are not established by local compilation. |
| D22 | Actual SDK constants for #4977 retain signed native types despite unsigned API parameters. | Rust convenience typing is separate work; no retagging or issue closure. |
| D23 | `UIAutomationCoreApi.h` adds 98 imports on all targets, 397 constants, 17 records, six enums and six aliases. All preceding partitions and coverage rows stay identical. | Nine provider/lookup APIs have native import/signature coverage. Generated-Rust/live-provider qualification remains open. |
| D24 | `ProcessPrng` stays outside SDK capture; a declaration/import supplement would be separate authored policy. | Documentation or DLL exports cannot manufacture compiler evidence. #5107 is not implemented or closed. |
| D25 | Publication phases, record storage and native record/value checks have separate owners. All 675 partitions, configuration, coverage and provenance match the UIA control; normalized omissions are unchanged. | The extraction changes no contract, exclusion or scope. |
| D26 | The 51 native names / 127 pairs preceding UIA are accepted as candidate scope. | Observed names, targets and providers only; not final ABI/consumer qualification. |
| D27-D28 | Native/macro identity separation recovers three GUIDs without losing retained declarations. GUID_NULL and IID_IResourceManager move from foundation to their macro-owning headers. | Independently published macros do not define native objects. Declaration-only native objects retain explicit missing-definition diagnostics. |

D26's exact set is the non-UIA `new`, `native-declaration` rows in the `structure-after` function
report. Family labels do not approve future members:

| Native additions | Target pairs | Provider |
| --- | ---: | --- |
| 27 K32 PSAPI names | 81 | kernel32.dll |
| LsnEqual, LsnGreater, LsnIncrement, LsnInvalid, LsnLess, LsnNull | 18 | clfsw32.dll |
| SystemFunction036, SystemFunction040, SystemFunction041 | 9 | advapi32.dll |
| CMP_WaitNoPendingInstallEvents | 3 | setupapi.dll |
| UrlIsA | 3 | shlwapi.dll |
| I_RpcConnectionInqSockBuffSize, I_RpcConnectionSetSockBuffSize, NdrAsyncClientCall2, NdrClientCall, NdrClientCall4, NdrDcomAsyncClientCall2, NdrMesProcEncodeDecode4, NdrServerCall, NdrServerMarshall, NdrServerUnmarshall, NdrStubCall | 11, x86 only | rpcrt4.dll |
| MapDebugInformation, UnmapDebugInformation | 2, x86 only | dbghelp.dll |

The comparison CSV intentionally keeps raw status and provenance separate from this scope approval.
Its summary states that scope approvals are recorded separately and consumer qualification remains
required; it does not infer approvals from provider names or silently change CSV semantics.

## Required omissions and policy boundaries

The current 15 missing constant names are:

| Owner | Missing names |
| --- | --- |
| wincon | CONSOLE_REAL_INPUT_HANDLE, CONSOLE_REAL_OUTPUT_HANDLE |
| d3dkmdt | D3DKMDT_ROTATION_SUPPORT_MASK, D3DKMDT_SCALING_SUPPORT_MASK |
| d3dcompiler | D3D_COMPILE_STANDARD_FILE_INCLUDE |
| wsdxmldom | ElementType, TextType |
| wingdi | HGDI_ERROR |
| http | HTTP_VERSION |
| mq | MQ_MTS_TRANSACTION, MQ_SINGLE_MESSAGE, MQ_XA_TRANSACTION |
| ntddndis | NDIS_WMI_OUTPUT_INFO_REVISION_1 |
| wtsapi32 | WTS_SECURITY_CURRENT_USER_ACCESS |
| wmcodecdsp | g_wszSpeechFormatCaps |

Remaining required function losses include volatile storage, multiple-vptr interfaces,
packed/aligned records, variadic callbacks and unresolved import evidence. Examples include
`WaitOnAddress`, VSS/DRM/WSD families, `MiniDumpWriteDump`, PDH raw-counter APIs,
`ldap_set_dbg_routine`, and both missing `RtlCaptureContext2` pairs. The generated function report
owns the exact target/name inventory. These contracts are not approved exclusions.

WIC requires native constant precedence rather than active macro substitution.
`wincodec.h` declares the unsuffixed CLSIDs with version-1 values, then defines same-named macros
referring to version 2 under the selected SDK context. The frozen baseline publishes the native
version-1 values; publishing the active macros would change those two existing constants. Both
suffixed versions are already present in the candidate. `GUID_CLASS_INPUT` similarly has a native
declaration in poclass and a macro in hidclass, but both have the same GUID value. The current macro
probes fail to evaluate references whose definitions are in another input. Do not treat restoring
the native declarations and publishing macro aliases as the same policy, or silently change the WIC
values to improve coverage.

D27 extends the shared native/macro identity distinction to constants in reconciliation,
cross-target association, pruning and publication. Failed probes cannot remove native constants.
Strict public collisions fail; partial output retains defined native constants with explicit macro
omissions, including equal-value collisions. A macro cannot supply a missing native definition.
Native conflicts and conflicts between macro observations remain fatal before pruning.

Focused fixtures reproduce the failed native/macro comparison and cover separate definition inputs,
input/target order and both metadata round trips. The SDK fixture preserves both WIC version-1
values and GUID_CLASS_INPUT without evaluating aliases against guessed definitions or changing
the SDK context.

Declaration-only coexistence is distinct from WIC's defined native constants.
`Transact.h` declares `extern` IID objects, then defines same-named `__uuidof` macros in its C++
branch. `ks.h` similarly defines GUID_NULL through an explicit UUID-tag macro. The macro values
are captured independently. Publishing an independently evaluated macro is not evidence of a
native object definition.

D28 permits partial publication of the independently captured macro beside a declaration-only
native object, with a missing-native-definition diagnostic. The declaration remains native
evidence through subsequent pruning rounds; a macro is not substituted into its observations.
Defined native constants retain precedence. Failed native definitions and unpublishable defined
constants still block same-named macros. The scalar fixtures cover both failure boundaries and
input order; the SDK fixture checks GUID_NULL and IID_ITransaction through both metadata round
trips on all targets.

The full comparison against `structure-after` adds only GUID_CLASS_INPUT and the two unsuffixed
WIC CLSIDs, retaining their native values. GUID_NULL moves from foundation to ks and
IID_IResourceManager from foundation to txdtc because their published values belong to macros.
All retained declaration text is unchanged, including types, values and architecture attributes.
Five partitions change; no partition is added or removed. Capture configuration and function
coverage CSV are byte-identical to the control. All 63 GUIDs affected by the initial collision
rule remain present. The missing constant count falls from 18 to 15 in both comparison controls;
additional omission diagnostics include the retained declaration-only native objects.

The PCH reproducer remains a separate compiler defect. Explicit GUID capture succeeds where PCH
discovery reports incomplete `_GUID`, even with a complete source definition. A native diagnostic
probe using `ExternalASTSource::CompleteRedeclChain` restores the compiler-owned GUID fields on all
targets and still rejects a genuinely incomplete type. No suitable pre-parse hook is established.
Only a small companion adjustment is in scope: no libclang rebuild, parser duplication, guessed
GUIDs or per-failure source retries. The retry prototype exceeded 515.9 seconds without completing
capture versus 338.8 seconds without confirmation and was removed. These are historical
observations, not budget qualification. The historical 460 ksmedia GUID figure is not the current
loss inventory.

Definition inputs retain native provenance and reconciliation. Olectl's dependency preloading and
split `DEFINE_GUID`/`INITGUID` state, topology's `_KS_` context and selected lean GUID includes must
be re-evaluated together at acceptance. Recovering required values through independent definitions
does not close the compiler bug or approve additional constants. Separately declared IID/CLSID
objects cannot borrow a tag's UUID without their own definition evidence.

`CFunctionDiscoveryNotificationWrapper` is a C++ implementation helper with method bodies. Its
baseline empty `no_guid` interface does not establish that restoring it is correct. Likewise,
`__RPC_unique_pointer` and `__RPC_full_pointer` are phase-independent typedef properties, not
blanket optional call arguments. Both need semantic dispositions, not count-driven recovery.

## Native imports and platform evidence

`sdk_import_availability` verifies 89 declarations and their exact symbols on every target. The
original 90 missing pairs comprise 68 x86 `User*64` marshaling pairs, 14 x86 UMS pairs, six x86 RPC
pairs and x86/ARM64 `RtlCaptureContext2`. The separate NDR fixture adds three approved x86
corrections, using `NdrClientCall2` as an all-target control.

On Windows 10.0.26709.0 with inspected DLL version 10.0.26709.1000, the x86 loader finds the
original 89 exports absent and x64 finds them present. The three extra NDR exports are absent on
x86 and present on x64. These diagnostics resolve exports without invoking the APIs.
The ARM64 archive contains `RtlCaptureContext2` only as ARM64EC, which is not native ARM64 evidence.
[MIDL protocol restrictions][midl-protocol] and [UMS support][ums-support] distinguish a working
feature from an exported stub; ARM64 UMS imports remain evidence despite unsupported functionality.

The approved provider transitions are x86 `RtlCompareMemory`, `RtlRaiseException` and
`RtlRestoreContext`, plus ARM64 `RtlRaiseException`, from kernel32.dll to ntdll.dll. Native-machine
SDK imports establish each transition with unchanged compiler ABI/export name. The x86 loader
likewise finds these exports only in ntdll; x64 finds them in both providers. Native ARM64 behavior
and older Windows versions remain unmeasured.

Named-export consumers cover Winsock, Active Directory helpers, context capture, DHCP/DNS,
allocation, PSAPI, variadics and selected other imports. Their no-std PE probe avoids accidentally
measuring std's own Winsock import library, which can select ordinals independently of generated
bindings. Separate ordinary consumers retain std. x86/x64 execution and three-target compile/link
checks do not qualify every restored API or every supported OS.

## Consumer and metadata compatibility

Updated, paired readers and generators are approved; old consumers are not a compatibility target.
Actual historical builds against the same current fixture WinMD establish these boundaries:

| Reader/generator | Mixed pointer qualifiers | Typed bitfield MemberType |
| --- | --- | --- |
| `51eac0a953` | Rejects modifier code 31; seven field signatures fail decoding | Rejects attribute type code 80 |
| Frozen `6e68e56081` | Decodes and emits pointer bindings identically to current tools | Rejects attribute type code 80 |
| Current matched tools | Preserves each pointer level | Preserves aliases, enums, bool and target-specific types |

Mixed-pointer support entered at `cf1806f01d`; typed MemberType support entered at `b742a59a59`.
Equal 0.100.0 development version strings do not establish compatibility. The fixture WinMD hashes
used for these probes are `BDC8447791A249C447C5CC738795DC391CC202C98805D6B253CDF8A7AF137439` (mixed
pointers) and `6A8F44E32856CFD7302DC48E6B5D42821541ABBBB383832D41C1EE04B6896E44` (typed bitfields).

Other measured silent-misprojection boundaries must also be qualified:

| Contract | Evidence and acceptance requirement |
| --- | --- |
| Inherited override slots | Frozen x64 generation emits AVI vtables of 136/104/64 bytes instead of 112/80/40 and stream Info at offset 56 instead of 32. Updated RDL clears NewSlot; bindgen validates and reuses inherited entries. |
| Fixed 64-bit pointers | Frozen generation accepts the modifier but emits `PVOID64 = *mut c_void`; its x86 size/alignment both fail. The union's integer member can hide the field defect. |
| Native references | Old native callable lowering can emit reference arguments by value. Native-backed import/callback/COM producer and consumer fixtures check pointer ABI and mutable storage. WinRT byrefs remain separate. |
| Member alignment | Older generators ignore field-owned alignment. Updated storage preserves public field types with target-evaluated uninitialized padding; nested/array owners omit Debug/equality. |
| Conditional initialized length | A reader must honor `NullFallbackParamIndex` before dereferencing an optional output count. |
| Scoped termination | Readers must preserve double-NUL entry/success properties and residual qualifier ownership, not merely accept the attribute. |

The shared `windows-core::Interface::assume_vtable` change remains an independent runtime decision.
Synthetic packed-object tests exercise calls, query, clone/drop and weak-reference forwarding on
x86/x64, with ARM64 compilation. The object-vptr load is unaligned; the vtable itself must be
aligned. Packed SDK declarations do not prove that Windows returns unaligned objects.

Separate consumer obligations remain: GUID-less IUnknown-derived interfaces currently receive a zero
IID from bindgen; high-level variadics reject explicitly; legacy untyped union bitfield accessors
have their own unsafe-read defect. Raw storage or compilation success does not qualify these
behaviors. `NativeInput(ReadOnly = false)` is not a writable guarantee, and bindgen does not consume
that distinction. Further vocabulary refinement requires concrete consumer semantics.

The bitfield consumer repair handles wrapped/enumerated and pointer-sized backing storage and
target-varying aliases, retaining bounds failures rather than truncating fields. The old package
control's x86 `PSAPI_WORKING_SET_BLOCK.VirtualPage` incorrectly spans 52 bits; the native fixture
establishes 20 x86 bits and 52 x64/ARM64 bits at offset 12. Current candidate metadata preserves
that distinction. This is independent of required API and paired-release acceptance.

## Issue-linked evidence

| Issue/case | Checked boundary | Still not claimed |
| --- | --- | --- |
| #4943 | Source-owned roots exclude resource/UCRT/MSVC declarations unless required or explicit; synthetic probes cannot become roots. | Removal or renaming of required CRT dependencies. |
| #5047 | Native flag-enum attributes retain backing types, negative values and conflict checks. | Flags inferred from values. |
| #4932 | Public data classes, anonymous unions and intended x86 COM convention. | Constructors/destructors or general C++ lifetime support. |
| #5062 | Private template members cannot suppress explicitly selected unrelated roots. | General template publication. |
| #4935 | Nine non-finite cases remain omissions; finite controls retain exact bits. | Non-finite metadata support. |
| #4982 / #4995 | Identical C redeclarations merge; native/macro identities remain separate, and conflicts fail. | Public alias substitution that erases native signatures. |
| #5042 | Real SDK single/double-NUL phase ownership and both metadata round trips. | A qualified consuming API or issue closure. |
| #4977 / #4967 | Native signed constants and bounded field-aligned storage have dedicated fixtures. | Retagged constants, packed combinations, by-value ABI or issue closure. |
| #5083 / #5084 | PdhMsg's 88 baseline status constants match inspected master; `S_PDH` is separately observed. | Automatic scope approval for the extra GUID. |
| #5081 | Pinned SDK and current metadata both give SOMAXCONN = 2147483647. | Behavior of every published crate version. |
| #5111 | Approved UIA header expansion and native provider/API-ID fixture. | Live provider or generated-Rust qualification. |
| #5107 | Separate authored-policy boundary for ProcessPrng. | SDK declaration capture or API implementation. |
| SDK acceptance | SAPI, IWTS, DOS-header layout, NtQuerySystemInformation, memorybuffer/robuffer and ten composition interop interfaces. | Missing APIs waived by similar passing fixtures. |

Legacy fixtures remain. Ported cases distinguish fixture setup (imports, macros, linkage and x86
conventions) from semantic failures. Untyped nested enum fields remain unsupported; the original
data-class and private-template fixtures do not justify broader C++ support.

## Reproduction and delivery status

Run the [repository commands](crates/windows-clang2.md#validation) against matching source,
candidate RDL and authoritative WinRT metadata. `package_acceptance` compiles current Win32/WDK
inputs, uses actual package routing and rich/raw filters, and checks all features with warnings
denied on x86/x64/ARM64. The current D27-D28 candidate, including the UIA additions and macro-owned
GUID relocations, passes this package check on all three targets. This is local compile
qualification, not live UIA-provider behavior or native ARM64 execution.

The frozen package control needs its approved `WM_CTLCOLOR` source supplement:
`crates/tools/win32-2/src/baseline/windowsx.rdl`. The compiled baseline contains this shipped API,
but the frozen RDL routing inputs omit it. The supplement preserves its existing i32 value 25
without a fallback namespace or name-specific routing. Production `tool-package` must not combine
partial candidate routes with a different committed WinMD.

CI selects SDK policy/metadata cases, cross-target consumers, package acceptance and named-export
snapshot regeneration. Local runs do not establish hosted execution or a fresh clean checkout.
Native ARM64 execution, release pins, final independent review and maintainer sign-off remain open.
No issues are closed and no production callers are switched by this evidence.

Final scope and performance limits remain proposals: no slower than a matched legacy run,
provisionally <=480 seconds and <=8 GB peak process-tree working set. Ask for a quiet machine before
measuring. Historical concurrent timings and single-process samples are not budget results. Retain
machine/configuration/toolchain identity and distinguish warm dependency state.

Older per-iteration logs and archives remain recovery evidence in Copilot sessions
`1810274b-2842-4eda-8537-79e6ac99f61c`, `e8bd0450-292d-42c8-a2b2-45d86a4711e5` and
`74520e25-3a04-448f-bca8-a0140634a829`. They are not inputs to the repository acceptance commands.
Current decisions and outstanding obligations live in these maintained pages, not those sessions.

[midl-protocol]: https://learn.microsoft.com/en-us/windows/win32/midl/-protocol
[ums-support]: https://learn.microsoft.com/en-us/windows/win32/procthread/user-mode-scheduling
