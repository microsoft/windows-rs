fn main() {
    windows::build! {
        Windows::Foundation::{IClosable, IStringable},
        Windows::Foundation::Collections::{IVectorView, IIterable},
        Windows::UI::Xaml::{Application, Controls::Button},
    };
}
