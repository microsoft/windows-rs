## windows-sys

The `windows-sys` crate is a zero-overhead fallback for the most demanding situations and primarily where the absolute best compile time is essential. It only includes function declarations (externs), structs, and constants. No convenience helpers, traits, or wrappers are provided.

- [Getting started](https://kennykerr.ca/rust-getting-started/)
- [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
- [Releases](https://github.com/microsoft/windows-rs/releases)
- [Feature search](https://microsoft.github.io/windows-rs/features)

Start by adding the following to your Cargo.toml file (see [Dependency Version Ranges](#dependency-version-ranges) for guidance on version specification):

```toml
[dependencies.windows-sys]
version = "0.61"
features = [
    "Win32_Security",
    "Win32_System_Threading",
    "Win32_UI_WindowsAndMessaging",
]
```

Make use of any Windows APIs as needed:

```rust,no_run
use windows_sys::{
    core::*, Win32::Foundation::*, Win32::System::Threading::*, Win32::UI::WindowsAndMessaging::*,
};

unsafe {
    let event = CreateEventW(std::ptr::null(), 1, 0, std::ptr::null());
    SetEvent(event);
    WaitForSingleObject(event, 0);
    CloseHandle(event);

    MessageBoxA(0 as _, s!("Ansi"), s!("Caption"), MB_OK);
    MessageBoxW(0 as _, w!("Wide"), w!("Caption"), MB_OK);
}
```

## Dependency Version Ranges

When adding `windows-sys` as a dependency, consider using looser semver ranges to improve ecosystem compatibility and reduce duplicate dependencies in your build graph. This is especially important for `windows-sys` since it's widely used throughout the Rust ecosystem:

```toml
# Specific version - may cause duplicate dependencies
[dependencies.windows-sys]
version = "0.61"

# Better: Use a wider range for ecosystem compatibility
[dependencies.windows-sys]
version = ">=0.59, <=0.61"  # Compatible with multiple versions
```

**Benefits of wider version ranges:**

- Reduces likelihood of duplicate `windows-sys` versions in dependency graphs
- Prevents `clippy::multiple-crate-versions` warnings in your consumers
- Improves compatibility with other crates that also depend on `windows-sys`
- Allows users more flexibility in dependency resolution

**Important:** When using wider version ranges, be sure to test your code with the minimum supported version. One approach is by using `cargo-minimal-versions`:

```pwsh
# Install cargo-minimal-versions and cargo-hack
cargo install --locked cargo-minimal-versions 
cargo install --locked cargo-hack

# Test with minimal dependency versions
cargo minimal-versions build
cargo minimal-versions test
```

This ensures your crate actually works with the minimum version you specify, not just the latest.
