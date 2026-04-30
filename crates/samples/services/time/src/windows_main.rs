mod bindings;
use bindings::*;

use std::io::Write;
use windows_services::*;

fn main() {
    let mut log = std::fs::File::create("D:\\service.txt").unwrap();

    Service::new()
        .can_stop()
        .can_accept(SERVICE_ACCEPT_TIMECHANGE)
        .run(|_service, command| {
            writeln!(log, "Command: {command:?}").unwrap();

            if let Command::Extended(command) = command {
                if command.control == SERVICE_CONTROL_TIMECHANGE {
                    unsafe {
                        let data = &*(command.data as *const SERVICE_TIMECHANGE_INFO);

                        writeln!(log, "{data:#?}").unwrap();

                        let old = convert(data.liOldTime);
                        let new = convert(data.liNewTime);

                        writeln!(log, "{old:#?}\n{new:#?}").unwrap();
                    }
                }
            }
        })
        .unwrap();
}

fn convert(time: i64) -> SYSTEMTIME {
    unsafe {
        let mut local = FILETIME::default();
        FileTimeToLocalFileTime(&time as *const i64 as _, &mut local);

        let mut time = SYSTEMTIME::default();
        FileTimeToSystemTime(&local, &mut time);
        time
    }
}
