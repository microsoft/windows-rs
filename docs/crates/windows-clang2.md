# windows-clang2

`windows-clang2` is the unpublished replacement path for header -> RDL -> WinMD capture.
Production generators use `windows-clang`; this crate has no legacy extractor, default metadata,
or Rust projection dependency. See the [user-facing introduction](../../crates/libs/clang2/readme.md)
and the [fixed replacement matrix](windows-clang.md#fixed-implementation-matrix).

## C2 scope and completion contract

| Area | Selected contract |
| --- | --- |
| Required roots | Explicit qualified source names in exact compiler-identified files |
| Scalars | Windows MSVC builtin kinds and checked size/alignment on x86, x64, ARM64 |
| Typedefs | Written edges retained; expansion agrees with independently captured canonical types |
| Qualifiers | Local const/volatile, pointer levels, references, and array qualification |
| Records | Incomplete named struct/class/union identity only; no invented storage |
| Dependencies | Closed consumed graph, including unselected typedefs and forward tags |
| Naming | Source namespace components appended to the caller's metadata namespace |
| Output | Separate closed output model, shared RDL emission, semantic WinMD readback |
| Lifetime | Reuse, concurrent capture, TLS restoration, snapshot survival after compiler destruction |

The reviewed `test_clang2` headers/RDL cover scalar widths, alias chains, mixed pointers, array and
reference qualifiers, forward tags, and namespace references. `alias_chains.h` supplies the selected
`TEXT`/`WTEXT` closure; its callable/record roots are recorded but not emitted. `smoke.h` supplies
the `Point` record anchor and callable rejection controls. The pinned SDK `winnt.h` profile selects
`CHAR`, `SHORT`, `LONG`, `ULONG`, `LONGLONG`, `ULONGLONG`, `WCHAR`, and `PVOID` on all three targets.
Its exact roots include `minwindef.h`, which owns `ULONG` under the `windows.h` include contract.

Negative controls cover source errors, missing/empty scopes, unavailable root files, behavioral
records, functions/callbacks, enums, rvalue references, attributes, and target overrides. Required
failures do not return partial snapshots. Nonselected roots are explicit inventory entries, not
emitted coverage.

## Ownership and checked output

`Compiler` manually loads one `Arc<clang_sys::SharedLibrary>`. Each session installs that library
in the current thread's runtime bindings and restores the previous binding on every exit.
Session destruction disposes the TU before the index, while its binding guard remains installed.
Concurrent captures share library ownership but use independent indices, TUs, and thread bindings.
Callbacks contain Rust panics before returning through C.

Index creation takes an exclusive process-wide target-registry lock. A live session holds a shared
lock through parsing, inspection, and TU/index disposal. Native sessions can coexist, but new index
creation waits for active sessions: libclang registers LLVM targets on every `clang_createIndex`
call, not only at first use. LLVM forbids registration concurrent with registry access. A warm-up
call or a mutex around index creation alone does not enforce that full contract. Code calling
libclang outside this crate must also honor the native registration contract.

An error/fatal diagnostic prevents AST traversal. Capture records the actual compiler version and
library path, input arguments, checked target, source locations, canonical cursor identity/USR,
written/canonical spelling, and layout evidence. No cursor, compiler type, TU, or index escapes.
Cursor equality establishes identity; source offsets and matching shapes do not merge declarations.

The private graph retains distinct compiler scalar kinds even when their RDL primitive is the
same: `int`/`long` and `double`/`long double` remain different native observations. Width does not
turn a typedef into `usize`/`isize`. Alias expansion checks canonical type structure and layout.
For agreement only, C++ array qualifiers are distributed to elements; the original written tree
remains the output input. This accounts for libclang placing qualifiers on different array nodes
in written and canonical types.

`Snapshot::rdl` checks publication names, collisions, scalar/pointer target contracts, and
unsupported restrict qualification, then builds a closed output model. A same-spelled record
typedef disappears only when its captured target is that exact record identity; other aliases
remain separate. Names such as `self`, `Self`, and `_` reject rather than being silently renamed
by the shared emitter. Output is sorted by publication name, not traversal/selection order.
Nominal references are fully qualified so source names such as `String`, `i32`, and `Char16`
cannot turn into RDL primitives. Predefined `System.Guid`/`Windows.Foundation.HResult` publication
identities reject rather than activating the shared emitter's special shorthand.

The crate returns RDL text rather than replacing files or publishing metadata. Tests compile the
reviewed text through the existing RDL reader and inspect typedef payloads, qualifiers, pointers,
and opaque descriptors. Expected RDL is never overwritten by ordinary tests.

## Validation

Use the repository's pinned libclang. SDK tests require the cached SDK package
`microsoft.windows.sdk.cpp` 10.0.28000.2270 and matching LLVM 22.1.8 resource headers. Missing inputs
fail explicitly. `WINDOWS_CLANG2_SDK_INCLUDE` and `CLANG_RESOURCE_DIR` can select those cached inputs.

```powershell
$env:LIBCLANG_PATH = (cargo run -p tool-clang --quiet -- path)
$env:WINDOWS_CLANG2_LAYOUT_NATIVE = (cargo run -p tool-clang --quiet -- native-layout)
$env:RUSTFLAGS = '-D warnings'
cargo fmt -p windows-clang2 -p test_clang2 -p tool-clang
cargo clippy -p windows-clang2 -p test_clang2 -p tool-clang --all-targets
cargo test -p windows-clang2 --lib
cargo test -p test_clang2
```

## Remaining packages

### C3-A selected storage contract

C3-A covers named, nonempty plain-data structs/classes and unions with scalar, pointer, fixed-array,
alias, and by-value named-record fields. Source field order and nominal identities remain intact,
including recursive pointer graphs and forward declarations resolved by a complete definition.
Only natural layout is accepted: every byte offset, aggregate size, and alignment must agree with
the emitted field types. No packing value is inferred from a matching layout.

The storage model is implemented. Reviewed `records.h`/RDL, `smoke.h`'s `Point`, and
`data_classes.h`'s `BitmapData` are the selected fixtures. The pinned SDK `POINT`/`RECT` closure
preserves the `tagPOINT`/`tagRECT` definitions, aliases, and original `LONG` member types.
Native MSVC static assertions check sizes, alignments, and field offsets on x86/x64/ARM64;
three-target WinMD readback checks the same field/type graph and storage meaning.

Empty records, bit-fields, anonymous/nested storage, inheritance, methods, non-POD classes,
nonpublic members, references as storage, flexible/zero-length arrays, attributes, and adjusted
packing/alignment are explicit negative cases. The remaining C3 storage families and the OAIDL,
NVMe, and WDK anchors are not covered by this slice.

Capture keeps owned field locations, written/canonical types, layouts, and byte offsets. Pending
nominal identities close recursive pointer graphs; by-value cycles and alias cycles still reject.
A complete definition supplies fields even when the canonical cursor is a forward declaration.
Checked output compares each field offset and the aggregate size/alignment with natural storage
before rendering a record. Packing/alignment changes are not guessed or converted into opaque tags.

### C3-B explicit packing/alignment evidence gate

The implemented slice is record-level pragma packing and MSVC `__declspec(align(...))`, with the
C3-A field/type graph retained. Bit-fields, anonymous/nested definitions, inheritance, and explicit
field alignment remain separate boundaries. Existing RDL `#[packed(N)]` and `#[align(N)]` syntax
supplies the output vocabulary; native requests must be captured before choosing those attributes.

| Required fact | Evidence source | Forbidden substitute |
| --- | --- | --- |
| Active record packing | Compiler-owned maximum-field-alignment attribute | A pack value fitted to offsets |
| Requested record alignment | Compiler-evaluated alignment attribute in its declaration context | Aggregate `alignof` |
| Storage agreement | Native member offsets, size, alignment, and required alignment | Successful RDL compilation |
| Ownership | Same native declaration/TU as the captured record | Matching only a spelling or offset |

`layout_contracts.h` isolates the missing facts without adding SDK headers. The private capture
test checks the libclang C API on x86/x64/ARM64. `layout_contracts.cpp` independently checks MSVC
size, alignment, and offsets on the same targets:

| Source requests | Observed storage | Libclang attribute evidence |
| --- | --- | --- |
| Pack 2 vs pack 4 around `char`/`short` | Both size 4, alignment 2, second member at 2 | No packing cursor |
| Record align 1 vs align 4 around `int` | Both size 4, alignment 4 | Aligned cursor, no expression/value |
| Macro expression `(8 + 8)` for record alignment | Size/alignment 16 | Aligned cursor, no expression/value |
| Pack 1 vs GNU packed attribute | Both size 5, alignment 1 | Only the GNU attribute has a cursor |
| Explicit alignment on an `int` member | Member type alignment 4, member offset 16 | Field attribute is separate |

The approved native bridge reads `MaxFieldAlignmentAttr`, compiler-evaluated record alignment, and
`ASTRecordLayout` required alignment from the captured record's own clean TU. It returns owned
numbers, not AST handles. No token arithmetic, pragma scanner, inferred packing, or separate AST
merged by source name supplies those facts.

`Compiler::with_layout_bridge` enables this provider. Its parser and bridge must be in the same
canonical directory; bridge ABI, imported parser function identity, and the parser binary's SHA256
must match. `Snapshot::layout_bridge` retains the bridge path, ABI, and parser fingerprint after
compiler disposal. `record_layouts` exposes the owned requests, including requests with no storage
effect. `Compiler::new` retains the C2/C3-A path; its natural-storage checks do not establish the
absence of packing requests hidden by the C API.

Storage checking limits each field's natural alignment by packing, then applies its required
alignment. Explicit required alignment propagates through by-value named records, aliases, and
arrays, but not through pointers. Every member offset and aggregate size/alignment must match
compiler evidence before output. Reviewed `layout_contracts.rdl` retains distinct pack 2/4 and
align 1/4 requests using existing RDL attributes, without changing the metadata format.

The selected C3-B gate passes on x86/x64/ARM64: exact reviewed RDL, semantic WinMD packing/alignment,
and independent MSVC size/alignment/offset assertions. Packed pointers, unions, and arrays of aligned
records exercise required alignment overriding packing and propagating into enclosing records.
Wrong-directory, ABI, and fingerprint failures reject before querying ASTs; concurrent captures and
snapshot use after compiler disposal also pass. Field alignment, GNU packed/aligned attributes,
C++ `alignas`, bit-fields, inheritance, and behavioral records still reject. Anonymous/nested
storage has the separate C3-C contract below. Other alignment spellings need their own
required-alignment contract before publication.

### C3-C nested and anonymous storage

The selected slice covers public named POD record/union definitions and typedefs in record scopes,
plus single direct unqualified fields of unnamed records. Unnamed records may contain further
direct anonymous storage. Named nested types retain shared references, fixed arrays, pointers,
recursive identity, and forward declarations; equal shapes in different owners remain distinct.
Selecting a named nested type closes its enclosing record graph, not only the child's fields.

| Fact | Compiler evidence | Output |
| --- | --- | --- |
| Named type owner | Canonical semantic-parent cursor | `NativeTypeOwnerAttribute` with a type reference |
| Direct fields and order | `clang_Type_visitFields`, including implicit fields | Original field sequence |
| Field source name | Matched bridge's `FieldDecl` identifier fact | Original name or unnamed marker |
| Injected anonymous member | `FieldDecl::isAnonymousStructOrUnion` | `NativeAnonymousFieldAttribute` |
| Unnamed type identity | Canonical cursor and owning field position | Inline record and `NestedClass` |
| Storage | Compiler offsets, type layout, packing/alignment contract | Checked fields and existing RDL attributes |

Libclang's ordinary child traversal omits implicit anonymous fields. Its field visitor includes
them, but cursor spelling returns a diagnostic type name for an unnamed member. The bridge reads
identifier presence and anonymous-member status directly; strings containing "anonymous" do not
establish either fact. Without the bridge, implicit fields reject rather than being omitted.
The bridge ABI is 2; parser identity/fingerprint and lifetime checks remain in force.

`Evidence::id` identifies a declaration within one snapshot. `FieldEvidence::record_id` retains
direct canonical record edges, and `ownerships` exposes semantic parent edges. USRs remain source
evidence, not identity keys: the fixture's distinct anonymous unions have the same USR. No USR,
source offset, filename, or structural shape merges their canonical cursor identities.

Named nested declarations use source scope components as publication namespaces, with an explicit
native owner descriptor distinguishing a record scope from a namespace. Direct unnamed records use
existing inline RDL. An implicit field receives a collision-checked `Anonymous_<ordinal>` metadata
helper name and the unnamed marker; its captured source name stays empty. A real source field named
`Anonymous_7` stays named and unmarked. These two descriptors use the existing custom-attribute
grammar and the authored `metadata/metadata.rdl` vocabulary, not default API metadata.

Reviewed `nested_records.h`/RDL exercises forward nested tags, local typedefs, shared union types,
same-named records in different owners, arrays, self-pointers, named fields of unnamed records,
injected unions/structs, and helper-name collisions. `nested_packed.rdl` checks packing on both
named and inline nested types. The real pinned SDK `DECIMAL` closure comes from `wtypes.h`; it
retains `tagDEC`, its typedef and member aliases, and both anonymous union/struct trees.

The selected gate passes on x86/x64/ARM64 with independent MSVC size/alignment/offset assertions,
exact reviewed RDL, semantic WinMD readback, and RDL round-trip checks. Filename/selection changes,
snapshot cloning, and use after compiler disposal preserve output. Metadata validation rejects
malformed owners, cycles, duplicates, and unnamed fields without directly owned nested storage.
Architecture merge includes both descriptors in shape agreement, preventing a same-layout
ownership or named/unnamed distinction from collapsing.

Arrays/pointers to unnamed records, shared unnamed types, qualified unnamed storage, named types
inside unnamed owners, private/behavioral owners, bit-fields, and unsupported C3-B attributes remain
explicit boundaries. No original field is promoted into its parent or flattened across a union.
The next selected storage slice is bit-fields; C3 as a whole and production promotion remain open.

### Native layout provisioning

`cargo run -p tool-clang --quiet -- native-layout` builds the bridge on an x64 Windows host with
MSVC and CMake. It prints the matched parser/bridge directory. The build supports all three Windows
capture targets, not x86 or ARM64 host DLLs. The x64 CI job provisions it; the layout test uses a child
with the matched parser while other tests keep their DevDiv `LIBCLANG_PATH`.

The development archive supplies both the parser and its headers/static libraries. DevDiv's runtime
NuGet DLL is not combined with independently built private AST APIs.

| Dependency | Pin |
| --- | --- |
| Official LLVM archive | `clang+llvm-22.1.8-x86_64-pc-windows-msvc.tar.xz` |
| Archive SHA256 | `d96c2cc1736f4eb7fa43cb9bbdf56d93551a9ae0a9aadb9c99c3c3b2b712a234` |
| LLVM source revision | `ca7933e47d3a3451d81e72ac174dcb5aa28b59d1` |
| Bridge ABI | `2` |

`native/llvm.json` pins the archive and source. The build verifies the archive checksum, extraction
marker, source revision, and clean source checkout. It extracts the pinned upstream cursor accessor,
uses the SDK's release static CRT and AST libraries, and checks LLVM version, ABI-breaking-check,
and iterator-debug settings. The runtime loader verifies the parser fingerprint embedded at build.
The cold archive download is 862,053,924 bytes; extraction, source checkout, and build add provisioning
cost. The cache is under `target/tool-clang/native-layout`; full cold provisioning time is unmeasured.
This bridge is the plan's narrow compiler-inspection boundary, not a production backend switch.

### Native parser registration and reliability

On 2026-10-12, the concurrent C3 suite stalled in the selected SDK `POINT`/`RECT` profile. The test
was stopped after several minutes. A non-invasive native stack showed the caller waiting in
`clang_parseTranslationUnit2` and the parser worker executing inside libclang, before owned capture.
The same profile passed alone; successful reruns did not establish a repair. A fixed subprocess
matrix reproduced the stall with independently created compiler owners. The initial matrix held
an extra library owner in that control; the corrected control removed it.

Direct native controls use the same source, arguments, targets, index/TU lifecycle, and eight
rounds, without owned capture or output. A barrier aligns the first three index creations in a
fresh process. Eight cold/warm pairs produced this result:

| Control | Completed children | Deadline failures | Failure phase |
| --- | --- | --- | --- |
| Cold concurrent native initialization | 5/8 | 3/8 | X86's first parse; other targets finish |
| Main-thread initialization before concurrency | 8/8 | 0/8 | None observed |

All three failed children created their indices successfully. X86 entered its first parse and
never returned; x64 and ARM64 completed all eight rounds. The reproduction needs neither SDK
declaration traversal nor the Rust graph. Each failure retains its log and kills/reaps only the
owned child at the 20-second deadline.

The pinned LLVM source identifies a violated synchronization contract. `clang_createIndex_Impl`
calls `InitializeAllTargets`, `InitializeAllTargetMCs`, and the assembly initializers on every
index creation. `TargetRegistry::RegisterTarget` changes a shared linked list without
synchronization; the registry's registration APIs require clients to exclude concurrent access.
This explains why independently created indices are not sufficient isolation. The evidence
supports the registration race as the stall mechanism; private native symbols were not available
to inspect the failed registry itself.
Sources: pinned [libclang index creation][cindex], [target registration][registry-cpp], and
[registry synchronization contract][registry-h].

The wrapper enforces that contract with exclusive registration and shared native-session guards.
It does not retry, omit diagnostics/bodies, pin a permanent library, or replace the parser.
The ordinary deadline regression covers serial, reload-serial, shared, independent,
barrier-aligned cold independent, mixed-success/error lifecycles, and direct native serial parsing.
Four complete protected matrices passed: 672 SDK captures and 1,024 short-source captures, plus
the native serial controls. The C2/C3-A fixture suite and native layout assertions also pass.

The unsafe direct-native contrast is an ignored diagnostic, not an ignored wrapper regression.
It bypasses the wrapper's synchronization and can fail on the pinned compiler:

```powershell
cargo test -p test_clang2 --test reliability direct_native_registration_matrix -- `
    --ignored --nocapture
```

Logs live under `OUT_DIR\parser-reliability\<run>`. The failing native contrast run is
`1791821954642346100`; the three separate protected repetitions are `1791822017183191100`,
`1791822045503905400`, and `1791822073754017100`.

Registration exclusion has a measured cost: the three-target shared case takes about 3.2 seconds,
versus 1.2 seconds without the protocol. Repeated index creation can limit overlap between
captures. These small controls do not close production throughput or full-input performance gates.

### Bounded execution-model comparison

The 2026-10-12 gate in `test_clang2`'s `reliability/execution.rs` compares fresh synchronized
in-process capture with three isolated workers, one architecture and one active capture per worker.
Both models use the existing pinned `windows.h` source and resource/include defaults. Each target
runs eight rounds of the selected `winnt.h` scalar closure and eight of the `POINT`/`RECT` closure,
including checked RDL emission, WinMD compilation, and native-contract validation. No filter,
reviewed golden, library capture policy, production tool, or canonical metadata changes.

The shared child loads one `Compiler` and runs three target threads with the library's registration
protocol. Each isolated child loads one `Compiler` and executes its target serially. All children
wait at a parent-controlled start gate. Three fresh paired repetitions reverse model order in the
middle pair, without retries. Every RDL matches its reviewed golden; all 144 per-target/round RDL
pairs and all 144 WinMD pairs per campaign are byte-identical between models. Both models compile
each WinMD with the same basename, since the output filename contributes to metadata identity.

| Pair | Shared startup (s) | Isolated startup (s) | Shared wall (s) | Isolated wall (s) | Shared peak (MiB) | Isolated peak (MiB) |
| --- | --- | --- | --- | --- | --- | --- |
| 0 | 0.021 | 0.031 | 6.774 | 2.462 | 91.266 | 216.414 |
| 1 | 0.021 | 0.031 | 6.742 | 2.433 | 91.402 | 221.711 |
| 2 | 0.021 | 0.032 | 6.753 | 2.427 | 91.055 | 216.434 |

Startup is measured from before the first process spawn to readiness of every required child,
including test-process startup and `Compiler::new`, before any capture starts. Separate child
timers measure actual library initialization: 0.008-0.011 seconds, not a startup proxy. Work
intervals, from gate release until every child exits, are 6.746/6.714/6.725 seconds for shared and
2.425/2.396/2.388 seconds for isolated. Wall time includes startup, work, complete-artifact checks,
and publication of the test staging directory; it excludes Cargo compilation and cross-model byte
comparison. These are debug builds on a shared Windows x64 host. A separate initial campaign also
passed all three pairs, with shared wall times of 7.368/9.240/15.380 seconds and isolated times of
2.945/2.751/5.698 seconds. Its startup ranges were 0.021-0.054/0.030-0.066 seconds and memory ranges
91.113-141.293/215.641-216.383 MiB. Timing variation is visible; these bounded campaigns are not
a production benchmark or a statistical confidence claim.

Peak memory is Windows Job Object `PeakJobMemoryUsed`, expressed in MiB: the kernel-recorded maximum
aggregate simultaneous private committed bytes of all processes in that model's job. It is not
the sum of independent worker high-water marks. The shared job contains one process; the isolated
job contains all three concurrent workers. Both include process/runtime/libclang overhead and
startup allocations, exclude the supervising test process and shared DLL pages, and do not measure
physical working set or system memory pressure. Shared-session scheduling can change overlap and
therefore its peak. Isolated capture improves paired wall time by 2.5-3.4x here, with a higher
aggregate private-commit peak.

Each batch has a 20-second deadline and a dedicated non-breakaway, kill-on-close Windows job.
Workers cannot capture or spawn descendants until the parent assigns their owned process handles
to that job. Failure or deadline terminates the owned job, reaps direct workers, and checks that
job accounting reports zero active processes. There is no process-name kill or retry.
The injected failure exits with code 7 only after both peer targets have staged complete outputs.
The injected hang reaches the same point, spawns a child that acknowledges its PID, and blocks.
Both controls pass: failed/hung batches return errors, the hung descendant is terminated too,
partial staging is removed, and a preexisting publication sentinel remains unchanged. Successful
batches publish only after all targets exit successfully and all expected artifacts exist.
This is test orchestration, not a production publication/recovery implementation.

Reproduce without parallel test load:

```powershell
$env:LIBCLANG_PATH = (cargo run -p tool-clang --quiet -- path)
$env:RUSTFLAGS = '-D warnings'
cargo test -p test_clang2 --test reliability execution:: -- --nocapture --test-threads=1
```

Logs, initialization timings, measurements, and outputs remain below
`OUT_DIR\execution-model\<run>`. Comparison run `1791823027882651600` and containment run
`1791823002180250400` record the initial campaign. Final comparison run `1791823195789422300`
and containment run `1791823173295689400` record the table and repeated controls. Strict clippy for
`windows-clang2` and `test_clang2` passes. The final serial test invocation passes all 18 ordinary
tests across `capture`, `records`, and `reliability`, including MSVC layout assertions on all three
targets and the protected parser matrix. Three subprocess/native-diagnostic entry points are
ignored by ordinary test discovery; the parent tests invoke the protected workers explicitly.

Prefer three isolated architecture workers for future coarse tool batches when their extra
aggregate memory is acceptable: this bounded evidence supports both better overlap and process-tree
timeout containment. Keep the synchronized library usable in-process for direct consumers and
smaller work. Subprocess policy belongs to tool/test orchestration; no configurable worker backend
or production migration is introduced. Full-input throughput, additional profiles, release-build
costs, and broader failure modes remain unmeasured. This execution gate stops before C3-B.

The separate 2026-10-12 release check confirmed that NuGet's absolute latest
`libclang.runtime.win-x64` and `libclang.runtime.win-arm64` versions are both 22.1.8, including
prereleases. The repository pin and local DLL file/product versions match. The separate
`libClangSharp.runtime.win-x64` package is 22.1.8.2; it is a wrapper library, not a newer libclang
parser used by this Rust path. No newer published compiler package was available for an upgrade
comparison.
Sources: [x64 runtime](https://www.nuget.org/packages/libclang.runtime.win-x64/),
[ARM64 runtime](https://www.nuget.org/packages/libclang.runtime.win-arm64/), and
[ClangSharp pins](https://github.com/dotnet/ClangSharp/blob/main/Directory.Packages.props).

The selected C2 operational gate and C3-A natural-storage gate are closed with this protocol and
bounded evidence. This is not a claim that all libclang inputs are reliable. The next C3 work is
adjusted layouts, anonymous/nested storage, and bit-fields under their own checked contracts.
Production promotion remains blocked by the remaining C0-C11 obligations.

C2 establishes the bounded session/type/alias path, not full header correctness or production
replacement. Further records/storage are C3; enums/values are C4; callables are C5; interfaces are
C6; raw annotations and broader type/declaration contracts are C7; macro state/evaluation is C8;
imports/data are C9. Cross-profile/architecture agreement and partition ownership remain C10.
Production promotion remains C11. No native calling or Rust binding claim follows from the C2
storage and metadata checks.

[cindex]: https://github.com/llvm/llvm-project/blob/llvmorg-22.1.8/clang/tools/libclang/CIndex.cpp
[registry-cpp]: https://github.com/llvm/llvm-project/blob/llvmorg-22.1.8/llvm/lib/MC/TargetRegistry.cpp
[registry-h]: https://github.com/llvm/llvm-project/blob/llvmorg-22.1.8/llvm/include/llvm/MC/TargetRegistry.h
