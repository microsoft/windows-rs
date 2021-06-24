fn main() {
    windows::build! {
        Windows::UI::Xaml::{
            Controls::TextBox,
            Hosting::DesktopWindowXamlSource,
        },
        Windows::Win32::{
            Foundation::HWND,
            System::WinRT::IDesktopWindowXamlSourceNative2,
            UI::WindowsAndMessaging::SetWindowPos,
        },
    };
}
