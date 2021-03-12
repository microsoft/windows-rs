# 0.4.0

- Win32 unions and nested structs are now generated correctly.
- Improved support for many existing Win32 APIs and type signatures.
- Much smaller code gen, leading to faster build times.
- The Windows crate now uses generated bindings rather than hand-written bindings internally.
- Improved error handling functions.
- Improved support for common types like `BOOL`, `BSTR`, `PWSTR`, `PSTR`.
- Updated metadata providing many fixes and improvements to Win32 APIs.
- Simpler internal metadata representation to support a broader set of type systems more naturally.
- Many more improvements and fixes.

# 0.3.1

- Many improvements to COM support including interface hierarchies. ([#448](https://github.com/microsoft/windows-rs/pull/448))
- New COM helpers simplify common operations. ([#496](https://github.com/microsoft/windows-rs/pull/496))
- New `CoString` type representing null-terminated UTF16 strings backed by the COM task allocator required by some Windows APIs. ([#514](https://github.com/microsoft/windows-rs/pull/514))
- The `windows` crate is now dual-licensed under MIT or Apache. ([#476](https://github.com/microsoft/windows-rs/pull/476))
- COM interface methods are now marked `unsafe`. ([#508](https://github.com/microsoft/windows-rs/pull/508))
- Many new examples have been added to the [examples](https://github.com/microsoft/windows-rs/tree/master/examples) folder. ([#501](https://github.com/microsoft/windows-rs/pull/501))
- Improvements to error handling and propagation support.
- Improvements to numerics support.
- Improvements to build time.
- Other minor changes and fixes.
