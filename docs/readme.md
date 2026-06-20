# Rust for Windows — documentation

This repo provides a comprehensive set of Rust crates for building Windows applications — from low-level API access to high-level declarative UI.

* [Getting started](https://kennykerr.ca/rust-getting-started/)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)
* [Feature search](https://microsoft.github.io/windows-rs/features)

## How these docs are organized

- **[User guide](guide/readme.md)** — one task-oriented page per crate.
- **[Internals](internals/readme.md)** — one page per crate on how it is built and maintained (the `tool_bindings` / `tool_reactor` / `tool_package` codegen pipeline, generated files, and conventions).
- **API reference** — generated rustdoc on [docs.rs](https://docs.rs); follow the link on any guide page.

## Crates

### Core & projection

| Crate | Description | Guide | Internals |
| --- | --- | --- | --- |
| [windows](https://crates.io/crates/windows) | Safer projection of C-style, COM, and WinRT APIs. | [guide](guide/windows.md) | [internals](internals/windows.md) |
| [windows-sys](https://crates.io/crates/windows-sys) | Zero-overhead raw bindings for C-style Windows APIs. | [guide](guide/windows-sys.md) | [internals](internals/windows-sys.md) |
| [windows-core](https://crates.io/crates/windows-core) | Fundamental COM and Windows type support. | [guide](guide/windows-core.md) | [internals](internals/windows-core.md) |

### Error handling & strings

| Crate | Description | Guide | Internals |
| --- | --- | --- | --- |
| [windows-result](https://crates.io/crates/windows-result) | Windows error handling and propagation. | [guide](guide/windows-result.md) | [internals](internals/windows-result.md) |
| [windows-strings](https://crates.io/crates/windows-strings) | Windows string interop types and macros. | [guide](guide/windows-strings.md) | [internals](internals/windows-strings.md) |

### Values & collections

| Crate | Description | Guide | Internals |
| --- | --- | --- | --- |
| [windows-numerics](https://crates.io/crates/windows-numerics) | Graphics math types (vectors and matrices). | [guide](guide/windows-numerics.md) | [internals](internals/windows-numerics.md) |
| [windows-collections](https://crates.io/crates/windows-collections) | Stock WinRT collection types. | [guide](guide/windows-collections.md) | [internals](internals/windows-collections.md) |
| [windows-reference](https://crates.io/crates/windows-reference) | Stock `IReference<T>` implementation. | [guide](guide/windows-reference.md) | [internals](internals/windows-reference.md) |
| [windows-time](https://crates.io/crates/windows-time) | WinRT `TimeSpan` and `DateTime`. | [guide](guide/windows-time.md) | [internals](internals/windows-time.md) |

### Async & threading

| Crate | Description | Guide | Internals |
| --- | --- | --- | --- |
| [windows-future](https://crates.io/crates/windows-future) | WinRT async bridged to Rust futures. | [guide](guide/windows-future.md) | [internals](internals/windows-future.md) |
| [windows-threading](https://crates.io/crates/windows-threading) | Safe wrapper over the Win32 thread pool. | [guide](guide/windows-threading.md) | [internals](internals/windows-threading.md) |

### System services

| Crate | Description | Guide | Internals |
| --- | --- | --- | --- |
| [windows-registry](https://crates.io/crates/windows-registry) | Safe Windows registry access. | [guide](guide/windows-registry.md) | [internals](internals/windows-registry.md) |
| [windows-services](https://crates.io/crates/windows-services) | Author Windows services in Rust. | [guide](guide/windows-services.md) | [internals](internals/windows-services.md) |
| [windows-version](https://crates.io/crates/windows-version) | Query the Windows version at runtime. | [guide](guide/windows-version.md) | [internals](internals/windows-version.md) |

### COM macros & linking

| Crate | Description | Guide | Internals |
| --- | --- | --- | --- |
| [windows-implement](https://crates.io/crates/windows-implement) | `#[implement]` proc macro for COM/WinRT. | [guide](guide/windows-implement.md) | [internals](internals/windows-implement.md) |
| [windows-interface](https://crates.io/crates/windows-interface) | `#[interface]` proc macro for COM interfaces. | [guide](guide/windows-interface.md) | [internals](internals/windows-interface.md) |
| [windows-link](https://crates.io/crates/windows-link) | Raw-dylib import support (`link!`). | [guide](guide/windows-link.md) | [internals](internals/windows-link.md) |
| [windows-targets](https://crates.io/crates/windows-targets) | Import libs for older compilers. | [guide](guide/windows-targets.md) | [internals](internals/windows-targets.md) |

### UI & graphics (experimental)

| Crate | Description | Guide | Internals |
| --- | --- | --- | --- |
| windows-reactor | Declarative UI library backed by WinUI 3. | [guide](guide/windows-reactor.md) | [internals](internals/windows-reactor.md) |
| windows-canvas | 2D graphics built on Direct2D. | [guide](guide/windows-canvas.md) | [internals](internals/windows-canvas.md) |
| windows-animation | Wrapper around the Windows Animation Manager. | [guide](guide/windows-animation.md) | [internals](internals/windows-animation.md) |
| windows-reactor-setup | Windows App Runtime installer for reactor apps. | [guide](guide/windows-reactor-setup.md) | [internals](internals/windows-reactor-setup.md) |

### Codegen & metadata tooling

| Crate | Description | Guide | Internals |
| --- | --- | --- | --- |
| [windows-bindgen](https://crates.io/crates/windows-bindgen) | Code generator for Windows metadata. | [guide](guide/windows-bindgen.md) | [internals](internals/windows-bindgen.md) |
| [windows-metadata](https://crates.io/crates/windows-metadata) | Low-level ECMA-335 metadata library. | [guide](guide/windows-metadata.md) | [internals](internals/windows-metadata.md) |
| [windows-rdl](https://crates.io/crates/windows-rdl) | RDL parser and ECMA-335 generator. | [guide](guide/windows-rdl.md) | [internals](internals/windows-rdl.md) |
| [riddle](https://crates.io/crates/riddle) | Windows metadata compiler. | [guide](guide/riddle.md) | [internals](internals/riddle.md) |
| [cppwinrt](https://crates.io/crates/cppwinrt) | Bundles the C++/WinRT compiler. | [guide](guide/cppwinrt.md) | [internals](internals/cppwinrt.md) |
