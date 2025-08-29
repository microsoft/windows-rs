# Calling your first API with the windows crate

So you want to get a feel for calling a simple Windows API. Where to start? Let's look at a relatively simple API for submitting callbacks to the thread pool. You can read [more about this API here](https://learn.microsoft.com/en-us/archive/msdn-magazine/2011/august/windows-with-c-the-windows-thread-pool-and-work).

The first step is to add a dependency on the [windows](https://crates.io/crates/windows) crate and indicate which features you'd like to access:

```toml
[dependencies.windows]
version = "0.52"
features = [
    "Win32_Foundation",
    "Win32_System_Threading",
]
```

Why these two features? Well, the thread pool API is defined in the `Win32::System::Threading` module and we'll also use a handful of definitions from the `Win32::Foundation` module. If you're unsure, the docs for any given API provide a helpful comment indicating which features are required. For example, here are the docs for [WaitForThreadpoolWorkCallbacks](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.WaitForThreadpoolWorkCallbacks.html) where you can see it depends on both of these features since it is defined in the `Win32::System::Threading` module and depends on `BOOL` which is defined in the `Win32::Foundation` module.

Cargo will now handle the heavy lifting, tracking down the dependencies and making sure the import libs are present, so that we can simply call these APIs in Rust without any further configuration. We can employ a `use` declaration to make these APIs a little more accessible:

```rust
use windows::{core::Result, Win32::System::Threading::*};
```

In order to "prove" that the code works and yet keep it real simple let's just use the thread pool to increment a counter some number of times. Here we can use a reader-writer lock for safe and multi-threaded access to the counter variable:  

```rust
static COUNTER: std::sync::RwLock<i32> = std::sync::RwLock::new(0);
```

For this example, I'll just use a simple `main` function with a big `unsafe` block since virtually everything here is going to be `unsafe`. Why is that? Well the `windows` crate lets you call foreign functions and these are generally assumed to be `unsafe`. 

```rust
fn main() -> Result<()> {
    unsafe {
        
    }

    Ok(())
}
```

The thread pool API is modeled as a set of "objects" exposed via a traditional C-style API. The first thing we need to do is create a work object:

```rust
let work = CreateThreadpoolWork(Some(callback), None, None)?;
```

The first parameter is a pointer to a callback function. The remaining parameters are optional and you can read more about them in my thread pool series on MSDN.

The callback itself must be a valid C-style callback according to the signature expected by the thread pool API. Here's a simple callback that will increment the count:

```rust
extern "system" fn callback(_: PTP_CALLBACK_INSTANCE, _: *mut std::ffi::c_void, _: PTP_WORK) {
    let mut counter = COUNTER.write().unwrap();
    *counter += 1;
}
```

The parameters can safely be ignored but do come in handy from time to time. At this point, we have a valid work object but nothing is happening yet. In order to kick off some "work", we need to submit the work object to the thread pool. You can do so as many times as you'd like, so lets go ahead and do it ten times:

```rust
for _ in 0..10 {
    SubmitThreadpoolWork(work);
}
```

You can now expect the callbacks to run concurrently, hence the `RwLock` above. Of course, with all of that concurrency we need some way to tell when the work is done. That's the job of the `WaitForThreadpoolWorkCallbacks` function:

```rust
WaitForThreadpoolWorkCallbacks(work, false);
```

The second parameter indicates whether we would like to cancel any pending callbacks that have not started to execute. Passing false here thus indicates that we would like the wait function to block until all of the submitted work has completed. At that point, we can safely close the work object to free its memory:

```rust
CloseThreadpoolWork(work);
```

And just to prove that it works reliably, we can print out the counter's value:

```rust
let counter = COUNTER.read().unwrap();
println!("counter: {}", *counter);
```

Running the sample should print something like this:

```
counter: 10
```

Here's the [full sample for reference](https://github.com/microsoft/windows-rs/blob/master/crates/samples/windows/thread_pool_work/src/main.rs).
