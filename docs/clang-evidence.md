# Clang replacement evidence

This page records the current acceptance evidence, not replacement readiness. The
[acceptance plan and decision register](clang-acceptance.md) own scope and approvals.
[windows-clang](crates/windows-clang.md) describes implemented contracts and repository-owned
reproduction commands. Implementation approval is not final consumer or cutover acceptance.
The active work is downstream repair and generation qualification against the
[canonical attribute policy](clang-attributes.md). Local checks do not settle public-scope or
delivery decisions.

The D33 investigation accounts for 614 direct annotations. D33 normalizes 556 to bounded typed
predicates, retaining 58 as evidence. Shared SDK declarations explain the predicates on `PNTSTATUS`
and `PNDIS_STATUS`; they are not grounds for a pointer-success interpretation. Coverage is unchanged.

## Controls and current accounting

The current candidate includes D27-D28, D29 canonical spelling improvements, D30 input normalization
and D31 native-only exception specifications, plus D32 result use, D33 success predicates and
D34 direction evidence, D35 explicit RDL directions, D36 named buffer extents and D37 returned
buffers with optional counts.
Subsequent header/RDL slices include returned strings/nullability, arithmetic counts and integer
ranges. Their completed candidate contains 1,905 residual SAL occurrences, 8,366 buffer extents
and 265 ranges. SDK capture, publication and WinMD compilation complete with the declaration totals
below; omission diagnostics and frozen constant/function coverage are unchanged.

MIDL-only directions add 10,586 `in` and 1,234 `out` attributes across 143 partitions. All other RDL
text is identical to the preceding candidate. The `midl_comments` and `sdk_midl_inputs` fixtures
cover direction flags, native agreement and `IBindStatusCallback::GetBindInfo` on all three targets.
D38 unifies new count arithmetic. Exactly 81 nonzero offsets per target become typed expressions;
all other attribute payloads remain unchanged on x86, x64 and ARM64. Five RDL partitions change
spelling: optional arithmetic uses `if_present(*count) + 1`, and field additions drop the redundant
`i32` suffix. Declaration totals, byte-for-byte omissions and frozen coverage reports are unchanged.
Legacy Offset payloads retain their full attribute spelling and exact values through round trips.
This completes the bounded arithmetic polish, not publication-policy or replacement acceptance.

D27-D28 use
`structure-after` as their comparison control. D29 compares against the completed D27-D28 candidate,
with the same header selection and compiler configuration.

| Control | Identity |
| --- | --- |
| Regeneration comparison | Canonical `metadata/win32`, `metadata/wdk` and combined WinMD |
| Historical source review control | `d0a38039eb` |
| Latest independent review | `d0a38039eb` |
| Candidate | D27-D37 plus returned-string/nullability, count/range, MIDL-direction, D38 arithmetic harmonization and termination owner/storage validation |
| Native toolchain | LLVM 22.1.8, companion ABI 19, SDK 10.0.28000.2270 |
| Workload | 372 selected headers, seven translation units, x86/x64/ARM64 |

The migration measurements below record earlier comparisons, not generation prerequisites.
Current review uses the canonical metadata diff and native fixtures rather than frozen snapshots
or separate approval files.

| Current output | Count |
| --- | ---: |
| Records | 10,294 |
| Enums | 4,537 |
| Aliases, including callbacks | 13,528 |
| Constants | 88,454 |
| Functions | 14,251 |
| Interfaces | 4,287 |
| Partitions | 675 |
| Omission diagnostics, including dependents | 29,530 |
| Function-target pairs with unchanged linkage | 42,068 |
| Approved provider corrections | 4 |
| Raw missing function-target pairs | 289 |
| Approved availability corrections | 91 |
| Unapproved required missing pairs | 198 across 70 names |
| Additional function-target pairs | 421 native, 4 compatibility aliases |
| Missing constant names in either control | 15 |

