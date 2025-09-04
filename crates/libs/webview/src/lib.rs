#![doc = include_str!("../readme.md")]
#![cfg(windows)]
#![allow(non_snake_case, dead_code, non_camel_case_types)]

mod pcwstr;
use pcwstr::*;

mod bindings;
// TODO: need to hide this
use bindings::*;

use std::sync::mpsc::*;
use windows_core::*;
use windows_link::*;

// TODO: maybe keep these manually defined anyway so we can give the following options:
// - delay load
// - process load
// - static library
link!("WebView2Loader.dll" "system" fn CreateCoreWebView2EnvironmentWithOptions(browserExecutableFolder: PCWSTR, userDataFolder: PCWSTR, environmentOptions: Ref<ICoreWebView2EnvironmentOptions>, environmentCreatedHandler: Ref<ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler>) -> HRESULT);
link!("WebView2Loader.dll" "system" fn CreateCoreWebView2Environment(environmentCreatedHandler: Ref<ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler>) -> HRESULT);

unsafe impl Send for HWND {}
unsafe impl Sync for HWND {}

pub fn create_environment_when<F>(completed: F) -> Result<()>
where
    F: FnOnce(Result<&ICoreWebView2Environment>) + Send + 'static,
{
    let completed = core::cell::RefCell::new(Some(completed));

    unsafe {
        CreateCoreWebView2Environment(
            (&ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler::new(
                move |result, environment| {
                    if let Some(completed) = completed.take() {
                        completed(result.and_then(|| environment.ok()));
                    }
                    Ok(())
                },
            ))
                .into(),
        )
        .ok()
    }
}

pub fn create_environment() -> Result<Environment> {
    let (sender, receiver) = channel();

    create_environment_when(move |environment| {
        sender
            .send(
                environment
                    .cloned()
                    .map(|environment| Environment(environment)),
            )
            .unwrap();
    })?;

    receiver.recv().unwrap()
}

pub struct Environment(ICoreWebView2Environment);
unsafe impl Send for Environment {}
unsafe impl Sync for Environment {}

pub struct Controller(ICoreWebView2Controller);
unsafe impl Send for Controller {}
unsafe impl Sync for Controller {}

pub struct View(ICoreWebView2);
unsafe impl Send for View {}
unsafe impl Sync for View {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl From<Rect> for RECT {
    fn from(from: Rect) -> Self {
        Self {
            left: from.left,
            top: from.top,
            right: from.right,
            bottom: from.bottom,
        }
    }
}

impl Environment {
    pub fn create_controller_when<F>(
        &self,
        window_handle: *mut core::ffi::c_void,
        completed: F,
    ) -> Result<()>
    where
        F: FnOnce(Result<&ICoreWebView2Controller>) + Send + 'static,
    {
        let completed = core::cell::RefCell::new(Some(completed));

        unsafe {
            self.0.CreateCoreWebView2Controller(
                HWND(window_handle),
                &ICoreWebView2CreateCoreWebView2ControllerCompletedHandler::new(
                    move |result, controller| {
                        if let Some(completed) = completed.take() {
                            completed(result.and_then(|| controller.ok()));
                        }
                        Ok(())
                    },
                ),
            )
        }
    }

    pub fn create_controller(&self, window_handle: *mut core::ffi::c_void) -> Result<Controller> {
        let (sender, receiver) = channel();

        self.create_controller_when(window_handle, move |controller| {
            sender
                .send(controller.cloned().map(|controller| Controller(controller)))
                .unwrap();
        })?;

        wait_for_result(receiver)
    }
}

impl Controller {
    pub fn set_bounds(&self, rect: Rect) -> Result<()> {
        unsafe { self.0.SetBounds(rect.into()) }
    }

    pub fn view(&self) -> Result<View> {
        unsafe { self.0.CoreWebView2().map(|view| View(view)) }
    }

    pub fn set_visible(&self, visible: bool) -> Result<()> {
        unsafe { self.0.SetIsVisible(visible) }
    }
}

impl View {
    pub fn navigate(&self, uri: &str) -> Result<()> {
        unsafe { self.0.Navigate(pcwstr(uri).as_raw()) }
    }
}

fn wait_for_result<T>(receiver: Receiver<Result<T>>) -> Result<T> {
    const ERROR_CANCELLED: u32 = 1223;
    const E_CANCELLED: HRESULT = HRESULT::from_win32(ERROR_CANCELLED);

    windows_link::link!("user32.dll" "system" fn PeekMessageA(lpmsg : *mut MSG, hwnd :  HWND, wmsgfiltermin : u32, wmsgfiltermax : u32, wremovemsg : u32) -> i32);
    windows_link::link!("user32.dll" "system" fn TranslateMessage(lpmsg : *const MSG) -> i32);
    windows_link::link!("user32.dll" "system" fn DispatchMessageA(lpmsg : *const MSG) -> isize);

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    struct MSG {
        pub hwnd: HWND,
        pub message: u32,
        pub wParam: usize,
        pub lParam: isize,
        pub time: u32,
        pub pt: POINT,
    }

    const PM_REMOVE: u32 = 1u32;
    const WM_QUIT: u32 = 18u32;

    loop {
        if let Ok(result) = receiver.try_recv() {
            return Ok(result?);
        }

        let mut message = MSG::default();

        if 0 != unsafe { PeekMessageA(&mut message, HWND(std::ptr::null_mut()), 0, 0, PM_REMOVE) } {
            unsafe {
                _ = TranslateMessage(&message);
                DispatchMessageA(&message);
            }

            if message.message == WM_QUIT {
                return Err(Error::from_hresult(E_CANCELLED));
            }
        }
    }
}
