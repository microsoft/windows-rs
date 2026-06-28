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
    let iterations = iterations();

    #[cfg(target_env = "msvc")]
    {
        stage_component(component_file());

        unsafe extern "system" {
            fn lang_perf_cpp(iterations: u64) -> i32;
        }

        let hr = unsafe { lang_perf_cpp(iterations) };
        if hr < 0 {
            eprintln!("lang_perf_cpp failed: {hr:#010x}");
            std::process::exit(1);
        }
    }

    #[cfg(not(target_env = "msvc"))]
    {
        let _ = iterations;
        eprintln!("lang_perf_cpp requires the MSVC toolchain");
    }
}

// The component cdylibs use distinct names so every language's build can coexist in one
// target directory. Copy this consumer's own component in as LangPerf.dll -- the name
// WinRT activation probes -- right next to the executable so it is the one that loads.
#[cfg(target_env = "msvc")]
fn stage_component(file: &str) {
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        let _ = std::fs::copy(dir.join(file), dir.join("LangPerf.dll"));
    }
}

// `--component rust|cpp` selects which language's component this consumer activates, so the
// matrix benchmark can point every consumer at either implementation. Defaults to C++.
#[cfg(target_env = "msvc")]
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
    "langperf_cpp.dll"
}
