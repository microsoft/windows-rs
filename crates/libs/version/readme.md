## windows-version

The [windows-version](https://crates.io/crates/windows-version) crate reads the Windows version
without relying on the application manifest.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-version.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-version]
version = "0.100"
```

```rust,no_run
use windows_version::*;

println!("Current version: {:?}", OsVersion::current());

if is_server() {
    println!("Running on a Windows Server release.");
}

if OsVersion::current() >= OsVersion::new(10, 0, 0, 12345) {
    println!("Can use a feature available on this version or later.")
}
```
