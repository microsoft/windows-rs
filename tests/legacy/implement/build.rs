fn main() {
    windows::build! {
        Windows::Foundation::Collections::{IIterable, IVectorView},
        Windows::Foundation::{IClosable, IStringable, Uri},
        Windows::Storage::Streams::Buffer,
        Windows::Win32::Foundation::E_BOUNDS,
        Windows::Win32::System::Com::IPersistStream,
        Windows::Win32::System::WinRT::{
            IBufferByteAccess, IDisplayPathInterop, ISwapChainInterop,
        },
        Windows::UI::Xaml::{Application, Controls::Button},
    };
}