The declaration and omission counts come from full canonical regeneration. Linkage and missing-pair
counts are historical review evidence, not a fresh comparison or an approved exclusion.
These are presence/linkage and inventory measures, not signature or behavior parity. Moved constant
ownership is not absence; new names do not offset required losses. Unselected macros do not appear
in omission counts, and multiline diagnostic text must not be counted as separate omissions. Refresh
required-surface accounting only after successful generation.

## Current review disposition

The independent review of `d0a38039eb` confirms the direction, buffer, success and range contracts.
Its duplicate count-encoding finding is addressed by D38. Termination validation uses the existing
single/double-NUL family rather than extending the vocabulary. The proposed publication choices
and cutover sequence are not approvals; downstream obligations remain deferred, not waived.

| Finding | Verified status | Required action |
| --- | --- | --- |
| Test-suite health | The focused Clang, bindgen/RDL and Reactor library results below remain valid. A keep-going x64 workspace check reports failures in 26 packages, including the `test_no_std` workspace feature-unification failure; that crate passes alone. The other failures include tests, samples and Reactor functional coverage. | Repair workspace integration before claiming branch health. Package compilation does not cover handwritten consumers or WinRT authoring. |
| WinRT authored directions | RDL rejects missing or combined WinRT argument directions at the source location. Bindgen independently rejects these states, missing argument rows and malformed WinRT parameter sequences. Eight interop fixtures now declare their intended input/output contracts; all 24 affected Rust, C++/WinRT and C#/WinRT cases pass. | Keep Win32 unspecified direction intact. Do not infer flags from types or change implementations to output signatures merely to make them compile. |
| Full package acceptance | `candidate_packages` passes rich/raw compilation on x86, x64 and ARM64. Canonical `POOL_TYPE` retains enum values 0 and 4 and ARM64 macro values 512 and 516. Bindgen emits the explicit constants on ARM64 and the enum-derived constants on x86/x64. | This resolves Rust name projection without changing native capture, canonical publication or omissions. Native ARM64 execution and hosted qualification remain separate gates. |
| Direct termination owner/type validation | Corrected. Short termination syntax requires parameter-owned pointer, reference or array storage at the selected indirection. Alias resolution and storage rules are checked by both reader and writer. Scalars, callback values, fields, return rows and declarations are rejected in short form; explicit full metadata remains lossless. | Keep single/double-NUL phases in the existing attribute family. `null_terminated` covers storage, owner and full-metadata round trips; `scoped_termination` covers native contracts and #5042's `WCHAR*` case on all targets. |
| Unknown direction becomes asserted direction | Corrected under D34-D35. Known directions are explicit; missing direction has no attribute and preserves absent flags. | Do not infer direction from types; retain raw-pointer, alias, COM and callback coverage. |
| Conditional direction | RegQueryValueExA/W now publish shared output/optional facts while retaining both conditions. The suggested unconditional direction union is not adopted. | Do not infer exhaustive conditions generally or drop the conditional input requirement. |
| Duplicate normalized/residual buffers | Addressed for the supported D36-D37 contracts. Capacity, initialized length, phases and returned storage share the extent model. Unsupported combinations retain evidence. | Do not discard residual SAL merely because part of a macro has a canonical representation. |
| Unspecified direction in bindgen | Raw-pointer, alias, COM and callback fixtures preserve absent direction without mutable-to-const or output-conversion guesses. Native named-import consumers pass. | Keep direction evidence separate from type/ownership handling; do not infer direction in clang to compensate. |
| COM argument ownership | Direct interface arguments retain borrowed references, including optional values and producer parameters without direction flags. The lifetime fixture verifies caller ownership and release; raw buffer restrictions remain in place. | Residual evidence still cannot authorize slice, query or retval conversion. BSTR ownership is unchanged. |
| Generation drift | Native generation and `tool-roundtrip` produce byte-identical WinRT, Win32 and WDK RDL. Shared attribute formatting, canonical variant sort keys and resolution-only writer references preserve generated spelling and ordering. | Workspace-build drift in the tracked robot WinMD files and full generator qualification remain open. Clean RDL roundtrips do not establish replacement readiness. |
| Direction-repair generation | Package, internal binding and Reactor regeneration introduce no generated changes. Composition regeneration refreshes retained native aliases in both stacks. WebView generation retains all five loader imports after nominal COM typedef validation; its 33 live host/browser fixtures pass with regenerated bindings. Canonical SDK/WDK output is unchanged. | WebView retains 69 explicit omissions for RPC proxy/stub import ownership, LIBID definitions and C++ helper operators. Full generator qualification remains open. |
| Proposed analysis-only removals | Not approved as a group. Pointer invalidation, allocation and execution-context annotations can carry useful contracts. Result inspection also has non-return owners. | Review meaning and ownership individually; lack of a consumer is not enough. SAL1 result checking remains a separate equivalence decision. |
| CI generation input | `gen.yml` runs full SDK/WDK generation and checks tracked metadata drift with a shallow checkout. | Consumer jobs use committed metadata. Full package qualification remains separate from successful generation. |
| Required missing APIs | The 198-pair/70-name total remains historical. Current metadata and diagnostics reconfirm WaitOnAddress, MiniDumpWriteDump, XAudio2Create, IsEqualGUID, CfRegisterSyncRoot and RoGetActivatableClassRegistration gaps. `RtlCaptureContext2` remains x64-only. Missing `ACCESS_STATE` also breaks the rich/raw layout tests. | Classify native support, public requirements and baseline correctness. Fix or obtain a justified disposition; do not treat zero differences as the goal. |
| Unrecorded partition losses | `activationregistration` and `windowsfoundation` are absent. The Arm64 `__FIMapView_2_HSTRING_IInspectable_t` alias fails on an unsupported `TemplateRef IMapView`; `IActivatableClassRegistration`, its pointer alias and the RoGet import are pruned as dependents. | Add a focused canonical metadata case and decide the required surface. ActivationType, RegistrationScope and RegisteredTrustLevel are also absent with this partition; this is not an approved exclusion. |
| UIA expansion | D23's 98 imports and 397 constants remain candidate scope. `CustomNavigationPattern_Navigate` still lacks configured import ownership. | Keep the unresolved import and acceptance boundary explicit. Defer downstream/live-provider qualification. |
| Driver size | `main.rs` remains about 1,650 lines; the review identifies `run` and `sdk_library_policy` as the next large responsibilities. | Defer an output-neutral split until the metadata model is settled. Moving SDK contexts into data did not eliminate their semantic review obligation. |
| Process-document noise | Historical review-response sections duplicated implemented work; session recovery references were not acceptance evidence. | The plan now has a current sequence and gate table. Retain the approval register, concise evidence and a focused attribute inventory instead of iteration narratives. |
| Single snapshot commit | The reviewed snapshot combines production bindgen behavior, parser refactoring, policy, scope and documentation. | Maintainer-owned landing work: separate production bindgen changes and their rationale from parser/policy changes. No automatic commit rewriting. |
| Native evidence publication | Input direction uses flags, and C++ defaults/exception specifications remain native-only. Seven unused seed definitions are retired. | Four qualified families remain: SAL, const methods, deprecation and base markers. SAL stays opaque; const-method and base publication remain open decisions. |

