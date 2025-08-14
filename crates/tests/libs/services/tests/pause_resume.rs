use std::sync::{Arc, Mutex};

use windows_services::{Command, ServiceBuilder};
use windows_sys::Win32::System::Services::*;

#[test]
fn pause_resume() {
    let log = Arc::new(Mutex::new(vec![]));
    let log_c = log.clone();

    ServiceBuilder::new()
        .can_fallback(|service| {
            service.handler(SERVICE_CONTROL_PAUSE, 0, std::ptr::null_mut());
            service.handler(SERVICE_CONTROL_CONTINUE, 0, std::ptr::null_mut());
        })
        .run(move |_, command| {
            log_c.lock().unwrap().push(command);
        })
        .unwrap();

    assert_eq!(
        *log.lock().unwrap(),
        [
            Command::Start,
            Command::Pause,
            Command::Resume,
            Command::Stop,
        ]
    );
}
