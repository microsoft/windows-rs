# windows-clang2

An experimental replacement for `windows-clang`, built directly on
libclang. The existing crate and production generators are unchanged.

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

Projection currently covers ordinary C-layout records, integer-backed enums, fixed-prototype
functions, scalar and raw pointer constants, UUID-bearing local COM interfaces, and external type
bindings. Local interfaces support a single base and pure virtual system-ABI methods, but reject
by-value record results until the downstream aggregate-return ABI is covered. Supported
parameter annotations include direction, optional pointers and buffers, COM output pointers, and
parameter-bound element/byte counts. Constant element counts support nonnegative decimal literals.
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
Interface-valued constants have an omission reason in `Plan::omitted`; unsupported projections are
errors. General SAL lowering and production generator integration are not implemented.

The test crate generates raw Rust bindings and links a C++ fixture to check record layout,
free-function aggregate calls, and inherited virtual dispatch in both directions on x64 and x86.
It also generates normal Rust wrappers from SDK `IUnknown`/`IClassFactory` declarations and checks
reference counts, identity, output ownership, and failure paths against a C++ implementation.
Pinned WebView2 fixtures cover all 79 roots in the production binding filter across main and interop
translation units. Native evidence includes anonymous aggregate fields and their dependency graphs;
local anonymous-aggregate projection remains unsupported. Explicit SDK bindings for `IStream` and
`VARIANT` let the interop graph project after its native evidence has been checked.

The consumer gate preserves six handle inputs and two host-object inputs, with exact generated
wrapper-signature assertions. A native C++ fixture checks handle setters/getters and UTF-16 string
inputs on x64 and x86. Remaining flags-enum and output-name differences still prevent drop-in
replacement; broader annotation and production-consumer coverage is also required. See the
continuation page for differential commands, remaining gates, and timing/memory measurements.

The API and diagnostic dump are experimental. See
[`docs/crates/windows-clang2.md`](../../../docs/crates/windows-clang2.md) for the supported subset,
validation contract, and remaining prototype work.
