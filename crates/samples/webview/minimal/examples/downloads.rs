//! Watching downloads and reporting per-operation progress and state.

use webview_minimal::*;

fn main() -> Result<()> {
    run("WebView2 downloads - windows-rs", |_controller, webview| {
        // The download handler owns the per-download registrations so they stay
        // alive for as long as the handler does.
        let mut downloads: Vec<EventRegistration> = Vec::new();

        let download = webview.on_download_starting(move |args| {
            let Ok(operation) = args.download_operation() else {
                return;
            };
            println!(
                "download starting: {} ({} bytes) -> {}",
                operation.uri(),
                operation.total_bytes_to_receive(),
                operation.result_file_path()
            );

            let bytes = operation.on_bytes_received_changed(|operation| {
                println!(
                    "download progress: {} / {} bytes",
                    operation.bytes_received(),
                    operation.total_bytes_to_receive()
                );
            });

            let state = operation.on_state_changed(|operation| {
                println!("download state: {:?}", operation.state());
            });

            downloads.extend(bytes.into_iter().chain(state));
        })?;

        webview.navigate("https://learn.microsoft.com/windows/dev-environment/")?;
        Ok(vec![download])
    })
}
