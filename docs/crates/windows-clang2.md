# windows-clang2

An unpublished native capture and metadata projection experiment. See the
[readme](../../crates/libs/clang2/readme.md) for usage.

`runtime` owns compiler sessions, `capture` builds owned IR, `native` wraps compiler queries, and
`resolve` checks same-target native record and enum observations. `merge` associates groups across
targets without collapsing layouts or type edges. Each worker reuses a precreated index across
TUs. Snapshots retain declarations separately from entities; resolution borrows them rather than
overwriting evidence. Type and cursor lookup use hash indexes. File paths are interned.

The optional `projection` module maps merged native types to public names, header partitions,
architecture variants, and RDL. It checks sequential struct and explicit union layouts against
captured ABI evidence. Native records retain their struct/union kind, including forwards.
Union fields must have offset zero; size is the largest field size rounded to the union alignment.
Nested records and bitfields remain unsupported rather than flattened or assigned synthetic names.
Its target-local plan separates native record identity from public alias correspondence:

- Only unqualified by-value alias bindings connect candidate record families. Pointer and array
  aliases do not establish pointee/element equivalence.
- Complete definitions must agree in record kind, field names/order, offsets, packing, alignment
  requests, layout, and canonical types/qualifiers. Record edges must belong to independently
  established families.
- The alphabetically first by-value alias names each family; other reachable value names are transparent
  aliases. A name can be a record on one architecture and an alias on another.
- Alias ownership prefers a unique declaration header containing the corresponding native
  definition; otherwise its declaration header must be unique. Ambiguity is an error or omission.

All alias bindings are checked before a representative is used. Native and public comparison share
the same type checker with different record relations. The plan preserves native identities.

Partial capture records failed entities, follows reverse dependency edges across snapshots, and
compacts the surviving IR before reconciliation. It does not reparse individual declarations.
Unsupported tag and alias kinds are excluded across targets. Diagnostic causes are deterministic;
workers deduplicate diagnostics between TUs, and dependency edges are deduplicated on insertion.

Records and typedef/using aliases retain written and canonical types, attributes, and layouts.
Every type is checked through the native qualifier query before normalization or interning.
Only const, volatile, and restrict enter the IR; `__unaligned` and all other native qualifiers
are explicit capture failures, including qualifiers inherited through typedefs.
Typedef USRs can contain filenames or match across different canonical record bindings. Native alias
groups therefore preserve name-corresponding observations without claiming type equality; projection
checks their compatibility using public families. Macro declarations belong to their outer expansion
site, including token-pasted names. Configured roots include their dependency closure.
Compiler errors remain fatal. Unresolved attributes block even a single observation: matching
layouts or matching opaque cursor kinds do not prove identical packing/alignment requests.

Fixtures and callers live in `crates/tests/libs/clang2`, using only the public API. Add headers
under `input` and assertions under `tests`. The runner provides compact inspection and synthetic
scaling:

```text
cargo test -p test_clang2
cargo run -p test_clang2 -- inspect shared_a shared_b
cargo run --release -p test_clang2 -- scale
cargo test -p test_clang2 --test sdk -- --ignored
cargo test -p test_clang2 --test alignment -- --ignored
cargo run --release -p test_clang2 -- sdk-gate
cargo run --release -p test_clang2 -- sdk-scale 32
cargo run --release -p test_clang2 -- sdk-sweep breadth 8
cargo run --release -p test_clang2 -- sdk-sweep tus 32
cargo run -p tool-win32-2 --quiet
cargo run -p tool-win32-2 --quiet -- --partial
cargo run -p tool-win32-2 --release --quiet -- --partial --first=357
```

