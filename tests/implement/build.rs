fn main() {
    windows::build! {
        Windows::Foundation::Collections::{IIterable, IVectorView},
        Windows::Foundation::{IClosable, IStringable, Uri},
        Windows::UI::Xaml::{Application, Controls::Button},
    };
}
