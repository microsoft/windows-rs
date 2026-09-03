## windows-threading

The [windows-threading](https://crates.io/crates/windows-threading) crate wraps the Windows thread
pool.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-threading.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-threading]
version = "0.100"
```

Submit work to the default pool:

```rust,no_run
windows_threading::submit(|| {
    println!("thread: {}", windows_threading::thread_id());

    loop {
        println!(".");
        windows_threading::sleep(1000);
    }
});
```

Process an iterator in parallel:

```rust,no_run
let counter = std::sync::RwLock::<usize>::new(0);

windows_threading::for_each(0..10, |value| {
    println!("thread: {}, value: {value}", windows_threading::thread_id());
    let mut counter = counter.write().unwrap();
    *counter += value;
});

assert_eq!(*counter.read().unwrap(), 45);
```
