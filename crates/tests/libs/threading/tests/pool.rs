#[test]
fn join() {
    let pool = windows_threading::Pool::new();
    let counter = std::sync::RwLock::<usize>::new(0);

    pool.scope(|pool| {
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
    });
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

#[test]
fn multi() {
    let set = std::sync::RwLock::<std::collections::HashSet<u32>>::default();

    let pool = windows_threading::Pool::new();
    pool.set_thread_limits(2, 10);
    pool.scope(|pool| {
        for _ in 0..10 {
            pool.submit(|| {
                windows_threading::sleep(10);
                let mut writer = set.write().unwrap();
                writer.insert(windows_threading::thread_id());
            })
        }
    });
    assert!(set.read().unwrap().len() > 1);
}

#[test]
fn single() {
    let set = std::sync::RwLock::<std::collections::HashSet<u32>>::default();

    let pool = windows_threading::Pool::new();
    pool.set_thread_limits(1, 1);
    pool.scope(|pool| {
        for _ in 0..10 {
            pool.submit(|| {
                let mut writer = set.write().unwrap();
                writer.insert(windows_threading::thread_id());
            })
        }
    });
    assert_eq!(set.read().unwrap().len(), 1);
}

#[test]
fn send_sync() {
    fn send<T: Send>(_: T) {}
    fn sync<T: Sync>(_: T) {}

    send(windows_threading::Pool::new());
    sync(windows_threading::Pool::new());
}
