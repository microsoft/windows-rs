#[cfg(not(windows))]
fn main() {}

#[cfg(windows)]
mod bindings;

#[cfg(windows)]
fn main() -> windows::core::Result<()> {
    use crate::bindings::*;

    use windows::{
        core::*, Win32::Foundation::*, Win32::System::Com::*, Win32::System::LibraryLoader::*,
        Win32::UI::HiDpi::*, Win32::UI::WindowsAndMessaging::*,
    };

    #[implement(ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler)]
    struct EnvironmentHandler(HWND);

    impl ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler_Impl for EnvironmentHandler_Impl {
        fn Invoke(
            &self,
            status: HRESULT,
            environment: Ref<ICoreWebView2Environment>,
        ) -> Result<()> {
            status.ok()?;
            let environment = environment.ok()?;

            let handler: ICoreWebView2CreateCoreWebView2ControllerCompletedHandler =
                ControllerHandler(self.0).into();

            unsafe {
                environment.CreateCoreWebView2Controller(self.0, &handler)?;
            }

            Ok(())
        }
    }

    #[implement(ICoreWebView2CreateCoreWebView2ControllerCompletedHandler)]
    struct ControllerHandler(HWND);

    impl ICoreWebView2CreateCoreWebView2ControllerCompletedHandler_Impl for ControllerHandler_Impl {
        fn Invoke(&self, status: HRESULT, controller: Ref<ICoreWebView2Controller>) -> Result<()> {
            status.ok()?;
            let controller = controller.ok()?;

            unsafe {
                let mut rect = RECT::default();
                GetClientRect(self.0, &mut rect)?;
                controller.SetBounds(rect)?;
                let webview = controller.CoreWebView2()?;

                let uri = w!("https://github.com/microsoft/windows-rs");
                webview.Navigate(*uri)?;
            }

            // TODO: need to keep the controller alive.
            std::mem::forget(controller.clone());
            Ok(())
        }
    }

    extern "system" fn wndproc(
        window: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            match message {
                WM_DESTROY => {
                    PostQuitMessage(0);
                    LRESULT(0)
                }
                _ => DefWindowProcA(window, message, wparam, lparam),
            }
        }
    }

    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;
        SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2)?;
        let instance = GetModuleHandleA(None)?;

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: instance.into(),
            lpszClassName: s!("webview"),
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        RegisterClassA(&wc);

        let hwnd = CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            s!("webview"),
            s!("hello world"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            None,
            None,
        )?;

        let handler: ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler =
            EnvironmentHandler(hwnd).into();

        CreateCoreWebView2Environment(&handler)?;
        let mut message = MSG::default();

        while GetMessageA(&mut message, None, 0, 0).into() {
            DispatchMessageA(&message);
        }

        Ok(())
    }
}
