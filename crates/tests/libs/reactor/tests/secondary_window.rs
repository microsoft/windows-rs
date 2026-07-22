//! Headless coverage for the public secondary-window API (`ReactorWindow`).
//!
//! Opening a live window needs a running WinUI application, so those paths are
//! exercised by the `test_reactor_selftest` harness. Here we cover the parts
//! that fail before any WinUI object is created.

use windows_reactor::ReactorWindow;

#[test]
fn missing_icon_returns_error_without_running_the_factory() {
    // The icon file is validated up front, before the root factory runs or any
    // WinUI window is created, so a bad path is a cheap, side-effect-free error.
    let result = ReactorWindow::new()
        .title("should not open")
        .icon("windows-reactor-4703-this-icon-does-not-exist.ico")
        .render(|_cx| unreachable!("root factory must not run when icon validation fails"));

    let err = result.expect_err("a missing icon file must produce an error");
    assert!(
        err.message().contains("icon file not found"),
        "unexpected error: {err:?}"
    );
}
