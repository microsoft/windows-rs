fn main() {
    windows::core::build! {
        Windows::Foundation::Collections::{IIterable, IVectorView},
        Windows::Foundation::{IClosable, IStringable, Uri},
        Windows::Storage::Streams::Buffer,
        Windows::Win32::Foundation::E_BOUNDS,
        Windows::Win32::System::Com::{IPersistStream, IStream},
        Windows::Win32::System::WinRT::Composition::ISwapChainInterop,
        Windows::Win32::System::WinRT::Display::IDisplayPathInterop,
        Windows::Win32::System::WinRT::IBufferByteAccess,
        Windows::UI::Xaml::{Application, Controls::Button},
    };
}
