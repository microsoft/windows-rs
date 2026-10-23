# windows-clang-sys

Unpublished raw bindings used by `windows-clang`, `windows-clang`, and generator helpers. See the
[readme](../../crates/libs/clang-sys/readme.md) for setup and loading.

## Native boundary

The crate loads a matched `libclang.dll` and `windows_clang.dll` pair. The companion is one C++
source file exposing a small C API. It links against the official LLVM development archive and
imports stock libclang. Rust does not inspect private cursor fields or depend on Clang C++ class
layouts.

The companion reads the AST nodes behind libclang handles from the pinned libclang handle layouts:
cursors carry the declaration or parent declaration, the attribute or statement, and the unit;
types carry a `QualType` and the unit. Returned types are built by stock `clang_getPointeeType`
on a synthesized pointer, so libclang assigns their kind. Returned expression cursors come from
stock child visitation. Owned strings use libclang's `free`-released string flag; both modules
link the static UCRT, which allocates from the process heap. These layouts are checked by the
semantics fixture and pinned with `LIBCLANG_VERSION`.

| Query | Result |
| --- | --- |
| `windows_clang_getAttributeKindName` | Native attribute kind, including tokenless attributes. |
| `windows_clang_getTypeAttributeKindName` | Attribute kind on an attributed type. |
| `windows_clang_getTypeQualifiers` | CVR, `__unaligned`, and a flag for all other qualifiers. |
| `windows_clang_getMethodQualifiers` | Function prototype method qualifiers, using the same complete flag set. |
| `windows_clang_getVirtualMethodSlot` | Absolute method index in the primary Microsoft vftable. |
| `windows_clang_getParameterDefaultArgument` | TU-owned default expression cursor and compiler-printed expression. |
| `windows_clang_getMaxFieldAlignment` | Requested maximum field alignment in bits. |
| `windows_clang_getAlignment` | Evaluated alignment bits and GNU, standard, or declspec form. |
| `windows_clang_getTypeVisibility` | Default, hidden, or protected `TypeVisibilityAttr` payload. |
| `windows_clang_getTypedefDefinition` | Whether a direct typedef type specifier defines its tag. |
| `windows_clang_getTypedefImplicit` | Whether Clang supplies the typedef declaration implicitly. |
| `windows_clang_getFunctionInfo` | Function `ExtInfo` properties and extra-property flags. |
| `windows_clang_getImportConvention` | Written/defaulted outer convention and an eligible x86 stdcall type/symbol. |
| `windows_clang_getUuid` | Compiler-validated `UuidAttr` text as an owned `CXString`. |
| `windows_clang_getDeprecation` | One `DeprecatedAttr` message and replacement as owned strings. |
| `windows_clang_getLanguageLinkage` | Native C, C++, or absent language linkage. |
| `windows_clang_getIntegerConstantInitializer` | Integer constant-expression initializer validity. |
| `windows_clang_getConstantInitializer` | Checked static-initializer validity. |
| `windows_clang_getConstantPointerInitializer` | Numeric data-pointer static initializer, without relocations. |
| `windows_clang_getTypeOfOperand` | Original expression in a variable or typedef's `typeof` type. |
| `windows_clang_getPredefinedType` | Underlying type of compiler-predefined sugar, not user typedefs. |
| `windows_clang_getConstantIntegerElement` | Evaluated nonnegative integer leaf at a struct/array index path. |
| `windows_clang_probeMacros` | Classifies each named macro with type/initializer probes after one incremental parse. |

`windows_clang_probeMacros` parses the source once as an incremental TU, then parses each
candidate's probes as a separate input against that state. Each result reports undefined,
function-like, empty, rejected, value, or function, plus the active definition location. Rejected
probes keep their formatted diagnostics, and the diagnostics engine is reset before the next
candidate. Replacement lists with unbalanced brackets are rejected without a probe, because an
incremental input must not leave parser state open for the next one; failed probes' declarations
are removed from lookup as in clang-repl. Original-source errors return `WindowsClangParseFailed`
with diagnostics in `error`.
Callers dispose every returned string. The query links Clang's driver, parser, and Sema libraries
directly because libclang does not export incremental parsing.
Packing comes from `MaxFieldAlignmentAttr`, not observed record layout or explicit `AlignedAttr`.
The alignment query reads each `AlignedAttr`, including zero, type operands, and GNU requests
without an argument. Dependent requests return `WindowsClangDependent` without changing the
output. Requests are not inferred from effective record alignment.
The function query preserves both `has_regparm` and `regparm`, distinguishing explicit zero from
absence. It also reports `noreturn`, result ownership, caller-saved register, control-flow check,
and CMSE flags. Additional flags identify extended parameters, extra attributes, ARM SME
attributes, function effects, method qualifiers, ref qualifiers, and unchecked CFI callees.
These flags signal more information to handle or reject; they are not decoded payloads.

