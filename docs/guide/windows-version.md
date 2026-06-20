# windows-version

> Reliable OS version information without an application manifest.

- 📦 [crates.io](https://crates.io/crates/windows-version)
- 📖 [API reference (docs.rs)](https://docs.rs/windows-version)
- 🛠 [Internals](../internals/windows-version.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/version)

## Overview

`windows-version` reports the real operating system version even when the
process has no version manifest — useful for feature detection. `OsVersion`
implements `Ord`, so you can compare against a minimum build, and `is_server`
distinguishes Windows Server releases.

## Example

```rust
use windows_version::OsVersion;

let current = OsVersion::current();

// Every supported target is at least Windows 10.
assert!(current >= OsVersion::new(10, 0, 0, 0));
```