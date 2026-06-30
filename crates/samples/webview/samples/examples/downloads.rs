//! Watching downloads and reporting per-operation progress and state.

use webview_samples::*;

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

        // Trigger a download as soon as the page loads so the handler fires
        // without any user interaction. The blob is saved to the default
        // downloads folder.
        webview.navigate_to_string(
            "<!DOCTYPE html><html><body><script>\
             const blob = new Blob(['hello from windows-rs'], { type: 'application/octet-stream' });\
             const link = document.createElement('a');\
             link.href = URL.createObjectURL(blob);\
             link.download = 'windows-rs-sample.txt';\
             document.body.appendChild(link);\
             link.click();\
             </script></body></html>",
        )?;
        Ok(vec![download])
    })
}
