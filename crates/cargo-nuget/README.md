# cargo nuget

A simple utility for working with NuGet packages in a Rust project in a way that [WinRT-rs](https://github.com/microsoft/winrt-rs) can understand. 

## Installation

To install `cargo nuget` run the following:

```
cargo install --git https://github.com/microsoft/winrt-rs cargo-nuget
```

## Usage

### Install

To install NuGet packages, add NuGet dependencies to your projects Cargo.toml file like so:

```toml
[package.metadata.nuget_dependencies]
"Win2D.uwp" = "1.25.0"
```

Then to install run:

```
cargo nuget install
```