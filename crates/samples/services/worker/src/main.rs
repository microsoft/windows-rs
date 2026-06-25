#[cfg(not(windows))]
fn main() {}

#[cfg(windows)]
fn main() {
    use std::io::Write;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread::JoinHandle;
    use std::time::Duration;
    use windows_services::{Command, Service};

    let running = Arc::new(AtomicBool::new(true));
    let mut worker: Option<JoinHandle<()>> = None;

    Service::new()
        .can_stop()
        .run(move |service, command| match command {
            Command::Start => {
                let running = running.clone();

                // The handle is `Copy` and `'static`, so it can be moved into the worker thread to
                // run the service and observe its state.
                worker = Some(std::thread::spawn(move || {
                    let path = std::env::temp_dir().join("windows-service-worker.txt");
                    let mut log = std::fs::File::create(&path).unwrap();
                    let mut count = 0;

                    while running.load(Ordering::Relaxed) {
                        writeln!(log, "working {count} (state {:?})", service.state()).unwrap();
                        count += 1;
                        std::thread::sleep(Duration::from_secs(1));
                    }

                    writeln!(log, "stopped after {count} iterations").unwrap();
                }));
            }
            Command::Stop => {
                running.store(false, Ordering::Relaxed);

                if let Some(worker) = worker.take() {
                    _ = worker.join();
                }
            }
            _ => {}
        });
}
