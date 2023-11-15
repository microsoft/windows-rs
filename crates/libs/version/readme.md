## Windows version information

The [windows-version](https://crates.io/crates/windows-version) crate provides reliable operating system version information without the need for application manifest files.

* [Getting started](https://kennykerr.ca/rust-getting-started/)
* [Samples](https://github.com/microsoft/windows-rs/tree/0.52.0/crates/samples) <!-- link to samples for upcoming release -->
* [Releases](https://github.com/microsoft/windows-rs/releases)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-version]
version = "0.1"
```

Make use of Windows version information as needed:

```rust,no_run
use windows_version::*;

fn main() {
    println!("Current version: {:?}", OsVersion::current());

    if is_server() {
        println!("Running on a Windows Server release.");
    }

    if OsVersion::current() >= OsVersion::new(10, 0, 0, 12345) {
        println!("Can use a feature available on this version or later.")
    }
}
```