The Clang sweep used
`cargo test -p test_clang --no-fail-fast --quiet -- --include-ignored --skip candidate_packages`.
The separate bindgen/RDL sweep used `cargo test -p test_bindgen -p test_rdl --no-fail-fast --quiet`.

The readiness probes against `ca9093c65a` used the main CI package selection with
`cargo check --workspace --all-targets --target x86_64-pc-windows-msvc --keep-going`, excluding the
six system Composition packages listed in `test.yml`. This local check is not a hosted test run.
That probe found an isolated `test_interop` failure; the explicit-direction repair above resolves
it. `test_no_std` passes alone. The raw count of 26 failing packages is historical probe evidence,
not a count of independent or remaining defects.

The integration failures include fixed-width `LPARAM`/`WPARAM`/`SIZE_T` aliases versus handwritten
`isize`/`usize` arguments, mutable versus const null pointers, and explicit output/HRESULT
signatures versus callers expecting projected return values. Reactor's functional
`drag/source.rs` has these mismatches even though the Reactor library suite passes.

Clang publication and metadata writing use the same attribute formatter. It sorts full names
case-sensitively without changing argument order or the relative order of repeated attributes.
Clang's `projection::variants` and RDL's writer layout sort variants by canonical formatted text,
not producer-specific token order. `variant_order` covers architecture-specific import names and
signatures through native capture, RDL and WinMD.

