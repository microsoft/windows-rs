# windows-collections

WinRT collection interfaces backed by Rust containers. Keep data in `Vec` or `BTreeMap` inside
Rust, then convert when a Windows API requires a collection interface.

- 📦 [crates.io](https://crates.io/crates/windows-collections)
- 📖 [docs.rs](https://docs.rs/windows-collections)
- 🧩 [Sample](https://github.com/microsoft/windows-rs/tree/master/crates/samples/collections/collections)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/collections)

## Get started

```toml
[dependencies]
windows-collections = "0.100"
```

```rust
use windows_collections::IIterable;

let numbers = IIterable::<i32>::from(vec![1, 2, 3]);
for value in numbers {
    println!("{value}");
}
```

## Choose the receiving interface

| Interface | Backing container | Access |
| --- | --- | --- |
| `IIterable<T>` | `Vec` | Forward iteration |
| `IVectorView<T>` | `Vec` | Read-only indexed access |
| `IVector<T>` | `Vec` | Indexed mutation |
| `IObservableVector<T>` | `Vec` | Mutation with notifications |
| `IMapView<K, V>` | `BTreeMap` | Read-only lookup |
| `IMap<K, V>` | `BTreeMap` | Lookup and mutation |
| `IObservableMap<K, V>` | `BTreeMap` | Mutation with notifications |

Conversions consume the Rust container and need the default `std` feature.
Interface-valued elements may require `Option<T>` storage; use the type the receiving API expects.

Collection methods return `Result`; missing keys and invalid indexes report `E_BOUNDS`.
For collections created from Rust containers, `GetView` returns a snapshot, not a live view.
For observable collections, retain the event token and unregister when the subscription ends.
