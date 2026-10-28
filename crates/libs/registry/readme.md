# windows-registry

Safe Windows registry access with owned keys, typed values, enumeration, and transactions.

- 📦 [crates.io](https://crates.io/crates/windows-registry)
- 📖 [docs.rs](https://docs.rs/windows-registry)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/registry)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/registry)

## Get started

```toml
[dependencies]
windows-registry = "0.100"
```

```rust,no_run
use windows_registry::*;

fn main() -> Result<()> {
    let key = CURRENT_USER.create(r"Software\windows-rs")?;
    key.set_u32("number", 123)?;
    key.set_string("name", "Rust")?;
    println!("{}", key.get_u32("number")?);
    println!("{}", key.get_string("name")?);
    Ok(())
}
```

## Choose access and update values

`open` requests read access; `create` opens or creates a key with read/write access. A returned
`Key` closes its handle when dropped. Use `options()` for custom permissions.

Use `CURRENT_USER` for per-user settings. Machine-wide locations under `LOCAL_MACHINE` may need
elevation. A 32-bit process can see a different redirected view; choose `wow64_32` or `wow64_64`
when multiple processes must address the same data.

`keys()` and `values()` enumerate children and values. `get_value` and `set_value` preserve the
registry type when typed helpers are not enough. Expandable strings are returned as stored.

For transactional writes, associate each key operation with a `Transaction`, then `commit`.
Dropping an uncommitted transaction rolls it back. Surface creation and commit failures.
`remove_tree` is recursive; restrict it to a key your application owns.
