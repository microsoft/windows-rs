fn main() {
    use std::io::Write;

    let log_path = std::env::temp_dir().join("windows-rs-services-simple.log");
    let mut log = std::fs::File::create(&log_path).unwrap();

    let result =
        windows_services::Service::new()
            .can_pause()
            .can_stop()
            .run(|_service, command| {
                writeln!(log, "Command: {command:?}").unwrap();
            });

    if result.is_err() {
        println!(
            r#"Use service control manager to start service.

Install:
    > sc create ServiceName binPath= "{}"

Start:
    > sc start ServiceName

Query status:
    > sc query ServiceName

Stop:
    > sc stop ServiceName

Delete (uninstall):
    > sc delete ServiceName

Log:
    {}
"#,
            std::env::current_exe().unwrap().display(),
            log_path.display()
        );
    }
}
