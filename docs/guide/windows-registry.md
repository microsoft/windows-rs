# windows-registry

> Simple, safe, and efficient access to the Windows registry.

- 📦 [crates.io](https://crates.io/crates/windows-registry)
- 📖 [API reference (docs.rs)](https://docs.rs/windows-registry)
- 🛠 [Internals](../internals/windows-registry.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/registry)

## Overview

`windows-registry` wraps the Win32 registry APIs behind a small, safe surface.
Start from one of the predefined roots — `CURRENT_USER`, `LOCAL_MACHINE`, or
`CLASSES_ROOT` — then `create` or `open` keys and read or write typed values.
The `options()` builder adds fine-grained control, including transactions.

## Example

```rust,no_run
use windows_registry::CURRENT_USER;

fn main() -> windows_registry::Result<()> {
    let key = CURRENT_USER.create("software\\windows-rs")?;

    key.set_u32("number", 123)?;
    key.set_string("name", "Rust")?;

    assert_eq!(key.get_u32("number")?, 123);
    assert_eq!(key.get_string("name")?, "Rust");
    Ok(())
}
```