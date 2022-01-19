fn main() {
    windows::core::build_legacy! {Windows::Foundation::Collections::StringMap, Windows::Win32::System::Com::CoInitializeEx, Windows::Win32::System::WinRT::RoActivateInstance, Windows::Win32::UI::Input::Radial::IRadialControllerInterop};
}
