`windows-rs` provides Rust crates for Windows APIs, from COM support and system services to UI and
raw bindings.

Prefer the focused crates below when they cover the functionality you need. For additional APIs,
[`windows-bindgen`](../crates/libs/bindgen/readme.md) can generate a minimal project-specific
binding, while [`windows`](../crates/libs/windows/readme.md) and
[`windows-sys`](../crates/libs/sys/readme.md) provide broad API coverage behind feature flags for
binary applications.

Reusable libraries should avoid the `windows` and `windows-sys` umbrella crates. They add
dependency weight and version churn, and several libraries can bring incompatible versions of the
same large crate into one dependency tree. Prefer the focused crates for shared Windows types and
generate any remaining private bindings with `windows-bindgen`.

Each crate README provides setup, a working example, and essential usage guidance. Follow its
links for API documentation and samples.

## Crates

### Core types and interop

| Crate | Description |
| --- | --- |
| [windows-core](../crates/libs/core/readme.md) | COM and WinRT types, traits, and authoring macros. |
| [windows-result](../crates/libs/result/readme.md) | Windows error handling and propagation. |
| [windows-strings](../crates/libs/strings/readme.md) | Windows string types, conversions, and macros. |
| [windows-link](../crates/libs/link/readme.md) | Links C-style functions without import libraries. |

### Values and collections

| Crate | Description |
| --- | --- |
| [windows-collections](../crates/libs/collections/readme.md) | WinRT collections. |
| [windows-numerics](../crates/libs/numerics/readme.md) | Graphics vectors and matrices. |
| [windows-reference](../crates/libs/reference/readme.md) | Implementation of WinRT `IReference<T>`. |
| [windows-time](../crates/libs/time/readme.md) | WinRT `DateTime` and `TimeSpan` types. |

### Async and threading

| Crate | Description |
| --- | --- |
| [windows-future](../crates/libs/future/readme.md) | WinRT asynchronous operations as Rust futures. |
| [windows-threading](../crates/libs/threading/readme.md) | Safe wrapper over the Win32 thread pool. |

### System services

| Crate | Description |
| --- | --- |
| [windows-registry](../crates/libs/registry/readme.md) | Safe Windows registry access. |
| [windows-services](../crates/libs/services/readme.md) | Support for authoring Windows services. |
| [windows-version](../crates/libs/version/readme.md) | Queries the Windows version at runtime. |

### UI and graphics

| Crate | Description |
| --- | --- |
| [windows-reactor](../crates/libs/reactor/readme.md) | Declarative UI library backed by WinUI 3. |
| [windows-canvas](../crates/libs/canvas/readme.md) | 2D graphics built on Direct2D. |
| [windows-composition](../crates/libs/composition/readme.md) | Windows composition visuals. |
| [windows-webview](../crates/libs/webview/readme.md) | Safe wrapper around the WebView2 browser control. |
| [windows-window](../crates/libs/window/readme.md) | Window creation and message dispatch. |
| [windows-pickers](../crates/libs/pickers/readme.md) | Windows file and folder pickers. |
| [windows-animation](../crates/libs/animation/readme.md) | Wrapper around the Windows Animation Manager. |
| [windows-reactor-setup](../crates/libs/reactor-setup/readme.md) | Stages the Windows App Runtime. |

### Code generation and metadata

| Crate | Description |
| --- | --- |
| [windows-clang](../crates/libs/clang/readme.md) | Generates RDL from C and C++ headers using libclang. |
| [windows-clang-sys](../crates/libs/clang-sys/readme.md) | Raw Clang bindings and native semantic queries. |
| [windows-default](../crates/libs/default/readme.md) | Embedded Windows metadata for build tools. |
| [windows-metadata](../crates/libs/metadata/readme.md) | Reads and writes ECMA-335 metadata. |
| [windows-rdl](../crates/libs/rdl/readme.md) | Parses RDL and generates ECMA-335 metadata. |
| [cppwinrt](../crates/libs/cppwinrt/readme.md) | Packages the C++/WinRT compiler. |

### Macro implementation

These crates implement macros exported by `windows-core`. Applications should use the
`windows-core` exports rather than depend on them directly.

| Crate | Description |
| --- | --- |
| [windows-implement](../crates/libs/implement/readme.md) | Implements the `#[implement]` macro. |
| [windows-interface](../crates/libs/interface/readme.md) | Implements the `#[interface]` macro. |

### Windows API bindings

Use `windows-bindgen` when the focused crates above do not cover the APIs you need. Binary
applications may instead prefer the convenience of the pre-generated `windows` or `windows-sys`
umbrella crate.

| Crate | Description |
| --- | --- |
| [windows-bindgen](../crates/libs/bindgen/readme.md) | Generates Rust bindings from Windows metadata. |
| [windows](../crates/libs/windows/readme.md) | Typed bindings for C-style, COM, and WinRT APIs. |
| [windows-sys](../crates/libs/sys/readme.md) | Raw bindings for C-style Windows APIs. |
