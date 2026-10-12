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
$env:RUSTFLAGS = '-D warnings'
cargo fmt -p windows-clang2 -p test_clang2
cargo clippy -p windows-clang2 -p test_clang2 --all-targets
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
