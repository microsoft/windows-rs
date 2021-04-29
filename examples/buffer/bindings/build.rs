fn main() {
    windows::build!(
        Windows::Foundation::*,
        Windows::Win32::WinRT::IMemoryBufferByteAccess,
    );
}
