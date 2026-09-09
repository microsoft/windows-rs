use std::path::Path;
use windows_webview::*;

fn main() -> Result<()> {
    WebViewWindow::new("WebView2 downloads - windows-rs")
        .size(1024, 768)
        .run(|host| {
            let webview = host.webview();
            // Keep each download's event registrations alive in the starting handler.
            let mut downloads: Vec<EventRegistration> = Vec::new();

            let download = webview.on_download_starting(move |args| {
                let path = args.result_file_path();
                let cancel = Path::new(&path)
                    .file_name()
                    .is_some_and(|name| name.to_string_lossy().starts_with("windows-rs-cancelled"));

                if cancel {
                    println!("canceling download: {path}");
                    args.set_cancel(true).unwrap();
                    return;
                }

                let operation = args.download_operation().unwrap();
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
                    let state = operation.state();
                    println!("download state: {state:?}");
                    if state == DownloadState::Interrupted {
                        println!("interrupt reason: {:?}", operation.interrupt_reason());
                    }
                });

                downloads.extend(bytes.into_iter().chain(state));
            })?;

            webview.navigate_to_string(
                r#"<!DOCTYPE html>
<html>
<body>
  <button onclick="download('windows-rs-sample.txt')">Download and report progress</button>
  <button onclick="download('windows-rs-cancelled.txt')">Cancel before downloading</button>
  <script>
    function download(name) {
      const bytes = new Uint8Array(8 * 1024 * 1024);
      const blob = new Blob([bytes], { type: 'application/octet-stream' });
      const link = document.createElement('a');
      link.href = URL.createObjectURL(blob);
      link.download = name;
      link.click();
      URL.revokeObjectURL(link.href);
    }
  </script>
</body>
</html>"#,
            )?;
            host.retain(download);
            Ok(())
        })
}
