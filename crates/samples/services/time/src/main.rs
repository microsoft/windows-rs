#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() {
    use crate::bindings::*;

    use std::io::Write;
    use windows_services::*;

    fn convert(time: i64) -> SYSTEMTIME {
        unsafe {
            let mut local = FILETIME::default();
            FileTimeToLocalFileTime(&time as *const i64 as _, &mut local);

            let mut time = SYSTEMTIME::default();
            FileTimeToSystemTime(&local, &mut time);
            time
        }
    }

    let log_path = std::env::temp_dir().join("windows-rs-services-time.log");
    let mut log = std::fs::File::create(&log_path).unwrap();

    let result = Service::new()
        .can_stop()
        .can_accept(SERVICE_ACCEPT_TIMECHANGE as u32)
        .run(|_service, command| {
            writeln!(log, "Command: {command:?}").unwrap();

            if let Command::Extended(command) = command
                && command.control == SERVICE_CONTROL_TIMECHANGE as u32
            {
                unsafe {
                    let data = &*(command.data as *const SERVICE_TIMECHANGE_INFO);

                    writeln!(log, "{data:#?}").unwrap();

                    let old = convert(data.liOldTime);
                    let new = convert(data.liNewTime);

                    writeln!(log, "{old:#?}\n{new:#?}").unwrap();
                }
            }
        });

    if let Err(error) = result {
        println!("{error}");
        println!("Log: {}", log_path.display());
    }
}
