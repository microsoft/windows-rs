#[test]
fn join() {
    let pool = windows_threading::Pool::new();
    let counter = std::sync::RwLock::<usize>::new(0);

    for _ in 0..10 {
        pool.submit(|| {
            let mut writer = counter.write().unwrap();
            *writer += 1;
        });
    }

    pool.join();
    assert_eq!(*counter.read().unwrap(), 10);

    for _ in 0..10 {
        pool.submit(|| {
            let mut writer = counter.write().unwrap();
            *writer += 1;
        });
    }

    drop(pool);
    assert_eq!(*counter.read().unwrap(), 20);
}

#[test]
fn for_each() {
    let counter = std::sync::RwLock::<usize>::new(0);

    windows_threading::for_each(0..10, |value| {
        let mut writer = counter.write().unwrap();
        *writer += value;
    });

    assert_eq!(*counter.read().unwrap(), 45);
}