`tool-roundtrip` supplies compilation references to the writer as resolution-only inputs.
Referenced typedefs support short termination spelling without leaking SDK declarations into WDK
partitions. The writer excludes reference rows by source-file identity, not by public name.
The `null_terminated` reference fixture covers colliding functions, constants, records and enums
in single-file, split and partitioned output. Conflicting aliases retain full attributes; ambiguous
enum definitions retain numeric attribute values. Native generation followed by roundtripping
leaves every canonical RDL file byte-identical.

`projection::reference_values::validate_value_references` compares unqualified, unannotated COM
typedefs through `interface_type`, not the by-value storage path. Nominal identity, UUID, base and
native layout checks remain required. The `native_reference_aliases` header/RDL fixture covers a
pointer-containing record and imported consumer on all three targets, along with by-value,
annotated-alias, UUID, base and slot rejection. Referenced definitions remain external through both
metadata roundtrips.

WebView generation retains all five loader imports and emits the native `EventRegistrationToken`
struct. Its 33 live host/browser fixtures pass. The remaining 69 omissions comprise 60 target-specific
RPC proxy/stub import-ownership diagnostics, two LIBID declarations without definitions, and seven
C++ flag operators. They are not interface-alias dependency cascades and remain explicit.

| Current missing surface | Verified diagnostic boundary |
| --- | --- |
| `WaitOnAddress` | Volatile/restrict parameter projection |
| `MiniDumpWriteDump` | Packed callback records containing aligned types |
| `CfRegisterSyncRoot` | Policy-wrapper records with C++ constructors |
| `RoGetActivatableClassRegistration` | External collection-template alias dependency |
| `CustomNavigationPattern_Navigate` | No configured DLL/export ownership |
| `RtlCaptureContext2` on x86/ARM64 | No configured DLL/export ownership; x64 is retained |
| `ACCESS_STATE` | WDK `UNICODE_STRING::Buffer` typedef contracts differ from referenced SDK metadata |

| Repaired regression | Evidence |
| --- | --- |
| Implicit-alias stack overflow | Reference-ownership traversal uses a visited type worklist. `implicit_aliases` covers the compiler typedef's explicit self-redeclaration, input ordering and conflicts. |
| Header/RDL expectations | Reviewed regeneration preserves declaration-order changes, explicit MIDL directions, absent unannotated directions, and supported returned-buffer/range lowering. |
| Certbcli dependencies | Discovered function aliases capture the native `CSEDB_RSTMAPW` and `HCSBC` dependencies. Their definitions match the pinned header; all 24 constants retain their values and encoding checks. |
| Metadata parameter rows | Buffer assertions select sequence 2 rather than positional iteration. Argument counts exclude sequence-zero return rows; `CreateEventW` separately checks its nullable return. |
| Namespace consumer | `IBufferByteAccess::Buffer` uses explicit output storage rather than converting its unresolved buffer contract to a return value. All three target builds pass, with native execution on supported host targets. |
| Enum/macro names | `enum_macro_collisions` retains native enum fields and explicit macro constants through metadata round trips. Bindgen gives explicit namespace constants precedence over flattened enum fields only on overlapping architectures. The cross-target style/filter fixture checks exact values, scoped members and unavailable targets. Canonical metadata and omission counts remain unchanged. |
| Reactor callers | `CoInitializeEx` receives mutable null storage and icon messages use `WPARAM`. Reactor and the bindgen fixtures compile for x86 and ARM64. |

