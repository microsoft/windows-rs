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

The supplementary API exposes attribute identities, requested pragma packing, explicit alignment,
type visibility, type qualifiers, and function-type properties that stock libclang does not decode.
The type-visibility query distinguishes default, hidden, and protected requests without inferring
them from effective symbol visibility. Alignment queries retain evaluated bits and distinguish
GNU, standard, and declspec forms; dependent
requests return a status without evaluating them. The qualifier query identifies
CVR and `__unaligned`, and flags any remaining qualifiers for further decoding or rejection.
It does not supply a complete normalized type model: callers must also inspect ordinary libclang
type queries, attributed types, and additional property flags before accepting a signature.
The typedef-definition query exposes whether a direct type specifier defines its record or enum,
so consumers can distinguish defining typedefs from later aliases without parsing spelling.
Layout-expression queries distinguish evaluated array bounds and bitfield widths from unrelated
expressions. The field-initializer query also detects initializers hidden from cursor traversal.
The parameter-default query returns the compiler-owned expression cursor and a compiler-printed
expression string; it distinguishes absence and dependent defaults. Method qualifiers are decoded
separately from qualifiers on a function type, with the same residual-qualifier checks.
The virtual-slot query supplies compiler-owned Microsoft vftable indices and rejects slots needing
virtual-base or secondary-vptr adjustment. Declaration order does not establish overload slot order.
UUID and language-linkage queries expose semantic payloads without parsing declaration spelling.
Deprecation queries preserve both the message and replacement from the individual native attribute.
The integer and general constant-initializer queries reject runtime side effects that libclang's
expression folding can ignore. The general query also accepts boolean and floating-point
initializers. The integer-element query reads compiler-evaluated struct and array leaves, including
zero-filled tails, without exposing Clang's C++ value layout or parsing initializer tokens.
The `typeof` operand query preserves expression types before initializer conversions. A separate
query desugars compiler-predefined types such as the result of `sizeof`, without erasing typedefs.
Numeric data-pointer initializers have a separate static-initialization query. It rejects
relocations, function pointers, side effects, and undefined behavior, and returns address bits
sign-extended from the target pointer width.
UUID, deprecation, and default-expression strings are owned `CXString` values;
release them with `clang_disposeString`.

See the [crate notes](../../../docs/crates/windows-clang-sys.md) for generation and maintenance.
