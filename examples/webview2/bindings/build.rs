fn main() {
    windows::build!(
        Microsoft::Web::WebView2::Core::*,
        Windows::Foundation::*,
        Windows::Win32::Com::{CoInitialize, CoUninitialize},
        Windows::Win32::Debug::GetLastError,
        Windows::Win32::DisplayDevices::{POINT, RECT, SIZE},
        Windows::Win32::Gdi::UpdateWindow,
        Windows::Win32::HiDpi::{PROCESS_DPI_AWARENESS, SetProcessDpiAwareness},
        Windows::Win32::KeyboardAndMouseInput::SetFocus,
        Windows::Win32::MenusAndResources::HMENU,
        Windows::Win32::SystemServices::{GetCurrentThreadId, GetModuleHandleA, HINSTANCE, LRESULT, PWSTR},
        Windows::Win32::WindowsAndMessaging::*
    )
}