`tool-win32-2` is the edit/rerun loop: parse SDK headers on all three targets, merge, project,
compile, then replace `metadata/win32` wholesale. It shares `tool-win32`'s `src/headers.rs`
manifest and defaults to all headers. Main and satellite TUs follow the SDK's include rules.
`--first=N` selects a manifest prefix; positional arguments select headers, not types.
Strict mode fails on unsupported declarations. `--partial` emits the supported subset and
writes `target/win32-2/omissions.txt`; compiler errors and retained-definition conflicts remain
fatal. Both modes compile before replacing generated RDL; the directory's readme is preserved.
The partial WinMD stays under `target/win32-2`. Git's metadata diff is a review baseline, not a
correctness oracle; fixtures check native layouts and the RDL/WinMD round trip independently.
The omission report covers AST declarations, not uncaptured macro constants.

The full manifest includes `UNICODE_STRING` bindings to both `_LSA_UNICODE_STRING` and
`_UNICODE_STRING`. Native merge preserves both identities; publication checks their definitions
and emits the corresponding aliases. The `record_alias_a/b` and `publication_*` fixtures cover
this model, including recursive records, nominal pointer distinctions, equal-layout conflicts,
input-order independence, and architecture-specific record/alias variants.
The `union_*` fixtures cover union aliases, recursive forwards, packing, arrays, containing structs,
architecture-specific record kinds, and RDL/WinMD layout round trips. Partial exclusions propagate
from unsupported union fields across all targets.

Enums use separate typed IDs and observation tables, not integer aliases or synthetic records.
Native reconciliation checks backing types, layouts, scopedness, attributes, and ordered members.
It checks forward declarations as well as definitions. Public enum names use the first by-value
alias alphabetically, but aliases cannot equate distinct native enums. Scoped enums retain
`ScopedEnumAttribute`; no flags are inferred from numeric patterns. The `enum_*` fixtures cover
all eight integer widths/signs, 64-bit limits, expressions, duplicate values, empty definitions,
forwards, nominal conflicts, and architecture-specific backing types. Partial pruning uses shared
tag names so a failed enum also excludes a same-named record on another architecture.

The SDK case uses the `tool-win32` SDK pin and cached clang resource headers (or
`CLANG_RESOURCE_DIR`).
`sdk-gate` checks 26 SDK roots on all three targets and exits unsuccessfully if any are blocked.
`sdk-scale` measures 15 supported roots with 2, 8, or 32 TUs, checking comparison-work bounds.
`sdk-sweep` uses complete SDK header groups without type allowlists: breadth combines 1/2/4/8 groups
in one TU; tus distributes them over 2/8/32 TUs. Artifacts go under `target/clang2-review`, not
canonical metadata. Its merge timing is a separate probe; projection includes its own merge and
pruning passes. Public-family and alias checks are included in projection timing, not native merge
comparison counts. Native counts and owned-byte estimates are measured after capture pruning.
Unresolved attributes such as `Uuid` remain evidence, not verified semantics.

`TypeVisibility` is decoded by the native companion, including Clang's implicit default attribute
on `_GUID`. Only the default request enters the IR as `DefaultTypeVisibility`; hidden/protected
requests are explicit capture failures. Native definitions and public families compare its
presence, and an explicitly attributed record forward requires the same request on its definition.
The attribute controls visibility of a type and its associated C++ linkage objects, not field layout.
Supported plain records and integer enums do not project RTTI or vtables, so default type visibility
needs no RDL attribute. Ordinary symbol `Visibility` remains separate and unsupported; accepting
default type visibility does not authorize discarding other attributes.

The `visibility*` fixtures cover tokenless implicit attributes, records, unions, enums, forwards,
same-layout native/public conflicts, dependency omissions, RDL/WinMD round trips, and generated Rust.
The SDK fixture also captures and projects `_GUID` with its native size, alignment, and fields.

The native query boundary is visible in `opaque_packing.h`: the same named record under `pack(1)`
and `pack(2)` has identical fields and layout, and libclang reports the same tokenless
`UnexposedAttr` with no source location. The companion identifies `MaxFieldAlignment` and returns
8 versus 16 requested bits. Native reconciliation and public-family checks reject that difference
even when the layouts match. Projection emits the requested `#[packed(N)]` and checks every field
offset, final size, and alignment. The `packing` fixtures cover requests from 1 through 16 bytes,
unions, nested records, and preservation through RDL/WinMD round trips.
Clang's [MaxFieldAlignmentAttr declaration][max-field-alignment] has an alignment argument but
no spelling; the [libclang C API][clang-c-api] exposes layout queries, not that argument.
The [windows-clang-sys](windows-clang-sys.md) extension exposes this argument for owned capture.

