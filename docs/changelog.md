# 0.9.0



# 0.8.0

- Simplify convertible parameters. ([#694](https://github.com/microsoft/windows-rs/pull/694))
- ErrorCode is now called HRESULT. ([#693](https://github.com/microsoft/windows-rs/pull/693))
- Add deprecated feature to conditionally include deprecated APIs ([#680](https://github.com/microsoft/windows-rs/pull/680))
- Add default "macros" feature ([#663](https://github.com/microsoft/windows-rs/pull/663))
- Updated metadata providing many fixes and improvements to Win32 APIs.
- Other improvements and fixes.

# 0.7.0

- Preserve original API case. ([#646](https://github.com/microsoft/windows-rs/pull/646))
- Add support for struct packing. ([#636](https://github.com/microsoft/windows-rs/pull/636))
- Add bitwise assign operators. ([#635](https://github.com/microsoft/windows-rs/pull/635))
- Add guid constants. ([#634](https://github.com/microsoft/windows-rs/pull/634))
- Add WebView2 example. ([#647](https://github.com/microsoft/windows-rs/pull/647))
- Updated metadata providing many fixes and improvements to Win32 APIs.
- Other improvements and fixes.

# 0.6.0

- Add Linux build support (compile only).
- Add full x64 and x86 build support.
- Workarounds for Rust documentation issues.
- Updated metadata providing many fixes and improvements to Win32 APIs.

# 0.5.0

- Support for Win32 arrays. ([#608](https://github.com/microsoft/windows-rs/pull/608))
- Updated metadata providing many fixes and improvements to Win32 APIs.
- Many more improvements and fixes.

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
