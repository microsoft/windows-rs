fn main() {
    windows::build! {
        Windows::Foundation::{IClosable, IStringable, Uri},
        Windows::Foundation::Collections::{IVectorView, IIterable},
        Windows::UI::Xaml::{Application, Controls::Button},
    };
}
