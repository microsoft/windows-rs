use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use windows_webview::*;
use windows_window::*;

fn main() -> Result<()> {
    run("WebView2 downloads - windows-rs", |_controller, webview| {
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
        Ok(vec![download])
    })
}

fn run<F>(title: &str, setup: F) -> Result<()>
where
    F: FnOnce(&Controller, &WebView) -> Result<Vec<EventRegistration>>,
{
    let controller: Rc<RefCell<Option<Controller>>> = Rc::new(RefCell::new(None));

    let resize = controller.clone();
    let window = Window::new(title)
        .size(1024, 768)
        .on_resize(move |width, height| {
            if let Some(controller) = resize.borrow().as_ref() {
                controller.set_bounds(0, 0, width, height).unwrap();
            }
        })
        .create()?;

    // Host destruction reports E_ABORT or the message loop's success-coded empty error.
    let environment = match Environment::new() {
        Ok(environment) => environment,
        Err(error) if error.code().0 == E_ABORT || error.code().is_ok() => return Ok(()),
        Err(error) => return Err(error),
    };
    let handle = match environment.create_controller(&window) {
        Ok(handle) => handle,
        Err(error) if error.code().0 == E_ABORT || error.code().is_ok() => return Ok(()),
        Err(error) => return Err(error),
    };
    let (width, height) = window.client_size();
    handle.set_bounds(0, 0, width, height)?;

    let webview = handle.webview()?;
    let registrations = setup(&handle, &webview)?;

    *controller.borrow_mut() = Some(handle);
    windows_window::run();
    drop(registrations);
    Ok(())
}

const E_ABORT: i32 = 0x8000_4004u32 as i32;
