#![cfg(windows)]
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
            });
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
            });
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

#[test]
fn priority_order() {
    use windows_threading::Priority;
    let (gate_send, gate_recv) = std::sync::mpsc::channel::<()>();
    let order = std::sync::Mutex::new(Vec::new());

    let pool = windows_threading::Pool::new();
    pool.set_thread_limits(1, 1);
    pool.scope(|pool| {
        // Occupy the only thread so the other callbacks queue up behind it.
        pool.submit(move || gate_recv.recv().unwrap());
        pool.submit_with_priority(Priority::Low, || {
            order.lock().unwrap().push(Priority::Low);
        });
        pool.submit_with_priority(Priority::Normal, || {
            order.lock().unwrap().push(Priority::Normal);
        });
        pool.submit_with_priority(Priority::High, || {
            order.lock().unwrap().push(Priority::High);
        });
        gate_send.send(()).unwrap();
    });

    assert_eq!(
        *order.lock().unwrap(),
        [Priority::High, Priority::Normal, Priority::Low]
    );
}

#[test]
fn submit_with_priority() {
    let (send, recv) = std::sync::mpsc::channel();

    for priority in [
        windows_threading::Priority::High,
        windows_threading::Priority::Normal,
        windows_threading::Priority::Low,
    ] {
        let send = send.clone();
        windows_threading::submit_with_priority(priority, move || send.send(priority).unwrap());
    }

    let mut received: Vec<_> = (0..3).map(|_| recv.recv().unwrap()).collect();
    received.sort_by_key(|priority| *priority as u8);
    assert_eq!(
        received,
        [
            windows_threading::Priority::High,
            windows_threading::Priority::Normal,
            windows_threading::Priority::Low
        ]
    );
}

#[test]
fn submit_after() {
    use std::time::{Duration, Instant};
    let (send, recv) = std::sync::mpsc::channel();
    let start = Instant::now();

    for millis in [100, 0, 30] {
        let send = send.clone();
        windows_threading::submit_after(Duration::from_millis(millis), move || {
            send.send((millis, start.elapsed())).unwrap();
        });
    }

    let mut received: Vec<_> = (0..3).map(|_| recv.recv().unwrap()).collect();
    received.sort_by_key(|(millis, _)| *millis);
    assert_eq!(
        received
            .iter()
            .map(|(millis, _)| *millis)
            .collect::<Vec<_>>(),
        [0, 30, 100]
    );

    // Each closure was submitted after `start`, so its elapsed time is at least its delay. Allow
    // for the coarse system clock that timers are scheduled against.
    for (millis, elapsed) in received {
        assert!(elapsed + Duration::from_millis(2) >= Duration::from_millis(millis));
    }
}
