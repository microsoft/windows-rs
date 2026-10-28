# windows-interface

Declare a COM interface with the `#[interface]` macro. Prefer generated interfaces when metadata
already describes the ABI.

- 📦 [crates.io](https://crates.io/crates/windows-interface)
- 📖 [docs.rs](https://docs.rs/windows-interface)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/interface)

## Get started

```toml
[dependencies]
windows-core = "0.100"
```

```rust
use windows_core::{IUnknown, interface};

#[interface("57f61c8b-b400-4035-a7c2-abbddf9e3559")]
unsafe trait IValue: IUnknown {
    fn get(&self) -> u32;
}
```

`windows-core` re-exports this macro through its default `proc-macros` feature.
A direct `windows-interface` dependency is not needed for application code.

## Preserve the ABI

The IID, direct base, method order, parameter types, and result types must match the native
interface exactly. An incorrect declaration can corrupt memory even when `QueryInterface`
succeeds. Keep it beside an authoritative ABI definition.

Methods take `&self`. Generics, async methods, variadics, and default bodies are not supported.
For HRESULT-only methods, `Result<()>` provides projected error handling; explicit `HRESULT`
preserves the raw status. Do not replace native out parameters with `Result<T>` by analogy.

The macro supplies callable methods and a `*_Impl` trait for
[windows-implement](https://docs.rs/windows-implement). Caller methods remain unsafe: the
declaration cannot prove pointer validity or API-specific invariants.
