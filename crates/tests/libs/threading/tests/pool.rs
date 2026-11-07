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

    // Each closure was submitted after `start`, so its elapsed time is at least its delay.
    for (millis, elapsed) in received {
        assert!(elapsed >= Duration::from_millis(millis));
    }
}

#[test]
fn submit_after_never_early_and_once() {
    use std::time::{Duration, Instant};
    let (send, recv) = std::sync::mpsc::channel();

    // Delays that are not multiples of the system timer resolution are the ones that expire early.
    for millis in 1..=40 {
        let send = send.clone();
        let delay = Duration::from_millis(millis);
        let start = Instant::now();
        windows_threading::submit_after(delay, move || {
            send.send((millis, delay, start.elapsed())).unwrap();
        });
    }
    drop(send);

    let mut received: Vec<_> = recv.iter().collect();
    received.sort_by_key(|(millis, _, _)| *millis);
    assert_eq!(
        received
            .iter()
            .map(|(millis, _, _)| *millis)
            .collect::<Vec<_>>(),
        (1..=40).collect::<Vec<_>>()
    );
    for (millis, delay, elapsed) in received {
        assert!(elapsed >= delay, "{millis} ms ran after {elapsed:?}");
    }
}

#[test]
fn submit_after_drops_closure_once_after_running() {
    use std::sync::mpsc::Sender;
    use std::time::{Duration, Instant};

    struct Guard(Sender<(&'static str, Duration)>, Instant);

    impl Drop for Guard {
        fn drop(&mut self) {
            self.0.send(("dropped", self.1.elapsed())).unwrap();
        }
    }

    let (send, recv) = std::sync::mpsc::channel();
    let delay = Duration::from_millis(30);
    let guard = Guard(send, Instant::now());
    windows_threading::submit_after(delay, move || {
        guard.0.send(("ran", guard.1.elapsed())).unwrap();
    });

    // The sender lives in the guard, so the channel closes once the closure is dropped.
    let received: Vec<_> = recv.iter().collect();
    assert_eq!(
        received.iter().map(|(event, _)| *event).collect::<Vec<_>>(),
        ["ran", "dropped"]
    );
    assert!(received[0].1 >= delay);
}
