fn main() {
    windows::build! {
        Windows::Foundation::{IClosable, IStringable},
        Windows::UI::Xaml::Application,
    };
}
