fn main() {
    windows::build! {
        Microsoft::Web::WebView2::Win32::CompareBrowserVersions,
        Windows::Win32::Storage::FileSystem::{
            GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW,
        },
        Windows::Win32::UI::Shell::GetCurrentProcessExplicitAppUserModelID,
    };
}
