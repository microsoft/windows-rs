# How do I create stock collections for WinRT collection interfaces?

Beyond [implementing COM interfaces](how-to-implement-com-interface.md) yourself, the [windows](https://crates.io/crates/windows) crate provides stock collection implementations for common WinRT collection interfaces. Implementing WinRT collection interfaces can be quite challenging, so this should save you a lot of effort in many cases. The `implement` feature is required to make use of these stock implementations.

Let's consider a few examples. The WinRT collection interfaces are all defined in the `Foundation::Collections` module, so we'll start by adding a dependency on the `windows` crate and include the `Foundation_Collections` feature:

```toml
[dependencies.windows]
version = "0.52"
features = [
    "implement",
    "Foundation_Collections",
]
```

Creating a collection is as simple as using the `TryFrom` trait on existing `Vec` or `BTreeMap`, depending on the kind of collection:

| WinRT interface | From |
| --- | --- |
| `IIterable<T>` | `Vec<T::Default>` |
| `IVectorView<T>` | `Vec<T::Default>` |
| `IMapView<K, V>` | `BTreeMap<K::Default, V::Default>` |

So if you need a `IIterable` implementation of `i32` values you can create it as follows:

```rust
use windows::{core::*, Foundation::Collections::*};

fn main() -> Result<()> {
    let collection = IIterable::<i32>::try_from(vec![1, 2, 3])?;

    for n in collection {
        println!("{n}");
    }

    Ok(())
}
```

The resulting `collection` will implement all of the specialized `IIterable<i32>` methods. 

Did you notice the `T::Default` in the table above? The challenge is that when the WinRT collection contains nullable types, unlike `i32`, then the collection must necessarily support a backing implementation that support expressing this. The `Default` associated type just replaces `T` with `Option<T>` for such nullable, or reference, types. 

Let's consider a slightly more contrived example. Here we'll create an `IMapView` with strings for keys and interfaces for values. WinRT strings are not nullable but interfaces are. WinRT strings are represented by `HSTRING` in the `windows` crate and for the interface we'll just use an `IStringable` implementation:

```rust
use windows::Foundation::*;

#[implement(IStringable)]
struct Value(&'static str);

impl IStringable_Impl for Value {
    fn ToString(&self) -> Result<HSTRING> {
        Ok(self.0.into())
    }
}
```

We can now create a `std` collection as follows:

```rust
use std::collections::*;

let map = BTreeMap::from([
    ("hello".into(), Some(Value("HELLO").into())),
    ("hello".into(), Some(Value("WORLD").into())),
]);
```

The Rust compiler naturally infers the exact type: `BTreeMap<HSTRING, Option<IStringable>>`.

Finally, we can wrap that `BTreeMap` inside a WinRT collection with the `TryInto` trait as follows:

```rust
let map: IMapView<HSTRING, IStringable> = map.try_into()?;

for pair in map {
    println!("{} - {}", pair.Key()?, pair.Value()?.ToString()?);
}
```
