# windows-collections

> Stock WinRT collection types backed by Rust containers.

- 📦 [crates.io](https://crates.io/crates/windows-collections)
- 📖 [docs.rs](https://docs.rs/windows-collections)
- 🚀 [Getting started](../../crates/libs/collections/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/collections)

## When to use this crate

Use `windows-collections` when a WinRT API expects a collection interface and your data starts as a
Rust `Vec` or `BTreeMap`. The crate supplies stock implementations of the standard WinRT
interfaces, so application and component code do not need to implement their vtables.

This is an ABI adapter, not a replacement for Rust collections. Keep data as `Vec` or `BTreeMap`
while it remains inside Rust, and convert at the boundary where a WinRT collection is required.

## Getting started

The crate [README](../../crates/libs/collections/readme.md) contains the dependency declaration and
minimal vector and map examples. The stock implementations require the default `std` feature. For
a first workflow:

1. Identify the exact interface in the receiving API's signature.
2. Build the values in a `Vec` or `BTreeMap`.
3. Convert the container with `InterfaceType::from`.
4. Pass or return the projected interface and handle its methods as fallible operations.

```rust
use windows_collections::IVector;

fn numbers() -> IVector<i32> {
    let values = IVector::<i32>::from(vec![1, 2, 3]);
    values.Append(4).unwrap();
    values
}
```

In application code, propagate the `Result` returned by collection methods instead of unwrapping
when the operation can fail.

## Choosing a collection interface

| Interface | Backing input | Choose it when the consumer needs |
| --- | --- | --- |
| `IIterable<T>` | `Vec<T::Default>` | Forward iteration only |
| `IVectorView<T>` | `Vec<T::Default>` | Indexed, read-only access |
| `IVector<T>` | `Vec<T::Default>` | Indexed mutation |
| `IObservableVector<T>` | `Vec<T::Default>` | Mutation notifications |
| `IMapView<K, V>` | `BTreeMap<K::Default, V::Default>` | Read-only lookup |
| `IMap<K, V>` | `BTreeMap<K::Default, V::Default>` | Insert, remove, and lookup |
| `IObservableMap<K, V>` | `BTreeMap<K::Default, V::Default>` | Map mutation notifications |

`T::Default` is the Rust storage form defined by the type's `RuntimeType` implementation. For
ordinary value types it is usually `T`. Projected nullable or interface types may use an
`Option<T>` storage form, so let the compiler guide the concrete container type.

The mutable interfaces own their Rust containers behind synchronization. View conversions own
read-only values, and `GetView` on a mutable collection returns a snapshot rather than a live view.

## Common tasks

### Build and read a map

```rust
use std::collections::BTreeMap;
use windows_collections::IMapView;
use windows_strings::{h, HSTRING};

fn main() {
    let values = BTreeMap::from([
        (HSTRING::from("one"), 1),
        (HSTRING::from("two"), 2),
    ]);
    let values = IMapView::<HSTRING, i32>::from(values);

    assert!(values.HasKey(h!("one")).unwrap());
    assert_eq!(values.Lookup(h!("two")).unwrap(), 2);
}
```

Missing keys and invalid vector indexes return the WinRT `E_BOUNDS` error. Check with `HasKey` or
`Size` when absence is expected rather than using an error as normal control flow.

### Iterate projected values

Generated `IntoIterator` implementations allow a borrowed or owned collection to be used in a
`for` loop. Each item is still a projected value, and map iteration yields
`IKeyValuePair<K, V>`:

```rust
# use windows_collections::IIterable;
let values = IIterable::<i32>::from(vec![1, 2, 3]);

for value in &values {
    println!("{value}");
}
```

Iteration uses `GetMany` in blocks instead of making one iterator ABI call for each item. Reading
each map pair's `Key` and `Value` remains a separate ABI operation.

### Publish collection changes

Convert a container to `IObservableVector` or `IObservableMap` when consumers need
`VectorChanged` or `MapChanged`. Register the projected event handler, retain its token, and pass
that token to the corresponding remove method when the subscription ends. Mutating through the
observable interface raises insertion, removal, replacement, or reset notifications.

## Important choices and pitfalls

- Convert to the interface the API asks for. A mutable `IVector<T>` is unnecessary when the API
  accepts `IIterable<T>` or `IVectorView<T>`.
- Conversion consumes the Rust container. Clone it first only when Rust must retain an independent
  copy.
- `GetView` snapshots the current contents. Later writes to the mutable source are not reflected in
  that view.
- Map iteration snapshots entries when `First` is called. It does not hold a live traversal of the
  backing `BTreeMap`.
- Calls cross a WinRT ABI and return `Result`; repeated fine-grained lookups can cost more than
  processing the original Rust container before conversion.

## Samples and next steps

There is no dedicated collection sample group. Start with the two runnable snippets in the
[README](../../crates/libs/collections/readme.md), then consult the
[docs.rs API](https://docs.rs/windows-collections) for the methods on the interface required by
your Windows API. [`windows-reference`](windows-reference.md) covers boxed WinRT values that can
also appear as collection elements.

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is for contributors and
is **not needed to use `windows-collections`**.

### How it's built

`src/bindings.rs` is generated by `tool_bindings` from `crates/tools/bindings/src/collections.txt`;
the collection adapters are hand-written. Used internally by the `windows` crate.

Iterating a collection (`for x in &vector`) yields through `BufferedIterator`
(`src/buffered_iterator.rs`), which fetches elements a block at a time via `GetMany` rather than one
`IIterator::next` ABI call per element. `windows-bindgen` generates the `IntoIterator` impls that
reference it. The block is sized to keep the buffer near 2 KB regardless of element size.

This batching applies to **maps too**: `IMap`/`IMapView`/`IObservableMap` implement
`IIterable<IKeyValuePair<K, V>>`, so `for pair in &map` drives the same `BufferedIterator` (yielding
`IKeyValuePair` items) - there is no separate, slower map path. A map iteration still costs more
than a vector of scalars, but not because of the iterator: `GetMany` over `IVector<Int32>`
bulk-copies the values inline, whereas over a map it returns a block of `IKeyValuePair` COM objects
(one `AddRef` each) and reading every `pair.Value()`/`Key()` remains a per-pair ABI crossing - the
`IMap` ABI offers no bulk key/value read. Separately, the *component-side* stock map iterator
snapshots its entries once at `First()` so each step is O(1) rather than re-walking the tree,
keeping a full traversal linear.

### Testing

Run `cargo test -p windows-collections`; see also the workspace test crates.
