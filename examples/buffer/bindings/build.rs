fn main() {
    windows::build! {
        Windows::Foundation::{IMemoryBufferReference, MemoryBuffer},
        Windows::Win32::System::WinRT::IMemoryBufferByteAccess,
    };
}
