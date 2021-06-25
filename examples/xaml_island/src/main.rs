#![windows_subsystem = "windows"]

use std::mem::MaybeUninit;

use bindings::Windows::{
    Win32::Foundation::HWND,
    Win32::Graphics::Gdi::{BeginPaint, EndPaint, FillRect, HBRUSH, HDC, PAINTSTRUCT},
    Win32::System::WinRT::IDesktopWindowXamlSourceNative2,
    Win32::UI::WindowsAndMessaging::{SetWindowPos, COLOR_WINDOW, SWP_SHOWWINDOW},
    UI::Xaml::{Controls::TextBox, Hosting::DesktopWindowXamlSource},
};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use windows::*;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn redraw<T>(hwnd: HWND, f: impl FnOnce(HDC, &mut PAINTSTRUCT) -> T) -> T {
    let (hdc, mut paint_struct) = unsafe {
        let mut paint_struct = MaybeUninit::uninit();
        let hdc = BeginPaint(hwnd, paint_struct.as_mut_ptr());
        assert_ne!(hdc, HDC(0));
        (hdc, paint_struct.assume_init())
    };
    let result = f(hdc, &mut paint_struct);
    unsafe { EndPaint(hwnd, &paint_struct) };
    result
}

fn main() -> Result<()> {
    // winit calls OleInitialize for its drag-n-drop support,
    // so we don't have to call initialize_sta.
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("XAML Island Example")
        .build(&event_loop)
        .unwrap();
    let hwnd = match window.raw_window_handle() {
        RawWindowHandle::Windows(h) => HWND(h.hwnd as _),
        _ => panic!("window created is not Win32"),
    };

    let dwxs = DesktopWindowXamlSource::new()?;
    let dwxs_native: IDesktopWindowXamlSourceNative2 = dwxs.cast()?;
    unsafe { dwxs_native.AttachToWindow(hwnd) }?;
    dwxs.SetContent(TextBox::new()?)?;

    let xaml_island_hwnd = unsafe { dwxs_native.get_WindowHandle() }?;
    let on_resize = move |host_inner_size: PhysicalSize<u32>| {
        let PhysicalSize { width, height } = host_inner_size;
        unsafe {
            SetWindowPos(
                xaml_island_hwnd,
                HWND(0),
                (width / 4u32) as i32,
                (height / 4u32) as i32,
                (width / 2u32) as i32,
                (height / 2u32) as i32,
                SWP_SHOWWINDOW,
            )
        }
        .ok()
    };
    on_resize(window.inner_size())?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::Resized(host_inner_size),
                ..
            } => {
                on_resize(host_inner_size).unwrap();
                ControlFlow::Wait
            }
            Event::RedrawRequested(_) => {
                assert_ne!(
                    0,
                    redraw(hwnd, |hdc, paint_struct| unsafe {
                        FillRect(
                            hdc,
                            &paint_struct.rcPaint,
                            HBRUSH((COLOR_WINDOW.0 + 1u32) as isize),
                        )
                    })
                );
                ControlFlow::Wait
            }
            _ => ControlFlow::Wait,
        };
    })
}
