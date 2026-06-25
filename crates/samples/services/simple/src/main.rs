#[cfg(not(windows))]
fn main() {}

#[cfg(windows)]
fn main() {
    use std::io::Write;

    // Simple log file can be used to observe the service commands.
    let path = std::env::temp_dir().join("windows-service.txt");
    let mut log = std::fs::File::create(&path).unwrap();

    windows_services::Service::new()
        .can_pause()
        .can_stop()
        .run(move |_service, command| {
            writeln!(log, "Command: {command:?}").unwrap();
        });
}
