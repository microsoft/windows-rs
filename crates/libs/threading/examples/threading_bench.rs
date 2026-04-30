//! Benchmark `windows_threading::submit` (Win32 thread pool via
//! `TrySubmitThreadpoolCallback`) against `std::thread::spawn` for the
//! workload that `windows-future` actually issues: one fire-and-forget
//! submission per `IAsync*::spawn`, with a near-empty closure body.
//!
//! Goal: decide whether `windows-future` should keep its dependency on
//! `windows-threading` or just call `std::thread::spawn` directly. See
//! `docs/cross-todo.md`, Stage 4.
//!
//! Run on a Windows host (Release mode):
//!
//! ```text
//! cargo run --release --example threading_bench -p windows-threading
//! ```
//!
//! The benchmark prints a small table with three workloads:
//! - `single`:   submit one closure, wait, repeat, average over 1000 trials —
//!               approximates the `IAsync*::spawn(...).await` round trip.
//! - `burst`:    submit 10 000 closures back-to-back, wait for them all.
//! - `steady`:   submit 100 000 closures back-to-back, wait for them all
//!               (warm-up effects amortize, so this is the throughput number
//!               most representative of sustained load).
//!
//! Each closure does a single `fetch_add` on a shared counter so the
//! per-submit dispatch path dominates the measurement.

#[cfg(not(windows))]
fn main() {
    eprintln!("threading_bench is Windows-only; run it on a Windows host.");
}

#[cfg(windows)]
fn main() {
    bench::run();
}

#[cfg(windows)]
mod bench {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::{Duration, Instant};

    pub fn run() {
        println!(
            "{:<10} {:<10} {:>14} {:>14} {:>14}",
            "workload", "submitter", "elapsed", "per-submit", "submits/s"
        );
        println!("{}", "-".repeat(64));

        // (1) Single submit + wait, averaged over many trials.
        bench_single(
            "single",
            1_000,
            |f| windows_threading::submit(f),
            "win32-pool",
        );
        bench_single(
            "single",
            1_000,
            |f| {
                std::thread::spawn(f);
            },
            "std::spawn",
        );

        // (2) Burst.
        bench_burst(
            "burst",
            10_000,
            |f| windows_threading::submit(f),
            "win32-pool",
        );
        bench_burst(
            "burst",
            10_000,
            |f| {
                std::thread::spawn(f);
            },
            "std::spawn",
        );

        // (3) Steady-state.
        bench_burst(
            "steady",
            100_000,
            |f| windows_threading::submit(f),
            "win32-pool",
        );
        bench_burst(
            "steady",
            100_000,
            |f| {
                std::thread::spawn(f);
            },
            "std::spawn",
        );
    }

    /// Submit one closure, wait for it to finish, repeat `trials` times.
    fn bench_single<S>(name: &str, trials: usize, submit: S, label: &str)
    where
        S: Fn(Box<dyn FnOnce() + Send + 'static>) + Sync,
    {
        // Warm up so first-time pool/thread-creation costs don't pollute the average.
        for _ in 0..16 {
            let done = Arc::new(AtomicUsize::new(0));
            let d = done.clone();
            submit(Box::new(move || {
                d.fetch_add(1, Ordering::Release);
            }));
            spin_until(&done, 1);
        }

        let start = Instant::now();
        for _ in 0..trials {
            let done = Arc::new(AtomicUsize::new(0));
            let d = done.clone();
            submit(Box::new(move || {
                d.fetch_add(1, Ordering::Release);
            }));
            spin_until(&done, 1);
        }
        report(name, label, start.elapsed(), trials);
    }

    /// Submit `n` closures back-to-back, then wait for the last one.
    fn bench_burst<S>(name: &str, n: usize, submit: S, label: &str)
    where
        S: Fn(Box<dyn FnOnce() + Send + 'static>) + Sync,
    {
        // One warm-up pass at a smaller size so the very first submit isn't on the clock.
        {
            let done = Arc::new(AtomicUsize::new(0));
            for _ in 0..256 {
                let d = done.clone();
                submit(Box::new(move || {
                    d.fetch_add(1, Ordering::Release);
                }));
            }
            spin_until(&done, 256);
        }

        let done = Arc::new(AtomicUsize::new(0));
        let start = Instant::now();
        for _ in 0..n {
            let d = done.clone();
            submit(Box::new(move || {
                d.fetch_add(1, Ordering::Release);
            }));
        }
        spin_until(&done, n);
        report(name, label, start.elapsed(), n);
    }

    fn spin_until(counter: &AtomicUsize, target: usize) {
        // A short yield-loop; we don't want to add a sync primitive whose own
        // wake/sleep cost would be charged against the submit path.
        while counter.load(Ordering::Acquire) < target {
            std::thread::yield_now();
        }
    }

    fn report(workload: &str, label: &str, elapsed: Duration, n: usize) {
        let per = elapsed / n as u32;
        let per_us = per.as_secs_f64() * 1_000_000.0;
        let rate = n as f64 / elapsed.as_secs_f64();
        println!(
            "{:<10} {:<10} {:>11.3?}  {:>11.2}us  {:>12.0}/s",
            workload, label, elapsed, per_us, rate
        );
    }
}
