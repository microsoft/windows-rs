# windows-clang-sys

Raw Clang C bindings and version-matched native semantic queries.
Most header-processing tools should use [windows-clang](https://docs.rs/windows-clang) instead.

- 📦 [crates.io](https://crates.io/crates/windows-clang-sys)
- 📖 [docs.rs](https://docs.rs/windows-clang-sys)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/clang-sys)

## Prepare the native libraries

The crate requires a matched `libclang.dll` and `windows_clang.dll` pair for its
`LIBCLANG_VERSION`. A stock libclang installation alone is not sufficient.

From this repository, build the pair and select it for the current PowerShell session:

```powershell
cargo run -p tool-clang-sys -- --native
$env:LIBCLANG_PATH = cargo run -q -p tool-clang -- path
```

Building requires Visual Studio C++ tools and its CMake/Ninja component. Windows x64 and ARM64
hosts can parse x86, x64, and ARM64 input. The first build downloads the LLVM development archive.
An explicit `LIBCLANG_PATH` is caller-owned; the tools do not replace its libraries.

## Load before calling

```toml
[dependencies]
windows-clang-sys = "0.100"
```

```rust,no_run
use windows_clang_sys::*;

fn main() {
    load().unwrap();
    unsafe {
        let version = clang_getClangVersion();
        let text = std::ffi::CStr::from_ptr(clang_getCString(version));
        println!("{}", text.to_string_lossy());
        clang_disposeString(version);
    }
}
```

`LIBCLANG_PATH` names the libraries' directory; otherwise the loader searches `PATH`.
Loading checks the runtime version and companion ABI. Distribute both libraries together.

Keep translation units alive while using their cursors and types. Dispose owned strings and
handles with the matching native functions. Raw handle types do not confer ownership or thread
safety. Check supplementary query statuses before reading their outputs.
