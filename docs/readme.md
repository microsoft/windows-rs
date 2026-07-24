`windows-rs` lets you call Windows APIs - past, present, and future - directly from Rust. It is not
a single crate but a family of them, from low-level API access to high-level declarative UI. This
page helps you find the right crate for the job; each crate's own documentation then covers how to
use it, with examples.

- [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
- [Releases](https://github.com/microsoft/windows-rs/releases)

## A family of crates

Most of these crates are small and focused - string types, error handling, the registry,
collections, and so on - and you depend only on what you actually use. For broad, exploratory access
to the *entire* Windows API surface, the [`windows`](crates/windows.md) and
[`windows-sys`](crates/windows-sys.md) crates project everything past, present, and future, gated
behind per-namespace features.

For most projects, prefer the focused crates below, and generate a minimal, project-specific binding
with [`windows-bindgen`](crates/windows-bindgen.md) for any additional APIs you need.

## Choosing your crates

Start with what you are trying to do and add the smallest crate that covers it. Follow the link for
usage and examples.

| If you need | Use |
| --- | --- |
| Windows error handling (`HRESULT`, `Error`, `Result`) | [windows-result](crates/windows-result.md) |
| Windows string types and macros (`HSTRING`, `PCWSTR`, `h!`, `w!`, `s!`) | [windows-strings](crates/windows-strings.md) |
| COM/WinRT type support (`IUnknown`, the `Interface` trait, `cast`, `GUID`) | [windows-core](crates/windows-core.md) |
| To declare or implement a COM/WinRT interface | [windows-core](crates/windows-core.md) (`#[interface]` / `#[implement]`) |
| Stock WinRT collections (`IVector`, `IMap`) | [windows-collections](crates/windows-collections.md) |
| WinRT values (`IReference<T>`, `TimeSpan`, `DateTime`) | [windows-reference](crates/windows-reference.md), [windows-time](crates/windows-time.md) |
| Graphics math (vectors, matrices) | [windows-numerics](crates/windows-numerics.md) |
| WinRT async bridged to Rust futures | [windows-future](crates/windows-future.md) |
| The Win32 thread pool | [windows-threading](crates/windows-threading.md) |
| The Windows registry | [windows-registry](crates/windows-registry.md) |
| To author a Windows service | [windows-services](crates/windows-services.md) |
| The OS version at runtime | [windows-version](crates/windows-version.md) |
| To link C-style functions without import libs (`link!`, `raw-dylib`) | [windows-link](crates/windows-link.md) |
| To generate your own focused bindings | [windows-bindgen](crates/windows-bindgen.md) |
| A declarative WinUI 3 UI, 2D graphics, a WebView, or a window | [windows-reactor](crates/windows-reactor.md), [windows-canvas](crates/windows-canvas.md), [windows-webview](crates/windows-webview.md), [windows-window](crates/windows-window.md) |

## Crates

The full categorized index follows. Each crate has one page under [`crates/`](crates) covering both
usage and internals - how the crate is built and maintained (the `tool_bindings` / `tool_reactor` /
`tool_package` codegen pipeline, generated files, and conventions). Each crate's own `readme.md` is
the user-facing introduction with a quick example, and the per-crate page links to it. Item-level
API reference is the generated rustdoc on [docs.rs](https://docs.rs), linked from every page.

### Core & error handling

| Crate | Description |
| --- | --- |
| [windows-core](crates/windows-core.md) | Fundamental COM and Windows type support, including the `#[interface]` / `#[implement]` authoring macros. |
| [windows-result](crates/windows-result.md) | Windows error handling and propagation. |
| [windows-strings](crates/windows-strings.md) | Windows string interop types and macros. |

The `#[interface]` and `#[implement]` macros are part of `windows-core`, split into the
[windows-interface](crates/windows-interface.md) and
[windows-implement](crates/windows-implement.md) crates only because Rust requires procedural macros
to live in a dedicated `proc-macro` crate.

### Values & collections

| Crate | Description |
| --- | --- |
| [windows-numerics](crates/windows-numerics.md) | Graphics math types (vectors and matrices). |
| [windows-collections](crates/windows-collections.md) | Stock WinRT collection types. |
| [windows-reference](crates/windows-reference.md) | Stock `IReference<T>` implementation. |
| [windows-time](crates/windows-time.md) | WinRT `TimeSpan` and `DateTime`. |

### Async & threading

| Crate | Description |
| --- | --- |
| [windows-future](crates/windows-future.md) | WinRT async bridged to Rust futures. |
| [windows-threading](crates/windows-threading.md) | Safe wrapper over the Win32 thread pool. |

### System services

| Crate | Description |
| --- | --- |
| [windows-registry](crates/windows-registry.md) | Safe Windows registry access. |
| [windows-services](crates/windows-services.md) | Author Windows services in Rust. |
| [windows-version](crates/windows-version.md) | Query the Windows version at runtime. |

### COM authoring macros & linking

These crates package functionality that is part of other crates but must ship separately.
`windows-interface` and `windows-implement` are part of [windows-core](crates/windows-core.md) (see
above); Rust requires their proc macros to live in a dedicated `proc-macro` crate.

| Crate | Description |
| --- | --- |
| [windows-implement](crates/windows-implement.md) | `#[implement]` proc macro for COM/WinRT - part of `windows-core`. |
| [windows-interface](crates/windows-interface.md) | `#[interface]` proc macro for COM interfaces - part of `windows-core`. |
| [windows-link](crates/windows-link.md) | Raw-dylib import support (`link!`). |

### UI & graphics

| Crate | Description |
| --- | --- |
| [windows-reactor](crates/windows-reactor.md) | Declarative UI library backed by WinUI 3. |
| [windows-canvas](crates/windows-canvas.md) | 2D graphics built on Direct2D. |
| [windows-composition](crates/windows-composition.md) | Retained-mode visual layer built on the Windows composition engine. |
| [windows-webview](crates/windows-webview.md) | Safe wrapper around the WebView2 browser control. |
| [windows-window](crates/windows-window.md) | Minimal window creation and message loop. |
| [windows-animation](crates/windows-animation.md) | Wrapper around the Windows Animation Manager. |
| [windows-reactor-setup](crates/windows-reactor-setup.md) | Windows App Runtime installer for reactor apps. |

### Codegen & metadata tooling

| Crate | Description |
| --- | --- |
| [windows-bindgen](crates/windows-bindgen.md) | Code generator for Windows metadata. |
| [windows-clang](crates/windows-clang.md) | Generates RDL from C/C++ headers using libclang. |
| [windows-metadata](crates/windows-metadata.md) | Low-level ECMA-335 metadata library. |
| [windows-rdl](crates/windows-rdl.md) | RDL parser and ECMA-335 generator. |
| [riddle](crates/riddle.md) | Windows metadata compiler. |
| [cppwinrt](crates/cppwinrt.md) | Bundles the C++/WinRT compiler. |

### Full Windows API projection

These crates project the entire Windows API surface. For new projects, prefer a focused binding
generated with [windows-bindgen](crates/windows-bindgen.md), or compose the smaller crates above.

| Crate | Description |
| --- | --- |
| [windows](crates/windows.md) | Safer projection of C-style, COM, and WinRT APIs. |
| [windows-sys](crates/windows-sys.md) | Zero-overhead raw bindings for C-style Windows APIs. |

## Building & maintenance

- [Dependencies](dependencies.md) - every external SDK, header set, metadata file, and runtime the
  build and tooling depend on: what version is used, where it is set, how it is obtained, and how to
  update it.
