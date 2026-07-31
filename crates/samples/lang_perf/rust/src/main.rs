#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
mod bindings;

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
    if let Err(error) = run(iterations(), component_file()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run(iterations: u64, component: &str) -> windows_core::Result<()> {
    use bindings::*;
    use std::time::Instant;
    use windows_core::*;

    stage_component(component);

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

    {
        let _revoker = object.Event(|_sender, _value| {})?;
        let start = Instant::now();
        for _ in 0..iterations {
            object.Raise()?;
        }
        report("Event", start);
    }

    let start = Instant::now();
    for _ in 0..iterations {
        let _revoker = object.Event(|_sender, _value| {})?;
    }
    report("AddRemove", start);

    {
        let count = iterations.min(u32::MAX as u64) as u32;
        let vector = object.Items(count)?;

        let start = Instant::now();
        let mut sum = 0i32;
        for value in &vector {
            sum = sum.wrapping_add(value);
        }
        std::hint::black_box(sum);
        report("IterateVector", start);

        let mut buffer = vec![0i32; count as usize];
        let start = Instant::now();
        let _ = vector.GetMany(0, &mut buffer)?;
        std::hint::black_box(&buffer);
        report("GetMany", start);

        let map = object.Map(count)?;
        let start = Instant::now();
        let mut sum = 0i32;
        for pair in &map {
            sum = sum.wrapping_add(pair.Value()?);
        }
        std::hint::black_box(sum);
        report("Map", start);
    }

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = object.Operation()?.join()?;
    }
    report("Async", start);

    let start = Instant::now();
    for _ in 0..iterations {
        object.SetReferenceProperty(Some(0))?;
        let _ = object.ReferenceProperty()?;
    }
    report("Reference", start);

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

fn stage_component(file: &str) {
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        // WinRT activation probes the namespace-derived module name.
        let _ = std::fs::copy(dir.join(file), dir.join("LangPerf.dll"));
    }
}

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

#[cfg(test)]
mod tests {
    #[test]
    fn interop() {
        super::run(200, super::component_file()).unwrap();
    }
}
