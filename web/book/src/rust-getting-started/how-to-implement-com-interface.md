# How do I implement an existing COM interface?

In some cases, you may need to implement an existing COM interface rather than simply calling an existing implementation provided by the operating system. This is where the `implement` feature and macro come in handy. The [windows](https://crates.io/crates/windows) crate provides optional implementation support hidden behind the `implement` feature. Once enabled, the [implement](https://docs.rs/windows-implement/latest/windows_implement/attr.implement.html) macro may be used to implement any number of COM interfaces. The macro takes care of implementing `IUnknown` itself. 

Let's implement a simple interface defined by Windows to illustrate. The `IPersist` interface is defined in the `Win32::System::Com` module, so we'll start by adding a dependency on the `windows` crate and include the `Win32_System_Com` feature:

```toml
[dependencies.windows]
version = "0.52"
features = [
    "implement",
    "Win32_System_Com",
]
```

The `implement` feature unlocks the implementation support. 

The `implement` macro is included by the `windows::core` module so we'll keep things simple by including it all as follows:

```rust
use windows::{core::*, Win32::System::Com::*};
```

Now its time for the implementation:

```rust
#[implement(IPersist)]
struct Persist(GUID);
```

The `implement` macro will provide the necessary implementation for the `IUnknown` interface's lifetime management and interface discovery for whatever interfaces are included in the attribute. In this case, only `IPersist` is to be implemented. 

The implementation itself is defined by a trait that follows the `<interface name>_Impl` pattern and its up to us to implement it for our implementation as follows:

```rust
impl IPersist_Impl for Persist_Impl {
    fn GetClassID(&self) -> Result<GUID> {
        Ok(self.0)
    }
}
```

The IPersist interface, originally [documented here](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-ipersist), has a single method that returns a `GUID`, so we'll just implement it by returning the value contained within our implementation. The `window` crate and `implement` macro will take care of the rest by providing the actual COM virtual function call and virtual function table layout needed to turn this into a heap-allocated and reference-counted COM object. 

All that remains is to move, or box, the implementation into the COM implementation provided by the `implement` macro through the `Into` trait:

```rust
let guid = GUID::new()?;
let persist: IPersist = Persist(guid).into();
```

At this point, we can simply treat `persist` as the COM object that it is:

```rust
let guid2 = unsafe { persist.GetClassID()? };
assert_eq!(guid, guid2);
println!("{:?}", guid);
```

Here's a [complete example](https://github.com/microsoft/windows-rs/tree/master/crates/samples/windows/bits).
