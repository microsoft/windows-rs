use std::sync::{Arc, Mutex};

use windows_services::{Command, ServiceBuilder};

#[test]
fn start_stop() {
    let log = Arc::new(Mutex::new(vec![]));
    let log_c = log.clone();

    ServiceBuilder::new()
        .can_fallback(|_| {})
        .run(move |service, command| {
            assert!(service.handle().is_null());
            log_c.lock().unwrap().push(command);
        })
        .unwrap();

    assert_eq!(*log.lock().unwrap(), [Command::Start, Command::Stop,]);
}
