fn main() {
    windows::build!(
        Windows::Foundation::*,
        Windows::Win32::System::WinRT::IMemoryBufferByteAccess,
    );
}
