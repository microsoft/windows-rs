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
windows-collections = "0.101"
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

All seven interfaces provide the same storage choices:

| Constructor | Retained values | Cost |
| --- | --- | --- |
| `From` / `into()` | `Send + Sync` default values | Direct storage |
| Unsafe `from_unchecked` | Caller-guaranteed access, cloning, comparison, and release | Direct storage |
| `from_marshaled` | Interface values resolved in the calling apartment | Agile references |

The unsafe constructor performs no per-object agility checks or wrapping. Its contract covers the
collection and its views, iterators, and pairs for their entire lifetimes. For mutable collections,
it also covers every value later supplied by any writer. Mutable direct storage still synchronizes
the container.

`from_marshaled` returns `Result` and needs COM initialization and apartment dispatch. Map keys
remain statically checked `Send + Sync` values. Failed marshaling does not commit a mutation or raise
a notification. Marshaling preserves apartment ownership; it does not make an incorrectly implemented
agile object thread-safe. Non-null marshaled storage is unsupported outside Windows.

`IVector<IInspectable>::from_read_only` creates an immutable vector with agile-reference storage.
It resolves values in the calling apartment and rejects mutation with `E_NOTIMPL`. Construction
can fail if a value cannot be marshaled. Its unsafe `from_read_only_unchecked` counterpart stores
caller-trusted objects directly, without a data lock or per-object wrapping.

Collection methods return `Result`; missing keys and invalid indexes report `E_BOUNDS`.
The Rust iterator adapters treat read errors as the end of iteration. Use `Result`-returning methods
when failures must be distinguished from exhaustion. Scalar interface getters report null as an
error with `HRESULT(0)`; vector and iterator `GetMany` preserve null entries as `None`.
For collections created from Rust containers, `GetView` returns a snapshot, not a live view.
For observable collections, retain the event token and unregister when the subscription ends.
