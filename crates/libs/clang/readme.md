## windows-clang

The [windows-clang](https://crates.io/crates/windows-clang) crate extracts declarations from C and
C++ source with libclang and emits [RDL](https://crates.io/crates/windows-rdl). It is the
header-facing stage of the Windows metadata pipeline:

```text
headers -> windows-clang -> RDL -> windows-rdl -> WinMD
```

Add the crate to your Cargo.toml:

```toml
[dependencies.windows-clang]
version = "0.100"
```

Pass each translation unit as an [`Input`][input], extract an immutable snapshot, and emit RDL:

```rust,no_run
let source = std::fs::read_to_string("Example.h").unwrap();
let input = windows_clang::Input::new("Example.h", source);
let snapshot = windows_clang::extract(
    [input],
    &["-x", "c++", "--target=x86_64-pc-windows-msvc"],
)
.unwrap();
let rdl = snapshot.emit("Example").unwrap();
std::fs::write("Example.rdl", rdl).unwrap();
```

The caller owns libclang installation, compiler arguments, package versions, import-library
mapping, output promotion, and RDL-to-WinMD compilation. See the
[crate documentation][docs] for the extraction and emission model.

[input]: https://docs.rs/windows-clang/latest/windows_clang/struct.Input.html
[docs]: https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-clang.md
