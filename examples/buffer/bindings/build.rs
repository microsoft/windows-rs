fn main() {
    windows::build! {
        Windows::Foundation::{MemoryBuffer, IMemoryBufferReference},
        Windows::Win32::System::WinRT::IMemoryBufferByteAccess,
    };
}
