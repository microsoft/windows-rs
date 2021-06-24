fn main() {
    windows::build! {
        Windows::Win32::{
            Foundation::HWND, System::WinRT::IDesktopWindowXamlSourceNative2,
            UI::WindowsAndMessaging::SetWindowPos,
        },
        Windows::UI::Xaml::{Controls::TextBox, Hosting::DesktopWindowXamlSource},
    };
}
