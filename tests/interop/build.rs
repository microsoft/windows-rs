fn main() {
    windows::build!(
        Windows::Win32::WinRT::RoActivateInstance,
        Windows::Foundation::Collections::StringMap,
    );
}
