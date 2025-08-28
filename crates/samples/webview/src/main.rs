use windows_core::*;
use windows_webview as wv;

use windows_sys::{
    Win32::Foundation::*, Win32::Graphics::Gdi::ValidateRect,
    Win32::System::LibraryLoader::GetModuleHandleA, Win32::UI::WindowsAndMessaging::*,
};

fn main() {
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

        let window_handle = wv::HWND(CreateWindowExA(
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
        ));

        // TODO: these callbacks should be like windows-future's when with a single Result parameter like Result<ICoreWebView2Controller> and the HRESULT being made 
        // available through the Err variant. This should work for all of the XxxxCompletedHandler callbacks and the XxxEventHandler callbacks would have two 
        // parameters as usual.

        println!("CreateCoreWebView2Environment");

        wv::CreateCoreWebView2Environment(core::mem::transmute_copy(&wv::ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler::new(move |result, environment| {
            println!("handler {result} {environment:?}");
            result.unwrap();

            let window_handle = window_handle.clone();

            environment.unwrap().CreateCoreWebView2Controller(window_handle, &wv::ICoreWebView2CreateCoreWebView2ControllerCompletedHandler::new(|result, controller| {
                println!("handler {result} {controller:?}");
                result.unwrap();

                Ok(())
            }) ).unwrap();

            Ok(())
        }) )).unwrap();

        let mut message = std::mem::zeroed();

        while GetMessageA(&mut message, core::ptr::null_mut(), 0, 0) != 0 {
            DispatchMessageA(&message);
        }
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
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
