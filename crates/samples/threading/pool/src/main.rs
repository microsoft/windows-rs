use std::collections::HashMap;
use std::sync::RwLock;

fn main() {
    let counts = RwLock::<HashMap<u32, usize>>::default();

    let pool = windows_threading::Pool::new();
    pool.set_thread_limits(2, 4);
    pool.scope(|pool| {
        for _ in 0..10 {
            pool.submit(|| {
                windows_threading::sleep(10);
                *counts
                    .write()
                    .unwrap()
                    .entry(windows_threading::thread_id())
                    .or_default() += 1;
            });
        }
    });

    println!("work distribution across pool threads:");
    println!("{:#?}", counts.read().unwrap());
}
