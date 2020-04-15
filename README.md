[![Build status](https://github.com/microsoft/winrt-rs/workflows/Build%20and%20Test/badge.svg)](https://github.com/microsoft/winrt-rs/actions)

## The Rust/WinRT language projection

Provides native and familiar support for the Windows Runtime for Rust developers. 

This repo is still in early and active development. You can find more information about future plans for
[this project here](https://kennykerr.ca/2019/11/05/rust/).

The Rust winmd parser is part of the [xlang](https://github.com/microsoft/xlang) family of projects that help developers create APIs that can run on multiple platforms and be used with a variety of languages.

## Getting started

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
