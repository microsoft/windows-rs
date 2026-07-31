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

#[cfg(target_env = "msvc")]
unsafe extern "system" {
    fn lang_perf_cpp(iterations: u64) -> i32;
}

fn main() {
    let iterations = iterations();

    #[cfg(target_env = "msvc")]
    {
        stage_component(component_file());

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

#[cfg(target_env = "msvc")]
fn stage_component(file: &str) {
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        // WinRT activation probes the namespace-derived module name.
        let _ = std::fs::copy(dir.join(file), dir.join("LangPerf.dll"));
    }
}

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

#[cfg(all(test, target_env = "msvc"))]
mod tests {
    #[test]
    fn interop() {
        super::stage_component(super::component_file());
        let hr = unsafe { super::lang_perf_cpp(200) };
        assert!(hr >= 0, "lang_perf_cpp failed: {hr:#010x}");
    }
}
