#[expect(nonstandard_style, clippy::upper_case_acronyms)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() {
    use bindings::*;

    extern "system" fn wndproc(
        window: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            match message as i32 {
                WM_PAINT => {
                    println!("WM_PAINT");
                    ValidateRect(window, std::ptr::null());
                    0
                }
                WM_DESTROY => {
                    println!("WM_DESTROY");
                    PostQuitMessage(0);
                    0
                }
                _ => DefWindowProcA(window, message, wparam, lparam),
            }
        }
    }

    unsafe {
        let instance = GetModuleHandleA(std::ptr::null());
        debug_assert!(!instance.is_null());

        let window_class = c"window".as_ptr().cast();

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(core::ptr::null_mut(), IDC_ARROW.cast()),
            hInstance: instance,
            lpszClassName: window_class,
            style: (CS_HREDRAW | CS_VREDRAW) as u32,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: core::ptr::null_mut(),
            hbrBackground: core::ptr::null_mut(),
            lpszMenuName: std::ptr::null(),
        };

        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        CreateWindowExA(
            0,
            window_class,
            c"This is a sample window".as_ptr().cast(),
            (WS_OVERLAPPEDWINDOW | WS_VISIBLE) as u32,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            instance,
            std::ptr::null_mut(),
        );

        let mut message = std::mem::zeroed();

        while GetMessageA(&mut message, core::ptr::null_mut(), 0, 0) != 0 {
            DispatchMessageA(&message);
        }
    }
}
