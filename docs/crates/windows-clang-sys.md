# windows-clang-sys

Unpublished raw bindings used by `windows-clang`, `windows-clang2`, and generator helpers. See the
[readme](../../crates/libs/clang-sys/readme.md) for setup and thread binding.

## Native boundary

The crate loads a matched `libclang.dll` and `libClangSharp.dll` pair. The companion adds a small
C API to pinned ClangSharp sources and links against the official LLVM development archive. Rust
does not inspect private cursor fields or depend on Clang C++ class layouts.

| Query | Result |
| --- | --- |
| `windows_clang_getAttributeKindName` | Native attribute kind, including tokenless attributes. |
| `windows_clang_getTypeAttributeKindName` | Attribute kind on an attributed type. |
| `windows_clang_getTypeQualifiers` | CVR, `__unaligned`, and a flag for all other qualifiers. |
| `windows_clang_getMaxFieldAlignment` | Requested maximum field alignment in bits. |
| `windows_clang_getAlignment` | Evaluated alignment bits and GNU, standard, or declspec form. |
| `windows_clang_getTypeVisibility` | Default, hidden, or protected `TypeVisibilityAttr` payload. |
| `windows_clang_getTypedefDefinition` | Whether a direct typedef type specifier defines its tag. |
| `windows_clang_getFunctionInfo` | Function `ExtInfo` properties and extra-property flags. |

Packing comes from `MaxFieldAlignmentAttr`, not observed record layout or explicit `AlignedAttr`.
The alignment query reads each `AlignedAttr`, including zero, type operands, and GNU requests
without an argument. Dependent requests return `WindowsClangDependent` without changing the
output. Requests are not inferred from effective record alignment.
The function query preserves both `has_regparm` and `regparm`, distinguishing explicit zero from
absence. It also reports `noreturn`, result ownership, caller-saved register, control-flow check,
and CMSE flags. Additional flags identify extended parameters, extra attributes, ARM SME
attributes, function effects, method qualifiers, ref qualifiers, and unchecked CFI callees.
These flags signal more information to handle or reject; they are not decoded payloads.

The qualifier query includes qualifiers inherited through typedefs. After identifying const,
volatile, restrict, and unaligned, it checks the remaining native qualifier set, rather than
assuming those four exhaust Clang's type qualifiers. Address spaces and other undecoded qualifiers
set `WindowsClangOtherQualifiers`. The extension ABI version is 5.

The type-visibility query reads the attribute itself, including implicit attributes with no tokens.
It is distinct from ordinary symbol `VisibilityAttr` and from `clang_getCursorVisibility`, which
reports effective declaration visibility. The native fixture checks all three payloads and a type
whose symbol visibility and type visibility differ.

The typedef query uses `TypedefNameDecl` type-source information and `TagTypeLoc::isDefinition`.
It distinguishes `typedef struct Tag { ... } Name` from references, forward declarations,
alias chains, and pointer/array declarators. Multiple value typedefs in the same definition each
retain this evidence. No declaration ordering, token matching, or source-range heuristic is used
by the Rust capture or projection layers.

These queries supplement calling convention, argument, result, exception, variadic, and other
stock type queries. A zero extra-property mask is not permission to discard attributed types or
declaration attributes. Consumers retain responsibility for defining their accepted type model.
`windows-clang2` captures requested packing and alignment and gates C/stdcall callback prototypes
on these queries.
Unknown attributes and unsupported function properties remain explicit errors or omissions.

Attribute-name strings are borrowed from the library. Query cursors and types must belong to a
live translation unit created by the matching runtime. Wrong-kind and null-output status failures
leave output arguments unchanged. Raw API calls retain the library for the duration of the call,
including callbacks, but native handles must not outlive their library owner.

## Building and generation

`LIBCLANG_VERSION` in `src/lib.rs` pins LLVM. `native/build.ps1` separately pins a matching
ClangSharp commit. The script downloads their archives, builds the companion with the static
MSVC runtime used by the LLVM libraries, and copies both DLLs and license notices into
`target\tool-clang\native\<version>\<host-triple>\bin`. The initial x64 LLVM archive is about
862 MB. Visual Studio C++ tools and its CMake/Ninja component are required.

```powershell
cargo run -p tool-clang-sys -- --native
cargo run -p tool-clang-sys
```

The first command rebuilds native source changes. Helper provisioning builds only when a DLL is
missing; it does not detect changed native sources. The second command regenerates
`src/bindings.rs`, which must not be hand-edited. CI regenerates the bindings and rejects a diff.

The generator parses pinned `clang-c/Index.h` and the extension header with `windows-clang`,
compiles RDL to WinMD, and emits raw types through `windows-bindgen`. It transforms generated link
declarations into a dynamic function table. Anonymous callback members are lifted into named
typedefs with compiler-resolved `decltype`, then emitted as typed optional function pointers.
Compile-time size, alignment, and field-offset assertions check the generated 64-bit record layouts
against captured native layouts. Scratch files stay under `target\tool-clang\bindings`.

Committed bindings bootstrap this pipeline: no native library is needed to compile the Rust
generator, and `--native` builds the pair without loading it. After changing the native ABI, bump
its version and update the loader's expectation together.

`LIBCLANG_PATH` selects the pair directory or a DLL within it. Otherwise the loader searches PATH.
It checks the extension ABI, LLVM version, and identity of the companion's imported libclang.
Version checks do not make arbitrary same-version builds interchangeable; distribute the pair
produced by the native build, not independently sourced DLLs.

## Coverage

`test_clang_sys` exercises x86, x64, and ARM64 targets. It distinguishes char-only `pack(1)` and
`pack(2)` records despite identical size/alignment, plain versus explicit-zero and nonzero
`regparm`, `noreturn`, attributed calling conventions, and unsupported prototype properties.
Qualifier cases include CVR, typedef inheritance, `__unaligned`, and address-space detection.
Type-visibility cases include Clang's implicit default attribute on `_GUID`.
It also covers query errors, shared library thread binding, and missing-library errors.

Native x86 processes are unsupported. Cross-target parsing from a supported host does not require
an x86 DLL. ARM64 host execution is covered by the ARM64 CI job, not by parsing an ARM64 target
from an x64 process.
