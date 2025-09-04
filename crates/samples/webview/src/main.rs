use windows_core::*;
use windows_webview::*;

use windows_sys::{
    Win32::Foundation::*, Win32::Graphics::Gdi::ValidateRect,
    Win32::System::LibraryLoader::GetModuleHandleA, Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {
    unsafe {
        let instance = GetModuleHandleA(std::ptr::null());
        debug_assert!(!instance.is_null());

        let window_class = s!("window");

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(core::ptr::null_mut(), IDC_ARROW),
            hInstance: instance,
            lpszClassName: window_class.0,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: core::ptr::null_mut(),
            hbrBackground: core::ptr::null_mut(),
            lpszMenuName: std::ptr::null(),
        };

        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        let window_handle = CreateWindowExA(
            0,
            window_class.0,
            s!("This is a sample window").0,
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            instance,
            std::ptr::null(),
        );

        let environment = create_environment()?;
        let controller = environment.create_controller(window_handle)?;

        SetWindowLongPtrA(window_handle, GWLP_USERDATA, &controller as *const _ as _);

        let mut rect = Default::default();
        GetClientRect(window_handle, &mut rect);

        controller.set_bounds(Rect {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        })?;

        let view = controller.view()?;
        view.navigate("https://github.com/microsoft/windows-rs")?;


        let mut message = std::mem::zeroed();

        while GetMessageA(&mut message, core::ptr::null_mut(), 0, 0) != 0 {
            DispatchMessageA(&message);
        }

        Ok(())
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
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
            WM_SIZE => {
                let controller = GetWindowLongPtrA(window, GWLP_USERDATA) as *const Controller;

                if !controller.is_null() {
                    let controller  = &*controller;
                    
                    let mut rect = Default::default();
                    GetClientRect(window, &mut rect);

                    controller
                        .set_bounds(Rect {
                            left: rect.left,
                            top: rect.top,
                            right: rect.right,
                            bottom: rect.bottom,
                        })
                        .unwrap();
                }
                0
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
