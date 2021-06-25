fn main() {
    windows::build! {
        Windows::Win32::{
            Foundation::HWND,
            Graphics::Gdi::{BeginPaint, EndPaint, FillRect, HBRUSH, HDC, PAINTSTRUCT},
            System::WinRT::IDesktopWindowXamlSourceNative2,
            UI::WindowsAndMessaging::{SetWindowPos, SET_WINDOW_POS_FLAGS, SYS_COLOR_INDEX},
        },
        Windows::UI::Xaml::{Controls::TextBox, Hosting::DesktopWindowXamlSource},
    };
}
