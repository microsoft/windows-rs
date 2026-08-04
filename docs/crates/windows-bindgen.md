# windows-bindgen

> The code generator that turns Windows metadata (`.winmd`) into Rust bindings.

- 📦 [crates.io](https://crates.io/crates/windows-bindgen)
- 📖 [docs.rs](https://docs.rs/windows-bindgen)
- 🚀 [Getting started](../../crates/libs/bindgen/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/bindgen)

`windows-bindgen` generates Rust bindings from Windows metadata. It powers the `windows` and
`windows-sys` crates. You can also use it from `build.rs` to make a small binding set for the APIs
that your crate calls.

The crate includes the standard Windows metadata. Most projects only need an output file and a
filter. If an API has no metadata, use [`windows-rdl`](windows-rdl.md) to create a `.winmd` file.
Then pass that file to `windows-bindgen`.

## Getting started

Add `windows-bindgen` as a build dependency. Add `windows-link` or `windows-core` as the runtime
dependency that the generated code uses:

```toml
[dependencies]
windows-link = "0.100"

[build-dependencies]
windows-bindgen = "0.100"
```

Generate bindings from `build.rs` with either command-line-style arguments or the builder:

```rust,no_run
windows_bindgen::bindgen([
    "--out", "src/bindings.rs",
    "--flat",
    "--sys",
    "--filter", "GetTickCount",
]);
```

```rust,no_run
windows_bindgen::Bindgen::new()
    .output("src/bindings.rs")
    .flat()
    .sys()
    .filter("GetTickCount")
    .write();
```

Include the generated file as a module in your crate.

## Filters

A filter selects which APIs appear in the output. Rule specificity controls how much of a type is
generated. This works like a Rust `use` declaration. Use a bare name for the full item. Use braces
to select a smaller surface.

- A namespace, such as `Windows.Win32.System.Com`, includes all types under it.
- A bare type includes the full type. Examples are `HWND`, `OSVERSIONINFOEXW`, and
  `Windows.Win32.Foundation.HWND`.
- `Namespace.Type::{}` emits a name-only shell. Use it for a dependency that you only pass through
  signatures.
- `Namespace.Type::{Method1, Method2}` emits only the named methods. `Namespace.Type::Method` is the
  single-method form.
- `Property` and `Event` names expand to accessor pairs. Properties expand to `get_` and `put_`.
  Events expand to `add_` and `remove_`.
- `Namespace.Class::CreateInstance` emits class activation support. A bare class projects its
  default interface but no constructor.

Prefix a rule with `!` to exclude it. A selected type also pulls in the types that its signatures
require. Those dependency types are emitted as shells.

For more than a few names, keep the arguments in a response file. Pass it with `--etc`. Lines that
start with `//` are comments:

```text
--out crates/libs/version/src/bindings.rs
--flat --sys

--filter
    RtlGetVersion
    OSVERSIONINFOEXW
    VER_NT_WORKSTATION
```

```rust,no_run
windows_bindgen::bindgen(["--etc", "bindings.txt"]);
```

The in-repo crates use this pattern. `tool_bindings` runs
`bindgen(["--etc", "crates/tools/bindings/src/<crate>.txt"])` for each library.

## Choosing the output shape

Two independent choices control the generated code. The first choice is style. The second choice is
layout.

Style:

- Default style emits rich bindings. It includes class wrappers, inherited-interface forwarders,
  handle types, and free-function wrappers. The `windows` crate uses this style.
- `--sys` or `.sys()` emits raw FFI. It emits bare `extern` functions and plain structs. It links
  through `link!` macros. Add `--extern` or `.extern_fns()` to emit `extern { fn ... }` blocks
  instead of `link!`. The `windows-sys` crate uses this style.
- `--minimal` or `.minimal()` starts from default style. It omits per-class wrappers, inherited
  forwarders, handle helpers, and free-function wrappers. Use it for small binding sets.
  `windows-canvas` and `windows-reactor` use it. It is mutually exclusive with `--sys`.

WinRT event accessors are always collapsed into an `Event` wrapper. This applies to all styles and
layouts. See [Event accessors](#event-accessors).

### Variadic native functions

Native variadic exports carry `MethodCallAttributes::VARARG` in the method signature. Only
`--sys` emits them, because sys output can retain the literal `...` tail in a raw foreign
declaration. Both `link!` output and `--sys --extern` preserve metadata `C` and `system` calling
conventions. Rust lowers a Windows `system` C-variadic declaration to the compatible C variadic ABI
on X86 while retaining `system` for fixed signatures.

Default and minimal output cannot forward an unknown variadic tail through a Rust wrapper. Broad
filters omit those exports. Selecting one by exact function name reports that rich and minimal
bindings cannot project it and directs the caller to `--sys`; it never emits the fixed prefix as a
callable function. Stable Rust cannot declare a `fastcall` C-variadic function, so broad sys
generation omits that metadata shape and exact selection reports the unsupported convention.

The published `windows-sys` crate therefore retains raw declarations such as
`AuthzReportSecurityEvent(...)`, while the `windows` crate omits the 31 Win32 and WDK variadic
exports that previously appeared as unsafe fixed-prefix wrappers.

Layout:

- The default layout emits one Rust module per metadata namespace.
- `--flat` or `.flat()` emits one flat list of items.
- `--package` or `.package()` emits one file per namespace. It also writes a `Cargo.toml` with
  per-namespace features. The `windows` and `windows-sys` crates use this layout. It is mutually
  exclusive with `--flat`.

The style and layout choices are independent. The repository uses only the combinations below.

| Style + layout        | Purpose                                           | Examples                                  |
| --------------------- | ------------------------------------------------- | ----------------------------------------- |
| default + `--flat`    | Helper crate with one bindings file              | `windows-collections`, `windows-future`   |
| default + `--package` | Published umbrella crate                         | `windows`                                 |
| `--sys` + `--flat`    | Raw FFI helper crate with one bindings file      | `windows-result`, `windows-registry`      |
| `--sys` + `--package` | Published raw FFI crate                          | `windows-sys`                             |
| `--minimal` + `--flat`| Small binding set                                | `windows-core`, `windows-canvas`, `windows-reactor` |
| any + modules         | Namespace-per-module output for direct consumers | External binding generation               |

`--minimal` and `--package` are not used together. Minimal output targets small binding sets.
Package output targets the full API surface.

### Empty modules in package mode

In `--sys --package` mode, a namespace can contain only COM interfaces. Raw FFI style emits no
interface bodies, so the namespace has no items. `write_package` prunes that empty namespace. It
removes the module declaration, file, Cargo feature, and feature dependency references.

Pruning is recursive. A parent namespace is pruned only when it and all children are empty. This
applies only to `--sys`. The full `windows` crate emits interfaces, so those modules are not empty.

### Event accessors

Each WinRT `add_X` and `remove_X` pair becomes one method:

Event add methods return `Result<EventRevoker>`.

The method takes the closure directly. It returns an
[`EventRevoker`](https://docs.rs/windows-core/latest/windows_core/struct.EventRevoker.html). The
revoker calls the matching `remove_X` slot on drop. Call `.forget()` or `.into_token()` to opt out.

This rule changes only the consumer side. Implementing an event source still requires both `add_X`
and `remove_X`.

### Other useful options

- `--in`, `.input(..)`, and `.inputs(..)` add `.winmd` files or directories. Use the literal
  `"default"` to include the bundled Windows metadata.
- `--derive` and `.derive(..)` add derives to generated types.
- `--implement` and `.implement(..)` emit `_Impl` scaffolding for WinRT interface implementations.
- `--rustfmt` and `.rustfmt(..)` set the formatter for the output.
- `--dead-code` and `.dead_code()` emit `pub(crate)` for callable items. This lets the compiler flag
  unused generated callables.

## Committing generated bindings

A `build.rs` can regenerate bindings on each build. Published crates usually use a different
pattern. Commit `src/bindings.rs` as source. Depend only on [`windows-link`](windows-link.md) at
runtime. Consumers then build without code generation, metadata files, or a `windows-bindgen`
dependency.

The pattern has three parts.

**1. The published crate depends only on `windows-link`** and includes the committed bindings:

```toml
# tickcount/Cargo.toml
[dependencies]
windows-link = "0.100"
```

The library module includes the generated bindings and exposes the safe API used by the binary.

**2. A separate, unpublished binary owns code generation.** Keep it as a workspace member. It does
not become a dependency of the published crate:

```toml
# gen/Cargo.toml
[package]
name = "gen"
publish = false

[dependencies]
windows-bindgen = "0.100"
```

```rust,no_run
// gen/src/main.rs
windows_bindgen::bindgen([
    "--out", "tickcount/src/bindings.rs",
    "--flat",
    "--sys",
    "--filter", "GetTickCount64",
]);
```

`--out` is resolved relative to the current directory. Run the tool from the workspace root:

```sh
cargo run -p gen
```

**3. A CI check keeps the committed bindings current.** Regenerate, then fail if the result differs
from the checked-in file:

```yaml
- run: cargo run -p gen
- run: git diff --exit-code
```

This repository uses the same arrangement.
[`tool_bindings`](https://github.com/microsoft/windows-rs/tree/master/crates/tools/bindings)
regenerates each crate's `bindings.rs` from a `.txt` filter. The
[`gen.yml`](https://github.com/microsoft/windows-rs/blob/master/.github/workflows/gen.yml) workflow
runs the tools and rejects any diff.

## Consuming APIs outside the default projection

The published `windows` crate projects public, documented APIs behind Cargo features. Some consumers
need a smaller slice or an API that is not in public metadata.

Use `windows-bindgen` for these cases instead of expanding the `windows` crate.

- If the API is public but belongs to a broad feature, generate a small binding set with a filter.
  For example, a crate can select `IPropertyStore` and `PROPVARIANT` without enabling the full
  feature surface that contains them.
- If the API is not in public metadata, author metadata with [`windows-rdl`](windows-rdl.md). Then
  feed that metadata to `windows-bindgen`. This keeps the FFI surface generated and typed.

---

## Internal documentation

The rest of this page covers how the crate is built and maintained. It is for contributors and is
not needed to use `windows-bindgen`.

### How it's built

`windows-bindgen` is hand-written. It is the generator that other crates use. It reads ECMA-335
metadata through [`windows-metadata`](windows-metadata.md). The bundled metadata inputs live in
the [`windows-default`](windows-default.md) crate.

Two tools drive it in this repository:

- `tool_bindings` reads the per-crate `.txt` filters in `crates/tools/bindings/src`.
- `tool_package` produces the published `windows` and `windows-sys` crates.

### Output policies

The generator models output style as named policies. These policies keep style checks in one place
and make call sites describe intent.

- `Style::emit_class_methods` emits per-class wrapper methods.
- `Style::emit_inherited_forwarders` emits inherited-interface forwarders.
- `Style::emit_iterable_into_iterator` emits the `IntoIterator` bridge for an inherited
  `IIterable<T>`.
- `Style::minimal_string_input` and `Style::minimal_string_return` expose `HSTRING` parameters and
  returns as `&str` and `String`.
- `Config::emit_runtime_name` emits the WinRT `NAME` runtime-name constant.
- `Style::derive_std_traits` emits `Default`, `Debug`, and `PartialEq` derives.
- `Style::emit_core_traits` emits the `windows-core` trait block.
- `Style::emit_bare_typedef` emits handle structs and unscoped enums as type aliases.

`--dead-code` visibility is centralized in `Config::item_vis()`. It is used for callables such as
methods and delegate constructors. Nameable public items stay `pub`, because hand-written crates can
re-export them or reference them from exported macros.

Repeated layout helpers also live on `Config`:

- `Config::doc_hidden_in_package` emits `#[doc(hidden)]` in package mode.
- `Config::write_value_name_const` writes the `RuntimeType::NAME` constant for value types.

### Type selection

For precise filters, `TypeClosure::build` starts from the selected types and walks signature
dependencies. It emits selected entry points as full types. It emits dependency types as shells
unless they are selected directly.

For broad filters and package generation, `TypeMap::filter` scans namespaces from the top down. This
is used for full namespace and package output.

The `--minimal` flag affects rendering only. It does not change which referenced types are included.

### WinRT and Win32 code generation

The metadata reader classifies types as WinRT or Win32/COM from the metadata type attributes. Shared
code handles names, signatures, dependencies, and type remapping. The writers stay separate where
the ABI rules differ.

The main differences are:

- WinRT vtable methods return `HRESULT`. The projection wraps them in `Result`.
- COM methods keep their native return shape. `ReturnHint` controls the projected shape for common
  COM patterns.
- WinRT supports generics, runtime signatures, activation, and `RuntimeType`.
- WinRT delegates are COM interfaces with `Invoke`. COM callback types can be raw function pointers.
- Win32 also has free exports, constants, handles, unions, nested types, and architecture-specific
  layout.

Some writer pieces are shared. Interface vtable method fields and `_Impl` method iteration use
common helpers. Enum constant and flag operator emission also use common helpers.

### Generating bit-field accessors

Win32 structs frequently pack several logical members into one storage unit with C bit-fields:

```c
typedef struct _MIB_IF_ROW2 {
    // ...
    struct {
        BOOLEAN HardwareInterface : 1;
        BOOLEAN FilterInterface : 1;
        BOOLEAN ConnectorPresent : 1;
        // ...
    } InterfaceAndOperStatusFlags;
} MIB_IF_ROW2;
```

The winmd format has no bit-field concept. The scrape coalesces each run of bit-fields into one
backing integer field named `_bitfield`. If a struct has more runs, the fields are named
`_bitfield1`, `_bitfield2`, and more. The backing field is emitted as public FFI data:

`windows-bindgen` also generates a typed getter and setter for each logical member. The data comes
from `NativeBitfieldAttribute` metadata on the backing field. The accessors are generated for
non-`sys` styles.

Generated accessors read and update each member without manual bit arithmetic.

A width-1 member projects as `bool`. Wider members project as the backing integer type. Reads shift
through the backing type so signed backing fields sign-extend and unsigned backing fields
zero-extend. Writes clear the target bits and OR in the masked value. Identity shifts are omitted so
generated code stays clean under `-D warnings`.

The RDL spelling is a C-like bit-field block on the backing field:

```text
_bitfield: u8 { HardwareInterface: 1, ... }
```

See [`windows-rdl`](windows-rdl.md) for RDL input. Test coverage lives in
`crates/tests/libs/clang/input/bitfields.h` and
`crates/tests/libs/bindgen/input/struct_bitfield.rdl`.

### Counted-buffer metadata

`NativeArrayInfoAttribute` and `MemorySizeAttribute` can replace a raw pointer/count pair with a
slice parameter in rich output. `windows-metadata::reader::MethodParam::buffer_relationship`
decodes the literal signed relationship:

- `CountParamIndex` identifies an element-count parameter.
- `BytesParamIndex` identifies a byte-count parameter.
- `CountConst` supplies a fixed element count.

The metadata reader checks property names and value types. Invalid or conflicting relationships
return `None`, which keeps the raw ABI shape. It does not interpret parameter positions, pointer
shapes, or public projection policy.

Before `CppMethod` indexes a related parameter, it rejects negative, out-of-range, and self-relative
indexes and verifies that the count is one input scalar used by one buffer. Byte counts still
require byte-sized elements. A fixed `CountConst` must be nonnegative and fit the maximum Rust
object size on 32-bit Windows. If any check fails, generation keeps the pointer and count parameters
exactly as the ABI declares them and adds no slice or array sugar.

### Parameter direction and retval policy

`windows-metadata::reader::MethodParam` supplies the raw direction, optional, reserved, and retval
facts. Bindgen's local `Param::is_input_only` then applies Rust policy: `Input` and `Unspecified`
are input-only, while `Output` and `InputOutput` take the output-capable branch. This is why an
In+Out pointer remains `*mut T` and an eligible counted buffer becomes `&mut [T]`; treating the
presence of `In` as input-only would incorrectly make writable storage const.

Bindgen also keeps its optional-or-reserved `Option` shaping local. Metadata does not combine those
facts, and windows-csharp does not use the same public-surface rule.

A trailing parameter becomes a projected return only when it is an output-only, required,
non-reserved, uncounted pointer. An explicit `RetValAttribute` bypasses the heuristic requirements
that every preceding parameter be input-only, that the pointee is not void, and that it fit the
existing 128-bit size limit; it does not bypass the other candidate checks. Without that attribute,
any preceding `Output` or `InputOutput` parameter keeps the trailing pointer in the parameter list.

### Testing

Dedicated test crates cover the generator and related metadata tools: `test_bindgen`, `test_rdl`,
and `test_clang`. `variadic_fn*` covers rich, minimal, sys-link, sys-extern, `C`, `system`, and
unsupported `fastcall` output. `buffer_relationships` covers valid, negative, out-of-range,
self-relative, byte-counted, and fixed-count metadata. The existing `interface_out_array` golden
pins valid counted-buffer output. `method_params` pins In+Out mutable projection, and
`method_return` covers explicit and heuristic retval selection with In+Out, optional, reserved, and
counted exclusions plus explicit void-pointer and large-pointee returns.

## Investigation: lessons from windows-csharp

The windows-csharp generator starts from the same metadata but builds a different public language
surface. Comparing the two generators is useful when it exposes duplicated metadata decoding,
missing ABI tests, or measured costs. C# runtime mechanisms are not assumptions that the Rust
projection should adopt.

This investigation excludes two larger API changes:

- `Bindgen::write` remains the existing panic-based API. A fallible generation API needs a separate
  design for errors produced across filtering, metadata conversion, formatting, and file output.
- Broad-filter omission reporting remains deferred. Exact selection already reports unsupported
  requested shapes, while broad generation intentionally retains the supported subset.

### Changes supported by the comparison

#### Shared literal buffer relationships

Both generators decoded the same three metadata properties. That decoding now lives on
`windows-metadata::reader::MethodParam` as `BufferRelationship`. The values stay signed and retain
their literal metadata meaning. The Rust and C# generators separately decide whether an index is
valid and whether the pointer/count pair can become a slice or span.

This boundary removes duplicate attribute parsing without forcing the languages to share public API
policy. It also tightens the C# path: an `I16` value on an unrelated property is no longer accepted
as a count parameter.

#### Unchanged generated files

`write_to_file` now compares the completed output with the existing file and skips an identical
write. Generation and formatting still run, so this does not make the generator itself faster. It
does preserve the file timestamp and avoids needless downstream compilation when a build script
regenerates committed bindings.

The comparison happens after generation and formatting. This keeps output validation unchanged and
does not introduce a separate cache or stale-input problem.

### Changes not supported by the comparison

#### A new signature projection plan

windows-csharp has an explicit `ParamProjection` model because C# spans, strings, raw pointers, and
generated COM companions need several distinct managed surfaces. Rust already has one shared
`method_signature` path for WinRT methods, native COM methods, delegates, and functions.
`CppMethod` then adds native-only buffer, retval, optional, and conversion policy.

Adding another plan object would move the existing boundary without removing duplicate
construction. A larger rewrite is not justified unless a new ABI shape causes the writers to
derive the same policy in different places.

#### Copied runtime ownership mechanisms

Rust already has the runtime properties that required new machinery in C#:

- interface owners are one pointer and release deterministically through `Drop`;
- borrowed interface parameters do not create another owner;
- vtables and generic interface identifiers are compile-time data;
- `AsyncFuture` caches `IAsyncInfo` rather than querying it on each poll;
- collection iteration batches through `GetMany`;
- `EventRevoker` is an inline owner and does not allocate its own heap object.

C# finalizers, synchronized owners, generated runtime support, and callback-confined borrowing solve
managed-runtime constraints. Copying them would add state or code to Rust without addressing a Rust
problem.

Borrowed HSTRING construction also does not transfer directly. A managed `string` is already UTF-16,
while Rust `str` is UTF-8 and must be transcoded for an HSTRING.

### ABI test transfer

The C# work found several cases that matter to any projection: sequence-correct parameter rows,
native COM record returns, failed `HRESULT` cleanup, required success-null interface outputs,
generic default-interface identifiers, malformed counted buffers, and variadic functions.

The shared branch work already added the missing Rust coverage:

- `method_params` covers sparse and out-of-order parameter rows and direction flags.
- `method_return` and `com_implement` cover native record returns and retval shaping.
- `buffer_relationships` covers valid and malformed count relationships.
- `variadic_fn*` covers rich omission and raw sys declarations.
- existing interface and runtime tests cover null interface conversion and generic identifiers.

Duplicating C# fake-vtable harnesses in the bindgen tests would test the same generated contracts
with more maintenance. New runtime tests should be added only when they exercise ownership or
cleanup that a golden file cannot establish.

### Generation measurements

Set `WINDOWS_BINDGEN_TIMINGS` to print metadata, selection, planning, rendering, formatting,
writing, and total times for each output:

```powershell
$env:WINDOWS_BINDGEN_TIMINGS = "1"
cargo run -p tool_bindings --quiet
```

The first timed run showed that `tool_bindings` rebuilt the same default metadata reader for every
output. Each rebuild took about 0.64-1.26 seconds. Selection, planning, rendering, and writing were
usually measured in milliseconds. The 17-output run took 15.5 seconds.

The default-only reader is now cached for the life of the process. Explicit metadata combinations
still get independent readers. The same timed run then took 1.74 seconds:

| Phase | Aggregate time |
| --- | ---: |
| Metadata | 667 ms |
| Selection | 30 ms |
| Planning and references | 6 ms |
| Rendering | 70 ms |
| Formatting | 953 ms |
| Writing | 2 ms |

Metadata initialization now occurs once, and later default-only generations reuse the immutable
reader. This also avoids leaking one metadata index per output.

Formatting is the largest remaining cost in this multi-output tool. Most outputs spend about
50-80 ms starting and running rustfmt even when the generated file is small. Replacing rustfmt or
its whitespace post-processing is not justified by this measurement alone. Batching or parallelizing
independent outputs belongs in the multi-output tool rather than the `Bindgen` API.

Skipping identical writes does not reduce generation or formatting time. Its benefit is avoiding
filesystem churn and downstream rebuilds.

### Runtime measurements and optimization limits

The cross-language benchmark shows that ordinary windows-rs calls, casts, and collection iteration
are already competitive. Two possible optimizations need narrower evidence:

- Event add/remove churn is slower in the general language benchmark. The Rust path creates a
  delegate box and returns an `EventRevoker`; the C# benchmark uses a raw token. Profiling must
  separate delegate allocation, source cloning, registration, and revocation before changing the
  public event model.
- An inherited-interface forwarder performs QI for each call. Callers that repeat several base
  interface calls can already cast once and reuse that interface. Adding a cache to every owner
  would make the one-pointer representation larger or require shared side state.

`BufferedIterator` already batches calls and moves values out of its buffer, and its measured vector
iteration is strong. Replacing its `Vec` with inline storage is low priority. `init_mta` also
intentionally keeps MTA usage alive for the process; tying its cookie to thread destruction is not
an automatic correctness improvement.

### Remaining investigation

The following work needs evidence or a separate compatibility decision:

| Area | Current conclusion |
| --- | --- |
| Event churn | Profile allocation, QI, add, and remove costs independently. |
| Repeated inherited calls | Benchmark forwarders against one explicit cast. |
| Flat-name collisions | Stable winner selection prevents drift but can hide an item. |
| Multi-output formatting | Consider driver-level parallelism if generator latency matters. |
| Iterator storage | Existing batching and benchmark results do not justify an inline buffer. |

Flat-name collision diagnostics should be designed with filter diagnostics rather than added as an
isolated warning. The current `TypeTree` sorts colliding rows and retains a stable winner, which
makes output reproducible but does not prove that the retained item is the intended one.
