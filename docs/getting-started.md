# Getting started

`windows-rs` lets you call Windows APIs — past, present, and future — directly
from Rust. This page helps you find the right crate for the job; each crate's own
documentation then covers how to use it, with examples.

- 📖 [Crate index and docs](readme.md)
- 📁 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
- 🚀 [Releases](https://github.com/microsoft/windows-rs/releases)

## A family of crates

`windows-rs` is not a single crate but a family of them. Most are small and
focused — string types, error handling, the registry, collections, and so on —
and you depend only on what you actually use. For broad, exploratory access to
the *entire* Windows API surface, the [`windows`](crates/windows.md) and
[`windows-sys`](crates/windows-sys.md) crates project everything past, present,
and future, gated behind per-namespace features.

For most projects, prefer the focused crates below, and generate a minimal,
project-specific binding with [`windows-bindgen`](crates/windows-bindgen.md) for
any additional APIs you need.

## Choosing your crates

Start with what you are trying to do and add the smallest crate that covers it.
Follow the link for usage and examples.

| If you need… | Use |
| --- | --- |
| Windows error handling (`HRESULT`, `Error`, `Result`) | [windows-result](crates/windows-result.md) |
| Windows string types and macros (`HSTRING`, `PCWSTR`, `h!`, `w!`, `s!`) | [windows-strings](crates/windows-strings.md) |
| COM/WinRT type support (`IUnknown`, the `Interface` trait, `cast`, `GUID`) | [windows-core](crates/windows-core.md) |
| To declare or implement a COM/WinRT interface | [`#[interface]`](crates/windows-interface.md) / [`#[implement]`](crates/windows-implement.md) |
| Stock WinRT collections (`IVector`, `IMap`, …) | [windows-collections](crates/windows-collections.md) |
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

The full categorized index — including the metadata and code-generation tooling —
lives in the [crate docs hub](readme.md).

## Where to go next

- **[Crate docs and index](readme.md)** — one page per crate, covering both usage
  and internals, plus the full categorized list.
- **[Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)** —
  runnable examples across Win32, COM, WinRT, DirectX, and the UI crates.
- **API reference** — every crate is documented on [docs.rs](https://docs.rs),
  where the docs note which feature enables a given API.
- **[Releases](https://github.com/microsoft/windows-rs/releases)** — changelog and
  version history.
