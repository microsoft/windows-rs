use std::io::Write;

fn main() {
    // Simple log file can be used to observe the service commands.
    let mut log = std::fs::File::create("D:\\service.txt").unwrap();

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
"#,
            std::env::current_exe().unwrap().display()
        );
    }
}
