## Windows registry

The [windows-registry](https://crates.io/crates/windows-registry) crate reads and writes the Windows
registry.

* [Getting started](https://github.com/microsoft/windows-rs/blob/master/docs/readme.md)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-registry]
version = "0.6"
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

Use `options()` to select access rights, creation behavior, and a transaction:

```rust,no_run
use windows_registry::*;

fn main() -> Result<()> {
    let tx = Transaction::new()?;

    let key = CURRENT_USER
        .options()
        .read()
        .write()
        .create()
        .transaction(&tx)
        .open(r"software\windows-rs")?;

    key.set_u32("name", 123)?;

    tx.commit()?;

    Ok(())
}
```