fn main() -> windows::core::Result<()> {
    use windows::Win32::{windef::*, winuser::*};
    use windows_window::{Window, run};

    // `on_message` exposes every message with the raw `wparam`/`lparam` for apps
    // that need to handle messages the crate doesn't model. Return `Some(result)`
    // to handle a message, or `None` to fall through to default processing.
    let _window = Window::new("Window Messages")
        .on_message(|hwnd, message, wparam, lparam| match message {
            WM_PAINT => {
                println!("WM_PAINT");
                unsafe { _ = ValidateRect(Some(HWND(hwnd)), None) };
                Some(0)
            }
            WM_LBUTTONDOWN => {
                let x = (lparam & 0xffff) as i16;
                let y = ((lparam >> 16) & 0xffff) as i16;
                println!("WM_LBUTTONDOWN at ({x}, {y})");
                Some(0)
            }
            WM_KEYDOWN => {
                println!("WM_KEYDOWN, virtual key {}", wparam as u32);
                Some(0)
            }
            _ => None,
        })
        .create()?;

    run();
    Ok(())
}