The tracked `vadefs` partition is also absent, consistently with the approved compiler/CRT root
ownership boundary. It is not the same category as the unresolved activation-registration loss.
The review does not approve removing the remaining APIs, extending UIA scope, or changing
annotation semantics. Canonical metadata and native fixtures are the review inputs.

## Scope decisions and measured deltas

| Decisions | Evidence retained by the current candidate | Qualification limit |
| --- | --- | --- |
| D02-D03 | DLL ownership uses exact compiler symbols, SDK machine/kind/provider records and compiler-derived x86 alternatives. Ordinal-backed records publish the compiler-declared C name. | Named publication is not DLL export-table proof. No decoration parsing, global stdcall default or ordinal consumer expansion. Added APIs and restored families still need consumer qualification. |
| D04-D05 | Target-local import publication preserves validated variants. Native fixtures check pinned SDK availability and providers independently of historical snapshots. | Missing evidence is not target absence. Both missing `RtlCaptureContext2` pairs remain required. |
| D06 | Explicit target/name selection retains 130 compatibility pairs across 46 names and removes 6,302 unapproved macro-alias pairs without required losses or retained linkage changes. | Four x86 Get/SetWindowLongPtr pairs extend the frozen metadata but match existing generator compatibility APIs. No future aliases or targets are implicitly approved. |
| D07-D10 | Native typedef agreement, inherited slots, fixed-width storage and authoritative WinRT ownership have focused native/RDL/WinMD and cross-target consumer fixtures. | Native agreement does not settle Rust typedef usability, general C++ ABI, or external by-value/inheritance support. |
| D11 | Source-owned root selection removes 5,119 WinRT name strings and an unused support record; retained partitions and required coverage stay unchanged. | Required support dependencies retain their actual owners. No symbol-prefix filter. |
| D12 | Direct scoped single/double-NUL properties preserve entry/success phases. Seven SDK partitions change without coverage changes. | Residual scopes remain evidence, and #5042 still needs a consuming reader/generator contract. |
| D13 | Field alignment recovers CLFS scan APIs. Nested and multidimensional-array traversal includes external nested types without suppressing traits on pointer-only owners. | Aligned storage omits Debug/equality. Packed combinations and by-value calling ABI remain unqualified. |
| D14-D15 | Native reference ABI and query-buffer selection are corrected. Isolated rich/raw packages compile on three targets; five generators preserve 3,035 recorded source/configuration hashes at the alias-policy control. | Matched candidate routing remains required. Production default metadata is generated from the canonical SDK/WDK inputs. |
| D17 | Automatic macro discovery excludes three SAL support headers, removing only six internal constants and three partitions. All 674 retained partitions are identical. | Compilation, explicit selections and required `stdint` aliases remain. |
| D18 | Scalar/enum-only input-attribute omission removes 15,296 attributes in 280 partitions, retaining 28,322. Other RDL, coverage and omissions are unchanged. | Pointer-backed handles, pointers, references, COM objects and records retain their input attributes and native evidence. |
| D19-D20 | Lean definition inputs recover four storage/smart-card GUIDs with baseline values. Seven before/after driver configurations are identical after context extraction. | Remaining GUIDs/PCH policy and production WinRT-before-Win32 ordering stay open. |
| D21 | `package_acceptance` uses checked-in routing, filters, dependencies and lints against canonical candidate RDL. | Hosted/clean-checkout execution, native ARM64, paired releases and budgets remain open. |
| D22 | Actual SDK constants for #4977 retain signed native types despite unsigned API parameters. | Rust convenience typing is separate work; no retagging or issue closure. |
| D23 | `UIAutomationCoreApi.h` adds 98 imports on all targets, 397 constants, 17 records, six enums and six aliases. All preceding partitions and coverage rows stay identical. | Nine provider/lookup APIs have native import/signature coverage. Generated-Rust/live-provider qualification remains open. |
| D24 | `ProcessPrng` stays outside SDK capture; a declaration/import supplement would be separate authored policy. | Documentation or DLL exports cannot manufacture compiler evidence. #5107 is not implemented or closed. |
| D25 | Publication phases, record storage and native record/value checks have separate owners. All 675 partitions, configuration, coverage and provenance match the UIA control; normalized omissions are unchanged. | The extraction changes no contract, exclusion or scope. |
| D26 | The 51 native names / 127 pairs preceding UIA are accepted as candidate scope. | Observed names, targets and providers only; not final ABI/consumer qualification. |
| D27-D28 | Native/macro identity separation recovers three GUIDs without losing retained declarations. GUID_NULL and IID_IResourceManager move from foundation to their macro-owning headers. | Independently published macros do not define native objects. Declaration-only native objects retain explicit missing-definition diagnostics. |
| D29 | The inventory covers every attribute family. Direction cleanup removes 357,026 bytes with byte-identical WinMD. Typed bitfields and compact override/alignment/returned-pointer/termination forms remove another 673,265 bytes with identical canonical metadata and architecture-specific signatures, flags and attributes. | NativeInput and every other custom-attribute payload remain unchanged. Semantic removals still require decisions. Metadata-only fixture paths keep downstream qualification out of these iterations. |
| D30 | Removes 28,329 NativeInput uses in 432 partitions and 1,648,268 bytes. All other RDL text, metadata signatures, flags and attributes remain unchanged, as do omissions and coverage. CoIsHandlerConnected now uses inferred input with no extra attribute. | Native SAL/RPC distinctions still reject incompatible observations. Legacy schema support remains for explicit older metadata inputs; no new NativeInput uses are published. |
| D31 | Removes 5,807 exception attributes in 43 partitions and 470,310 bytes. All other RDL text, signatures, flags, attributes, omissions and coverage remain unchanged, including four SDK noreturn contracts. Anonymous noexcept/empty-throw pointers have C/stdcall FNPTR metadata coverage. | Native conflicts and callback distinctions remain checked. Computed/dependent/nonempty forms stay unsupported. No unwind ABI, exception translation or downstream qualification is claimed. |
| D32 | Normalizes 1,421 callable-result SAL annotations to must_use in 55 partitions, removing 84,868 bytes. Canonical RDL is 17,917,433 bytes, with 6,742 qualified attributes and 4,815 residual SAL occurrences. All unrelated text, signatures, flags, attributes, omissions and coverage remain unchanged. | Native conflicts remain checked. Parameter/global inspection, conditional annotations and success predicates stay separate. Duplicate and inherited callback contracts have header/RDL and metadata round-trip coverage. Downstream diagnostics are deferred. |
| D33 | Normalizes 556 success annotations in 74 partitions, removing 35,090 bytes. RDL is 17,882,343 bytes, with 6,186 qualified attributes and 4,259 residual SAL occurrences. Three value aliases retain status-type links instead of flattening to primitives. All other text, signatures, flags, constants and attributes remain unchanged on all targets, as do omissions and coverage. | Typed predicates preserve callable overrides versus typedef defaults. Fifty-eight direct success annotations remain evidence. Native type guards, cross-input alias ownership, conflicting defaults and round trips have focused coverage. No downstream Result conversion is claimed. |
| D34 | Corrects 29,938 unknown directions and adds shared out/opt facts on the two registry-query count parameters. Its explicit-marker control is 18,660,759 bytes; D35 supersedes that spelling. All other signatures, custom attributes, constants, omissions and coverage remain unchanged. | All-target metadata comparison and full RDL/WinMD round trip preserve the corrected flags. Conditional input remains evidence, not an unconditional requirement. No downstream conversion is qualified. |
| D35 | Removes all 29,938 unspecified markers and writes every known direction explicitly. Canonical RDL is 18,239,397 bytes, 421,362 bytes smaller than D34, with 149,452 attributes. | Regenerated SDK WinMD is byte-identical to D34 and retains signatures, flags, constants and attributes through a full RDL round trip on all three architectures. Migrated WinRT/WDK metadata is also byte-identical. The source migration preserves 116,317 parameter flag sets across 1,598 inputs; clang fixtures are regenerated from native evidence rather than retaining old guesses. Omissions and frozen coverage are unchanged. |
| D36 | Emits 3,262 readable and 4,018 writable extents; resolves 915 residual SAL occurrences without adding any. Canonical RDL is 18,169,981 bytes, with 150,096 attributes and 3,344 residual SAL occurrences across 142 names. | All-target comparison preserves signatures, parameter names/flags, constants and non-buffer attributes. Full SDK RDL/WinMD round trips preserve the new contracts. Existing initialized-length, null-fallback, field and RPC post-writable cases have typed assertions; SDK fixtures distinguish entry capacity, successful length and `on_entry(*count)`. Phase-free field spelling and the `on_entry` refinement preserve byte-identical SDK WinMD. Declaration totals, omission diagnostics and frozen constant/function coverage are unchanged. Consumer interpretation remains unqualified. |
| D37 | Adds 338 indirect success extents from 333 returned-buffer SAL occurrences: 21 readable and 317 writable. Canonical RDL contains 489 `if_present` counts, 150,101 attributes and 3,011 residual SAL occurrences across 136 names, totaling 18,156,697 bytes. | All-target comparison preserves signatures, names, flags, constants and non-buffer contracts, including ten explicit null-on-failure pointer contracts. Existing extents differ only by optional-count availability. New extents use successful counts; full-buffer forms do not acquire writable capacity. All metadata survives a complete SDK RDL/WinMD round trip. Declaration totals, byte-for-byte omission diagnostics and frozen constant/function coverage are unchanged. Native fixtures cover all 24 families, real IMFMediaBuffer::Lock, optional storage/counts, nullability, pointer aliases/arrays, conflicting observations and matching pointer/buffer conjunctions. Consumer interpretation remains unqualified. |

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

