`windows-rs` provides Rust crates for Windows APIs, from COM support and system services to UI and
raw bindings.

Prefer the focused crates below when they cover the functionality you need. For additional APIs,
[`windows-bindgen`](crates/windows-bindgen.md) can generate a minimal project-specific binding,
while [`windows`](crates/windows.md) and [`windows-sys`](crates/windows-sys.md) provide broad API
coverage behind feature flags.

Each crate page covers usage and maintenance. Generated API documentation is available on
[docs.rs](https://docs.rs).

## Crates

### Core types and interop

| Crate | Description |
| --- | --- |
| [windows-core](crates/windows-core.md) | COM and WinRT types, traits, and authoring macros. |
| [windows-result](crates/windows-result.md) | Windows error handling and propagation. |
| [windows-strings](crates/windows-strings.md) | Windows string types, conversions, and macros. |
| [windows-link](crates/windows-link.md) | Links C-style functions without import libraries. |

### Values and collections

| Crate | Description |
| --- | --- |
| [windows-collections](crates/windows-collections.md) | WinRT collections. |
| [windows-numerics](crates/windows-numerics.md) | Graphics vectors and matrices. |
| [windows-reference](crates/windows-reference.md) | Implementation of WinRT `IReference<T>`. |
| [windows-time](crates/windows-time.md) | WinRT `DateTime` and `TimeSpan` types. |

### Async and threading

| Crate | Description |
| --- | --- |
| [windows-future](crates/windows-future.md) | WinRT asynchronous operations as Rust futures. |
| [windows-threading](crates/windows-threading.md) | Safe wrapper over the Win32 thread pool. |

### System services

| Crate | Description |
| --- | --- |
| [windows-registry](crates/windows-registry.md) | Safe Windows registry access. |
| [windows-services](crates/windows-services.md) | Support for authoring Windows services. |
| [windows-version](crates/windows-version.md) | Queries the Windows version at runtime. |

### UI and graphics

| Crate | Description |
| --- | --- |
| [windows-reactor](crates/windows-reactor.md) | Declarative UI library backed by WinUI 3. |
| [windows-canvas](crates/windows-canvas.md) | 2D graphics built on Direct2D. |
| [windows-composition](crates/windows-composition.md) | Windows composition visuals. |
| [windows-webview](crates/windows-webview.md) | Safe wrapper around the WebView2 browser control. |
| [windows-window](crates/windows-window.md) | Window creation and message dispatch. |
| [windows-animation](crates/windows-animation.md) | Wrapper around the Windows Animation Manager. |
| [windows-reactor-setup](crates/windows-reactor-setup.md) | Stages the Windows App Runtime. |

### Code generation and metadata

| Crate | Description |
| --- | --- |
| [windows-clang](crates/windows-clang.md) | Generates RDL from C and C++ headers using libclang. |
| [windows-default](crates/windows-default.md) | Embedded Windows metadata for build tools. |
| [windows-metadata](crates/windows-metadata.md) | Reads and writes ECMA-335 metadata. |
| [windows-rdl](crates/windows-rdl.md) | Parses RDL and generates ECMA-335 metadata. |
| [riddle](crates/riddle.md) | Checks and compiles RDL from the command line. |
| [cppwinrt](crates/cppwinrt.md) | Packages the C++/WinRT compiler. |

### Macro implementation

These crates implement macros exported by `windows-core`. Applications should use the
`windows-core` exports rather than depend on them directly.

| Crate | Description |
| --- | --- |
| [windows-implement](crates/windows-implement.md) | Implements the `#[implement]` macro. |
| [windows-interface](crates/windows-interface.md) | Implements the `#[interface]` macro. |

### Windows API bindings

Use these crates when the focused crates above do not cover the APIs you need.

| Crate | Description |
| --- | --- |
| [windows-bindgen](crates/windows-bindgen.md) | Generates Rust bindings from Windows metadata. |
| [windows](crates/windows.md) | Typed bindings for C-style, COM, and WinRT APIs. |
| [windows-sys](crates/windows-sys.md) | Raw bindings for C-style Windows APIs. |
