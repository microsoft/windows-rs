# windows-default

> The default Windows metadata used by the build-time crates.

- 📦 [crates.io](https://crates.io/crates/windows-default)
- 📖 [docs.rs](https://docs.rs/windows-default)
- 🚀 [Getting started](../../crates/libs/default/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/default)

`windows-default` embeds the canonical Windows Runtime and Windows API metadata as byte slices.
Build tools use `WINRT` and `WIN32` when they need the standard metadata without locating separate
`.winmd` files.

Applications should use a focused crate, `windows-bindgen`, or, for binaries that want a broad
pre-generated surface, [`windows`](windows.md) or [`windows-sys`](windows-sys.md). Most code
generators should use `input_default` or `reference_default` on the builder they already use. Use
`windows-default` directly only when implementing a metadata tool that accepts in-memory winmd
bytes.

The crate exposes two static byte slices:

| Static | Embedded file | Contents |
| --- | --- | --- |
| `WINRT` | `Windows.winmd` | Windows Runtime contracts |
| `WIN32` | `Windows.Win32.winmd` | Windows SDK and WDK APIs |

Both files are included in the published crate and embedded with `include_bytes!`. The crate does
not extract files or perform I/O.

## First workflow: add standard metadata to a custom index

1. Construct `windows_metadata::reader::File` values from `WINRT` and `WIN32`.
2. Add component-specific winmd files.
3. Build one `windows_metadata::reader::Index` over the combined files.
4. Query or transform the index without locating an SDK installation.

`tool_reactor` uses this pattern when its metadata resolver combines WinUI winmds with the standard
definitions. If you are compiling RDL or generating Rust, use the neighboring builder methods
instead; they already perform the byte-to-file conversion.

## Integration

The build crates depend on `windows-default` and expose the metadata through their own builders.
Callers therefore do not need a separate dependency or a path into the Windows SDK.

| Crate | Default behavior |
| --- | --- |
| [`windows-bindgen`](windows-bindgen.md) | Implicit with no input; explicit with `input_default`. |
| [`windows-rdl`](windows-rdl.md) | `reference_default` or writer `input_default`. |
| [`windows-clang`](windows-clang.md) | `reference_default` and WinRT-only `resolution_default`. |
| [`windows-metadata`](windows-metadata.md) | Construct `reader::File` values from the slices. |

The `windows-bindgen` textual adapter accepts `--in default`. The bindgen, RDL, and Clang builders
use explicit default methods instead; their path-style input methods treat `"default"` as an
ordinary path. Byte-input APIs remain available for custom metadata that is already in memory.

Programs that link one of these build crates include both metadata payloads in the binary. These
crates are intended for build tools rather than runtime dependencies.

## Pitfalls

- `WIN32` is one flat `Windows.Win32` metadata set. Published `windows` and `windows-sys` feature
  namespaces are created later by `tool_package`.
- Adding any explicit bindgen input disables the implicit default input. Call `input_default` when
  the custom metadata references standard types.
- `reference_default` and `resolution_default` are not synonyms in `windows-clang`. The former can
  suppress already-defined declarations; the latter only classifies WinRT ABI projections.
- Linking this crate embeds both payloads even if the tool reads only one static.
- The bytes are build inputs. They do not provide Windows DLLs or make an API available on the
  running operating system.

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is for contributors and
is **not needed to use `windows-default`**.

### How it's built

The crate has no dependencies. `src/lib.rs` exposes each committed `.winmd` through
`include_bytes!`.

| File | Generator | Reviewable source |
| --- | --- | --- |
| `Windows.winmd` | `cargo run -p tool_winrt` | `metadata/winrt` |
| `Windows.Win32.winmd` | `cargo run -p tool_win32` | `metadata/win32` and `metadata/wdk` |

`tool_winrt` merges the Windows SDK contract metadata, writes canonical RDL, and compiles that RDL
back to `Windows.winmd`. `tool_win32` scrapes the Windows SDK and WDK headers for X64, Arm64, and
X86, merges the architecture-specific RDL, and writes `Windows.Win32.winmd`.

See [Dependencies](../dependencies.md#windows-sdk-wdk-and-winrt-contracts) for the package versions
and provenance.

### Packaging

The repository `.gitignore` normally excludes `.winmd` files, with an exception for this crate's
two committed payloads. Cargo packages tracked files by default, so `Cargo.toml` does not need a
separate `include` list. `cargo package -p windows-default` must contain both `.winmd` files.

### Testing

The metadata is exercised through its consumers:

- `test_bindgen`, `test_rdl`, and `test_clang` cover default and byte inputs.
- `tool_winrt`, `tool_win32`, and `tool_roundtrip` verify deterministic metadata generation.
- The `tool_yml` generated workflows build and document `windows-default` like the other library
  crates.

After changing the payloads or their generators, run the owning generator and confirm that the
other generators remain output-neutral.
