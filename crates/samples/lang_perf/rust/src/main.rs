#[cfg(windows)]
#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
mod bindings;

// Default to a tiny count so `cargo run`/CI stays fast. Pass `--iterations N` or set
// `LANG_PERF_ITER=N` for a real measurement (the original used 10_000_000).
const DEFAULT_ITERATIONS: u64 = 1_000;

fn iterations() -> u64 {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--iterations"
            && let Some(value) = args.next()
        {
            return value.parse().expect("invalid --iterations value");
        }
    }
    std::env::var("LANG_PERF_ITER")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(DEFAULT_ITERATIONS)
}

fn main() {
    #[cfg(windows)]
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }

    #[cfg(not(windows))]
    eprintln!("lang_perf_rust requires Windows");
}

#[cfg(windows)]
fn run() -> windows_core::Result<()> {
    use bindings::*;
    use std::time::Instant;
    use windows_core::*;

    let iterations = iterations();
    println!("# Rust - {iterations} iterations");

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = Class::new()?;
    }
    report("Create", start);

    let object = Class::new()?;

    let start = Instant::now();
    for _ in 0..iterations {
        object.SetInt32Property(123)?;
        let _ = object.Int32Property()?;
    }
    report("Int32", start);

    // Build the HSTRING once, up front, so the loop measures only the set/get ABI
    // traffic - matching how C++'s `L"value"` literal is already a UTF-16 string.
    let value: HSTRING = "value".into();
    let start = Instant::now();
    for _ in 0..iterations {
        object.SetStringProperty(&value)?;
        let _ = object.StringProperty()?;
    }
    report("String", start);

    let start = Instant::now();
    for _ in 0..iterations {
        object.SetObjectProperty(&object)?;
        let _ = object.ObjectProperty()?;
    }
    report("Object", start);

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = object.NewObject()?.cast::<INonDefault>()?.Value()?;
    }
    report("Cast", start);

    Ok(())
}

#[cfg(windows)]
fn report(label: &str, start: std::time::Instant) {
    println!("{label}: {} ms", start.elapsed().as_millis());
}
