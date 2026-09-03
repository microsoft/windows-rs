## windows-registry

The [windows-registry](https://crates.io/crates/windows-registry) crate reads and writes the Windows
registry.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-registry.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-registry]
version = "0.100"
```

```rust,no_run
use windows_registry::*;

fn main() -> Result<()> {
    let key = CURRENT_USER.create(r"software\windows-rs")?;

    key.set_u32("number", 123)?;
    key.set_string("name", "Rust")?;

    println!("{}", key.get_u32("number")?);
    println!("{}", key.get_string("name")?);

    Ok(())
}
```