Explicit alignment uses the companion's `AlignedAttr` query. Capture retains evaluated bits and
the GNU, standard, or declspec form, including zero and multiple requests. Native and public-family
checks compare sorted, deduplicated request sets, not just their maximum or resulting layout.
An aligned forward must have its requests present on the complete definition; an unadorned
forward does not assert absence.

Complete structs and unions emit the maximum nonzero request as `#[align(N)]`, within RDL's
`u16` limit. Layout verification respects over-aligned fields and checks offsets, size, and final
alignment against native evidence. The `alignment_*` fixtures cover all three parsing targets,
RDL/WinMD round trips, same-layout request conflicts, and generated Rust layout assertions on
the host. The ignored alignment test compiles those assertions for all three MSVC targets and
requires their Rust standard libraries to be installed. Field, typedef, and enum alignment remain
unsupported, including primary record aliases and non-value typedefs. Combined packing/alignment
and packed containment of aligned records are rejected; Rust disallows that containment even
through arrays and intermediate records.
Partial projection removes these declarations and their dependents rather than losing alignment.

The runtime is pinned through `windows-clang-sys::LIBCLANG_VERSION`. LLVM 23.1.3's stock C API
also lacks the packing payload accessor, so newer Rust bindings alone cannot expose it.
Runtime and matching resource-header upgrades are separate from declaration coverage changes.

