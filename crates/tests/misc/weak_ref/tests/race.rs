use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use windows::core::imp::WeakRefCount;

const TARGET: u32 = 100_000;
const CONCURRENCY: usize = 8;
#[expect(clippy::declare_interior_mutable_const)] // don't want to disturb concurrency testing
const ATOMIC_U32_ZERO: AtomicU32 = AtomicU32::new(0);
static PROGRESS: [AtomicU32; CONCURRENCY] = [ATOMIC_U32_ZERO; CONCURRENCY];

// This function runs in a separate thread. It increases the ref count, and advances the progress.
fn run_increment(ref_count: Arc<WeakRefCount>, progress: &AtomicU32) {
    for _ in 0..TARGET {
        ref_count.add_ref();
        progress.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
#[expect(clippy::needless_range_loop)] // don't want to disturb concurrency testing
fn test_race() {
    let ref_count = Arc::new(WeakRefCount::new());
    let mut threads = Vec::with_capacity(CONCURRENCY);
    for i in 0..CONCURRENCY {
        let ref_count = ref_count.clone();
        threads.push(std::thread::spawn(move || {
            run_increment(ref_count, &PROGRESS[i])
        }));
    }

    let mut last_progress = [0; CONCURRENCY];
    loop {
        std::thread::sleep(core::time::Duration::from_millis(500));
        let mut new_progress = [0; CONCURRENCY];
        for i in 0..CONCURRENCY {
            new_progress[i] = PROGRESS[i].load(Ordering::Relaxed);
            eprint!("Progress {i} = {}, ", new_progress[i]);
        }
        eprintln!();

        if new_progress.iter().all(|&p| p == TARGET) {
            break;
        }

        // Normally, the progress for each thread should advance within a long time period, say 1 sec here.
        // Otherwise, it indicates that there is an infinite loop.
        assert!(
            !new_progress
                .iter()
                .zip(last_progress.iter())
                .any(|(&new, &old)| new == old && new != TARGET),
            "Progress did not increase during the last second"
        );
        last_progress.copy_from_slice(&new_progress[..]);
    }

    threads.into_iter().for_each(|t| t.join().unwrap());
}
