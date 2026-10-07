# windows-clang2

An experimental replacement for `windows-clang`, built directly on
libclang. It drives `tool-webview`; the other production scrapers still use the existing crate.

The prototype captures selected native declarations and their dependencies into an owned graph,
checks agreement across translation units, and builds a closed RDL projection plan. It retains
separate redeclaration observations, written typedef edges, canonical callable types, native layout,
and annotation origins. Projection cannot bypass native resolution.

```rust,no_run
use windows_clang2::{Input, ProjectionOptions, capture};

let snapshot = capture(
    [Input::new("api.hpp", "struct Packet { int value; };")],
    &["-x", "c++", "--target=x86_64-pc-windows-msvc"],
    &["Packet"],
)?;
let resolved = snapshot.resolve()?;
assert!(resolved.report().incomplete.is_empty());
let plan = resolved.project(&ProjectionOptions::new("Test"))?;
println!("{}", plan.rdl());
# Ok::<(), windows_clang2::Error>(())
```

The caller must make libclang available through `LIBCLANG_PATH` or its normal loader search paths.
Repository tests and the inspection example use the shared pinned dependency helper.

`discover` inventories specified headers by exact Clang file identity, leaving selection policy to
the caller. After the complete captured graph passes resolution, `Resolved::project_roots` can
assess selected subsets without reparsing or suppressing native conflicts. Combined projection
still checks alias/output-name conflicts. `Plan::rdl_by_header` emits source-owned partitions,
including dependency headers, separately from native identity and spelling evidence.

Projection currently covers ordinary C-layout records, integer-backed enums, fixed-prototype
functions, scalar and raw pointer constants, UUID-bearing local COM interfaces, and external type
bindings. Local interfaces support a single base and pure virtual system-ABI methods, but reject
by-value record results until the downstream aggregate-return ABI is covered. Same-name COM methods
are rejected until native vtable ordering is covered; disambiguating Rust names is not enough.
Inherited-slot overrides are captured but rejected until projection models slot reuse.
Explicit `clang::flag_enum` and `DEFINE_ENUM_FLAG_OPERATORS` markers survive without changing enum
representation. MIDL `propget`/`propput` comments preserve property semantics without renaming
native methods or guessing from a `get_`/`put_` prefix.

Increased member/record alignment preserves native storage using padding unions and alignment
attributes. By-value calls involving adjusted layouts reject until their calling ABI is covered.
Explicitly selected aliases can name record types unless their named tag is also selected;
competing selected names reject.
Fixed-size arrays retain their element types, extents, and native layout. Externally bound record
fields retain native storage layout under the caller's ABI contract; by-value calls involving
external records or adjusted layouts remain rejected, including through arrays and local records.
Caller-supplied `ProjectionOptions::imports` maps native linker symbols to `FunctionImport` DLL
and export names without changing captured evidence. Unmapped functions require an explicit
`library` fallback; the BCrypt and WebView2 fixtures instead derive imports from target-specific SDK
COFF libraries.

Fully specified struct/array initializer lists retain compiler-evaluated values in the native graph.
Projection supports GUID and property-key storage shapes, checking component types, offsets, sizes,
and alignment. RDL and generated Rust preserve their native records and field names. Partial,
dynamic, copied-record, and other unsupported initializers remain explicit failures. Callers can
capture an SDK's definition mode alongside declaration-only inputs; no macro-name or argument-text
parser supplies the values. Conflicting initialized observations still fail native resolution.

