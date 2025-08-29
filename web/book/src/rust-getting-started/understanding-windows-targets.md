# Understanding the windows-targets crate

The [windows](https://crates.io/crates/windows) and [windows-sys](https://crates.io/crates/windows-sys) crates depend on the [windows-targets](https://crates.io/crates/windows-targets) crate for linker support. The `windows-targets` crate includes import libs, supports semantic versioning, and optional support for `raw-dylib`. It provides explicit import libraries for the following targets:

* i686_msvc
* x86_64_msvc
* aarch64_msvc
* i686_gnu
* x86_64_gnu
* x86_64_gnullvm
* aarch64_gnullvm

An import lib contains information the linker uses to resolve external references to functions exported by DLLs. This allows the operating system to identify a specific DLL and function export at load time. Import libs are both toolchain- and architecture-specific. In other words, different lib files are required depending on whether you're compiling with the MSVC or GNU toolchains and whether you're compiling for the x86 or ARM64 architectures. Note that import libraries don't contain any code, as static libraries do.

While the GNU and MSVC toolchains often provide some import libs to support C++ development, those lib files are often incomplete, missing, or just plain wrong. This can lead to linker errors that are very difficult to diagnose. The `windows-targets` crate ensures that all functions defined by the `windows` and `windows-sys` crates can be linked without relying on implicit lib files distributed by the toolchain. This ensures that dependencies can be managed with Cargo and streamlines cross-compilation. The `windows-targets` crate also contains version-specific lib file names ensuring semver compatibility. Without this capability, the linker will simply pick the first matching lib file name and fail to resolve any missing or mismatched imports. 

> **Note**: Ordinarily, you don't need to think about the `windows-targets` crate at all. The `windows` and `windows-sys` crates depend on the `windows-targets` crate automatically. Only in rare cases will you need to use it directly. 

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-targets]
version = "0.52"
```

Use the `link` macro to define the external functions you wish to call:

```rust
windows_targets::link!("kernel32.dll" "system" fn SetLastError(code: u32));
windows_targets::link!("kernel32.dll" "system" fn GetLastError() -> u32);
```

Make use of any Windows APIs as needed:

```rust
fn main() {
    unsafe {
        SetLastError(1234);
        assert_eq!(GetLastError(), 1234);
    }
}
```

By default the `link` macro will cause the linker to use the bundled import libs. Compiling with the `windows_raw_dylib` Rust build flag will cause Cargo to skip downloading the import libs altogether and instead use `raw-dylib` to resolve imports automatically. The Rust compiler will then create the import entries directly. This works without having to change any of your code. Without the `windows-targets` crate, switching between linker and `raw-dylib` imports requires very intricate code changes. As of this writing, the `raw-dylib` feature is not yet stable.
