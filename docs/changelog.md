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
