# Frequently Asked Questions

## Basics

### How do I choose between the `windows` and `windows-sys` crates?

In general, `windows-sys` is meant to give users access to raw, zero-overhead bindings to the Windows API, while `windows` is meant as a more idiomatic, and when possible safer alternative. Below is a list of reasons you may choose `windows-sys` or `windows`.

| Reason | `windows` | `windows-sys`|
| --- | --- | --- |
| Fast compile times possible are one of your top concerns| | ✅ |
| You need COM or WinRT support. | ✅ | |
| You would prefer to use APIs that feel idiomatic to Rust. | ✅ | |
| Reason | You can't afford the very small cost of overhead | | ✅ |

### Where can I find the API I need?

The documentation for the `windows` crate lives [here](https://microsoft.github.io/windows-docs-rs). Since Windows types and functions usually have unique names, you can simply search for the API you need in the search bar, and the docs for that type will be displayed. This will also let you know what namespace your API lives in, so you can use the proper `use` statement.

### Why is the API I need missing?

The `windows` and `windows-sys` crates are generated from metadata such as the [win32metadata project](https://github.com/microsoft/win32metadata). It could be that the API you are searching for is missing from the metadata. In that case, the best thing to do is to [file an issue](https://github.com/microsoft/win32metadata/issues/new).

### Why are some equivalents to C/C++ macros from the Windows SDK missing?

The `windows` and `windows-sys` crates are generated from metadata such as the [win32metadata project](https://github.com/microsoft/win32metadata). These metadata definitions only include type definitions and function signatures and no actual implementation details. As such C/C++ macros are not included. You may find some equivalents of common C/C++ macros in the `windows` crate.

TODO: list equivalent of some common macros

### Is the nightly compiler needed for some features?

Currently, the `interface` and `implementation` proc macros (used to define and implement COM interfaces) require some nightly features. However, those features are in the process of being stablized and so the `interface` and `implementation` macros should soon be available on stable Rust. See [this issue](https://github.com/microsoft/windows-rs/issues/1523) for more information.

## COM

### How to query for a specific COM interface?

While raw COM uses the `QueryInterface` method for querying for an interface, the `windows` crate provides the safe `cast` method as an alternative.

```rust
 // Assume `a: IStringable`
 let b: IClosable = a.cast()?;
```

The cast method returns a Rust `Result` type allowing you to handle calls to `cast` just like you would any other fallible Rust function.

### How do I declare a new COM interface?

The `windows` crate comes with definitions for all the interfaces you're likely to encounter when using the Windows SDK, but you might still have the need to define an interface yourself.

To do so, you'll need to use the `interface` feature which (like any Cargo feature) can be enabled in your project's Cargo.toml file.

```toml
windows = { version = "..", features = ["interface"] }
```

Then inside your project you define the interface in much the same way you define a Rust crate just with the `#[windows::interface]` annotation above the trait definition.

```rust
/// My interface
#[windows::core::interface("BD1AE5E0-A6AE-11CE-BD37-504200C10000")]
unsafe trait IMyInterface: IUnknown {
    unsafe fn MyFunction(&self) -> windows::core::HRESULT;
}
```

### How do I implement an existing COM interface?

If you need to implement a COM interface for a type, you'll need to add the `implement` feature which (like any Cargo feature) can be enabled in your project's Cargo.toml file.

### How are optional COM interfaces handled?

```toml
windows = { version = "..", features = ["implement"] }
```

Then you'll need to declare that your type implements a particular interface by adding the `#[implement]` proc macro to your type and then writing an `impl` block for the interface. For a trait called `IMyTrait` you will need to implement the `IMyTrait_Impl` trait (note the trailing `_Impl` in the name).

```rust
#[windows::core::implement(IMyInterface)]
struct MyStruct;

impl IMyInterface_Impl for MyStruct {
    fn MyMethod(&self) -> windows::core::HRESULT {
        todo!("Your implementation goes here");
    }
}

```

## Translating the Windows API into Rust

### What is `IntoParam<...>`?

TODO

### What's the idiomatic Windows and Rust way to handle errors from Win32 APIs (`HRESULT`, `BOOL`, etc.)?

TODO

### How do I handle the various string types Windows uses?

TODO

## Other

### What is the `alloc` feature for?

TODO

### How do I create a `VARIANT`?

TODO

### Why does MSDN say a function takes an argument of type X but the windows crate says it takes an argument of type Y?

TODO

### How do I work with IAsyncOperation (async in general)?

TODO