The import-convention query follows outer type source information, including function typedefs,
without descending into parameter or result callbacks. A defaulted C declaration on MSVC x86 can
supply an alternative from `ASTContext::getFunctionType` and `ASTNameGenerator`. Only the outer
convention changes. Explicit conventions, variadics, definitions, asm labels, and unsupported
prototype properties supply no alternative. The query neither mutates the original declaration
nor inserts synthetic declarations into TU traversal. The returned type belongs to the TU; dispose
the symbol with `clang_disposeString`. An alternative is not evidence of DLL ownership.

The qualifier query includes qualifiers inherited through typedefs. After identifying const,
volatile, restrict, and unaligned, it checks the remaining native qualifier set, rather than
assuming those four exhaust Clang's type qualifiers. Address spaces and other undecoded qualifiers
set `WindowsClangOtherQualifiers`. Method qualifiers use the same decoding, independently of the
function type's own qualifiers. The extension ABI version is 20.

`windows_clang_getPointerInfo` decodes native, fixed 64-bit, signed 32-bit, unsigned 32-bit and
other pointer representations. Written pointer-size attributes survive typedef desugaring.
The returned pointee removes only pointer-size address spaces; CVR and unrelated address spaces
remain available to the qualifier query. Canonical native-width forms may normalize to the native
representation. The semantics fixture checks these distinctions on all three MSVC targets.

The virtual-slot query reads `MicrosoftVTableContext::getMethodVFTableLocation`, rejecting
locations that need virtual-base or secondary-vptr adjustment. Nonvirtual methods and non-Microsoft
ABIs return wrong-kind status; dependent or incomplete parents return dependent status.
MSVC overload slots can differ from declaration order, including overloads separated by other
methods. The native fixture checks their exact slots on all three targets. Record base offsets
already come from stock `clang_getOffsetOfBase`; no extra query is needed for value-base layout.

The default-argument query returns a TU-owned expression cursor and caller-owned `CXString`.
No default returns a null cursor and empty string; unparsed/uninstantiated or dependent defaults
return `WindowsClangDependent`. The cursor preserves implicit conversions so it matches the
parameter's actual child rather than an expression guessed from tokens or source offsets.

UUID payloads come from the native attribute, not macro spelling. Dispose their strings with
`clang_disposeString`. Language linkage is distinct from external/internal symbol linkage and
cannot be inferred from a mangled-name heuristic.
Deprecation payloads come from the individual attribute rather than effective platform availability.
Both message and replacement strings are caller-owned, including empty strings.

The integer-only initializer query checks `Expr::isIntegerConstantExpr`. The general query requires
`Expr::isConstantInitializer` and successful `EvaluateAsRValue` with neither `HasSideEffects` nor
`HasUndefinedBehavior`. This accepts compiler-supported static field-offset expressions that are
not C++ core constant expressions, without accepting runtime reads, calls, or signed overflow.
Stock `clang_Cursor_Evaluate` accepts `(call(), 3)` and returns `3`; evaluation alone is insufficient.
The native fixture exercises these distinctions on all three targets, as well as UUID/linkage
results and wrong-kind/null-output errors.

Numeric pointer initializers use a separate static-initialization query: integer-to-pointer casts
need not be C++ constant expressions. It requires `isConstantInitializer`, rejects side effects and
undefined behavior, and accepts only a compiler-evaluated address without a relocation base.
The result is sign-extended from the target pointer width, not the host width. Function pointers
and symbol addresses are rejected.

The `typeof` query returns the original operand before initializer conversions. Its type retains
cast typedefs and string-literal arrays; an initializer cursor can instead carry the synthetic
variable's `typeof` type. Compiler-predefined sugar, such as the result type of `sizeof`, has its own
query. Only native `PredefinedSugarType` is desugared; unknown types and user typedefs are not
canonicalized as a fallback.

The integer-element query requires a C++11 constant expression and traverses the resulting
`APValue` by field and array indices, including implicit array fillers. It accepts nonnegative
integer leaves fitting `u64`; negative values, other leaf kinds, invalid paths, and structs with
bases return wrong-kind status without changing the output. Non-constant initializers return
`WindowsClangDependent`. The caller supplies a variable with an initializer, following the compiler's
definition cursor if needed. No object bytes or source tokens cross this interface.

