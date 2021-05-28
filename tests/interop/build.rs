fn main() {
    windows::build! {
        Windows::Win32::System::WinRT::RoActivateInstance,
        Windows::Foundation::Collections::StringMap,
        Windows::Win32::UI::RadialInput::IRadialControllerInterop,
    };
}
