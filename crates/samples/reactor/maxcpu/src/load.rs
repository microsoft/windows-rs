use crate::bindings::*;
use std::os::windows::io::AsRawHandle;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread::{self, JoinHandle};

#[derive(Default)]
struct State {
    active: AtomicUsize,
    stopping: AtomicBool,
}

impl State {
    fn run(&self, index: usize, mut work: impl FnMut()) {
        while !self.stopping.load(Ordering::Relaxed) {
            if index < self.active.load(Ordering::Relaxed) {
                work();
            } else {
                thread::park();
            }
        }
    }
}

pub struct Load {
    state: Arc<State>,
    workers: Vec<JoinHandle<()>>,
}

impl Load {
    pub fn new(processors: usize) -> Self {
        let mut load = Self {
            state: Arc::default(),
            workers: Vec::with_capacity(processors),
        };
        for index in 0..processors {
            let state = Arc::clone(&load.state);
            load.workers.push(
                thread::Builder::new()
                    .name(format!("Max CPU {index}"))
                    .spawn(move || state.run(index, std::hint::spin_loop))
                    .unwrap(),
            );
            let handle = load.workers.last().unwrap().as_raw_handle();
            unsafe { SetThreadPriority(handle, THREAD_PRIORITY_LOWEST) }
                .ok()
                .unwrap();
        }
        load
    }

    pub fn active(&self) -> usize {
        self.state.active.load(Ordering::Relaxed)
    }

    pub fn set(&self, count: usize) {
        assert!(count <= self.workers.len());
        self.state.active.store(count, Ordering::Relaxed);
        for worker in &self.workers {
            worker.thread().unpark();
        }
    }
}

impl Drop for Load {
    fn drop(&mut self) {
        self.state.stopping.store(true, Ordering::Relaxed);
        for worker in &self.workers {
            worker.thread().unpark();
        }
        for worker in self.workers.drain(..) {
            worker.join().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::{self, RecvTimeoutError};
    use std::time::Duration;

    #[test]
    fn workers_wake_park_and_join_on_shutdown() {
        let state = Arc::default();
        let (sender, events) = mpsc::channel();
        let workers = (0..2)
            .map(|index| {
                let state: Arc<State> = Arc::clone(&state);
                let sender = sender.clone();
                thread::spawn(move || {
                    let mut previous = 0;
                    state.run(index, || {
                        let active = state.active.load(Ordering::Relaxed);
                        if index < active && active != previous {
                            previous = active;
                            sender.send((index, false)).unwrap();
                        }
                        // Stand in for CPU work without consuming a processor during tests.
                        thread::park();
                    });
                    sender.send((index, true)).unwrap();
                })
            })
            .collect();
        let load = Load { state, workers };
        let idle = Duration::from_millis(50);
        let timeout = Duration::from_secs(5);

        assert_eq!(events.recv_timeout(idle), Err(RecvTimeoutError::Timeout));
        load.set(1);
        assert_eq!(events.recv_timeout(timeout).unwrap(), (0, false));
        load.set(0);
        assert_eq!(events.recv_timeout(idle), Err(RecvTimeoutError::Timeout));
        load.set(2);
        let mut running = [
            events.recv_timeout(timeout).unwrap(),
            events.recv_timeout(timeout).unwrap(),
        ];
        running.sort_unstable();
        assert_eq!(running, [(0, false), (1, false)]);

        drop(load);
        let mut stopped = [
            events.recv_timeout(timeout).unwrap(),
            events.recv_timeout(timeout).unwrap(),
        ];
        stopped.sort_unstable();
        assert_eq!(stopped, [(0, true), (1, true)]);
    }
}
