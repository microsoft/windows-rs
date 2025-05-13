use std::io::Write;

fn main() {
    // Simple log file can be used to observe the service commands.
    let mut log = std::fs::File::create("D:\\service.txt").unwrap();

    windows_services::Service::new()
        .can_pause()
        .can_stop()
        .run(|command| {
            writeln!(log, "Command: {command:?}").unwrap();
        })
}
