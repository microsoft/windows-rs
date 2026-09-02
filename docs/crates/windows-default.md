# windows-default

> The default Windows metadata used by the build-time crates.

- 📦 [crates.io](https://crates.io/crates/windows-default)
- 📖 [docs.rs](https://docs.rs/windows-default)
- 🚀 [Getting started](../../crates/libs/default/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/default)

`windows-default` embeds the canonical Windows Runtime and Windows API metadata as byte slices.
Build tools use `WINRT` and `WIN32` when they need the standard metadata without locating separate
`.winmd` files.

Most callers should use the default-input API provided by the tool they already depend on:

```rust,no_run
windows_bindgen::builder()
    .input_default()
    .filter("GetTickCount")
    .output("bindings.rs")
    .write();
```

Use `windows-default` directly only when building another metadata tool or API that accepts metadata
bytes.

## Getting started

Add the crate as a build dependency:

```toml
[build-dependencies]
windows-default = "0.100"
```

The crate exposes two static byte slices:

| Static | Embedded file | Contents |
| --- | --- | --- |
| `WINRT` | `Windows.winmd` | Windows Runtime contracts |
| `WIN32` | `Windows.Win32.winmd` | Windows SDK and WDK APIs |

Both files are included in the published crate and embedded with `include_bytes!`. The crate does
not extract files or perform I/O.

## Build-tool integration

The build crates depend on `windows-default` and expose the metadata through their own builders.
Callers therefore do not need a separate dependency or a path into the Windows SDK.

| Crate | Default behavior |
| --- | --- |
| [`bindgen`](windows-bindgen.md) | Implicit if no input; explicit with `.input_default()`. |
| [`rdl`](windows-rdl.md) | `.reference_default()` adds both files as references. |
| [`clang`](windows-clang.md) | `.reference_default()` plus WinRT-only `.resolution_default()`. |

The `windows-bindgen` textual adapter accepts `--in default`. The bindgen, RDL, and Clang builders
use explicit default methods instead; their path-style input methods treat `"default"` as an
ordinary path. Byte-input APIs remain available for custom metadata that is already in memory.

Programs that link one of these build crates include both metadata payloads in the binary. These
crates are intended for build tools rather than runtime dependencies.

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
