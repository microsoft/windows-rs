# windows-version

Windows version information without application-manifest version virtualization.
Prefer capability detection when an API offers it; use version checks for documented build
requirements.

- 📦 [crates.io](https://crates.io/crates/windows-version)
- 📖 [docs.rs](https://docs.rs/windows-version)
- 🧩 [Sample](https://github.com/microsoft/windows-rs/tree/master/crates/samples/version/version)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/version)

## Get started

```toml
[dependencies]
windows-version = "0.100"
```

```rust,no_run
use windows_version::OsVersion;

const REQUIRED: OsVersion = OsVersion::new(10, 0, 0, 22_000);

println!("Current version: {:?}", OsVersion::current());
if OsVersion::current() >= REQUIRED {
    println!("The required OS build is available.");
}
```

## Make a version decision

Keep the threshold beside the feature it guards. `OsVersion` compares major version, minor version,
service pack, and build in that order. A sufficient version does not guarantee optional components,
drivers, hardware, or policies; check those capabilities separately.

`is_server()` distinguishes server from workstation releases, not installed server roles.
`revision()` returns the update build revision (UBR), separate from `OsVersion` comparisons.
A revision of zero can also mean its registry query failed.
