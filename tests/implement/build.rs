fn main() {
    windows::build!(
        Windows::UI::Xaml::Application,
        Windows::Foundation::{IStringable, IClosable},
    );
}
