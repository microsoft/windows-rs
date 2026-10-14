# windows-clang-sys

Unpublished raw Clang C bindings and version-matched native semantic queries.

The native companion builds against a pinned LLVM development package and pinned ClangSharp
sources. Rust callers use C functions, not Clang C++ object layouts or private cursor fields.
The companion and its matching `libclang.dll` must be distributed together.

From the repository root, build the native libraries and run the raw API fixtures:

```powershell
cargo run -p tool-clang-sys -- --native
$env:LIBCLANG_PATH = cargo run -q -p tool-clang -- path
cargo test -p test_clang_sys
```

Building requires Visual Studio C++ tools and its CMake/Ninja component. Native hosts are Windows
x64 and ARM64; either host can parse x86, x64, and ARM64 input. The first build downloads the LLVM
development archive and ClangSharp sources. Subsequent builds reuse `target\tool-clang\native`.

Call `load()` before using a raw function on a thread. For worker threads, share an
`Arc<SharedLibrary>` from `load_manually()` and bind it with `set_library()`. Dispose every native
handle before releasing its library owner. Raw pointers and opaque handle declarations do not
confer ownership or make native objects safe to construct in Rust.

The supplementary API exposes attribute identities, requested pragma packing, type qualifiers,
and function-type properties that stock libclang does not decode. The qualifier query identifies
CVR and `__unaligned`, and flags any remaining qualifiers for further decoding or rejection.
It does not supply a complete normalized type model: callers must also inspect ordinary libclang
type queries, attributed types, and additional property flags before accepting a signature.

See the [crate notes](../../../docs/crates/windows-clang-sys.md) for generation and maintenance.
