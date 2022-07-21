# Frequently Asked Questions

## How do I choose between the `windows` and `windows-sys` crates?

`windows-sys` provides raw, zero-overhead bindings to the Windows API, while `windows` provides a more idiomatic, and when possible, safer alternative. Listed below are a few reasons why you may choose one over the other.

| Reason | `windows` | `windows-sys`|
| --- | --- | --- |
| Fast compile times are one of your top concerns | | ✅ |
| You need COM or WinRT support | ✅ | |
| You would prefer to use APIs that feel idiomatic to Rust | ✅ | |

## Where can I find the API I need?

You can find documentation for the `windows` crate [here](https://microsoft.github.io/windows-docs-rs) and for `windows-sys` [here](https://docs.rs/windows-sys). You can search for the API you need from the search bar. The documentation will also list which namespace the API lives in, so you can pick the right features and build the correct `use` statement.

## Why is the API I need missing?

The `windows` and `windows-sys` crates are generated from [metadata](https://github.com/microsoft/windows-rs/tree/master/crates/libs/metadata/default). If you cannot find the API you need in the documentation, it's very possible the API is missing from metadata. In that case, the best thing to do is to [create an issue here](https://github.com/microsoft/win32metadata/issues/new).

## Why are some equivalents to C/C++ macros from the Windows SDK missing?

The `windows` and `windows-sys` crates are generated from metadata such as the [Win32 Metadata project](https://github.com/microsoft/win32metadata). This metadata only includes type definitions and function signatures, not macros, header-only functions, or function bodies. You may find some equivalents of common C/C++ helper macros and functions in the `windows` crate.

## How do I query for a specific COM interface?

While raw COM uses the `QueryInterface` method for querying for an interface, the `windows` crate provides the safe `cast` method, provided by the `Interface` trait, as an alternative.

```rust
 // Assume `a: IStringable`
 let b: IClosable = a.cast()?;
```

The cast method returns a Rust `Result` type allowing you to handle calls to `cast` just like you would any other fallible Rust function.

## How do I declare a new COM interface?

The `windows` crate comes with definitions for all the interfaces you're likely to encounter when using the Windows SDK, but you might still have the need to define an interface yourself.

To do so, you'll need to use the `interface` feature which (like any Cargo feature) can be enabled in your project's Cargo.toml file.

```toml
windows = { version = "..", features = ["interface"] }
```

Then inside your project you define the interface in much the same way you define a Rust trait just with the `#[windows::interface]` annotation above the trait definition.

```rust
/// My interface
#[windows::core::interface("1FFDDAD0-4799-4BCD-9A28-A583DA58F605")]
unsafe trait IMyInterface: IUnknown {
    unsafe fn MyFunction(&self) -> windows::core::HRESULT;
}
```

## How do I implement an existing COM interface?

If you need to implement a COM interface for a type, you'll need to add the `implement` feature which (like any Cargo feature) can be enabled in your project's Cargo.toml file.

```toml
windows = { version = "..", features = ["implement"] }
```

Then you'll need to declare that your type implements a particular interface by adding the `#[implement]` proc macro to your type and then writing an `impl` block for the interface. For an interface called `IMyInterface` you will need to implement the `IMyInterface_Impl` trait (note the trailing `_Impl` in the name).

```rust
#[windows::core::implement(IMyInterface)]
struct MyStruct;

impl IMyInterface_Impl for MyStruct {
    fn MyMethod(&self) -> windows::core::HRESULT {
        todo!("Your implementation goes here");
    }
}
```
