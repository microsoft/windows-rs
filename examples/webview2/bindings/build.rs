fn main() {
    windows::build!(
        Microsoft::Web::WebView2::Core::*,
        Windows::Foundation::*,
        Windows::Win32::Graphics::Gdi::UpdateWindow,
        Windows::Win32::Foundation::{HINSTANCE, LRESULT, POINT, PWSTR, RECT, SIZE},
        Windows::Win32::System::LibraryLoader::GetModuleHandleA,
        Windows::Win32::System::Threading::GetCurrentThreadId,
        Windows::Win32::UI::HiDpi::{
            PROCESS_DPI_AWARENESS, SetProcessDpiAwareness, PROCESS_PER_MONITOR_DPI_AWARE
        },
        Windows::Win32::UI::KeyboardAndMouseInput::SetFocus,
        Windows::Win32::UI::WindowsAndMessaging::*
    )
}