Macro discovery probes every candidate in one incremental native parse per target, without a PCH,
so Clang's builtin `_GUID` definition stays available. Discovered GUID macros publish their native
values, and `uuid_macros` checks parity with explicit capture.

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
| #5042 | The `MultiString` / `WCHAR*` case retains `_Post_ _NullNull_terminated_` as `null_terminated(double_success)`, with synthetic and real SDK SAL, all targets and both metadata round trips. | A fix to the old `windows-clang` parser, a qualified consuming API or issue closure. |
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

Run the [repository commands](crates/windows-clang.md#validation) against matching source,
candidate RDL and authoritative WinRT metadata. `package_acceptance` compiles current Win32/WDK
inputs, uses actual package routing and rich/raw filters, and checks all features with warnings
denied on x86/x64/ARM64. The current SDK/WDK candidate passes on all three targets, including the
target-aware enum/constant projection. Compilation alone does not establish live UIA-provider
behavior or native ARM64 execution.

Production `tool-package` must not combine partial candidate routes with a different committed
WinMD. CI selects SDK policy/metadata cases and cross-target consumers; generation checks the
canonical metadata diff. Local runs do not establish hosted execution or a fresh clean checkout.
Native ARM64 execution, release pins, final independent review and maintainer sign-off remain open.
No issues are closed by this evidence.

Final scope and performance limits remain proposals: no slower than a matched legacy run,
provisionally <=480 seconds and <=8 GB peak process-tree working set. Ask for a quiet machine before
measuring. Historical concurrent timings and single-process samples are not budget results. Retain
machine/configuration/toolchain identity and distinguish warm dependency state.

[midl-protocol]: https://learn.microsoft.com/en-us/windows/win32/midl/-protocol
[ums-support]: https://learn.microsoft.com/en-us/windows/win32/procthread/user-mode-scheduling
