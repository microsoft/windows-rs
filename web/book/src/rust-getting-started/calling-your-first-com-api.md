# Calling your first COM API

COM APIs are unique in that they expose functionality through interfaces. An interface is just a collection of virtual function pointers grouped together in what is known as a vtable, or virtual function table. This is not something that Rust supports directly, like C++ does, but the [windows](https://crates.io/crates/windows) crate provides the necessary code gen to make it possible and seamless. A COM API will still typically start life through a traditional C-style function call in order to get your hands on a COM interface. From there you might call other methods via the interface. 

Some COM-based APIs can get real complicated so let's start with a very simple example. The `CreateUri` function is [officially documented on MSDN](https://learn.microsoft.com/en-us/previous-versions/windows/internet-explorer/ie-developer/platform-apis/ms775098(v=vs.85)) as returning the `IUri` interface representing the results of parsing the given URI. The Rust [docs for the windows crate](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Com/fn.CreateUri.html) indicate that it resides in the `Win32::System::Com` module so we can configure our `windows` crate dependency accordingly:

```toml
[dependencies.windows]
version = "0.52"
features = [
    "Win32_System_Com",
]
```

And we can employ a `use` declaration to make this API a little more accessible. The `windows` crate's `core` module also provides a few helpers to make it easier to work with COM interfaces, so we'll include that as well:

```rust
use windows::{core::*, Win32::System::Com::*};
```

For this example, I'll just use a simple `main` function with a big `unsafe` block since virtually everything here is going to be `unsafe`. Why is that? Well the `windows` crate lets you call foreign functions and these are generally assumed to be `unsafe`. 

```rust
fn main() -> Result<()> {
    unsafe {
        
        Ok(())
    }
}
```

The only "interesting" point here is the use of the `Result` type from the `windows::core` module that provides Windows error handling to simplify the following API calls. And with that, we can call the `CreateUri` function as follows:

```rust
let uri = CreateUri(w!("http://kennykerr.ca"), Uri_CREATE_CANONICALIZE, 0)?;
```

There's quite a lot going on here. The first parameter is actually a `PCWSTR`, representing a null-terminated wide string used by many Windows APIs. The `windows` crate provides the handy `w!` macro for creating a valid null-terminated wide string as a compile-time constant. The second parameter is just the default flag specified by the official documentation. The third parameter is reserved and should thus be zero.

The resulting `IUri` object has various methods that we can now use to inspect the URI. The [official documentation](https://learn.microsoft.com/en-us/previous-versions/windows/internet-explorer/ie-developer/platform-apis/ms775038(v=vs.85)) describes the various interface methods and [the Rust docs](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Com/struct.IUri.html) give you a quick glimpse at their various signatures so that you can quickly figure out how to call them in Rust. For this example, let's just call two of them to print out the URI's domain and the HTTP port number:

```rust
let domain = uri.GetDomain()?;
let port = uri.GetPort()?;

println!("{domain} ({port})");
```

Under the hood, those methods will invoke the virtual functions through the COM interface and into the implementation provided by the API. They also provide a bunch of error and signature transformation to make it very natural to use from Rust. And that's it, running the sample should print something like this:

```
kennykerr.ca (80)
```

Here's the [full sample for reference](https://github.com/microsoft/windows-rs/blob/master/crates/samples/windows/com_uri/src/main.rs).
