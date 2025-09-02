# How do I query for a specific COM interface?

COM and WinRT interfaces in the [windows](https://crates.io/crates/windows) crate implement the [ComInterface](https://microsoft.github.io/windows-docs-rs/doc/windows/core/trait.ComInterface.html) trait. This trait provides the `cast` method that will use `QueryInterface` under the hood to cast the current interface to another interface supported by the object. The `cast` method returns a `Result<T>` so that failure can be handled in a natural way in Rust.

For example, it is often necesary to get the `IDXGIDevice` interface for a given Direct3D device to interop with other rendering APIs. This is how you might create a swap chain for drawing and presenting to a Direct3D device. Let's imagine a simple function that accepts a Direct3D device and returns the underlying DXGI factory:

```rust
fn get_dxgi_factory(device: &ID3D11Device) -> Result<IDXGIFactory2> {
}
```

The first thing you need to do is query or cast the Direct3D device for its DXGI interface as follows:

```rust
let device = device.cast::<IDXGIDevice>()?;
```

If its more convenient, you can also make use of type inference as follows:

```rust
let device: IDXGIDevice = device.cast()?;
```

With the COM interface in hand, we need an `unsafe` block to call its methods:

```rust
unsafe {
}
```

Within the `unsafe` block, we can retrieve the device's physical adapter:

```rust
let adapter = device.GetAdapter()?;
```

And just for fun (or debugging), we might print out the adapter's name:

```rust
if cfg!(debug_assertions) {
    let mut desc = Default::default();
    adapter.GetDesc(&mut desc)?;
    println!("{}", String::from_utf16_lossy(&desc.Description));
}
```

Finally, we can return the adapter's parent and also the DXGI factory object for the device:

```rust
adapter.GetParent()
```

Running the sample I get the following impressive results:

```
AMD FirePro W4100
```

Here's a [more comprehensive DirectX example](https://github.com/microsoft/windows-rs/tree/master/crates/samples/windows/direct2d).

The `cast` method works equally well for WinRT classes and interfaces. It is particularly useful for [interop with WinRT APIs](https://github.com/microsoft/windows-rs/blob/master/crates/samples/windows/uiautomation/src/main.rs). 
