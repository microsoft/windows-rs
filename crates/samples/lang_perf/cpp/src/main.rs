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
