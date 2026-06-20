# Internals

Contributor notes on how each crate is built and maintained — the codegen pipeline (`tool_bindings`, `tool_reactor`, `tool_package`), generated files, and conventions.

## Core & projection

- [windows](windows.md) — Safer projection of Windows APIs — C-style, COM, and WinRT — generated on the fly from Windows metadata.
- [windows-sys](windows-sys.md) — Zero-overhead raw bindings for C-style Windows APIs — externs, structs, and constants only.
- [windows-core](windows-core.md) — Fundamental COM and Windows type support that nearly every other `windows-*` crate builds on.

## Error handling & strings

- [windows-result](windows-result.md) — Efficient Windows error handling and propagation for Win32, COM, and WinRT.
- [windows-strings](windows-strings.md) — Windows string interop types and literal macros.

## Values & collections

- [windows-numerics](windows-numerics.md) — Graphics math types matching `Windows.Foundation.Numerics`.
- [windows-collections](windows-collections.md) — Stock implementations of the WinRT collection interfaces.
- [windows-reference](windows-reference.md) — Stock implementation of `IReference<T>`, the WinRT equivalent of `Nullable<T>`.
- [windows-time](windows-time.md) — WinRT `TimeSpan` and `DateTime` with idiomatic Rust conversions.

## Async & threading

- [windows-future](windows-future.md) — WinRT async types bridged to Rust `Future`s.
- [windows-threading](windows-threading.md) — A thin, safe wrapper over the Win32 thread pool.

## System services

- [windows-registry](windows-registry.md) — Safe, ergonomic access to the Windows registry.
- [windows-services](windows-services.md) — Author Windows services (daemons) in Rust.
- [windows-version](windows-version.md) — Query the running Windows version at runtime.

## COM macros & linking

- [windows-implement](windows-implement.md) — The `#[implement]` proc macro for implementing COM and WinRT interfaces in Rust.
- [windows-interface](windows-interface.md) — The `#[interface]` proc macro for declaring COM interfaces in Rust.
- [windows-link](windows-link.md) — Raw-dylib import support via the `link!` macro.
- [windows-targets](windows-targets.md) — Import libraries for older Rust compilers that predate `raw-dylib`.

## UI & graphics (experimental)

- [windows-reactor](windows-reactor.md) — A declarative UI library for Rust, backed by WinUI 3.
- [windows-canvas](windows-canvas.md) — A safe, fast 2D graphics library backed by Direct2D, Direct3D 11, DXGI, DirectWrite, and WIC.
- [windows-animation](windows-animation.md) — A safe wrapper around the Windows Animation Manager (`IUIAnimationManager`).
- [windows-reactor-setup](windows-reactor-setup.md) — Installs the Windows App Runtime dependency required by `windows-reactor` apps.

## Codegen & metadata tooling

- [windows-bindgen](windows-bindgen.md) — The code generator that turns Windows metadata (`.winmd`) into Rust bindings.
- [windows-metadata](windows-metadata.md) — A low-level reader/writer for ECMA-335 (`.winmd`) metadata.
- [windows-rdl](windows-rdl.md) — Parser for RDL (a readable IDL) plus an ECMA-335 metadata generator.
- [riddle](riddle.md) — A metadata compiler that turns RDL into `.winmd`.
- [cppwinrt](cppwinrt.md) — Bundles the C++/WinRT compiler (`cppwinrt.exe`) for use from Rust build scripts.
