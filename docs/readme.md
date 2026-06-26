This repo provides a comprehensive set of Rust crates for building Windows applications — from low-level API access to high-level declarative UI.

* [Getting started](getting-started.md)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

## How these docs are organized

Each crate has one page under [`crates/`](crates) covering both usage and internals — how the crate is built and maintained (the `tool_bindings` / `tool_reactor` / `tool_package` codegen pipeline, generated files, and conventions). Each crate's own `readme.md` is the user-facing introduction with a quick example, and the per-crate page links to it. Item-level API reference is the generated rustdoc on [docs.rs](https://docs.rs), linked from every page.

## Crates

### Core & error handling

| Crate | Description |
| --- | --- |
| [windows-core](crates/windows-core.md) | Fundamental COM and Windows type support. |
| [windows-result](crates/windows-result.md) | Windows error handling and propagation. |
| [windows-strings](crates/windows-strings.md) | Windows string interop types and macros. |

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

### COM macros & linking

| Crate | Description |
| --- | --- |
| [windows-implement](crates/windows-implement.md) | `#[implement]` proc macro for COM/WinRT. |
| [windows-interface](crates/windows-interface.md) | `#[interface]` proc macro for COM interfaces. |
| [windows-link](crates/windows-link.md) | Raw-dylib import support (`link!`). |
| [windows-targets](crates/windows-targets.md) | Import libs for older compilers. |

### UI & graphics

| Crate | Description |
| --- | --- |
| [windows-reactor](crates/windows-reactor.md) | Declarative UI library backed by WinUI 3. |
| [windows-canvas](crates/windows-canvas.md) | 2D graphics built on Direct2D. |
| [windows-webview](crates/windows-webview.md) | Safe wrapper around the WebView2 browser control. |
| [windows-window](crates/windows-window.md) | Minimal window creation and message loop. |
| [windows-animation](crates/windows-animation.md) | Wrapper around the Windows Animation Manager. |
| [windows-reactor-setup](crates/windows-reactor-setup.md) | Windows App Runtime installer for reactor apps. |

### Codegen & metadata tooling

| Crate | Description |
| --- | --- |
| [windows-bindgen](crates/windows-bindgen.md) | Code generator for Windows metadata. |
| [windows-metadata](crates/windows-metadata.md) | Low-level ECMA-335 metadata library. |
| [windows-rdl](crates/windows-rdl.md) | RDL parser and ECMA-335 generator. |
| [riddle](crates/riddle.md) | Windows metadata compiler. |
| [cppwinrt](crates/cppwinrt.md) | Bundles the C++/WinRT compiler. |

### Full Windows API projection

These crates project the entire Windows API surface. For new projects, prefer a focused binding generated with [windows-bindgen](crates/windows-bindgen.md), or compose the smaller crates above.

| Crate | Description |
| --- | --- |
| [windows](crates/windows.md) | Safer projection of C-style, COM, and WinRT APIs. |
| [windows-sys](crates/windows-sys.md) | Zero-overhead raw bindings for C-style Windows APIs. |
