[![Build status](https://github.com/microsoft/winrt-rs/workflows/Build%20and%20Test/badge.svg)](https://github.com/microsoft/winrt-rs/actions)

## The Rust/WinRT language projection

Rust/WinRT follows in the tradition established by [C++/WinRT](https://github.com/microsoft/cppwinrt) of building language projections for the Windows Runtime using standard languages and compilers, providing a natural and idiomatic way for Rust developers to call Windows APIs. Rust/WinRT lets you call any WinRT API past, present, and future using code generated on the fly directly from the metadata describing the API and right into your Rust package where you can call them as if they were just another Rust module.

The Windows Runtime is based on Component Object Model (COM) APIs under the hood and is designed to be accessed through language projections like C++/WinRT and Rust/WinRT. Those language projections take the metadata describing various APIs and provide natural bindings for the target programming language. As you can imagine, this allows developers to more easily build apps and components for Windows using their desired language. You can then use those Windows APIs to build desktop apps, store apps, or something more unique like a component, NT service, or device driver.

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

For a more complete example, take a look at Robert Mikhayelyan's [Minesweeper](https://github.com/robmikh/minesweeper-rs).
