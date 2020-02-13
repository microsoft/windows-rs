
# The Rust/WinRT language projection

Provides native and familiar support for the Windows Runtime for Rust developers. 

This repo is still in early and active development. You can find more information about future plans for
[this project here](https://kennykerr.ca/2019/11/05/rust/).

The Rust winmd parser is part of the [xlang](https://github.com/microsoft/xlang) family of projects that help developers create APIs that can run on multiple platforms and be used with a variety of languages.

# Getting started

Start by adding the following to your Cargo.toml file:

```
[dependencies]
winrt = { git = "https://github.com/microsoft/winrt-rs" }
```

This will allow Cargo to download, build, and cache the Rust/WinRT support as a package directly from GitHub (once the repo is public).

Now use the `import` macro to import the desired winmd files and modules:

```rust
use winrt::*;

import!(
    dependencies
        "os"
    modules
        "windows.foundation"
        "windows.ui"
);
```

Finally, make use of any WinRT APIs as needed. For example, here is an example of using the `Windows.Foundation.Uri` class:

```rust
fn main() -> Result<()> {
    use windows::foundation::*;

    let uri = Uri::create_uri("http://kennykerr.ca")?;
    println!("domain: {}", uri.domain()?);
    println!("port: {}", uri.port()?);
    println!("string: {}", uri.to_string()?);

    Ok(())
}
```

This program will print the following output:

```
domain: kennykerr.ca
port: 80
string: http://kennykerr.ca/
```

And here is an example of using the `Windows.UI.Colors` class to produce a red `Color` struct:

```rust
fn main() -> Result<()> {
    use windows::ui::*;

    let red = Colors::red()?;
    assert!(red.a == 255);
    assert!(red.r == 255);
    assert!(red.g == 0);
    assert!(red.b == 0);
    println!("{:?}", red);

    Ok(())
}
```

This program will print the following output:

```
Color { a: 255, r: 255, g: 0, b: 0 }
```

# Contributing

This project welcomes contributions and suggestions.  Most contributions require you to agree to a
Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us
the rights to use your contribution. For details, visit https://cla.opensource.microsoft.com.

When you submit a pull request, a CLA bot will automatically determine whether you need to provide
a CLA and decorate the PR appropriately (e.g., status check, comment). Simply follow the instructions
provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/).
For more information see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or
contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.
