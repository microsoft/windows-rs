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

For ordinary header-to-RDL generation, configure and run the high-level builder:

```rust,no_run
windows_clang::clang()
    .input("Example.h")
    .args(["-x", "c++", "--target=x86_64-pc-windows-msvc"])
    .reference_default()
    .namespace("Example")
    .library("example.dll")
    .output("Example.rdl")
    .write()
    .unwrap();
```

The builder reads inputs and references, invokes the extractor, emits RDL, and writes the output.
Use [`Input`][input], `extract`, and `EmitOptions` directly when a generator needs to inspect or
combine immutable snapshots before emission.

The caller owns libclang installation, compiler arguments, package versions, import-library
discovery, architecture merging, output promotion, and RDL-to-WinMD compilation. See the [crate
documentation][docs] for both APIs and the extraction model.

[input]: https://docs.rs/windows-clang/latest/windows_clang/struct.Input.html
[docs]: https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-clang.md
