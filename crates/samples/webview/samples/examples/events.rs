//! The navigation lifecycle plus window, new-window, permission, and
//! process-failed events.

use webview_samples::*;

fn main() -> Result<()> {
    run("WebView2 events - windows-rs", |_controller, webview| {
        let completed = webview.clone();

        let registrations = vec![
            webview.on_navigation_starting(|args| {
                println!(
                    "navigation {} starting: {} (user initiated = {})",
                    args.navigation_id(),
                    args.uri(),
                    args.is_user_initiated()
                );
            })?,
            webview.on_content_loading(|args| {
                println!(
                    "content loading: navigation {} (error page = {})",
                    args.navigation_id(),
                    args.is_error_page()
                );
            })?,
            webview.on_navigation_completed(move |args| {
                println!(
                    "navigation {} completed: success = {}, title = {:?}",
                    args.navigation_id(),
                    args.is_success(),
                    completed.document_title()
                );
            })?,
            webview.on_document_title_changed(|title| {
                println!("document title changed: {title}");
            })?,
            webview.on_new_window_requested(|args| {
                println!("blocking new window for: {}", args.uri());
                args.set_handled(true).unwrap();
            })?,
            webview.on_permission_requested(|args| {
                println!("denying {:?} permission for {}", args.kind(), args.uri());
                args.set_state(PermissionState::Deny).unwrap();
            })?,
            webview.on_window_close_requested(|| {
                println!("page requested window close");
            })?,
            webview.on_process_failed(|args| {
                println!("process failed: {:?}", args.kind());
            })?,
        ];

        webview.navigate("https://learn.microsoft.com/windows/dev-environment/")?;
        Ok(registrations)
    })
}
