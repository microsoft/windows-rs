# windows-clang2

`windows-clang2` is the unpublished C2 replacement path for header -> RDL -> WinMD capture.
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
complete-record/callable rejection controls. The pinned SDK `winnt.h` profile selects `CHAR`,
`SHORT`, `LONG`, `ULONG`, `LONGLONG`, `ULONGLONG`, `WCHAR`, and `PVOID` on all three targets.
Its exact roots include `minwindef.h`, which owns `ULONG` under the `windows.h` include contract.

Negative controls cover source errors, missing/empty scopes, unavailable root files, complete
records, functions/callbacks, enums, rvalue references, attributes, and target overrides. Required
failures do not return partial snapshots. Nonselected roots are explicit inventory entries, not
emitted coverage.

## Ownership and checked output

`Compiler` manually loads one `Arc<clang_sys::SharedLibrary>`. Each session installs that library
in the current thread's runtime bindings and restores the previous binding on every exit.
Session destruction disposes the TU before the index, while its binding guard remains installed.
Concurrent captures share library ownership but use independent indices, TUs, and thread bindings.
Callbacks contain Rust panics before returning through C.

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

C2 establishes the bounded session/type/alias path, not full header correctness or production
replacement. Complete records/storage are C3; enums/values are C4; callables are C5; interfaces are
C6; raw annotations and broader type/declaration contracts are C7; macro state/evaluation is C8;
imports/data are C9. Cross-profile/architecture agreement and partition ownership remain C10.
Production promotion remains C11. No native calling or Rust binding claim follows from the C2
storage and metadata checks.