[ClangSharp's native extension][clangsharp-extension] exposes precise attribute kinds through
`clangsharp_Cursor_getAttrKind`. Its `clangsharp_Cursor_getMaxAlignment` calls
`Decl::getMaxAlignment`, which reads explicit `AlignedAttr` requests, not `MaxFieldAlignmentAttr`.
The sys crate adds packing and per-attribute alignment queries to a version-matched native companion
without replacing the owned IR, reconciliation, or projection. Both parser crates use its generated
stock bindings.

Rapid iteration can continue on declarations whose semantics are available through the current
API. Keep opaque attributes and their dependent declarations in the omission report. Revisit this
boundary before adding support that depends on those attributes or claiming complete coverage;
never infer packing or discard unknown attributes to make a batch pass.
An attribute's native kind identifies what still needs decoding; it does not justify discarding
an unknown payload.

With the pinned runtime and attributed-type capture enabled, the `opaque_callbacks.h` x86 probe
gives:

| Declaration | Written function kind | Calling convention | Attribute cursor children |
| --- | --- | --- | --- |
| Plain callback | `FunctionProto` | C | None |
| Explicit `__cdecl` | `Attributed` | C | None |
| Explicit `__stdcall` | `Attributed` | stdcall | None |
| `regparm(2)` callback | `FunctionProto` | C | None |
| `noreturn` callback | `FunctionProto` | C | None |

The plain and `regparm(2)` callbacks have identical queried result types, parameter types,
variadic flags, and exception specifications, but different canonical Clang types and x86 argument
passing. A compiler `static_assert` in the fixture checks the type distinction. Their spellings
also differ; neither spelling nor type equality supplies a decoded attribute payload for the IR.
Rejecting `CXType_Attributed` alone therefore does not make function-prototype capture safe.
`windows_clang_getFunctionInfo` exposes these hidden properties and flags additional prototype
information that callers must handle or reject. The sys fixtures verify explicit `regparm(0)`
versus absence, nonzero `regparm`, `noreturn`, and method-qualifier detection.
Callback capture accepts plain C and stdcall prototypes only after the supplementary query returns
no unsupported properties. Explicit `regparm(0)` is rejected as well as nonzero `regparm`;
`noreturn`, other extra properties, variadics, exception specifications, and parameter annotations
remain unsupported. Known cdecl/stdcall attributed types retain their written and equivalent types.

Owned function types retain result and parameter edges plus calling convention. Native/public type
comparison follows all signature edges, not just pointer size. Partial pruning likewise follows
every result and parameter dependency, including nested callbacks. These edges can branch, unlike
a pointer or array chain.

Projection preserves written value-typedef and callback names in fields and callback signatures,
including pointers, arrays, results, and nested named callbacks. Reachable value aliases are
published even when they are not explicit roots. Alias declarations retain canonical storage types;
non-value aliases such as `void` expand at use sites. Named callback typedefs become RDL `extern fn`
declarations with positional parameter names. Unnamed function-pointer types use the alphabetically
first compatible callback name across the target.
The plan validates public families and alias bindings before comparing callback candidates with
the shared type checker. Calling convention and arity narrow the search, but every candidate must
match qualifiers, layouts, scalar kinds, nominal enum identities, public record families, and all
signature edges. Failed candidate comparisons do not share assumed type pairs with later attempts.
Renaming or reordering TUs cannot change name availability. Unnamed signatures without a
corresponding declaration are omitted. Bare function aliases are not projected as values.
The output's alias count includes callback declarations.

A failed value-alias or callback projection removes the declaration and its dependents. Removing
only its root marker would not make progress, since dependency publication would select it again.
Non-value aliases can remain as native evidence without being public roots. The callback
fixtures cover this distinction, C/stdcall metadata ABI values, nested results/parameters, arrays,
pointer indirection, same-layout native signature conflicts, and cross-target omission propagation.
`callback_qualifiers.h` proves that plain and unaligned callback types differ on x64/ARM64.
`callback_lookup.h` covers inline callback fields whose named counterparts exist only in another TU,
including equal-layout but distinct scalar and record parameter types.

After native compatibility is checked, every same-target record and callback observation must
produce the same public declaration. This additional output check reports name disagreements
instead of selecting a representative's spelling; it does not establish native equivalence.
`public_names.h` and its RDL golden check named scalar and pointer aliases, callback synonyms, and
dependencies through WinMD and compiled Rust bindings. The conflict and ownership fixtures cover
public-name disagreements and omitted named dependencies.

Native reconciliation is not a public API compatibility guarantee. Callback parameter names remain
positional. Before replacing a production consumer, compare its header names, RDL, WinMD, and
generated Rust against an explicit output contract. A successful partial SDK run proves only the
accepted subset; omissions and uncaptured macro constants remain outside that claim. New annotations
need both capture and consuming projection support, not just a place to store them.

Public ownership across headers remains unresolved. Equivalent scalar typedefs can have several
declaration headers, so preserving their names exposes ownership omissions in their dependents.
Revisit a shared foundation module or RDL partition after the rewrite is proven. No per-name header
preferences or canonical-metadata ownership routing are implemented.

Canonical record naming also remains unresolved. Selecting the alphabetically first by-value alias
can give an application-specific alias authority over a shared type: `_GUID` becomes
`BATTERY_CHARGER_ID` from `poclass.h`, with `GUID`, `IID`, and `CLSID` referring to that definition.
This is not canonical metadata parity even though the native layouts agree. Resolve this through
declaration evidence and an explicit public naming contract, not GUID-specific exceptions or
another ordering heuristic.

[max-field-alignment]:
  https://github.com/llvm/llvm-project/blob/llvmorg-22.1.8/clang/include/clang/Basic/Attr.td
[clang-c-api]:
  https://github.com/llvm/llvm-project/blob/llvmorg-23.1.3/clang/include/clang-c/Index.h
[clangsharp-extension]:
  https://github.com/dotnet/ClangSharp/blob/0c1c4098a7aab7cb392a2367b75d447bbac05d58/sources/libClangSharp/ClangSharp.cpp

The scaling report separates capture time, merge time, and comparison counts. Owned-byte
estimates cover snapshot capacities, not native memory, allocator overhead, or resolution tables.
