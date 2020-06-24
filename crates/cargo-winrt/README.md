# cargo winrt

A simple utility for working with WinRT in a Rust project.

## Installation

To install `cargo winrt` run the following:

```
cargo install --git https://github.com/microsoft/winrt-rs cargo-winrt
```

## Usage

### Install

To install NuGet packages, add NuGet dependencies to your projects Cargo.toml file like so:

```toml
[package.metadata.winrt.dependencies]
"Win2D.uwp" = "1.25.0"
```

Nuget packages at a given URL are also supported:

```toml
[package.metadata.winrt.dependencies]
"Win2D.uwp" = { url = "http://example.com/my/nuget/package" }
```

As well as unzipped on your local file system:
```toml
[package.metadata.winrt.dependencies]
"Win2D.uwp" = { path = "../../my-nuget-package" }
```

Then to install run:

```
cargo winrt install
```

### Build

Rust WinRT works great with plain cargo, but if you want a way to ensure that you have installed all WinRT dependencies,
before building, you can run:

```
cargo winrt build
```

This is equivalent to:

```
cargo winrt install
cargo build
```

### Run

Just like building your WinRT projects, you can use `cargo winrt` to run your project in one step:

```
cargo winrt run
```

This is equivalent to:

```
cargo winrt install
cargo run
```