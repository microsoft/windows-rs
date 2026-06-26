//! Serving a folder on disk through `set_virtual_host_name_to_folder_mapping`.

use webview_minimal::*;

fn main() -> Result<()> {
    run(
        "WebView2 local files - windows-rs",
        |_controller, webview| {
            let folder = concat!(env!("CARGO_MANIFEST_DIR"), "\\web");
            webview.set_virtual_host_name_to_folder_mapping(
                "app.example",
                folder,
                HostResourceAccessKind::Deny,
            )?;

            webview.navigate("https://app.example/index.html")?;
            Ok(Vec::new())
        },
    )
}