`windows_clang_getLayoutExpression` identifies constant array bounds through native `TypeLoc`
and bitfield widths through `FieldDecl`, ignoring implicit AST wrappers when comparing expressions.
It does not accept arbitrary expression children, variable bounds, or C array-parameter contracts
such as `static` extents and index qualifiers. `windows_clang_getFieldInitializer` checks
`hasInClassInitializer`, including bitfield initializers omitted by the stock child visitor.

The type-visibility query reads the attribute itself, including implicit attributes with no tokens.
It is distinct from ordinary symbol `VisibilityAttr` and from `clang_getCursorVisibility`, which
reports effective declaration visibility. The native fixture checks all three payloads and a type
whose symbol visibility and type visibility differ.

The typedef query uses `TypedefNameDecl` type-source information and `TagTypeLoc::isDefinition`.
It distinguishes `typedef struct Tag { ... } Name` from references, forward declarations,
alias chains, and pointer/array declarators. Multiple value typedefs in the same definition each
retain this evidence. No declaration ordering, token matching, or source-range heuristic is used
by the Rust capture or projection layers.

The implicit-typedef query reads `TypedefNameDecl::isImplicit()`, not source-location validity.
Compiler-provided aliases can have stable identities and underlying types even when the stock
translation-unit visitor omits their declarations. The native fixture checks such a declaration,
authored typedefs, and wrong-kind/null-output failures on all three targets.

These queries supplement calling convention, argument, result, exception, variadic, and other
stock type queries. A zero extra-property mask is not permission to discard attributed types or
declaration attributes. Consumers retain responsibility for defining their accepted type model.
`windows-clang` captures requested packing and alignment and gates C/stdcall callback prototypes
on these queries.
Unknown attributes and unsupported function properties remain explicit errors or omissions.

Attribute-name strings are borrowed from the library. Query cursors and types must belong to a
live translation unit created by the matching runtime. Wrong-kind and null-output status failures
leave output arguments unchanged. The libraries stay loaded for the life of the process.

## Building and generation

`LIBCLANG_VERSION` in `src/lib.rs` pins LLVM. `native/build.ps1` downloads its archive, builds
the companion with the static MSVC runtime used by the LLVM libraries, and copies both DLLs and the
LLVM license into
`target\tool-clang\native\<version>\<host-triple>\bin`. The initial x64 LLVM archive is about
862 MB. Visual Studio C++ tools and its CMake/Ninja component are required.

```powershell
cargo run -p tool-clang-sys -- --native
cargo run -p tool-clang-sys
```

The first command forces a native rebuild. Helper provisioning invokes the same script with
`-Ensure`, reusing a build only when its freshness stamp matches SHA-256 hashes of every file under
`native`, the LLVM version and host target, and both deployed DLLs. This includes the build recipe
A stale build is cleaned before compilation, so preserved source timestamps
cannot hide changed contents from Ninja. Downloads remain cached.

The script holds an exclusive cache lock through checking, building, and deployment. It removes
the stamp before building and writes it only after successful deployment with unchanged inputs.
Failed builds cannot certify a stale pair. Pinned dependency archives are trusted build inputs;
the stamp does not authenticate modified dependency caches or externally supplied libraries.

The second command regenerates `src/bindings.rs`, which must not be hand-edited. CI regenerates
the bindings and rejects a diff.

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
`tool-clang path` validates the selected pair before printing its path. An explicit external path
is never replaced or rebuilt by provisioning; its producer owns source freshness.
Version checks do not make arbitrary same-version builds interchangeable; distribute the pair
produced by the native build, not independently sourced DLLs.

## Coverage

`test_clang_sys` exercises x86, x64, and ARM64 targets. It distinguishes char-only `pack(1)` and
`pack(2)` records despite identical size/alignment, plain versus explicit-zero and nonzero
`regparm`, `noreturn`, attributed calling conventions, and unsupported prototype properties.
Qualifier cases include CVR, typedef inheritance, `__unaligned`, and address-space detection.
Type-visibility cases include Clang's implicit default attribute on `_GUID`.
It also covers query errors and calls from several threads. A unit test covers missing-library
errors.
`cargo test -p helpers native_freshness` exercises provisioning without an SDK, including input
changes with preserved timestamps, missing or replaced DLLs, and failed builds.

Native x86 processes are unsupported. Cross-target parsing from a supported host does not require
an x86 DLL. ARM64 host execution is covered by the ARM64 CI job, not by parsing an ARM64 target
from an x64 process.