Supported parameter annotations include direction, optional pointers and buffers, COM output
pointers, and parameter-bound element/byte counts. Constant element counts support nonnegative
decimal literals.
Output byte annotations preserve both capacity and the valid-byte extent on success, including
counts returned through writable integer pointers. Their `MemoryWrittenAttribute` definition is
in this crate's `metadata.rdl`; supply it when compiling the emitted RDL. This is an experimental
metadata extension, not part of the bundled default WinMD. Wrappers retain unsafe output pointers;
callers still check statuses, returned lengths, and API-specific retained-buffer lifetimes.
Scalar null-terminated strings require caller-supplied `ProjectionOptions::string_references`,
keyed by `StringKind`. These trusted metadata value types preserve native constness independently
of parameter direction; no string namespace or local alias is assumed.
Caller-bound scalar and pointer typedefs retain their identity through
`ProjectionOptions::references`. For example, an explicit `HRESULT` value binding enables generated
COM result wrappers without treating every native `long` as an error code. Conflicting typedef
contracts across observations fail projection even when their native types agree. Pointer bindings
preserve handles and string aliases without folding them into surrounding raw-pointer levels. SAL
string annotations still require a matching string binding.

Declaration-local MIDL prefix comments supply input/output direction when SAL has none. SAL controls
local-call direction; evidence from each source family must independently agree across declarations.
Parameterized MIDL relationships, optionality, and retval markers are not decoded.
Output/inout mutation through a single interface object pointer projects as a borrowed input, not
a writable interface slot. An additional native pointer level retains its output/inout direction.
Interface-valued constants have an omission reason in `Plan::omitted`; unsupported projections are
errors. General SAL lowering and integration with the full Win32 scraper remain open.

The test crate generates raw Rust bindings and links a C++ fixture to check record layout,
free-function aggregate calls, and inherited virtual dispatch in both directions on x64 and x86.
It also generates normal Rust wrappers from SDK `IUnknown`/`IClassFactory` declarations and checks
reference counts, identity, output ownership, and failure paths against a C++ implementation.
Pinned WebView2 fixtures cover all 79 roots in the production binding filter across main and interop
translation units. Native evidence includes anonymous aggregate fields and their dependency graphs;
local anonymous-aggregate projection remains unsupported. Explicit SDK bindings for `IStream` and
`VARIANT` let the interop graph project after its native evidence has been checked.

Real pinned WDK member types are checked against MSVC for every offset, size, and alignment, then
passed through native pointer calls. Generated BCrypt wrappers exercise a full hashing lifecycle,
caller-owned and CNG-owned storage, optional and embedded-NUL byte buffers, output guards,
independent hash/HMAC/PBKDF2 answers, and failure statuses.
These cases execute on x64/x86 in debug and release; ARM64 has capture/metadata coverage only.

The consumer gate preserves six handle inputs and two host-object inputs, with exact generated
wrapper-signature assertions. A native C++ fixture checks handle setters/getters and UTF-16 string
inputs on x64 and x86. `webview_consumer` generates the exact production filter into a fresh scratch
copy of `windows-webview`, then checks every feature combination without rewriting that copy's
application code. The consumer's all-bits constant uses `!0` with signed or unsigned bindings.
The pinned loader DLL also executes version comparisons and failure cases on x64/x86 in debug and
release. The actual `tool-webview` also uses clang2 to generate the committed bindings. All 33 live
`test-webview` fixtures and the x64 WinUI-to-COM bridge pass with those bindings. The ongoing
viability gate is x64 debug, not a full architecture certification matrix. Broader annotations and
other production consumers still need coverage. See the continuation page for commands and open
gates.

`tool-win32 --clang2-audio` is a bounded x64 path through the real main/satellite input assembly for
`mmdeviceapi.h` and `endpointvolume.h`. Header discovery feeds a persistent declaration-outcome
report and per-header RDL, then an isolated consumer reads real audio endpoint state without
changing settings. An additional SDK definition-mode input supplies all 18 property keys and four
device-interface GUIDs. The supported candidate emits 64 selected names; 38 remain rejected, including
declaration-only data, a coclass, and an ordinal import. The generator exits nonzero for incomplete
coverage even when the supported consumer works. The full Win32 scraper and committed metadata
remain on their existing path.

The API and diagnostic dump are experimental. See
[`docs/crates/windows-clang2.md`](../../../docs/crates/windows-clang2.md) for the supported subset,
validation contract, and remaining prototype work.
