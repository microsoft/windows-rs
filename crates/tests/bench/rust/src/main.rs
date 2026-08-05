#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
mod bindings;

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

// A pass-through global allocator that counts client-side heap bytes. The Rust WinRT
// component is a separate cdylib with its own allocator, so its native object allocations
// are invisible here -- exactly the projection-side memory the C# client's
// `GC.GetTotalAllocatedBytes` also measures.
struct Counting;

static ALLOCATED: AtomicU64 = AtomicU64::new(0);

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATED.fetch_add(layout.size() as u64, Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

const DEFAULT_ITERATIONS: u64 = 1_000;

// Element count of the `IVector<i32>` used by the Vector metric. Fixed and small so the vector is
// built once; the metric then reads one element per iteration, wrapping the index.
const VECTOR_LEN: u32 = 1_024;

// Entry count of the `IMap<i32, i32>` used by the Lookup metric. Built once, then one lookup per
// iteration wrapping the key.
const MAP_LEN: u32 = 1_024;

fn iterations() -> u64 {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--iterations"
            && let Some(value) = args.next()
        {
            return value.parse().expect("invalid --iterations value");
        }
    }
    DEFAULT_ITERATIONS
}

fn main() {
    if let Err(error) = run(iterations()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run(iterations: u64) -> windows_core::Result<()> {
    use bindings::*;
    use windows_core::*;

    stage_component();

    let object = Widget::new()?;
    println!("# Rust consumer -> Bench component - {iterations} iterations");

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = Widget::new()?;
    }
    report("Create", start);

    let start = Instant::now();
    for _ in 0..iterations {
        object.SetInt32Property(123)?;
        let _ = object.Int32Property()?;
    }
    report("Int32", start);

    let start = Instant::now();
    for _ in 0..iterations {
        object.SetStringProperty(h!("value"))?;
        let _ = object.StringProperty()?;
    }
    report("String", start);

    let start = Instant::now();
    let mut sum = 0i32;
    for i in 0..iterations {
        sum = sum.wrapping_add(object.Add(i as i32, 1)?);
    }
    std::hint::black_box(sum);
    report("Add", start);

    // Interface: acquire the interface once, then measure steady calls through it.
    let value = object.cast::<INonDefault>()?;
    let start = Instant::now();
    let mut sum = 0i32;
    for _ in 0..iterations {
        sum = sum.wrapping_add(value.Value()?);
    }
    std::hint::black_box(sum);
    report("Interface", start);

    // Object: set and get metadata `Object` (`IInspectable`). The setter borrows the input and the
    // getter returns a +1 reference released at the end of each iteration.
    let start = Instant::now();
    for _ in 0..iterations {
        object.SetObjectProperty(&object)?;
        let value = object.ObjectProperty()?;
        std::hint::black_box(&value);
    }
    report("Object", start);

    // Event: subscribe one handler, then raise the event N times. Each raise calls from the
    // component back into this consumer's delegate through one interface pointer. windows-rs
    // invokes the delegate with no per-raise heap allocation.
    let revoker = object.Changed(|_sender, _value| {})?;
    let start = Instant::now();
    for i in 0..iterations {
        object.Signal(i as i32)?;
    }
    report("Event", start);
    drop(revoker);

    // AddRemove: subscribe and unsubscribe a handler N times, measuring event-registration churn.
    // windows-rs returns an `EventRevoker` that unregisters on drop, so each iteration builds a
    // delegate, registers it, and removes it.
    let start = Instant::now();
    for _ in 0..iterations {
        let revoker = object.Changed(|_sender, _value| {})?;
        drop(revoker);
    }
    report("AddRemove", start);

    // Vector: read elements from a projected generic collection `IVector<i32>`. The vector is built
    // once (its element count bounded), then `GetAt` reads one element per iteration - the
    // per-element cost of a generic collection call across the ABI. windows-rs dispatches the
    // generic vtable call with no per-element allocation.
    let vector = object.Items(VECTOR_LEN)?;
    let start = Instant::now();
    let mut sum = 0i32;
    for i in 0..iterations {
        sum = sum.wrapping_add(vector.GetAt((i % u64::from(VECTOR_LEN)) as u32)?);
    }
    std::hint::black_box(sum);
    report("Vector", start);

    // IterateVector: `foreach`-style traversal of the whole collection. Each pass walks every
    // element of the vector; the pass count is reduced so the total element visits stay bounded.
    // windows-rs iterates with a `BufferedIterator` that fills a stack buffer via one `GetMany` per
    // block, so the per-element cost is a buffer read, not a virtual call.
    let iterate_passes = iterations.min(100_000);
    let start = Instant::now();
    let mut sum = 0i32;
    for _ in 0..iterate_passes {
        for v in &vector {
            sum = sum.wrapping_add(v);
        }
    }
    std::hint::black_box(sum);
    report("IterateVector", start);
    drop(vector);

    // GetMany: copy a vector sized to the requested iteration count into one caller-owned buffer.
    // Construction and allocation happen before the timer so this isolates the bulk ABI call.
    let bulk_count = iterations.min(i32::MAX as u64) as u32;
    let vector = object.Items(bulk_count)?;
    let mut buffer = vec![0i32; bulk_count as usize];
    let start = Instant::now();
    let actual = vector.GetMany(0, &mut buffer)?;
    std::hint::black_box((actual, &buffer));
    report("GetMany", start);
    drop(vector);

    // Map: enumerate an IMap<string, int> through IIterable<IKeyValuePair<string, int>>. Repeat a
    // bounded 1024-entry map enough times to keep total entry visits near the requested count.
    let map = object.StringMap(MAP_LEN)?;
    let map_passes = (iterations / u64::from(MAP_LEN)).max(1);
    let start = Instant::now();
    let mut sum = 0i32;
    for _ in 0..map_passes {
        for pair in &map {
            sum = sum.wrapping_add(pair.Value()?);
        }
    }
    std::hint::black_box(sum);
    report("Map", start);
    drop(map);

    // Lookup: read values from a projected generic dictionary `IMap<i32, i32>` by key. The map is
    // built once, then `Lookup` reads one value per iteration - the per-lookup cost of a generic
    // dictionary call across the ABI. windows-rs dispatches the generic vtable call with no
    // per-lookup allocation.
    let map = object.Map(MAP_LEN)?;
    let start = Instant::now();
    let mut sum = 0i32;
    for i in 0..iterations {
        sum = sum.wrapping_add(map.Lookup((i % u64::from(MAP_LEN)) as i32)?);
    }
    std::hint::black_box(sum);
    report("Lookup", start);
    drop(map);

    // VectorView: read elements from the read-only view `IVectorView<i32>`. Same per-element ABI
    // cost as `Vector` - a single generic vtable call - confirming the view projects and dispatches
    // like the mutable collection.
    let view = object.ItemsView(VECTOR_LEN)?;
    let start = Instant::now();
    let mut sum = 0i32;
    for i in 0..iterations {
        sum = sum.wrapping_add(view.GetAt((i % u64::from(VECTOR_LEN)) as u32)?);
    }
    std::hint::black_box(sum);
    report("VectorView", start);
    drop(view);

    // MapView: read values from the read-only view `IMapView<i32, i32>` by key. Same per-lookup ABI
    // cost as `Lookup`.
    let map_view = object.MapView(MAP_LEN)?;
    let start = Instant::now();
    let mut sum = 0i32;
    for i in 0..iterations {
        sum = sum.wrapping_add(map_view.Lookup((i % u64::from(MAP_LEN)) as i32)?);
    }
    std::hint::black_box(sum);
    report("MapView", start);
    drop(map_view);

    // Reference: box a nullable Int32 input as IReference<int> and unbox the returned reference.
    let start = Instant::now();
    for _ in 0..iterations {
        object.SetReferenceProperty(Some(0))?;
        let value = object.ReferenceProperty()?;
        std::hint::black_box(value);
    }
    report("Reference", start);

    // Async: obtain an already-completed IAsyncOperation<int> and synchronously read its result.
    let start = Instant::now();
    for _ in 0..iterations {
        let value = object.Operation()?.join()?;
        std::hint::black_box(value);
    }
    report("Async", start);

    // Error: call a method that always returns a failing HRESULT and observe the error. Rust
    // surfaces the failure as `Result` -- a branch on an integer, no stack unwinding -- so the
    // error path costs about the same as the success path. The exception-based projections pay to
    // throw and catch, so this loop runs a reduced count (throwing millions of exceptions would
    // dominate the run).
    let fail_iterations = iterations.min(1_000_000);
    let start = Instant::now();
    let mut errors = 0u64;
    for _ in 0..fail_iterations {
        if object.Fail().is_err() {
            errors += 1;
        }
    }
    std::hint::black_box(errors);
    report("Error", start);

    // Leak check: activate, cast, and drop N objects, then confirm the component's live instance
    // count returns to the baseline. Rust's `Drop` releases the interface pointer, and `cast`
    // returns an owned reference that is released when the temporary drops, so every AddRef
    // balances. This is the reference behaviour the other projections are checked against.
    let baseline = object.LiveCount()?;
    for _ in 0..iterations {
        let scratch = Widget::new()?;
        let _ = scratch.Value()?;
    }
    println!("Leak: {}", object.LiveCount()? - baseline);

    // Scalability: retain N live objects and report the client-side heap cost per object.
    // A projected `Widget` is a single COM pointer, so the only client allocation is the
    // Vec backing store -- the native object lives in the component's allocator.
    let live = iterations.min(1_000_000) as usize;
    let before = ALLOCATED.load(Ordering::Relaxed);
    let mut widgets = Vec::with_capacity(live);
    for _ in 0..live {
        widgets.push(Widget::new()?);
    }
    let after = ALLOCATED.load(Ordering::Relaxed);
    std::hint::black_box(&widgets);
    let bytes = after - before;
    println!(
        "Live-{live}: {bytes} bytes ({:.1} bytes/object)",
        bytes as f64 / live as f64
    );

    Ok(())
}

fn report(label: &str, start: Instant) {
    println!("{label}: {} ms", start.elapsed().as_millis());
}

// Copy this consumer's component cdylib in as `Bench.dll` -- the module WinRT activation
// probes for the `Bench` namespace -- next to the executable so it loads without
// registration.
fn stage_component() {
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        let _ = std::fs::copy(dir.join("bench_component.dll"), dir.join("Bench.dll"));
    }
}

// Runs the Rust consumer against the Rust component with a tiny iteration count so `cargo test`
// exercises the projection end to end -- activation, properties, a method, and a QueryInterface
// cast -- not just that the bindings compile. The component cdylib is a build dependency, so
// cargo stages it beside this test binary.
#[cfg(test)]
mod tests {
    #[test]
    fn interop() {
        super::run(200).unwrap();
    }
}
