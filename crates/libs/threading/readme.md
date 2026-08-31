## Windows threading

The [windows-threading](https://crates.io/crates/windows-threading) crate wraps the Windows thread
pool.

* [Getting started](https://github.com/microsoft/windows-rs/blob/master/docs/readme.md)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

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

Use `try_submit` when submission failure is recoverable. It returns the closure without calling it
if the Windows thread pool rejects the submission.

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

Use `Pool` to set thread limits and submit scoped work:

```rust,no_run
let set = std::sync::RwLock::<std::collections::HashMap<u32, usize>>::default();
let pool = windows_threading::Pool::new();
pool.set_thread_limits(2, 10);
pool.scope(|pool| {
    for _ in 0..10 {
        pool.submit(|| {
            windows_threading::sleep(10);
            let mut writer = set.write().unwrap();
            *writer.entry(windows_threading::thread_id()).or_default() += 1;
        })
    }
});

println!("{:#?}", set.read().unwrap());
```

Without `set_thread_limits`, Windows selects the pool size.
