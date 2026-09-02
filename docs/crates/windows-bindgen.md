# windows-bindgen

> The code generator that turns Windows metadata (`.winmd`) into Rust bindings.

- 📦 [crates.io](https://crates.io/crates/windows-bindgen)
- 📖 [docs.rs](https://docs.rs/windows-bindgen)
- 🚀 [Getting started](../../crates/libs/bindgen/readme.md)
- 🧩 [Samples](../../crates/samples/bindgen)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/bindgen)

Use `windows-bindgen` when a focused crate does not cover the APIs a project needs. It is the
preferred way for reusable libraries to own a narrow binding set without depending on the broad
[`windows`](windows.md) or [`windows-sys`](windows-sys.md) umbrella crate. Binary applications may
prefer those pre-generated crates when dependency size and version sharing are less important.

The generator also supports custom metadata and control over the generated Rust shape. It is a
build-time generator, not a Windows API runtime.

The crate is the final stage of the repository's metadata pipeline:

```text
C/C++ headers -> RDL -> .winmd -> windows-bindgen -> Rust source or a Rust package
```

Start with the crate [README](../../crates/libs/bindgen/readme.md) for dependency setup and the
smallest invocation. This page covers how to choose and maintain a real generation workflow.

## Choose the input

`windows-bindgen` reads ECMA-335 Windows metadata, not headers or IDL.

| Starting point | Path to bindings |
| --- | --- |
| Standard Windows API | Use the implicit default metadata and add filters. |
| Custom `.winmd` | Add it with `input`; also call `input_default` if it references Windows types. |
| RDL declarations | Compile them with [`windows-rdl`](windows-rdl.md), then add the `.winmd`. |
| C/C++ headers | Scrape with [`windows-clang`](windows-clang.md), compile with RDL, then bind. |
| Metadata bytes in memory | Use `input_bytes` or `input_byte_sets`. |

If no input is supplied, the builder reads the standard WinRT and Win32 metadata from
[`windows-default`](windows-default.md). Supplying a custom input replaces that implicit choice;
call `input_default` when custom definitions need standard types.

## First workflow: own a focused binding

For a reusable library, keep generation separate from normal consumer builds:

1. Decide whether the public surface needs rich wrappers or raw FFI.
2. Put the API names in a stable filter list.
3. Generate `src/bindings.rs` from an unpublished workspace tool.
4. Review and commit the generated file with the library.
5. Run that tool in CI and reject a generated diff.

The repository's `tool_bindings` follows this workflow. It reads command files from
`crates/tools/bindings/src/*.txt` and rewrites each library's committed `bindings.rs`.
`crates/samples/bindgen/vss_backup/build.rs` shows the alternative for an application: generate a
flat file in `OUT_DIR`, include it as a private module, and let each build recreate it.

Prefer committed output for a published library. Its users then need only the generated code's
runtime support, not the generator and metadata payloads. Treat generated types exposed by the
library as part of its public contract.

## Select APIs with filters

Filters resemble Rust paths. Specific filters produce smaller output:

| Filter | Result |
| --- | --- |
| `Windows.Win32.System.Com` | Everything in that namespace and its children. |
| `Windows.Win32.Foundation.HWND` | The full named type. |
| `Namespace.Type::{}` | A name-only shell. |
| `Namespace.Type::Method` | One method and required type dependencies. |
| `Namespace.Type::{Method1, Method2}` | A set of methods. |
| `!Namespace.Type` | Excludes a matching included item. |

Property and event names select their accessor pairs. Selecting a WinRT class's
`CreateInstance` member includes activation support; selecting only the class projects its default
interface without a constructor.

Signature dependencies are included automatically, usually as shells. A whole-type filter retains
the type hierarchy. Use `filter_file` or `filter_files` for filter-only files. Use the textual
`--etc` adapter when the file contains the whole command, including output and style options.
Blank lines and lines beginning with `//` are ignored in command files.

## Choose style and layout

Style controls the API shape. Layout controls where items are written.

| Style | Use it for | Key behavior |
| --- | --- | --- |
| Default | Rich WinRT and Win32 bindings | Wrappers, handle types, and inherited forwarders. |
| `sys` | Raw FFI | Plain structs and foreign functions; used by `windows-sys`. |
| `minimal` | Small hand-wrapped binding sets | Omits most convenience and inherited wrappers. |

`extern_fns` changes sys free functions from `windows-link` macros to `extern` blocks. `sys` and
`minimal` are mutually exclusive. Only sys style can emit native variadic functions because a rich
wrapper cannot forward an unknown argument tail. Stable Rust also cannot declare `fastcall`
variadics.

| Layout | Output |
| --- | --- |
| Default | Nested Rust modules matching metadata namespaces. |
| `flat` | One flat Rust source file. |
| `package` | Namespace files plus a `Cargo.toml` with namespace features. |

`flat` and `package` are mutually exclusive. Package mode is intended for broad projections such
as `windows` and `windows-sys`; focused libraries normally use one flat file. In sys package mode,
empty COM-only namespaces and their unused feature entries are pruned.

## Common generation tasks

- Use `implement` or `implements` to generate WinRT implementation traits for selected interfaces.
  `implement_all` applies to every interface in scope.
- In minimal mode, use `compose` for an explicitly filtered composable WinRT class. Filter the
  class and its factory method separately; `implement` does not select composition targets.
- Use `derive` or `derives` to add traits to generated types.
- Use `rustfmt` to select a formatter executable.
- Use `dead_code` for internal bindings whose unused callable items should be detected as
  `pub(crate)`.

WinRT event add/remove pairs project as one method returning `EventRevoker`. Dropping the revoker
unsubscribes; `forget` or `into_token` transfers that responsibility. Interface implementations
still provide both ABI accessors.

## Pitfalls

- A broad namespace filter can produce a large dependency closure. Begin with the callable or type
  names the wrapper actually owns.
- Default, sys, and minimal output are different contracts, not formatting choices. Pick the style
  before writing wrapper code.
- `minimal` changes rendering, not dependency selection. Narrow filters are still required.
- Exact selection of an unsupported variadic export reports an error; broad rich filters omit it.
- Output paths are relative to the generator's current directory. Make generator invocation
  location stable in scripts and CI.
- Do not make an entire library depend on the `windows` crate just to share one generated type.
  Use a focused foundational crate or make the binding's ownership explicit.

## Samples and neighboring tools

- `crates/samples/bindgen/vss_backup` generates rich COM bindings in `OUT_DIR`.
- `crates/samples/bindgen/context_alignment` generates a flat sys binding for `CONTEXT`.
- `crates/samples/robot/component` compiles custom RDL and generates implementation support.
- `crates/samples/robot/client` combines custom and default metadata for a WinRT client.
- `tool_package` uses package mode for the published `windows` and `windows-sys` crates.
- `tool_webview` demonstrates the complete header -> RDL -> winmd -> Rust pipeline.

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is for contributors and
is **not needed to use `windows-bindgen`**.

### How it is built

`windows-bindgen` is hand-written. It reads ECMA-335 metadata through
[`windows-metadata`](windows-metadata.md), while [`windows-default`](windows-default.md) supplies
the bundled inputs. `tool_bindings` generates focused library files and `tool_package` generates
the published `windows` and `windows-sys` packages.

### Output policies

Named policy methods keep style decisions out of individual writers:

- `Style::emit_class_methods` controls per-class wrapper methods.
- `Style::emit_inherited_forwarders` controls inherited-interface forwarding methods.
- `Style::emit_iterable_into_iterator` controls the inherited `IIterable<T>` bridge.
- `Style::minimal_string_input` and `minimal_string_return` map minimal strings.
- `Config::emit_runtime_name` controls WinRT runtime-name constants.
- `Style::derive_std_traits` and `emit_core_traits` control generated trait blocks.
- `Style::emit_bare_typedef` controls handle and unscoped-enum representation.

`Config::item_vis` applies `dead_code` visibility to callable items. Nameable items stay public
because handwritten code and exported macros may re-export or reference them.

### Type selection

For precise filters, `TypeClosure::build` starts from selected types and follows signature
dependencies. Selected entry points are full types; signature dependencies are shells unless
selected directly. A whole-type filter retains its hierarchy. A class member filter retains the
class-to-interface edge that provides the member. Signature-only dependencies do not add unrelated
hierarchy edges.

An interface selected as a shell can still supply `_Impl` scaffolding through `implement`.
Implementation closure retains every ABI method signature without emitting callable wrappers.
Select the whole interface too when one binding must call and implement it.

Minimal composable bindings require an explicit class target. Select the class's composable factory
method with a filter, select override interfaces with `implement`, and select the class with
`compose`. This prevents every class inheriting an implemented override interface from becoming a
composition target.

Broad filters and package generation use `TypeMap::filter`. `minimal` affects rendering only and
does not change which referenced types are included.

### WinRT and Win32 generation

The metadata reader classifies types from metadata attributes. Shared code handles names,
signatures, dependencies, and remapping. Separate writers preserve the different ABI rules:

- WinRT vtable methods return `HRESULT`; rich output projects them through `Result`.
- COM methods keep their native return shape, with `ReturnHint` for common projection patterns.
- WinRT supports generics, runtime signatures, activation, and `RuntimeType`.
- WinRT delegates are COM interfaces with `Invoke`; COM callbacks can be function pointers.
- Win32 also has free exports, constants, handles, unions, nested types, and architecture-specific
  layout.

### Bit-field accessors

Winmd has no bit-field syntax. The header pipeline stores each run in an integer field named
`_bitfield`, `_bitfield1`, and so on, with `NativeBitfieldAttribute` entries for logical members.
Non-sys bindgen output keeps the backing field and adds typed getters and setters.

Width-one members project as `bool`; wider members use the backing integer. Reads shift through the
backing type so signed fields sign-extend and unsigned fields zero-extend. Writes clear the target
range and OR in the masked value. Identity shifts are omitted to keep generated code clean under
`-D warnings`.

RDL spells the same shape as a block on the backing field. Coverage lives in
`test_clang/input/bitfields.h` and `test_bindgen/input/struct_bitfield.rdl`.

### Counted buffers

`NativeArrayInfoAttribute` and `MemorySizeAttribute` describe element counts, byte counts, and
fixed counts. `MethodParam::buffer_relationship` decodes only the literal relationship; bindgen
validates projection policy.

Before indexing a related parameter, bindgen rejects negative, out-of-range, self-relative, or
multiply-used count indexes. A count must be one input scalar. Byte counts require byte-sized
elements. Fixed counts must be nonnegative and fit the maximum Rust object size on 32-bit Windows.
Any failed check preserves the raw pointer and count.

Input or input/output buffers can become slices. Output-only buffers remain raw pointer/count pairs
because `&mut [T]` would require initialized storage before the call.

### Parameter direction and return values

`windows-metadata` supplies raw direction, optional, reserved, retval, and count facts. Bindgen
applies Rust policy: `Input` and `Unspecified` are input-only; `Output` and `InputOutput` take the
output-capable branch. An eligible input/output buffer becomes `&mut [T]`.

A trailing parameter becomes a projected return only when it is output-only, required,
non-reserved, uncounted, and pointer-shaped. `RetValAttribute` bypasses only the heuristic checks
for preceding output parameters, a void pointee, and the 128-bit size limit. It does not bypass the
other candidate checks.

### Variadic functions

Sys output retains the literal `...` tail and metadata `C` or `system` calling convention in either
link-macro or extern-block output. On X86, Rust lowers a `system` C-variadic declaration to the
compatible C variadic ABI. Rich and minimal writers never emit a callable fixed-prefix wrapper.

### Package pruning

Sys package output can leave a namespace empty when it contains only COM interfaces. Package
generation recursively removes that namespace's module, file, feature, and feature dependency.
A parent remains when it or any child still contains output.

### Determinism and testing

Generation sorts metadata-driven maps and formats output before writing. The generator must remain
output-neutral unless a projection change is intended. Run the owning `tool_*` generators after a
bindgen change and inspect all generated diffs.

`test_bindgen` covers filter closure, styles, layouts, methods, buffers, returns, implementation
support, and variadics. `test_rdl` and `test_clang` cover the input stages. CI regenerates committed
bindings and package output and rejects drift.
