fn main() {
    windows::build! {
        Windows::Foundation::Collections::{IIterable, IVectorView},
        Windows::Foundation::{IClosable, IStringable, Uri},
        Windows::Win32::Foundation::E_BOUNDS,
        Windows::UI::Xaml::{Application, Controls::Button},
    };
}
