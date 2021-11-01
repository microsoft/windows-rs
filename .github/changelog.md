# 0.24

- Pre-packaged libs for GNU targets
- Updated Win32 metadata
- Other small improvements and fixes

# 0.23

- Pre-packaged libs are now merged
- Other small improvements and fixes

# 0.22

- Pre-packaged APIs
- Pre-packaged libs for MSVC targets
- Updated Win32 and WinRT metadata for Windows 11
- Many other small improvements and fixes

# 0.21

- Improved support for COM calls
- Many other small improvements and fixes

# 0.20

- Improved support for Win32 handle types
- Improved build macro dependency tracking
- Considerably better test coverage
- Support implementing query-like functions
- Many other small improvements and fixes
- Update Win32 metadata

# 0.19.0

- Support for COM interface implementations
- Many improvements to implementation support 
- Remove binary packaging support
- Minor fixes and improvements

# 0.18.0

- Fix debug formatting of structs
- Support for binary packaging
- Copy sub-directories within .windows
- More flexible macro formatting
- Support for static libs
- Support for raw-dylib
- Support for generic interface implementations
- Transform NTSTATUS functions into Result<()>
- Remove unecessary helper functions
- Update WinRT metadata

# 0.15.0

- Minor fixes and improvements.

# 0.14.0

- Packaging support.
- Updated Win32 metadata.
- Other improvements and fixes.

# 0.13.0

- Simpler COM function signatures. ([#909](https://github.com/microsoft/windows-rs/pull/909)) 

# 0.12.0

- Smarter dependency tracking. ([#896](https://github.com/microsoft/windows-rs/pull/896))
- Other improvements and fixes.

# 0.11.0

- Added build macro formatting tool. ([#828](https://github.com/microsoft/windows-rs/pull/828))
- Improved cross-compiler support. ([#830](https://github.com/microsoft/windows-rs/pull/830))
- Simplify use of unscoped enums. ([#843](https://github.com/microsoft/windows-rs/pull/843))
- Support for WinRT inheritance (Xaml). ([#821](https://github.com/microsoft/windows-rs/pull/821))
- Fix groups inside namespaces in build macro. ([#806](https://github.com/microsoft/windows-rs/pull/806))
- Add Direct3D 12 sample. ([#791](https://github.com/microsoft/windows-rs/pull/791))
- Improved build macro performance.
- Improved CI build validation.
- Updated Win32 metadata.
- Other improvements and fixes.

# 0.10.0

- Add support for scoped and unscoped enum constants. ([#787](https://github.com/microsoft/windows-rs/pull/787))
- Improved compile-time error reporting. ([#773](https://github.com/microsoft/windows-rs/pull/773))
- Preserve original names for HSTRING and IInspectable. ([#759](https://github.com/microsoft/windows-rs/pull/759))
- Add support for weak references. ([#745](https://github.com/microsoft/windows-rs/pull/745))
- Updated metadata providing many fixes and improvements to Win32 APIs. Notably, many Win32 namespaces have been renamed.
- Other improvements and fixes.

# 0.9.0

- Transform QueryInterface-like functions into generic functions. ([#735](https://github.com/microsoft/windows-rs/pull/735))
- Updated metadata providing many fixes and improvements to Win32 APIs.
- Other improvements and fixes.

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
