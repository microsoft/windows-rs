use bindings::{
    windows::win32::gdi::ValidateRect,
    windows::win32::menus_and_resources::{LoadCursorA, HMENU},
    windows::win32::system_services::{
        GetModuleHandleA, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, HINSTANCE, IDC_ARROW, LRESULT,
        WM_DESTROY, WM_PAINT, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
    },
    windows::win32::windows_and_messaging::{
        CreateWindowExA, DefWindowProcA, DispatchMessageA, GetMessageA, PostQuitMessage,
        RegisterClassA, HWND, LPARAM, MSG, WNDCLASSA, WPARAM,
    },
    windows::Result,
};

fn main() -> Result<()> {
    unsafe {
        let instance = HINSTANCE(GetModuleHandleA(std::ptr::null()));
        debug_assert!(instance.0 != 0);
        let class_name = b"window\0";
        let window_title = b"Hello world\0";

        let mut wc = WNDCLASSA::default();
        wc.h_cursor = LoadCursorA(HINSTANCE(0), IDC_ARROW as *const i8);
        wc.h_instance = instance;
        wc.lpsz_class_name = class_name.as_ptr() as *mut u8 as *mut i8;
        wc.style = (CS_HREDRAW | CS_VREDRAW) as u32;
        wc.lpfn_wnd_proc = Some(wndproc);
        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        CreateWindowExA(
            0,
            class_name.as_ptr() as *const i8,
            window_title.as_ptr() as *const i8,
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            HWND(0),
            HMENU(0),
            instance,
            std::ptr::null_mut(),
        );

        let mut message = MSG::default();

        while GetMessageA(&mut message, HWND(0), 0, 0).into() {
            DispatchMessageA(&mut message);
        }

        Ok(())
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message as i32 {
            WM_PAINT => {
                println!("WM_PAINT");
                ValidateRect(window, std::ptr::null());
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
