use std::sync::{Arc, Mutex};

use windows_services::{Command, ExtendedCommand, ServiceBuilder};

#[test]
fn extended() {
    #[repr(C)]
    #[derive(Copy, Clone, PartialEq, Debug)]
    struct Data {
        a: u8,
        b: u8,
        c: u8,
    }
    static DATA: Data = Data { a: 7, b: 8, c: 9 };
    const DATA_PTR: *const std::ffi::c_void = &DATA as *const Data as *const std::ffi::c_void;

    let log = Arc::new(Mutex::new(vec![]));
    let log_c = log.clone();

    ServiceBuilder::new()
        .can_fallback(|service| {
            service.handler(123, 456, DATA_PTR);
        })
        .run(move |_, command| {
            log_c.lock().unwrap().push(command);

            if let Command::Extended(command) = command {
                unsafe {
                    let data = *(command.data as *const Data);
                    assert_eq!(data, Data { a: 7, b: 8, c: 9 });
                }
            }
        })
        .expect("Service should have initialized.");

    assert_eq!(
        *log.lock().unwrap(),
        [
            Command::Start,
            Command::Extended(ExtendedCommand {
                control: 123,
                ty: 456,
                data: DATA_PTR
            }),
            Command::Stop,
        ]
    );
}
