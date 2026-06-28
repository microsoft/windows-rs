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
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> windows_core::Result<()> {
    use bindings::*;
    use std::time::Instant;
    use windows_core::*;

    stage_component(component_file());

    let iterations = iterations();

    let object = Class::new()?;
    println!(
        "# Rust consumer -> {} component - {iterations} iterations",
        object.Lang()?.to_string_lossy()
    );

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = Class::new()?;
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
    for _ in 0..iterations {
        object.SetObjectProperty(&object)?;
        let _ = object.ObjectProperty()?;
    }
    report("Object", start);

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = object.ObjectProperty()?.cast::<INonDefault>()?.Value()?;
    }
    report("Cast", start);

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = object.Next();
    }
    report("Error", start);

    Ok(())
}

fn report(label: &str, start: std::time::Instant) {
    println!("{label}: {} ms", start.elapsed().as_millis());
}

// The component cdylibs use distinct names so every language's build can coexist in one
// target directory. Copy this consumer's own component in as LangPerf.dll -- the name
// WinRT activation probes -- right next to the executable so it is the one that loads.
fn stage_component(file: &str) {
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        let _ = std::fs::copy(dir.join(file), dir.join("LangPerf.dll"));
    }
}

// `--component rust|cpp` selects which language's component this consumer activates, so the
// matrix benchmark can point every consumer at either implementation. Defaults to Rust.
fn component_file() -> &'static str {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--component"
            && let Some(value) = args.next()
        {
            return match value.as_str() {
                "cpp" => "langperf_cpp.dll",
                "rust" => "langperf_rust.dll",
                other => panic!("unknown --component '{other}' (expected rust or cpp)"),
            };
        }
    }
    "langperf_rust.dll"
}
