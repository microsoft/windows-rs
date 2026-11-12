use crate::bindings::*;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread::{self, JoinHandle};

pub const BLOCK_MB: usize = 256;
pub const MAX_BLOCKS: usize = 64;

struct Block(*mut core::ffi::c_void);

impl Block {
    fn new(bytes: usize) -> windows_core::Result<Self> {
        let pointer = unsafe {
            VirtualAlloc(
                std::ptr::null_mut(),
                bytes,
                (MEM_RESERVE | MEM_COMMIT) as u32,
                PAGE_READWRITE as u32,
            )
        };
        if pointer.is_null() {
            return Err(windows_core::Error::from_thread());
        }
        // Write every page so the allocation contributes to the working set, not just commit.
        unsafe { pointer.cast::<u8>().write_bytes(0xa5, bytes) };
        Ok(Self(pointer))
    }
}

impl Drop for Block {
    fn drop(&mut self) {
        unsafe { VirtualFree(self.0, 0, MEM_RELEASE as u32) }
            .ok()
            .unwrap();
    }
}

pub struct Memory {
    requests: Option<Sender<usize>>,
    worker: Option<JoinHandle<()>>,
}

impl Memory {
    pub fn new(report: impl FnMut(usize, Option<String>) + Send + 'static) -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            requests: Some(sender),
            worker: Some(
                thread::Builder::new()
                    .name("Max CPU memory".into())
                    .spawn(move || run(receiver, BLOCK_MB * 1024 * 1024, report))
                    .unwrap(),
            ),
        }
    }

    pub fn set(&self, blocks: usize) {
        assert!(blocks <= MAX_BLOCKS);
        self.requests.as_ref().unwrap().send(blocks).unwrap();
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        drop(self.requests.take());
        self.worker.take().unwrap().join().unwrap();
    }
}

fn run(
    requests: Receiver<usize>,
    block_bytes: usize,
    mut report: impl FnMut(usize, Option<String>),
) {
    let mut blocks = Vec::new();
    let mut target = 0;
    loop {
        if blocks.len() == target {
            let Ok(request) = requests.recv() else {
                break;
            };
            target = request;
        }
        // Coalesce slider changes and check for shutdown between allocations.
        loop {
            match requests.try_recv() {
                Ok(request) => target = request,
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return,
            }
        }
        if blocks.len() > target {
            blocks.truncate(target);
        } else if blocks.len() < target {
            match Block::new(block_bytes) {
                Ok(block) => blocks.push(block),
                Err(error) => {
                    target = blocks.len();
                    report(target, Some(format!("Memory allocation failed: {error}")));
                    continue;
                }
            }
        }
        report(blocks.len(), None);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn allocation_touches_every_byte() {
        let bytes = 64 * 1024;
        let block = Block::new(bytes).unwrap();
        let data = unsafe { std::slice::from_raw_parts(block.0.cast::<u8>(), bytes) };
        assert!(data.iter().all(|byte| *byte == 0xa5));
    }

    #[test]
    fn worker_grows_shrinks_and_stops_with_memory_allocated() {
        let (requests, receiver) = mpsc::channel();
        let (reports, events) = mpsc::channel();
        let worker = thread::spawn(move || {
            run(receiver, 64 * 1024, |blocks, error| {
                reports.send((blocks, error)).unwrap();
            });
        });
        let receive = || {
            let (blocks, error) = events.recv_timeout(Duration::from_secs(5)).unwrap();
            assert!(error.is_none(), "{error:?}");
            blocks
        };

        requests.send(2).unwrap();
        assert_eq!(receive(), 1);
        assert_eq!(receive(), 2);
        requests.send(1).unwrap();
        assert_eq!(receive(), 1);
        requests.send(0).unwrap();
        assert_eq!(receive(), 0);
        requests.send(1).unwrap();
        assert_eq!(receive(), 1);
        drop(requests);
        worker.join().unwrap();
    }

    #[test]
    fn worker_uses_latest_target_between_allocations() {
        let (requests, receiver) = mpsc::channel();
        let (reports, events) = mpsc::channel();
        let mut updates = Some(requests.clone());
        let worker = thread::spawn(move || {
            run(receiver, 64 * 1024, |blocks, error| {
                if blocks == 1 {
                    let updates = updates.take().unwrap();
                    updates.send(4).unwrap();
                    updates.send(0).unwrap();
                }
                reports.send((blocks, error)).unwrap();
            });
        });
        requests.send(3).unwrap();
        assert_eq!(
            events.recv_timeout(Duration::from_secs(5)).unwrap(),
            (1, None),
        );
        assert_eq!(
            events.recv_timeout(Duration::from_secs(5)).unwrap(),
            (0, None),
        );
        drop(requests);
        worker.join().unwrap();
    }

    #[test]
    fn allocation_failure_is_reported_and_worker_can_stop() {
        let (requests, receiver) = mpsc::channel();
        let (reports, events) = mpsc::channel();
        let worker = thread::spawn(move || {
            run(receiver, usize::MAX, |blocks, error| {
                reports.send((blocks, error)).unwrap();
            });
        });
        requests.send(1).unwrap();
        let (blocks, error) = events.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(blocks, 0);
        assert!(error.unwrap().starts_with("Memory allocation failed:"));
        requests.send(0).unwrap();
        assert_eq!(
            events.recv_timeout(Duration::from_secs(5)).unwrap(),
            (0, None),
        );
        drop(requests);
        worker.join().unwrap();
    }
}
